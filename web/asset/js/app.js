// Long Multiplication Calculator - frontend (v2.0).
//
// Loads the Rust-compiled WebAssembly module once and exposes the
// `calculate` function to the form. No network round-trips: every
// computation happens in the browser.

import init, { calculate } from './wasm/long_multiplication_wasm.js';

const $ = (id) => document.getElementById(id);

const form = $('calculator');
const multiplier = $('multiplier');
const multiplicand = $('multiplicand');
const calculateBtn = $('calculate');
const clearBtn = $('clear');
const copyBtn = $('copy');
const status = $('status');
const statusText = $('status-text');
const solution = $('solution');

const DIGIT_RE = /^[0-9]+$/;
let lastResult = '';

// Initialise WASM, then enable the form.
(async () => {
    try {
        await init();
        calculateBtn.disabled = false;
    } catch (err) {
        showError(`Failed to load the calculator: ${err?.message ?? err}`);
    }
})();

form.addEventListener('submit', (ev) => {
    ev.preventDefault();
    run();
});

form.addEventListener('reset', () => {
    // Wait for the browser to clear the inputs, then reset the UI.
    queueMicrotask(() => {
        lastResult = '';
        solution.innerHTML = '<span class="empty-state">The table will appear here.</span>';
        copyBtn.disabled = true;
        hideError();
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
        showError(`Could not copy to clipboard: ${err?.message ?? err}`);
    }
});

function run() {
    hideError();

    const a = multiplier.value.trim();
    const b = multiplicand.value.trim();

    if (!DIGIT_RE.test(a) || !DIGIT_RE.test(b)) {
        showError('Both values must contain digits 0-9 only.');
        return;
    }

    try {
        const result = calculate(a, b);
        lastResult = result;
        solution.textContent = result;
        copyBtn.disabled = false;
    } catch (err) {
        const message = err?.message ?? String(err);
        showError(message);
    }
}

function showError(message) {
    statusText.textContent = message;
    status.hidden = false;
}

function hideError() {
    status.hidden = true;
    statusText.textContent = '';
}
