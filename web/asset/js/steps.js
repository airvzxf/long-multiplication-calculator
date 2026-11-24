// Long Multiplication Calculator - step-by-step walk through the table.
//
// The values come from the JSON of the process (the same bytes as the
// CLI's `--format json`); the cells are found in the table text, whose
// layout is contractual: every row ends with its label (`r ^`, `r R`,
// `c C`, `P`), every `Sub n.` section starts with its title, and the
// cells are `width` characters wide, separated by `│`.
//
// A step highlights a few cells with <mark>: the factors of a digit
// product, the digits added up in a column, or the two digits that
// make each `Sub n.` value and the product. Only the lines of the
// current step are rebuilt, so even the biggest table stays fast.

import { t } from './i18n.js?v=__ASSET_VERSION__';

/** Most terms written out in the description of a column sum. */
const MAX_TERMS = 12;

/** The lines of the table that hold each part of the process. */
function locate(table, columns) {
    const lines = table.split('\n');
    const carries = [];
    const units = [];
    // stages[0]: the column sums; stages[n]: the `Sub n.` section.
    // Each maps a column number to its line.
    const stages = [[]];
    let ops = -1;
    let product = -1;
    let stage = 0;
    lines.forEach((line, index) => {
        let match;
        if (line.startsWith('┃Ops.')) {
            ops = index;
        } else if ((match = /^┃Sub (\d+)\./.exec(line))) {
            stage = Number(match[1]);
            stages[stage] = [];
        } else if ((match = /┃ (\d+) \^$/.exec(line))) {
            carries[Number(match[1])] = index;
        } else if ((match = /┃ (\d+) R$/.exec(line))) {
            units[Number(match[1])] = index;
        } else if ((match = /┃ (\d+) C$/.exec(line))) {
            stages[stage][Number(match[1])] = index;
        } else if (/┃ P$/.test(line)) {
            product = index;
        }
    });
    if (ops < 0 || product < 0) {
        throw new Error('unexpected table layout');
    }
    // Inner width = columns * width + (columns - 1) separators.
    const inner = lines[product].indexOf('┃', 1) - 1;
    const width = (inner - (columns - 1)) / columns;
    return { lines, carries, units, stages, ops, product, width };
}

/** Number of decimal digits of `value`. */
function digitCount(value) {
    return String(value).length;
}

/**
 * The steps of a calculation.
 *
 * `process` is the parsed JSON, `table` the text shown in the page.
 */
export class Steps {
    constructor(process, table) {
        this.process = process;
        this.columns = process.columns;
        this.layout = locate(table, this.columns);
        this.a = process.multiplicand.length;
        this.b = process.multiplier.length;
        const columns = this.columns;
        // One section per part of the table; each step is a column, or
        // a digit pair for the products.
        this.sections = [
            { kind: 'products', label: t('steps.products'), count: this.a * this.b },
            { kind: 'sums', label: t('steps.sums'), count: columns },
            ...process.subtotals.map((_, index) => ({
                kind: 'sub',
                stage: index + 1,
                label: t('steps.sub', { n: index + 1 }),
                count: columns,
            })),
            { kind: 'final', stage: process.subtotals.length + 1, label: t('steps.final'), count: columns },
        ];
        let start = 0;
        for (const section of this.sections) {
            section.start = start;
            start += section.count;
        }
        this.count = start;
    }

    /** The section that holds step `index`. */
    sectionOf(index) {
        let found = 0;
        this.sections.forEach((section, at) => {
            if (index >= section.start) {
                found = at;
            }
        });
        return found;
    }

    /**
     * Step `index`: `{ text, marks }`, where `marks` maps a line number
     * to the `[start, end, kind]` ranges to highlight on it.
     */
    describe(index) {
        const section = this.sections[this.sectionOf(index)];
        const offset = index - section.start;
        const marks = new Map();
        const step = { marks, text: '' };
        switch (section.kind) {
            case 'products':
                step.text = this.product(offset, marks);
                break;
            case 'sums':
                step.text = this.sum(offset + 1, marks);
                break;
            default:
                step.text = this.carry(section, offset + 1, marks);
        }
        return step;
    }

    /** Highlight `count` cells from `column` leftwards on `line`. */
    mark(marks, line, column, kind, count = 1) {
        const { width } = this.layout;
        const highest = column + count - 1;
        const start = 1 + (this.columns - highest) * (width + 1);
        const end = 1 + (this.columns - column) * (width + 1) + width;
        if (!marks.has(line)) {
            marks.set(line, []);
        }
        marks.get(line).push([start, end, kind]);
    }

