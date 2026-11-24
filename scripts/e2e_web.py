#!/usr/bin/env python3
"""End-to-end test of the web calculator in headless Chrome.

Serves a copy of ``web/`` on a local port, with the version stamped
into the ``__WASM_VERSION__`` and ``__ASSET_VERSION__`` placeholders
(as ``scripts/build_wasm.bash`` does) and the headers of
``web/_headers`` (the Content Security Policy included), and drives
Chrome through chromedriver's WebDriver HTTP API. Only the Python
standard library is used.

The WASM module must be built first::

    ./scripts/build_wasm.bash
    python3 scripts/e2e_web.py                    # chromedriver from $PATH
    python3 scripts/e2e_web.py --chromedriver "$CHROMEWEBDRIVER/chromedriver"

``--chrome-binary`` (or ``$CHROME_BIN``) picks the browser when
chromedriver does not find it on its own, for example
``/usr/bin/chromium``.

``--base-url`` tests a deployed site instead of the local copy, for
example production after a deploy::

    python3 scripts/e2e_web.py --base-url https://multiplication.rovisoft.net --wait 300

The checkout must be the deployed commit: the expected footer and
asset versions come from it. The page must name that commit (its
footer's ``data-commit``) within ``--wait`` seconds, and the
responses must carry the headers of ``web/_headers``.

Checks, in English and in Spanish: the 13597 x 8642 table equals the
golden fixture, the summary line, the address bar link and Clear, a
link with letters, a link with too many digits (answered in less than
5 s), a link whose table is too large, and a console without errors
(a Content Security Policy violation is one). The downloaded JSON
must equal the CLI's golden fixture, and the step-by-step walk must
highlight, at every step, exactly the digits its description names.
The language selector must switch the page in place (status, walk
and footer included), keep the language in the links, remember the
choice, and let ``?lang=`` win without being saved.
"""

from __future__ import annotations

import argparse
import functools
import http.server
import json
import os
import re
import shutil
import socket
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.request
from pathlib import Path
from typing import Any, Callable

REPO = Path(__file__).resolve().parent.parent
WASM = Path("asset/js/wasm/long_multiplication_wasm_bg.wasm")
STAMPED = ["index.html", "asset/js/app.js", "asset/js/i18n.js", "asset/js/steps.js", "asset/js/worker.js"]


class Failure(Exception):
    """A check that did not hold."""


# --------------------------------------------------------------------------
# Local server: web/ with the version stamped and the _headers applied.
# --------------------------------------------------------------------------


def read_version() -> str:
    match = re.search(r'^version = "([^"]+)"', (REPO / "Cargo.toml").read_text(), re.MULTILINE)
    if not match:
        raise SystemExit("could not read the version from Cargo.toml")
    return match.group(1)


