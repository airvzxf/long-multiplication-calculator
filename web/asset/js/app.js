// Long Multiplication Calculator - frontend.
//
// The calculation runs in a Web Worker (`worker.js`) that loads the
// Rust-compiled WebAssembly module. No network round-trips: every
// computation happens in the browser, and a big table never blocks
// the page. `Clear` cancels a calculation in progress.

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
const EMPTY_STATE = '<span class="empty-state">The table will appear here.</span>';

let worker = null;
let limits = { maxDigits: Infinity };
let lastResult = '';
let pendingId = 0;

startWorker();

form.addEventListener('submit', (ev) => {
    ev.preventDefault();
    run();
});

form.addEventListener('reset', () => {
    // Wait for the browser to clear the inputs, then reset the UI.
    queueMicrotask(() => {
        if (pendingId !== 0) {
            // Stop the calculation in progress: a fresh worker is cheaper
            // than waiting for a table nobody wants any more.
            pendingId = 0;
            worker.terminate();
            startWorker();
        }
        lastResult = '';
        solution.innerHTML = EMPTY_STATE;
        copyBtn.disabled = true;
        hideStatus();
    });
});

copyBtn.addEventListener('click', async () => {
    if (!lastResult) {
        return;
    }
    try {
        await navigator.clipboard.writeText(lastResult);
        const original = copyBtn.textContent;
        copyBtn.textContent = 'Copied!';
        setTimeout(() => {
            copyBtn.textContent = original;
        }, 1500);
    } catch (err) {
        showStatus(`Could not copy to clipboard: ${err?.message ?? err}`, 'error');
    }
});

function startWorker() {
    calculateBtn.disabled = true;
    worker = new Worker(new URL('./worker.js?v=__WASM_VERSION__', import.meta.url), { type: 'module' });
    worker.addEventListener('message', onWorkerMessage);
    worker.addEventListener('error', (ev) => {
        showStatus(`Failed to load the calculator: ${ev.message ?? 'unknown error'}`, 'error');
    });
}

function onWorkerMessage({ data }) {
    switch (data.type) {
        case 'ready':
            limits = data;
            calculateBtn.disabled = false;
            if (appVersion) {
                // The footer shows the version of the binary that is
                // actually running, not a hardcoded number.
                appVersion.textContent = data.version;
            }
            break;
        case 'fatal':
            showStatus(`Failed to load the calculator: ${data.message}`, 'error');
            break;
        case 'result':
            if (data.id !== pendingId) {
                return;
            }
            finish();
            lastResult = data.table;
            solution.textContent = data.table;
            copyBtn.disabled = false;
            hideStatus();
            break;
        case 'error':
            if (data.id !== pendingId) {
                return;
            }
            finish();
            showStatus(data.message, 'error');
            break;
    }
}

function run() {
    const a = multiplicand.value.trim();
    const b = multiplier.value.trim();

    if (!DIGIT_RE.test(a) || !DIGIT_RE.test(b)) {
        showStatus('Both values must contain digits 0-9 only.', 'error');
        return;
    }
    if (a.length > limits.maxDigits || b.length > limits.maxDigits) {
        showStatus(
            `The web calculator accepts up to ${limits.maxDigits} digits per number. ` +
                'Use the desktop CLI for bigger numbers.',
            'error',
        );
        return;
    }

    pendingId += 1;
    calculateBtn.disabled = true;
    copyBtn.disabled = true;
    showStatus('Calculating… press Clear to cancel.', 'info');
    worker.postMessage({ id: pendingId, multiplicand: a, multiplier: b });
}

function finish() {
    pendingId = 0;
    calculateBtn.disabled = false;
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