    /** Digit `k` of the multiplicand (from the right) times digit `row` of the multiplier. */
    product(offset, marks) {
        const row = Math.floor(offset / this.a) + 1;
        const k = offset % this.a;
        const partial = this.process.rows[row - 1];
        const a = Number(this.process.multiplicand[this.a - 1 - k]);
        const b = partial.multiplier_digit;
        const unit = partial.units[k];
        const carry = partial.carries[k];
        const { ops, units, carries } = this.layout;
        this.mark(marks, ops + 2, k + 1, 'factor');
        this.mark(marks, ops + 3, row, 'factor');
        this.mark(marks, carries[row], k + row + 1, 'result');
        this.mark(marks, units[row], k + row, 'result');
        return t('steps.product', {
            row,
            a,
            b,
            p: a * b,
            unit,
            carry,
            unitColumn: k + row,
            carryColumn: k + row + 1,
        });
    }

    /** The sum of `column`: every unit and carry that lands in it. */
    sum(column, marks) {
        const terms = [];
        const { units, carries, stages } = this.layout;
        this.process.rows.forEach((partial, index) => {
            const row = index + 1;
            const fromCarry = column - row - 1;
            if (fromCarry >= 0 && fromCarry < this.a) {
                terms.push(partial.carries[fromCarry]);
                this.mark(marks, carries[row], column, 'source');
            }
            const fromUnit = column - row;
            if (fromUnit >= 0 && fromUnit < this.a) {
                terms.push(partial.units[fromUnit]);
                this.mark(marks, units[row], column, 'source');
            }
        });
        const total = this.process.sum[column - 1];
        this.mark(marks, stages[0][column], column, 'result', digitCount(total));
        if (terms.length > MAX_TERMS) {
            return t('steps.sumMany', { column, count: terms.length, sum: total });
        }
        return t('steps.sum', { column, terms: terms.join(' + '), sum: total });
    }

    /**
     * One column of a `Sub n.` section or of the product: the units of
     * the column in the previous stage plus what column - 1 carries.
     */
    carry(section, column, marks) {
        const { stages, product } = this.layout;
        const stage = section.stage;
        const previous = stage === 1 ? this.process.sum : this.process.subtotals[stage - 2];
        const before = stages[stage - 1];
        const own = previous[column - 1];
        const unit = own % 10;
        this.mark(marks, before[column], column, 'source');
        let carried = 0;
        if (column > 1) {
            const left = previous[column - 2];
            carried = Math.floor(left / 10);
            if (carried > 0) {
                // The tens of column - 1 are drawn from `column` leftwards.
                this.mark(marks, before[column - 1], column, 'source', digitCount(left) - 1);
            }
        }
        const value = unit + carried;
        if (section.kind === 'final') {
            this.mark(marks, product, column, 'result');
        } else {
            this.mark(marks, stages[stage][column], column, 'result', digitCount(value));
        }
        if (column === 1) {
            return t('steps.carryFirst', { label: section.label, unit, value });
        }
        return t('steps.carry', { label: section.label, column, previous: column - 1, unit, carried, value });
    }
}

/** Lines per block of the walk view. */
const BLOCK_LINES = 64;

/**
 * The table in a <pre>, one text node per line, so a step only
 * rebuilds the lines it highlights. The lines are grouped in blocks
 * (`.block`, `display: block`): the browser then lays out again only
 * the blocks that change, and skips the ones off screen, instead of
 * the whole table. The text itself never changes.
 */
export class StepView {
    constructor(pre, table) {
        this.pre = pre;
        this.table = table;
        const lines = table.split('\n');
        this.texts = lines.map((line, index) => (index < lines.length - 1 ? `${line}\n` : line));
        this.nodes = this.texts.map((text) => document.createTextNode(text));
        this.touched = [];
        const fragment = document.createDocumentFragment();
        for (let start = 0; start < this.nodes.length; start += BLOCK_LINES) {
            const block = document.createElement('span');
            block.className = 'block';
            const end = Math.min(start + BLOCK_LINES, this.nodes.length);
            block.append(...this.nodes.slice(start, end));
            // A block ends with a line break, which a block does not
            // draw; the page keeps the height of every line.
            block.style.containIntrinsicBlockSize = `auto ${(end - start) * 1.2}em`;
            fragment.append(block);
        }
        pre.replaceChildren(fragment);
    }

    /** Highlight `marks`; return the first result <mark>, to scroll to it. */
    show(marks) {
        for (const index of this.touched) {
            const plain = document.createTextNode(this.texts[index]);
            this.nodes[index].replaceWith(plain);
            this.nodes[index] = plain;
        }
        this.touched = [];
        let focus = null;
        for (const [index, ranges] of marks) {
            const text = this.texts[index];
            const span = document.createElement('span');
            let at = 0;
            for (const [start, end, kind] of [...ranges].sort((x, y) => x[0] - y[0])) {
                span.append(text.slice(at, start));
                const mark = document.createElement('mark');
                mark.className = kind;
                mark.textContent = text.slice(start, end);
                span.append(mark);
                if (kind === 'result' && !focus) {
                    focus = mark;
                }
                at = end;
            }
            span.append(text.slice(at));
            this.nodes[index].replaceWith(span);
            this.nodes[index] = span;
            this.touched.push(index);
        }
        return focus;
    }

    /** Put the plain table back. */
    destroy() {
        this.pre.textContent = this.table;
    }
}