def read_asset_version(version: str) -> str:
    """`<version>-<commit>`, the `?v=` of the asset URLs (see build_wasm.bash)."""
    try:
        commit = subprocess.run(
            ["git", "-C", str(REPO), "rev-parse", "--short=12", "HEAD"],
            capture_output=True, text=True, check=True,
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return version
    return f"{version}-{commit}"


def git(*args: str) -> str | None:
    try:
        return subprocess.run(
            ["git", "-C", str(REPO), *args], capture_output=True, text=True, check=True
        ).stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return None


def read_build() -> dict[str, str]:
    """The footer's build data, as build_wasm.bash stamps it."""
    build = {"commit": git("rev-parse", "HEAD") or "", "release": "", "ahead": ""}
    describe = git("describe", "--tags", "--long", "--match", "v[0-9]*") if build["commit"] else None
    if describe:
        match = re.fullmatch(r"v(.+)-(\d+)-g[0-9a-f]+", describe)
        if match:
            build["release"], build["ahead"] = match.groups()
    return build


def expected_site_version(version: str, lang: str) -> str:
    """The footer's site version, in `lang`, for the current checkout."""
    build = read_build()
    if not build["commit"]:
        return version
    short = build["commit"][:7]
    if not build["release"]:
        return f"{version}, commit {short}"
    ahead = int(build["ahead"])
    plural = "s" if ahead != 1 else ""
    if ahead == 0:
        return f"{build['release']} ({short})"
    if lang == "es":
        return f"{build['release']} y {ahead} commit{plural} más ({short})"
    return f"{build['release']} + {ahead} commit{plural} ({short})"


def check_site_version(browser: Browser, version: str, lang: str) -> None:
    expected = expected_site_version(version, lang)
    shown = browser.js("return document.getElementById('site-version').textContent")
    check(shown == expected, f"the footer's site version reads {expected!r}")
    commit = read_build()["commit"]
    if commit:
        href = browser.js("const a = document.querySelector('#site-version a'); return a && a.href")
        check(href == f"https://github.com/airvzxf/long-multiplication-calculator/commit/{commit}",
              "its commit links to GitHub")


def stage_site(web: Path, version: str, into: Path) -> Path:
    """Copy `web` into `into/web` and stamp the version placeholders."""
    if not (web / WASM).is_file():
        raise SystemExit(f"{web / WASM} is missing: run ./scripts/build_wasm.bash first")
    site = into / "web"
    shutil.copytree(web, site)
    for name in STAMPED:
        path = site / name
        text = path.read_text(encoding="utf-8")
        build = read_build()
        text = text.replace("__ASSET_VERSION__", read_asset_version(version)).replace("__WASM_VERSION__", version)
        text = text.replace("__SITE_COMMIT__", build["commit"]).replace("__SITE_RELEASE__", build["release"])
        text = text.replace("__SITE_AHEAD__", build["ahead"])
        path.write_text(text, encoding="utf-8")
    return site


def fetch(url: str) -> tuple[dict[str, str], str]:
    """GET `url`, bypassing caches; return its headers and its text."""
    request = urllib.request.Request(url, headers={"Cache-Control": "no-cache", "User-Agent": "e2e-web"})
    with urllib.request.urlopen(request, timeout=30) as response:
        return dict(response.headers.items()), response.read().decode("utf-8", errors="replace")


def wait_for_deploy(base: str, commit: str, timeout: float) -> None:
    """Wait until the page at `base` was built from `commit`."""
    deadline = time.monotonic() + timeout
    marker = f'data-commit="{commit}"'
    while True:
        try:
            seen = re.search(r'data-commit="([^"]*)"', fetch(base + "/")[1])
            if seen and seen.group(0) == marker:
                check(True, f"{base} serves commit {commit[:12]}")
                return
            found = seen.group(1) if seen else "no footer"
        except OSError as error:
            found = str(error)
        if time.monotonic() > deadline:
            raise Failure(f"{base} still serves {found!r}, not commit {commit} (waited {timeout:.0f} s)")
        time.sleep(5)


def check_remote_headers(base: str, rules: list[tuple[str, list[tuple[str, str]]]], asset: str) -> None:
    """The deployed responses carry the headers of web/_headers."""
    for path in ("/", f"/asset/js/app.js?v={asset}"):
        headers = {name.lower(): value for name, value in fetch(base + path)[0].items()}
        expected = [(name, value) for pattern, pairs in rules if matches(pattern, path.split("?", 1)[0])
                    for name, value in pairs]
        wrong = [name for name, value in expected if headers.get(name.lower()) != value]
        check(not wrong, f"{path} carries the {len(expected)} headers of web/_headers"
              + ("" if not wrong else f"; wrong or missing: {wrong}"))


def parse_headers(path: Path) -> list[tuple[str, list[tuple[str, str]]]]:
    """The rules of a Cloudflare Pages `_headers` file: (pattern, headers)."""
    rules: list[tuple[str, list[tuple[str, str]]]] = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.lstrip().startswith("#"):
            continue
        if not line[0].isspace():
            rules.append((line.strip(), []))
        elif rules:
            name, _, value = line.strip().partition(":")
            rules[-1][1].append((name.strip(), value.strip()))
    return rules


def matches(pattern: str, path: str) -> bool:
    if pattern.endswith("*"):
        return path.startswith(pattern[:-1])
    return path == pattern


class Handler(http.server.SimpleHTTPRequestHandler):
    rules: list[tuple[str, list[tuple[str, str]]]] = []
    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".js": "text/javascript",
        ".wasm": "application/wasm",
        ".manifest": "application/manifest+json",
    }

    def end_headers(self) -> None:
        path = self.path.split("?", 1)[0]
        for pattern, headers in self.rules:
            if matches(pattern, path):
                for name, value in headers:
                    self.send_header(name, value)
        super().end_headers()

    def log_message(self, format: str, *args: Any) -> None:  # noqa: A002 - stdlib signature
        pass


