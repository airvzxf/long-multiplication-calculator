// Long Multiplication Calculator - frontend.
//
// The calculation runs in a Web Worker (`worker.js`) that loads the
// Rust-compiled WebAssembly module. No network round-trips: every
// computation happens in the browser, and a big table never blocks
// the page. `Clear` cancels a calculation in progress.
//
// A calculation can be shared as a link:
// `/?multiplicand=13597&multiplier=8642` fills the form and runs it
// once the calculator is ready, with the same checks and limits as
// the form. After every successful calculation the address bar is
// updated to the link of that table.
//
// The language selector (`[data-lang]` links in the header) switches
// the page between English and Spanish in place and saves the choice
// (see i18n.js). An explicit language stays in the links the page
// builds: `/?multiplicand=13597&multiplier=8642&lang=es`.
//
// Every table can be downloaded as JSON (the same bytes as the CLI's
// `--format json`) and walked through step by step (`steps.js`). The
// worker writes the JSON only when one of them asks for it.

import { explicit, formatBytes, formatNumber, lang, setLanguage, t, translatePage } from './i18n.js?v=__ASSET_VERSION__';
import { StepView, Steps } from './steps.js?v=__ASSET_VERSION__';

const $ = (id) => document.getElementById(id);

const form = $('calculator');
const multiplicand = $('multiplicand');
const multiplier = $('multiplier');
const calculateBtn = $('calculate');
const copyBtn = $('copy');
const downloadBtn = $('download');
const stepsBtn = $('steps-toggle');
const stepsPanel = $('steps');
const stepFirst = $('step-first');
const stepPrev = $('step-prev');
const stepNext = $('step-next');
const stepLast = $('step-last');
const stepCounter = $('step-counter');
const stepSection = $('step-section');
const stepText = $('step-text');
const status = $('status');
const statusText = $('status-text');
const solution = $('solution');
const appVersion = $('app-version');
const languageLinks = document.querySelectorAll('a[data-lang]');
// The package version stamped into the footer, before showSiteVersion
// replaces the text.
const siteVersionText = $('site-version')?.textContent.trim() ?? '';

const DIGIT_RE = /^[0-9]+$/;

let worker = null;
let loading = false;
let sharedLinkPending = window.location.search.length > 1;
let limits = { maxDigits: Infinity };
let lastResult = '';
let pendingId = 0;
let pendingInputs = null;
// The operands of the table on screen, and its JSON once asked for.
let shownInputs = null;
let shownJson = null;
let jsonId = 0;
const jsonRequests = new Map();
// The step-by-step walk, while it is open.
let walk = null;
// The status line, as a function so it can be shown in another language.
let statusMessage = null;

translatePage();
updateLanguageLinks();
showSiteVersion();
showEmptyState();
startWorker();

for (const link of languageLinks) {
    link.addEventListener('click', (ev) => {
        if (ev.button !== 0 || ev.ctrlKey || ev.metaKey || ev.shiftKey || ev.altKey) {
            return; // A new tab or window follows the link itself.
        }
        ev.preventDefault();
        switchLanguage(link.dataset.lang);
    });
}

form.addEventListener('submit', (ev) => {
    ev.preventDefault();
    run();
});

form.addEventListener('reset', () => {
    // Wait for the browser to clear the inputs, then reset the UI.
    queueMicrotask(() => {
        lastResult = '';
        closeSteps();
        forgetTable();
        showEmptyState();
        copyBtn.disabled = true;
        setInvalid(multiplicand, false);
        setInvalid(multiplier, false);
        hideStatus();
        clearSharedLink();
        if (pendingId !== 0 || jsonRequests.size > 0) {
            // Stop the calculation in progress: a fresh worker is cheaper
            // than waiting for a table nobody wants any more.
            pendingId = 0;
            rejectJsonRequests();
            worker.terminate();
            startWorker();
        }
    });
});

for (const input of [multiplicand, multiplier]) {
    input.addEventListener('input', () => setInvalid(input, false));
}

copyBtn.addEventListener('click', async () => {
    if (!lastResult) {
        return;
    }
    try {
        await navigator.clipboard.writeText(lastResult);
        copyBtn.textContent = t('form.copied');
        setTimeout(() => {
            copyBtn.textContent = t('form.copy');
        }, 1500);
    } catch (err) {
        showStatus(() => t('status.copyFailed', { message: err?.message ?? err }), 'error');
    }
});

