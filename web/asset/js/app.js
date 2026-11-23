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

import { formatBytes, formatNumber, t, translatePage } from './i18n.js?v=__WASM_VERSION__';

const $ = (id) => document.getElementById(id);

const form = $('calculator');
const multiplicand = $('multiplicand');
const multiplier = $('multiplier');
const calculateBtn = $('calculate');
const copyBtn = $('copy');
const status = $('status');
const statusText = $('status-text');
const solution = $('solution');
const appVersion = $('app-version');

const DIGIT_RE = /^[0-9]+$/;

let worker = null;
let loading = false;
let sharedLinkPending = window.location.search.length > 1;
let limits = { maxDigits: Infinity };
let lastResult = '';
let pendingId = 0;
let pendingInputs = null;

translatePage();
showEmptyState();
startWorker();

form.addEventListener('submit', (ev) => {
    ev.preventDefault();
    run();
});

form.addEventListener('reset', () => {
    // Wait for the browser to clear the inputs, then reset the UI.
    queueMicrotask(() => {
        lastResult = '';
        showEmptyState();
        copyBtn.disabled = true;
        setInvalid(multiplicand, false);
        setInvalid(multiplier, false);
        hideStatus();
        clearSharedLink();
        if (pendingId !== 0) {
            // Stop the calculation in progress: a fresh worker is cheaper
            // than waiting for a table nobody wants any more.
            pendingId = 0;
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
        showStatus(t('status.copyFailed', { message: err?.message ?? err }), 'error');
    }
});

function startWorker() {
    calculateBtn.disabled = true;
    loading = true;
    showStatus(t('status.loading'), 'info');
    worker = new Worker(new URL('./worker.js?v=__WASM_VERSION__', import.meta.url), { type: 'module' });
    worker.addEventListener('message', onWorkerMessage);
    worker.addEventListener('error', (ev) => {
        loading = false;
        showStatus(t('status.loadFailed', { message: ev.message ?? 'unknown error' }), 'error');
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
            showStatus(t('status.loadFailed', { message: data.message }), 'error');
            break;
        case 'result':
            if (data.id !== pendingId) {
                return;
            }
            finish();
            lastResult = data.table;
            solution.textContent = data.table;
            copyBtn.disabled = false;
            // Announce a short summary, never the table itself: a
            // screen reader must not read megabytes of text aloud.
            showStatus(t('status.ready', { rows: formatNumber(data.rows) }), 'info');
            updateSharedLink(pendingInputs);
            break;
        case 'error':
            if (data.id !== pendingId) {
                return;
            }
            finish();
            showStatus(errorMessage(data), 'error');
            break;
    }
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
    url.search = new URLSearchParams(inputs).toString();
    window.history.replaceState(null, '', url);
}

function clearSharedLink() {
    if (window.location.search) {
        const url = new URL(window.location.href);
        url.search = '';
        window.history.replaceState(null, '', url);
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
        showStatus(t('error.digitsOnly'), 'error');
        return;
    }

    const aTooLong = a.length > limits.maxDigits;
    const bTooLong = b.length > limits.maxDigits;
    setInvalid(multiplicand, aTooLong);
    setInvalid(multiplier, bTooLong);
    if (aTooLong || bTooLong) {
        showStatus(t('error.tooLong', { max: limits.maxDigits }), 'error');
        return;
    }

    pendingId += 1;
    pendingInputs = { multiplicand: a, multiplier: b };
    calculateBtn.disabled = true;
    copyBtn.disabled = true;
    showStatus(t('status.calculating'), 'info');
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

function showEmptyState() {
    const empty = document.createElement('span');
    empty.className = 'empty-state';
    empty.textContent = t('solution.empty');
    solution.replaceChildren(empty);
}

function showStatus(message, kind) {
    statusText.textContent = message;
    status.dataset.kind = kind;
    status.hidden = false;
}

function hideStatus() {
    status.hidden = true;
    statusText.textContent = '';
}
