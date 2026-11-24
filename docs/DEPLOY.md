# Deployment

This document explains how to deploy `long-multiplication-calculator`
to Cloudflare Pages using the GitHub Actions workflow in
`.github/workflows/deploy.yml`.

## One-time setup

### 1. Create the Cloudflare Pages project

1. Sign in to <https://dash.cloudflare.com/>.
2. **Workers & Pages** → **Create application** → **Pages** → **Upload
   assets** (we use direct upload, not the Git integration, because
   we want to deploy the *built* `web/` directory including the
   compiled WASM).
3. Project name: `long-multiplication-calculator`.
4. Skip the build step; the deploy workflow pushes the pre-built
   `web/` directory.
5. (Optional) Add the custom domain `multiplication.rovisoft.net`.

### 2. Create the Cloudflare API token

1. <https://dash.cloudflare.com/profile/api-tokens> → **Create Token**.
2. Use the **Edit Cloudflare Pages** template, restricted to the
   `long-multiplication-calculator` project.
3. Copy the token.

### 3. Find your Cloudflare account ID

1. In the Cloudflare dashboard, click any **Workers & Pages** project.
2. Scroll to the right sidebar; the **Account ID** is shown there.

### 4. Add the secrets to GitHub

In your GitHub repository: **Settings** → **Secrets and variables**
→ **Actions** → **New repository secret**:

| Name | Value |
| --- | --- |
| `CLOUDFLARE_API_TOKEN` | the token from step 2 |
| `CLOUDFLARE_ACCOUNT_ID` | the ID from step 3 |

## Automatic deploys

The Pages project uses direct upload (it is not connected to Git).
Every push to `main` runs the `CI` workflow; when CI succeeds, the
`deploy.yml` workflow starts (`workflow_run`) and deploys that exact
commit:

1. Installs the Rust toolchain (with the `wasm32-unknown-unknown`
   target), `wasm-pack` (pinned version, via `taiki-e/install-action`)
   and `wasm-opt` (`binaryen`).
2. Runs `scripts/build_wasm.bash --no-revert` to compile and optimise
   the WASM module into `web/asset/js/wasm/`, stamp the version and
   the commit into every asset URL (`?v=<version>-<commit>`), and the
   version and `git describe --tags` into the footer: it reads, for
   example, "Site version: 2.4.0 + 1 commit (009d485)", with a link
   to the commit. The checkout fetches the whole history
   (`fetch-depth: 0`) so the tags are there.
3. Publishes the `web/` directory to Cloudflare Pages with
   `cloudflare/wrangler-action`.
4. Verifies production: `scripts/e2e_web.py --base-url
   https://multiplication.rovisoft.net --wait 300` waits until the
   page names the deployed commit (the footer's `data-commit`),
   checks that `/` and an asset carry the headers of `web/_headers`,
   and runs the whole end-to-end test against the live site. A
   failure marks the deploy run as failed (the site is already
   published; roll back if needed, see below).

A push whose CI fails is never deployed. Deploys run one at a time
(`concurrency` with `queue: max`), and a waiting deploy is never
cancelled.

Total time: typically three to four minutes, half of it the
end-to-end test of production.

## Manual deploys

To deploy without pushing to `main`:

1. **Actions** → **Deploy to Cloudflare Pages** → **Run workflow**.
2. The build runs against the current default branch (without
   waiting for CI, so use it only on a commit that already passed).

From a terminal, `npx wrangler pages deployment list --project-name
long-multiplication-calculator` lists the deploys.

## Rollback

In the Cloudflare dashboard: **Workers & Pages** →
`long-multiplication-calculator` → **Deployments**. Each successful
deploy is listed with a timestamp. Click any prior deployment and
choose **Rollback to this deploy**.

## Verifying a deploy

The workflow's last step, "End-to-end test of production", already
checks the live site. To run the same check by hand, from a checkout
of the deployed commit:

```bash
python3 scripts/e2e_web.py --base-url https://multiplication.rovisoft.net \
    --chrome-binary /usr/bin/chromium   # if chromedriver does not find Chrome
```

By hand you can also open the browser dev tools → **Network** →
reload: the page makes no outbound calls for the calculation (only
the static asset requests for HTML, CSS, JS, and WASM).

## Caching

`web/_headers` caches everything under `/asset/` for a year
(`immutable`). That is safe only because every asset URL carries the
version and the commit it was built from (`?v=<version>-<commit>`,
for example `?v=2.4.0-15e5355a1b2c`), including the `.wasm` binary,
which `worker.js` loads by an explicit versioned URL. With the
version alone, a deploy of `main` between two releases kept the old
URLs, and the Cloudflare edge served a cached `worker.js` next to the
new page. If you add a new asset, reference it with
`?v=__ASSET_VERSION__` and add its file to
`STAMP_FILES` in `scripts/build_wasm.bash` if the reference lives in
a new file. The HTML is revalidated on every visit.

## Local testing of the static site

```bash
./scripts/build_wasm.bash
cd web
python3 -m http.server 8000
# open http://localhost:8000
```