downloadBtn.addEventListener('click', async () => {
    const inputs = shownInputs;
    let json;
    try {
        json = await tableJson();
    } catch (err) {
        if (err) {
            showStatus(() => t('status.jsonFailed', { message: err.message ?? err }), 'error');
        }
        return;
    }
    const link = document.createElement('a');
    link.href = URL.createObjectURL(new Blob([json], { type: 'application/json' }));
    link.download = jsonFileName(inputs);
    document.body.append(link);
    link.click();
    link.remove();
    setTimeout(() => URL.revokeObjectURL(link.href), 0);
});

stepsBtn.addEventListener('click', async () => {
    if (walk) {
        closeSteps();
        return;
    }
    const table = lastResult;
    let json;
    stepsBtn.disabled = true;
    try {
        json = await tableJson();
    } catch (err) {
        if (err) {
            showStatus(() => t('status.jsonFailed', { message: err.message ?? err }), 'error');
        }
        return;
    } finally {
        stepsBtn.disabled = !lastResult;
    }
    if (table !== lastResult) {
        return;
    }
    openSteps(JSON.parse(json), table);
});

stepFirst.addEventListener('click', () => goToStep(0));
stepPrev.addEventListener('click', () => goToStep(walk.index - 1));
stepNext.addEventListener('click', () => goToStep(walk.index + 1));
stepLast.addEventListener('click', () => goToStep(walk.steps.count - 1));
stepSection.addEventListener('change', () => goToStep(walk.steps.sections[Number(stepSection.value)].start));

solution.addEventListener('keydown', (ev) => {
    if (!walk) {
        return;
    }
    const moves = {
        ArrowLeft: walk.index - 1,
        ArrowRight: walk.index + 1,
        Home: 0,
        End: walk.steps.count - 1,
    };
    if (ev.key in moves && !ev.altKey && !ev.ctrlKey && !ev.metaKey) {
        ev.preventDefault();
        goToStep(moves[ev.key]);
    }
});

function startWorker() {
    calculateBtn.disabled = true;
    loading = true;
    showStatus(() => t('status.loading'), 'info');
    worker = new Worker(new URL('./worker.js?v=__ASSET_VERSION__', import.meta.url), { type: 'module' });
    worker.addEventListener('message', onWorkerMessage);
    worker.addEventListener('error', (ev) => {
        loading = false;
        showStatus(() => t('status.loadFailed', { message: ev.message ?? 'unknown error' }), 'error');
    });
}

function onWorkerMessage({ data }) {
    switch (data.type) {
        case 'ready':
            limits = data;
            calculateBtn.disabled = false;
            if (loading) {
                loading = false;
                hideStatus();
            }
            if (appVersion) {
                // The footer shows the version of the binary that is
                // actually running, not a hardcoded number.
                appVersion.textContent = data.version;
            }
            if (sharedLinkPending) {
                sharedLinkPending = false;
                openSharedLink();
            }
            break;
        case 'fatal':
            loading = false;
            showStatus(() => t('status.loadFailed', { message: data.message }), 'error');
            break;
        case 'result':
            if (data.id !== pendingId) {
                return;
            }
            finish();
            closeSteps();
            lastResult = data.table;
            solution.textContent = data.table;
            copyBtn.disabled = false;
            shownInputs = pendingInputs;
            shownJson = null;
            downloadBtn.disabled = false;
            stepsBtn.disabled = false;
            // Announce a short summary, never the table itself: a
            // screen reader must not read megabytes of text aloud.
            showStatus(() => t('status.ready', { rows: formatNumber(data.rows) }), 'info');
            updateSharedLink(pendingInputs);
            break;
        case 'error':
            if (data.id !== pendingId) {
                return;
            }
            finish();
            showStatus(() => errorMessage(data), 'error');
            break;
        case 'json':
        case 'json-error': {
            const request = jsonRequests.get(data.id);
            if (!request) {
                return;
            }
            jsonRequests.delete(data.id);
            if (data.type === 'json') {
                request.resolve(data.json);
            } else {
                request.reject(new Error(data.message));
            }
            break;
        }
    }
}

