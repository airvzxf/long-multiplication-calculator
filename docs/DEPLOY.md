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

Every push to `main` triggers the `deploy.yml` workflow, which:

1. Installs the Rust toolchain (with the `wasm32-unknown-unknown`
   target) and `wasm-pack`.
2. Runs `scripts/build_wasm.sh --no-opt` to compile the WASM module
   into `web/asset/js/wasm/`.
3. Publishes the `web/` directory to Cloudflare Pages.

Total time: typically under two minutes.

## Manual deploys

To deploy without pushing to `main`:

1. **Actions** → **Deploy to Cloudflare Pages** → **Run workflow**.
2. The build runs against the current default branch.

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

## Local testing of the static site

```bash
./scripts/build_wasm.sh
cd web
python3 -m http.server 8000
# open http://localhost:8000
```