def serve(site: Path) -> tuple[http.server.ThreadingHTTPServer, str]:
    Handler.rules = parse_headers(site / "_headers")
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), functools.partial(Handler, directory=str(site)))
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server, f"http://127.0.0.1:{server.server_address[1]}"


# --------------------------------------------------------------------------
# A minimal WebDriver client.
# --------------------------------------------------------------------------


def free_port() -> int:
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


class Driver:
    """chromedriver, started on a free port."""

    def __init__(self, executable: str) -> None:
        self.port = free_port()
        self.process = subprocess.Popen(
            [executable, f"--port={self.port}"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
        )
        deadline = time.monotonic() + 15
        while True:
            try:
                if self.request("GET", "/status")["ready"]:
                    return
            except (OSError, Failure):
                pass
            if time.monotonic() > deadline:
                self.stop()
                raise SystemExit(f"chromedriver ({executable}) did not start")
            time.sleep(0.1)

    def request(self, method: str, path: str, body: Any = None) -> Any:
        data = None if body is None else json.dumps(body).encode()
        request = urllib.request.Request(
            f"http://127.0.0.1:{self.port}{path}",
            data=data,
            method=method,
            headers={"Content-Type": "application/json"},
        )
        try:
            with urllib.request.urlopen(request, timeout=60) as response:
                return json.load(response)["value"]
        except urllib.error.HTTPError as error:
            raise Failure(f"WebDriver {method} {path}: {error.read().decode(errors='replace')}") from None

    def stop(self) -> None:
        self.process.terminate()
        try:
            self.process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.process.kill()


class Browser:
    """One Chrome session whose interface language is `lang`."""

    def __init__(self, driver: Driver, base: str, lang: str, binary: str | None, downloads: Path) -> None:
        self.driver = driver
        self.base = base
        self.downloads = downloads
        options: dict[str, Any] = {
            "args": ["--headless=new", "--no-sandbox", "--disable-dev-shm-usage", f"--lang={lang}"],
            "prefs": {
                "intl.accept_languages": lang,
                "download.default_directory": str(downloads),
                "download.prompt_for_download": False,
            },
        }
        if binary:
            options["binary"] = binary
        capabilities = {
            "browserName": "chrome",
            "goog:loggingPrefs": {"browser": "ALL"},
            "goog:chromeOptions": options,
        }
        session = driver.request("POST", "/session", {"capabilities": {"alwaysMatch": capabilities}})
        self.session = f"/session/{session['sessionId']}"
        self.console: list[dict[str, Any]] = []

    def close(self) -> None:
        self.driver.request("DELETE", self.session)

    def open(self, path: str) -> None:
        self.driver.request("POST", f"{self.session}/url", {"url": self.base + path})

    def js(self, script: str, *args: Any) -> Any:
        return self.driver.request("POST", f"{self.session}/execute/sync", {"script": script, "args": list(args)})

    def element(self, css: str) -> str:
        found = self.driver.request("POST", f"{self.session}/element", {"using": "css selector", "value": css})
        return next(iter(found.values()))

    def click(self, css: str) -> None:
        self.driver.request("POST", f"{self.session}/element/{self.element(css)}/click", {})

    def keys(self, css: str, text: str) -> None:
        """Send keys to a focusable element that is not a text field."""
        self.driver.request("POST", f"{self.session}/element/{self.element(css)}/value", {"text": text})

    def type(self, css: str, text: str) -> None:
        element = self.element(css)
        self.driver.request("POST", f"{self.session}/element/{element}/clear", {})
        self.driver.request("POST", f"{self.session}/element/{element}/value", {"text": text})

    def collect_console(self) -> None:
        self.console.extend(self.driver.request("POST", f"{self.session}/se/log", {"type": "browser"}))

    def status(self) -> str:
        return self.js("const s = document.getElementById('status'); return s.hidden ? '' : s.textContent.trim();")

    def wait_for(self, what: str, condition: Callable[[], Any], timeout: float = 15) -> Any:
        deadline = time.monotonic() + timeout
        while True:
            value = condition()
            if value:
                return value
            if time.monotonic() > deadline:
                raise Failure(f"timed out after {timeout} s waiting for {what} (status: {self.status()!r})")
            time.sleep(0.05)

    def wait_ready(self) -> None:
        """Wait until the worker has loaded the WASM module."""
        self.wait_for(
            "the calculator to load",
            lambda: self.js("return document.getElementById('app-version').textContent !== ''"),
        )

    def wait_status(self, expected: str, timeout: float = 15) -> float:
        """Wait for the status line `expected`; return the seconds it took."""
        start = time.monotonic()
        self.wait_for(repr(expected), lambda: self.status() == expected, timeout)
        return time.monotonic() - start

    def invalid(self) -> tuple[str | None, str | None]:
        return tuple(
            self.js("return document.getElementById(arguments[0]).getAttribute('aria-invalid')", name)
            for name in ("multiplicand", "multiplier")
        )


# --------------------------------------------------------------------------
# The checks.
# --------------------------------------------------------------------------


def check(condition: bool, message: str) -> None:
    if not condition:
        raise Failure(message)
    print(f"  ✓ {message}")


def check_table(browser: Browser, fixture: str) -> None:
    table = browser.js("return document.getElementById('solution').textContent")
    # The page shows the table without the newline the CLI prints after it.
    check(table == fixture[:-1], "the 13597 x 8642 table equals tests/fixtures/13597_x_8642.txt")


def check_console(browser: Browser) -> None:
    browser.collect_console()
    errors = [entry["message"] for entry in browser.console if entry.get("level") == "SEVERE"]
    for entry in browser.console:
        if entry.get("level") == "WARNING":
            print(f"  ! console warning: {entry['message']}")
    check(not errors, "the console has no errors" + ("" if not errors else f": {errors}"))


RIGHT_ARROW = "\ue014"

# Every step, read in one call: its description and its <mark> cells.
WALK_ALL_STEPS = """
const next = document.getElementById('step-next');
const read = () => ({
    text: document.getElementById('step-text').textContent,
    counter: document.getElementById('step-counter').textContent,
    marks: [...document.querySelectorAll('#solution mark')].map((m) => [m.className, m.textContent]),
});
const steps = [read()];
while (!next.disabled) {
    next.click();
    steps.push(read());
}
return steps;
"""


def digits_of(marks: list[list[str]], kind: str) -> list[str]:
    """The number in each <mark> of `kind`, in page order (cells joined)."""
    return [re.sub(r"[\s│]", "", text) for found, text in marks if found == kind]


def check_walk(steps: list[dict[str, Any]], lang: str) -> int:
    """Every highlighted cell holds the digits the description names."""
    patterns = {
        "en": (
            r"Row (\d+): (\d) × (\d) = (\d+)\. The unit (\d) goes to column \d+ \(row \1 R\) "
            r"and the carry (\d) to column \d+ \(row \1 \^\)\.",
            r"Column \d+: ((?:\d+ \+ )*\d+) = (\d+)\.",
            r"Column \d+: its (\d+) digits add up to (\d+)\.",
            r"(?:Sub \d+|Product), column \d+: (\d) \+ (\d+) carried from column \d+ = (\d+)\.",
            r"(?:Sub \d+|Product), column 1: it keeps its units, (\d+)\.",
        ),
        "es": (
            r"Fila (\d+): (\d) × (\d) = (\d+)\. La unidad (\d) va a la columna \d+ \(fila \1 R\) "
            r"y el acarreo (\d) a la columna \d+ \(fila \1 \^\)\.",
            r"Columna \d+: ((?:\d+ \+ )*\d+) = (\d+)\.",
            r"Columna \d+: sus (\d+) dígitos suman (\d+)\.",
            r"(?:Sub \d+|Producto), columna \d+: (\d) \+ (\d+) que se lleva de la columna \d+ = (\d+)\.",
            r"(?:Sub \d+|Producto), columna 1: se quedan sus unidades, (\d+)\.",
        ),
    }[lang]
    for number, step in enumerate(steps, start=1):
        text, marks = step["text"], step["marks"]
        results, sources = digits_of(marks, "result"), digits_of(marks, "source")
        where = f"step {number} ({text!r}, marks {marks})"
        if match := re.fullmatch(patterns[0], text):
            _, a, b, product, unit, carry = match.groups()
            ok = (int(a) * int(b) == int(product) and int(unit) + 10 * int(carry) == int(product)
                  and results == [carry, unit] and digits_of(marks, "factor") == [a, b])
        elif match := re.fullmatch(patterns[1], text):
            terms = match.group(1).split(" + ")
            ok = sum(map(int, terms)) == int(match.group(2)) and sources == terms and results == [match.group(2)]
        elif match := re.fullmatch(patterns[2], text):
            ok = len(sources) == int(match.group(1)) and results == [match.group(2)]
            ok = ok and sum(map(int, sources)) == int(match.group(2))
        elif match := re.fullmatch(patterns[3], text):
            unit, carried, value = match.groups()
            ok = int(unit) + int(carried) == int(value) and results == [value]
            # In page order: the tens carried from the column on the right
            # (its line comes first), then the units of this column.
            ok = ok and sources[-1] == unit and (sources[:-1] == [carried] if carried != "0" else len(sources) == 1)
        elif match := re.fullmatch(patterns[4], text):
            ok = results == [match.group(1)] and sources == [match.group(1)]
        else:
            raise Failure(f"unexpected description at {where}")
        if not ok:
            raise Failure(f"the highlighted cells do not match the description at {where}")
    return len(steps)


def check_steps(browser: Browser, fixture: str, lang: str, labels: dict[str, str]) -> None:
    """Open the walk, check its labels and every step, then close it."""
    browser.click("#steps-toggle")
    browser.wait_for("the steps to open", lambda: browser.js("return !document.getElementById('steps').hidden"))
    check(browser.js("return document.getElementById('steps-toggle').getAttribute('aria-pressed')") == "true"
          and browser.js("return document.getElementById('steps-toggle').textContent") == labels["hide"],
          f"'{labels['show']}' opens the step-by-step walk and becomes '{labels['hide']}'")
    check(browser.js("return document.getElementById('step-next').getAttribute('aria-label')") == labels["next"],
          f"the next-step button is labelled {labels['next']!r}")
    first = browser.js("return document.getElementById('step-text').textContent")
    check(first == labels["first"], f"step 1 reads {first!r}")
    browser.keys("#solution", RIGHT_ARROW)
    check(browser.js("return document.getElementById('step-counter').textContent") == labels["second"],
          f"→ on the table moves to {labels['second']!r}")
    browser.click("#step-first")
    steps = browser.js(WALK_ALL_STEPS)
    check(steps[-1]["counter"] == labels["last"], f"the walk ends at {labels['last']!r}")
    check(check_walk(steps, lang) == len(steps), f"all {len(steps)} steps highlight the digits they describe")
    table = browser.js("return document.getElementById('solution').textContent")
    check(table == fixture[:-1], "the highlighted table still has the text of the fixture")
    browser.click("#steps-toggle")
    check(browser.js("return document.getElementById('steps').hidden && "
                     "document.querySelectorAll('#solution mark').length === 0"),
          "closing the walk removes every highlight")


def check_download(browser: Browser, name: str, expected: str) -> None:
    browser.click("#download")
    path = browser.downloads / name
    browser.wait_for(f"{name} to download", lambda: path.is_file() and path.read_text(encoding="utf-8") == expected,
                     timeout=10)
    check(True, f"Download JSON saves {name}, equal to the CLI's golden fixture")


LANGUAGE_STATE = """
return {
    lang: document.documentElement.lang,
    heading: document.querySelector('h1').textContent,
    current: [...document.querySelectorAll('a[data-lang][aria-current="true"]')].map((a) => a.dataset.lang),
    links: [...document.querySelectorAll('a[data-lang]')].map((a) => a.getAttribute('href')),
    search: location.search,
    glossary: !document.getElementById('glossary').hidden,
};
"""

HEADINGS = {"en": "Long Multiplication Calculator", "es": "Calculadora de multiplicación paso a paso"}


def check_language(browser: Browser, lang: str, search: str, why: str) -> None:
    state = browser.js(LANGUAGE_STATE)
    check(state["lang"] == lang and state["heading"] == HEADINGS[lang] and state["current"] == [lang],
          f"{why}: the page is in {lang!r} and the selector marks it")
    check(state["glossary"] == (lang == "es"),
          "the translation references of the table are " + ("shown" if lang == "es" else "hidden"))
    check(state["search"] == search, f"the address is {search or 'without a query'!r}")
    query = search.lstrip("?")
    expected = []
    for other in ("en", "es"):
        params = [part for part in query.split("&") if part and not part.startswith("lang=")]
        expected.append("?" + "&".join([*params, f"lang={other}"]))
    check(state["links"] == expected, f"the selector links keep the address: {expected}")


def check_glossary(browser: Browser, fixture: str) -> None:
    """The Spanish references translate every English term of the table."""
    terms = browser.js("return [...document.querySelectorAll('#glossary code')].map((c) => c.textContent)")
    legend = fixture.split("=======\n", 1)[1].split("\n┏", 1)[0].splitlines()
    needed = {line.split(" = ", 1)[0] for line in legend if " = " in line}
    needed |= {"Symbols", "* Replace 'n' for a number.", "Author", "E-mail", "License", "Project"}
    missing = needed - set(terms)
    unknown = [term for term in terms if term not in fixture]
    check(not missing and not unknown, f"the {len(terms)} translation references match the table's terms"
          + ("" if not missing and not unknown else f"; missing {sorted(missing)}, not in the table {unknown}"))


def language_selector(browser: Browser, fixture: str, version: str) -> None:
    """An English browser with nothing saved: switch, remember, override."""
    print("Language selector:")
    browser.open("/")
    browser.wait_ready()
    check_language(browser, "en", "", "an English browser")
    browser.type("#multiplicand", "13597")
    browser.type("#multiplier", "8642")
    browser.click("#calculate")
    browser.wait_status("Table ready: 89 rows.")
    browser.click("#steps-toggle")
    browser.click("#step-next")
    browser.click('a[data-lang="es"]')
    check_language(browser, "es", "?multiplicand=13597&multiplier=8642&lang=es", "Español")
    check(browser.status() == "Tabla lista: 89 filas.", "the status line is translated in place")
    check(browser.js("return [document.getElementById('step-counter').textContent, "
                     "document.getElementById('steps-toggle').textContent, "
                     "document.getElementById('step-text').textContent.startsWith('Fila 1: ')]")
          == ["Paso 2 de 47", "Ocultar pasos", True],
          "the open walk stays at step 2, now in Spanish")
    check_site_version(browser, version, "es")
    check_table(browser, fixture)
    browser.click("#steps-toggle")
    browser.click("#clear")
    browser.wait_for("Clear", lambda: browser.js("return document.querySelector('#solution .empty-state') !== null"))
    check(browser.js("return document.querySelector('#solution .empty-state').textContent")
          == "La tabla aparecerá aquí.", "Clear shows the empty table in Spanish")
    check_language(browser, "es", "?lang=es", "after Clear")

    browser.open("/")
    browser.wait_ready()
    check_language(browser, "es", "", "the saved choice on a new visit")
    browser.open("/?lang=en&multiplicand=13597&multiplier=8642")
    browser.wait_ready()
    browser.wait_status("Table ready: 89 rows.")
    check_language(browser, "en", "?multiplicand=13597&multiplier=8642&lang=en", "?lang=en over the saved choice")
    browser.open("/")
    browser.wait_ready()
    check_language(browser, "es", "", "?lang= is not saved")
    browser.click('a[data-lang="en"]')
    check_language(browser, "en", "?lang=en", "English")
    browser.open("/")
    browser.wait_ready()
    check_language(browser, "en", "", "the new choice is saved")


def english(browser: Browser, fixture: str, version: str) -> None:
    print("English (en-US):")
    browser.open("/")
    browser.wait_ready()
    check(browser.js("return document.documentElement.lang") == "en", "the page is in English")
    check(browser.js("return document.querySelector('h1').textContent") == "Long Multiplication Calculator",
          "the heading is in English")
    check(browser.js("return document.getElementById('app-version').textContent") == version,
          f"the footer shows version {version} for the WASM module")
    check_site_version(browser, version, "en")
    asset = read_asset_version(version)
    urls = browser.js("return [...document.querySelectorAll('script[src], link[rel=stylesheet]')]"
                      ".map((e) => e.src || e.href)")
    check(bool(urls) and all(url.endswith(f"?v={asset}") for url in urls),
          f"the asset URLs carry the commit: ?v={asset}")

    browser.type("#multiplicand", "13597")
    browser.type("#multiplier", "8642")
    browser.click("#calculate")
    browser.wait_status("Table ready: 89 rows.")
    check(True, "the status says 'Table ready: 89 rows.'")
    check_table(browser, fixture)
    check(browser.js("return location.search") == "?multiplicand=13597&multiplier=8642",
          "the address bar holds the link of the table")

    check_download(browser, "long-multiplication-13597x8642.json", read_fixture("13597_x_8642.json"))
    check_steps(browser, fixture, "en", {
        "show": "Step by step",
        "hide": "Hide steps",
        "next": "Next step",
        "first": "Row 1: 7 × 2 = 14. The unit 4 goes to column 1 (row 1 R) and the carry 1 to column 2 (row 1 ^).",
        "second": "Step 2 of 47",
        "last": "Step 47 of 47",
    })

    browser.click("#steps-toggle")
    browser.click("#clear")
    browser.wait_for("Clear to reset the page", lambda: browser.js(
        "return document.querySelector('#solution .empty-state') !== null"))
    check(browser.js("return location.search") == "" and browser.status() == "",
          "Clear empties the table, the status and the link")
    check(browser.js("return document.getElementById('steps').hidden && "
                     "document.getElementById('download').disabled && "
                     "document.getElementById('steps-toggle').disabled"),
          "Clear closes the walk and disables Download JSON and Step by step")

    nines = "9" * 12
    browser.open(f"/?multiplicand={nines}&multiplier={nines}")
    browser.wait_ready()
    rows = read_fixture("999999999999_x_999999999999.txt")[:-1].count("\n")
    browser.wait_status(f"Table ready: {rows} rows.")
    check_download(browser, f"long-multiplication-{nines}x{nines}.json",
                   read_fixture("999999999999_x_999999999999.json"))
    check_steps(browser, read_fixture("999999999999_x_999999999999.txt"), "en", {
        "show": "Step by step",
        "hide": "Hide steps",
        "next": "Next step",
        "first": "Row 1: 9 × 9 = 81. The unit 1 goes to column 1 (row 1 R) and the carry 8 to column 2 (row 1 ^).",
        "second": "Step 2 of 432",
        "last": "Step 432 of 432",
    })

    browser.open("/?multiplicand=12a3&multiplier=45")
    browser.wait_ready()
    browser.wait_status("Both values must contain digits 0-9 only.")
    check(browser.invalid() == ("true", None), "a link with letters marks only the multiplicand as invalid")

    digits = "7" * 2000
    start = time.monotonic()
    browser.open(f"/?multiplicand={digits}&multiplier=3")
    browser.wait_ready()
    browser.wait_status(
        "The web calculator accepts up to 1000 digits per number. Use the desktop CLI for bigger numbers.",
        timeout=5,
    )
    elapsed = time.monotonic() - start
    check(elapsed < 5, f"a link with 2000 digits is refused in {elapsed:.2f} s (< 5 s)")
    check(browser.invalid() == ("true", None), "the 2000-digit operand is the one marked invalid")
    language_selector(browser, fixture, version)
    check_console(browser)


def spanish(browser: Browser, fixture: str, version: str) -> None:
    print("Spanish (es):")
    browser.open("/?multiplicand=13597&multiplier=8642")
    browser.wait_ready()
    browser.wait_status("Tabla lista: 89 filas.")
    check(browser.js("return document.documentElement.lang") == "es", "the page is in Spanish")
    check(browser.js("return document.querySelector('h1').textContent")
          == "Calculadora de multiplicación paso a paso", "the heading is in Spanish")
    check(True, "a shared link calculates the table: 'Tabla lista: 89 filas.'")
    check_site_version(browser, version, "es")
    check_table(browser, fixture)
    check_glossary(browser, fixture)
    check(browser.js("return document.getElementById('download').textContent") == "Descargar JSON",
          "the download button reads 'Descargar JSON'")
    check_steps(browser, fixture, "es", {
        "show": "Paso a paso",
        "hide": "Ocultar pasos",
        "next": "Paso siguiente",
        "first": "Fila 1: 7 × 2 = 14. La unidad 4 va a la columna 1 (fila 1 R) y el acarreo 1 a la columna 2 (fila 1 ^).",
        "second": "Paso 2 de 47",
        "last": "Paso 47 de 47",
    })

    nines = "9" * 200
    browser.open(f"/?multiplicand={nines}&multiplier={nines}")
    browser.wait_ready()
    status = browser.wait_for("the size error", lambda: browser.status().startswith("La tabla ocuparía") and browser.status())
    check(
        re.fullmatch(r"La tabla ocuparía unos [\d.,]+ MiB \(límite 32,0 MiB\)\. "
                     r"Usa la versión de línea de comandos para tablas más grandes\.", status) is not None,
        f"200 nines are refused with a Spanish size error: {status!r}",
    )
    browser.open("/?lang=en")
    browser.wait_ready()
    check_language(browser, "en", "?lang=en", "a Spanish browser with ?lang=en")
    browser.open("/")
    browser.wait_ready()
    check_language(browser, "es", "", "without ?lang=, the browser's language")
    check_console(browser)


def read_fixture(name: str) -> str:
    return (REPO / "tests/fixtures" / name).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--chromedriver", default=os.environ.get("CHROMEDRIVER") or shutil.which("chromedriver"))
    parser.add_argument("--chrome-binary", default=os.environ.get("CHROME_BIN"))
    parser.add_argument("--web", type=Path, default=REPO / "web")
    parser.add_argument("--base-url", help="test this deployed site (built from the checked-out commit) "
                        "instead of serving --web locally")
    parser.add_argument("--wait", type=float, default=0, metavar="SECONDS",
                        help="with --base-url, how long to wait for the site to serve the checked-out commit")
    args = parser.parse_args()
    if not args.chromedriver:
        raise SystemExit("chromedriver not found: pass --chromedriver or set $CHROMEDRIVER")

    version = read_version()
    fixture = read_fixture("13597_x_8642.txt")
    with tempfile.TemporaryDirectory(prefix="e2e-web-") as tmp:
        server = None
        if args.base_url:
            base = args.base_url.rstrip("/")
            commit = read_build()["commit"]
            if not commit:
                raise SystemExit("--base-url needs a git checkout of the deployed commit")
            print(f"Testing {base} (version {version}, commit {commit[:12]})")
            try:
                wait_for_deploy(base, commit, args.wait)
                check_remote_headers(base, parse_headers(REPO / "web/_headers"), read_asset_version(version))
            except Failure as failure:
                print(f"  ✗ {failure}", file=sys.stderr)
                return 1
        else:
            server, base = serve(stage_site(args.web, version, Path(tmp)))
            print(f"Serving {args.web} (version {version}) at {base}")
        driver = Driver(args.chromedriver)
        try:
            for lang, scenario in (("en-US", lambda b: english(b, fixture, version)), ("es", lambda b: spanish(b, fixture, version))):
                downloads = Path(tmp) / f"downloads-{lang}"
                downloads.mkdir()
                browser = Browser(driver, base, lang, args.chrome_binary, downloads)
                try:
                    scenario(browser)
                finally:
                    browser.close()
        except Failure as failure:
            print(f"  ✗ {failure}", file=sys.stderr)
            return 1
        finally:
            driver.stop()
            if server:
                server.shutdown()
    print("All web checks passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