/**
 * The JSON of the table on screen, written by the worker the first
 * time it is needed. Rejects with `null` when the table is cleared or
 * replaced meanwhile.
 */
async function tableJson() {
    const inputs = shownInputs;
    if (!inputs) {
        throw null;
    }
    if (shownJson?.inputs === inputs) {
        return shownJson.json;
    }
    jsonId += 1;
    const id = jsonId;
    const json = await new Promise((resolve, reject) => {
        jsonRequests.set(id, { resolve, reject });
        worker.postMessage({ type: 'json', id, ...inputs });
    });
    if (shownInputs !== inputs) {
        throw null;
    }
    shownJson = { inputs, json };
    return json;
}

function rejectJsonRequests() {
    for (const request of jsonRequests.values()) {
        request.reject(null);
    }
    jsonRequests.clear();
}

/** `long-multiplication-13597x8642.json`, or the CLI's default name for long operands. */
function jsonFileName(inputs) {
    if (inputs && inputs.multiplicand.length + inputs.multiplier.length <= 80) {
        return `long-multiplication-${inputs.multiplicand}x${inputs.multiplier}.json`;
    }
    return 'long-multiplication-output.json';
}

function forgetTable() {
    shownInputs = null;
    shownJson = null;
    downloadBtn.disabled = true;
    stepsBtn.disabled = true;
}

function openSteps(process, table, { index = 0, focus = true } = {}) {
    const steps = new Steps(process, table);
    walk = { steps, view: new StepView(solution, table), index: -1, process, table };
    stepSection.replaceChildren(
        ...steps.sections.map((section, index) => new Option(section.label, String(index))),
    );
    stepsPanel.hidden = false;
    stepsBtn.setAttribute('aria-pressed', 'true');
    stepsBtn.textContent = t('form.stepsHide');
    goToStep(index);
    if (focus) {
        solution.focus({ preventScroll: true });
    }
}

function closeSteps() {
    if (!walk) {
        return;
    }
    walk.view.destroy();
    walk = null;
    stepsPanel.hidden = true;
    stepText.textContent = '';
    stepsBtn.setAttribute('aria-pressed', 'false');
    stepsBtn.textContent = t('form.steps');
}

function goToStep(index) {
    const { steps, view } = walk;
    const next = Math.max(0, Math.min(index, steps.count - 1));
    if (next === walk.index) {
        return;
    }
    walk.index = next;
    const step = steps.describe(next);
    const focus = view.show(step.marks);
    focus?.scrollIntoView({ block: 'nearest', inline: 'nearest' });
    stepText.textContent = step.text;
    stepCounter.textContent = t('steps.counter', {
        n: formatNumber(next + 1),
        total: formatNumber(steps.count),
    });
    stepSection.value = String(steps.sectionOf(next));
    stepFirst.disabled = stepPrev.disabled = next === 0;
    stepNext.disabled = stepLast.disabled = next === steps.count - 1;
}

/** Fill the form from `?multiplicand=…&multiplier=…` and calculate it. */
function openSharedLink() {
    const params = new URLSearchParams(window.location.search);
    if (!params.has('multiplicand') && !params.has('multiplier')) {
        return;
    }
    multiplicand.value = params.get('multiplicand') ?? '';
    multiplier.value = params.get('multiplier') ?? '';
    run();
}

function updateSharedLink(inputs) {
    if (!inputs) {
        return;
    }
    const url = new URL(window.location.href);
    url.search = linkQuery(inputs);
    window.history.replaceState(null, '', url);
    updateLanguageLinks();
}

/** Drop the table from the address; an explicit language stays. */
function clearSharedLink() {
    const url = new URL(window.location.href);
    url.search = linkQuery(null);
    if (url.href !== window.location.href) {
        window.history.replaceState(null, '', url);
        updateLanguageLinks();
    }
}

