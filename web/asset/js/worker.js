// Long Multiplication Calculator - calculation worker.
//
// Runs the WebAssembly module off the main thread, so a large table
// never freezes the page and the user can cancel it at any time.
//
// Every `?v=__WASM_VERSION__` is replaced with the package version by
// `scripts/build_wasm.bash`, so each release gets new URLs and the
// long-lived browser cache never mixes an old `.wasm` with new glue.

import init, { calculate, estimateTableBytes, maxDigits, maxOutputBytes, version } from './wasm/long_multiplication_wasm.js?v=__WASM_VERSION__';

const wasmUrl = new URL('./wasm/long_multiplication_wasm_bg.wasm?v=__WASM_VERSION__', import.meta.url);

const ready = init({ module_or_path: wasmUrl }).then(
    () => {
        self.postMessage({
            type: 'ready',
            version: version(),
            maxDigits: maxDigits(),
            maxOutputBytes: maxOutputBytes(),
        });
        return true;
    },
    (err) => {
        self.postMessage({ type: 'fatal', message: String(err?.message ?? err) });
        return false;
    },
);

self.addEventListener('message', async ({ data }) => {
    if (!(await ready)) {
        return;
    }
    try {
        // The page translates this error, so send the numbers rather
        // than the English message. `calculate` checks the limit too.
        const estimated = estimateTableBytes(data.multiplicand, data.multiplier);
        const max = maxOutputBytes();
        if (estimated > max) {
            self.postMessage({ type: 'error', id: data.id, code: 'too-large', estimated, max });
            return;
        }
        const table = calculate(data.multiplicand, data.multiplier);
        self.postMessage({ type: 'result', id: data.id, table, rows: countRows(table) });
    } catch (err) {
        self.postMessage({ type: 'error', id: data.id, message: String(err?.message ?? err) });
    }
});

/** Number of lines in the table, for the "Table ready" summary. */
function countRows(table) {
    let rows = 0;
    for (let i = table.indexOf('\n'); i !== -1; i = table.indexOf('\n', i + 1)) {
        rows += 1;
    }
    return table.endsWith('\n') || table === '' ? rows : rows + 1;
}
