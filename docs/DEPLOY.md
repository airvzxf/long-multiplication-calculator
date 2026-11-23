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
   the WASM module into `web/asset/js/wasm/` and stamp the package
   version into every asset URL (`?v=<version>`).
3. Publishes the `web/` directory to Cloudflare Pages with
   `cloudflare/wrangler-action`.

A push whose CI fails is never deployed. Deploys run one at a time
(`concurrency` with `queue: max`), and a waiting deploy is never
cancelled.

Total time: typically under two minutes.

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

After the workflow finishes:

1. The Actions run page shows the deploy URL.
2. Open <https://multiplication.rovisoft.net> and run a quick
   multiplication (e.g. `12345 × 6789`) to confirm WASM loaded and
   the table renders.
3. Open the browser dev tools → **Network** → reload. The page
   should make zero outbound network calls for the calculation
   (only the static asset requests for HTML, CSS, JS, and WASM).

## Caching

`web/_headers` caches everything under `/asset/` for a year
(`immutable`). That is safe only because every asset URL carries the
release version (`?v=<version>`), including the `.wasm` binary, which
`worker.js` loads by an explicit versioned URL. If you add a new
asset, reference it with `?v=__WASM_VERSION__` and add its file to
`STAMP_FILES` in `scripts/build_wasm.bash` if the reference lives in
a new file. The HTML is revalidated on every visit.

## Local testing of the static site

```bash
./scripts/build_wasm.bash
cd web
python3 -m http.server 8000
# open http://localhost:8000
```