function run() {
    const a = multiplicand.value.trim();
    const b = multiplier.value.trim();

    const aDigits = DIGIT_RE.test(a);
    const bDigits = DIGIT_RE.test(b);
    setInvalid(multiplicand, !aDigits);
    setInvalid(multiplier, !bDigits);
    if (!aDigits || !bDigits) {
        showStatus(() => t('error.digitsOnly'), 'error');
        return;
    }

    const aTooLong = a.length > limits.maxDigits;
    const bTooLong = b.length > limits.maxDigits;
    setInvalid(multiplicand, aTooLong);
    setInvalid(multiplier, bTooLong);
    if (aTooLong || bTooLong) {
        showStatus(() => t('error.tooLong', { max: limits.maxDigits }), 'error');
        return;
    }

    pendingId += 1;
    pendingInputs = { multiplicand: a, multiplier: b };
    calculateBtn.disabled = true;
    copyBtn.disabled = true;
    closeSteps();
    forgetTable();
    showStatus(() => t('status.calculating'), 'info');
    worker.postMessage({ id: pendingId, multiplicand: a, multiplier: b });
}

/** The message for an error reported by the worker, in the page language. */
function errorMessage(data) {
    if (data.code === 'too-large') {
        return t('error.tooLarge', {
            estimated: formatBytes(data.estimated),
            max: formatBytes(data.max),
        });
    }
    return data.message;
}

function finish() {
    pendingId = 0;
    calculateBtn.disabled = false;
}

function setInvalid(input, invalid) {
    if (invalid) {
        input.setAttribute('aria-invalid', 'true');
    } else {
        input.removeAttribute('aria-invalid');
    }
}

/**
 * The footer's site version: the last release, how many commits the
 * page is ahead of it and a link to the commit, for example
 * "2.4.0 + 1 commit (009d485)". `build_wasm.bash` stamps the data;
 * without it (a build outside git) the plain version stays.
 */
function showSiteVersion() {
    const site = $('site-version');
    const { commit = '', release = '', ahead = '', commitUrl = '' } = site?.dataset ?? {};
    if (!/^[0-9a-f]{40}$/.test(commit)) {
        return;
    }
    let template;
    if (release && /^\d+$/.test(ahead)) {
        const count = Number(ahead);
        const key = count === 0 ? 'footer.release' : count === 1 ? 'footer.aheadOne' : 'footer.ahead';
        template = t(key, { version: release, count: formatNumber(count) });
    } else {
        template = t('footer.build', { version: siteVersionText });
    }
    const link = document.createElement('a');
    link.href = commitUrl;
    link.textContent = commit.slice(0, 7);
    link.title = t('footer.commitTitle', { sha: commit });
    link.target = '_blank';
    link.rel = 'noopener noreferrer';
    const [before, after = ''] = template.split('{commit}');
    site.replaceChildren(before, link, after);
}

function showEmptyState() {
    const empty = document.createElement('span');
    empty.className = 'empty-state';
    empty.textContent = t('solution.empty');
    solution.replaceChildren(empty);
}

/** Show the status line; `message` returns its text in the current language. */
function showStatus(message, kind) {
    statusMessage = message;
    statusText.textContent = message();
    status.dataset.kind = kind;
    status.hidden = false;
}

function hideStatus() {
    statusMessage = null;
    status.hidden = true;
    statusText.textContent = '';
}

/** Show the page in `next` (`en` or `es`), chosen in the selector. */
function switchLanguage(next) {
    if (next === lang && explicit) {
        return;
    }
    setLanguage(next);
    const url = new URL(window.location.href);
    url.searchParams.set('lang', next);
    window.history.replaceState(null, '', url);
    updateLanguageLinks();
    showSiteVersion();
    if (solution.querySelector('.empty-state')) {
        showEmptyState();
    }
    copyBtn.textContent = t('form.copy');
    stepsBtn.textContent = t(walk ? 'form.stepsHide' : 'form.steps');
    if (walk) {
        // Rebuild the walk: its sections and descriptions are text.
        const { process, table, index } = walk;
        closeSteps();
        openSteps(process, table, { index, focus: false });
    }
    if (statusMessage) {
        statusText.textContent = statusMessage();
    }
}

/**
 * The query string of a link to `inputs` (none: no table), with the
 * language when it was chosen rather than taken from the browser.
 */
function linkQuery(inputs) {
    const params = new URLSearchParams(inputs ?? {});
    if (explicit) {
        params.set('lang', lang);
    }
    return params.toString();
}

/** Point each selector link at the current address in its language. */
function updateLanguageLinks() {
    for (const link of languageLinks) {
        const params = new URLSearchParams(window.location.search);
        params.set('lang', link.dataset.lang);
        link.href = `?${params}`;
    }
}
