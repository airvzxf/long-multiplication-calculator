// Long Multiplication Calculator - interface language.
//
// The page is written in English and can be shown in Spanish. Every
// element with `data-i18n="key"` gets the text of the interface
// language, and the messages built by app.js use the same dictionary.
// No reload and no network request: both languages ship in this file.
//
// The language, from the first source that names a known one:
//   1. `?lang=en` or `?lang=es` in the address (a shared link); it
//      applies to this visit and is not saved;
//   2. the visitor's choice in the language selector, saved in
//      localStorage;
//   3. the browser's preferred languages (`navigator.languages`);
//   4. English.
// The first two are explicit: the links the page builds keep them.

const MESSAGES = {
    en: {
        'page.title': 'Long Multiplication Calculator',
        'page.heading': 'Long Multiplication Calculator',
        'language.label': 'Language',
        'intro.heading': 'Introduction',
        'intro.text':
            'The long multiplication is the same as you did in the school when you were a kid. ' +
            'The intention is to show all the process of the multiplication including the carry ' +
            'numbers and the sums.',
        'intro.limits':
            'Note: the web calculator accepts up to 1000 digits per number and tables of up to 32 MiB. ' +
            'The desktop command-line version has no fixed limit: only your memory, disk and time.',
        'form.heading': 'Calculator form',
        'form.multiplicand': 'Multiplicand',
        'form.multiplier': 'Multiplier',
        'form.hint': 'Digits only, 0-9.',
        'form.calculate': 'Calculate',
        'form.clear': 'Clear',
        'form.copy': 'Copy result',
        'form.copied': 'Copied!',
        'form.download': 'Download JSON',
        'form.steps': 'Step by step',
        'form.stepsHide': 'Hide steps',
        'steps.first': 'First step',
        'steps.prev': 'Previous step',
        'steps.next': 'Next step',
        'steps.last': 'Last step',
        'steps.section': 'Section',
        'steps.keys': 'Tip: with the table focused, ← and → move between steps.',
        'steps.counter': 'Step {n} of {total}',
        'steps.products': 'Digit products',
        'steps.sums': 'Column sums',
        'steps.sub': 'Sub {n}',
        'steps.final': 'Product',
        'steps.product':
            'Row {row}: {a} × {b} = {p}. The unit {unit} goes to column {unitColumn} (row {row} R) ' +
            'and the carry {carry} to column {carryColumn} (row {row} ^).',
        'steps.sum': 'Column {column}: {terms} = {sum}.',
        'steps.sumMany': 'Column {column}: its {count} digits add up to {sum}.',
        'steps.carry': '{label}, column {column}: {unit} + {carried} carried from column {previous} = {value}.',
        'steps.carryFirst': '{label}, column 1: it keeps its units, {value}.',
        'solution.heading': 'Solution',
        'solution.empty': 'The table will appear here.',
        'footer.author': 'Author:',
        'footer.license': 'License:',
        'footer.source': 'Source:',
        'footer.siteVersion': 'Site version:',
        'footer.release': '{version} ({commit})',
        'footer.aheadOne': '{version} + {count} commit ({commit})',
        'footer.ahead': '{version} + {count} commits ({commit})',
        'footer.build': '{version}, commit {commit}',
        'footer.commitTitle': 'Commit {sha} on GitHub',
        'footer.appVersion': 'App version:',
        'status.loading': 'Loading calculator…',
        'status.loadFailed': 'Failed to load the calculator: {message}',
        'status.calculating': 'Calculating… press Clear to cancel.',
        'status.ready': 'Table ready: {rows} rows.',
        'status.copyFailed': 'Could not copy to clipboard: {message}',
        'status.jsonFailed': 'Could not prepare the JSON: {message}',
        'error.digitsOnly': 'Both values must contain digits 0-9 only.',
        'error.tooLong':
            'The web calculator accepts up to {max} digits per number. ' +
            'Use the desktop CLI for bigger numbers.',
        'error.tooLarge':
            'The table would take about {estimated} (limit {max}). ' +
            'Use the desktop CLI for bigger tables.',
    },
    es: {
        'page.title': 'Calculadora de multiplicación paso a paso',
        'page.heading': 'Calculadora de multiplicación paso a paso',
        'language.label': 'Idioma',
        'intro.heading': 'Introducción',
        'intro.text':
            'Es la misma multiplicación que aprendiste en la escuela. ' +
            'La intención es mostrar todo el proceso de la multiplicación, incluidos los números ' +
            'que se llevan (acarreos) y las sumas.',
        'intro.limits':
            'Nota: la calculadora web acepta hasta 1000 dígitos por número y tablas de hasta 32 MiB. ' +
            'La versión de escritorio para la línea de comandos no tiene un límite fijo: ' +
            'solo tu memoria, tu disco y tu tiempo.',
        'form.heading': 'Formulario de la calculadora',
        'form.multiplicand': 'Multiplicando',
        'form.multiplier': 'Multiplicador',
        'form.hint': 'Solo dígitos, 0-9.',
        'form.calculate': 'Calcular',
        'form.clear': 'Limpiar',
        'form.copy': 'Copiar resultado',
        'form.copied': '¡Copiado!',
        'form.download': 'Descargar JSON',
        'form.steps': 'Paso a paso',
        'form.stepsHide': 'Ocultar pasos',
        'steps.first': 'Primer paso',
        'steps.prev': 'Paso anterior',
        'steps.next': 'Paso siguiente',
        'steps.last': 'Último paso',
        'steps.section': 'Sección',
        'steps.keys': 'Consejo: con el foco en la tabla, ← y → cambian de paso.',
        'steps.counter': 'Paso {n} de {total}',
        'steps.products': 'Productos de dígitos',
        'steps.sums': 'Sumas de columnas',
        'steps.sub': 'Sub {n}',
        'steps.final': 'Producto',
        'steps.product':
            'Fila {row}: {a} × {b} = {p}. La unidad {unit} va a la columna {unitColumn} (fila {row} R) ' +
            'y el acarreo {carry} a la columna {carryColumn} (fila {row} ^).',
        'steps.sum': 'Columna {column}: {terms} = {sum}.',
        'steps.sumMany': 'Columna {column}: sus {count} dígitos suman {sum}.',
        'steps.carry': '{label}, columna {column}: {unit} + {carried} que se lleva de la columna {previous} = {value}.',
        'steps.carryFirst': '{label}, columna 1: se quedan sus unidades, {value}.',
        'solution.heading': 'Solución',
        'solution.empty': 'La tabla aparecerá aquí.',
        'footer.author': 'Autor:',
        'footer.license': 'Licencia:',
        'footer.source': 'Código fuente:',
        'footer.siteVersion': 'Versión del sitio:',
        'footer.release': '{version} ({commit})',
        'footer.aheadOne': '{version} y {count} commit más ({commit})',
        'footer.ahead': '{version} y {count} commits más ({commit})',
        'footer.build': '{version}, commit {commit}',
        'footer.commitTitle': 'Commit {sha} en GitHub',
        'footer.appVersion': 'Versión de la aplicación:',
        'status.loading': 'Cargando la calculadora…',
        'status.loadFailed': 'No se pudo cargar la calculadora: {message}',
        'status.calculating': 'Calculando… pulsa Limpiar para cancelar.',
        'status.ready': 'Tabla lista: {rows} filas.',
        'status.copyFailed': 'No se pudo copiar al portapapeles: {message}',
        'status.jsonFailed': 'No se pudo preparar el JSON: {message}',
        'error.digitsOnly': 'Los dos valores solo pueden contener dígitos 0-9.',
        'error.tooLong':
            'La calculadora web acepta hasta {max} dígitos por número. ' +
            'Usa la versión de línea de comandos para números más grandes.',
        'error.tooLarge':
            'La tabla ocuparía unos {estimated} (límite {max}). ' +
            'Usa la versión de línea de comandos para tablas más grandes.',
    },
};

/** The languages of the interface. */
export const LANGUAGES = Object.keys(MESSAGES);

const STORAGE_KEY = 'lang';

const initial = initialLanguage();

/** The interface language, `en` or `es`. */
export let lang = initial.lang;

/** Whether the language was chosen (in the address or the selector), not taken from the browser. */
export let explicit = initial.explicit;

function initialLanguage() {
    const fromUrl = new URLSearchParams(window.location.search).get('lang');
    if (LANGUAGES.includes(fromUrl)) {
        return { lang: fromUrl, explicit: true };
    }
    const saved = readSaved();
    if (LANGUAGES.includes(saved)) {
        return { lang: saved, explicit: true };
    }
    return { lang: browserLanguage(navigator.languages ?? [navigator.language]), explicit: false };
}

function browserLanguage(preferred) {
    for (const tag of preferred) {
        const base = String(tag ?? '').toLowerCase().split('-')[0];
        if (LANGUAGES.includes(base)) {
            return base;
        }
    }
    return 'en';
}

// localStorage can be missing or throw (private windows, blocked
// site data); the choice then lasts until the page is closed.
function readSaved() {
    try {
        return window.localStorage.getItem(STORAGE_KEY);
    } catch {
        return null;
    }
}

/** Switch to `next`, chosen in the selector: save it and translate the static page. */
export function setLanguage(next) {
    if (!LANGUAGES.includes(next)) {
        return;
    }
    lang = next;
    explicit = true;
    try {
        window.localStorage.setItem(STORAGE_KEY, next);
    } catch {
        // Not saved; see readSaved.
    }
    translatePage();
}

/** The message `key` in the interface language, with `{name}` placeholders filled in. */
export function t(key, params = {}) {
    const template = MESSAGES[lang][key] ?? MESSAGES.en[key] ?? key;
    return template.replace(/\{(\w+)\}/g, (match, name) => (name in params ? String(params[name]) : match));
}

/** A count with the grouping of the interface language, for example `1,234` or `1234`. */
export function formatNumber(value) {
    return new Intl.NumberFormat(lang).format(value);
}

/** A byte count with a binary unit and one decimal, for example `1.5 MiB` or `1,5 MiB`. */
export function formatBytes(bytes) {
    const units = ['KiB', 'MiB', 'GiB', 'TiB'];
    if (bytes < 1024) {
        return `${formatNumber(bytes)} bytes`;
    }
    let value = bytes / 1024;
    let unit = 0;
    while (value >= 1024 && unit < units.length - 1) {
        value /= 1024;
        unit += 1;
    }
    const number = new Intl.NumberFormat(lang, { minimumFractionDigits: 1, maximumFractionDigits: 1 });
    return `${number.format(value)} ${units[unit]}`;
}

/**
 * Translate the static page: `lang`, the title, the text of every
 * `[data-i18n]` element, the label (`aria-label` and `title`) of
 * every `[data-i18n-label]` element, such as the icon buttons, the
 * `aria-label` alone of every `[data-i18n-aria]` element, the
 * visibility of every `[data-lang-only]` element (shown in that
 * language only), and
 * `aria-current` in the language selector (`[data-lang]` links).
 */
export function translatePage() {
    document.documentElement.lang = lang;
    document.title = t('page.title');
    for (const link of document.querySelectorAll('a[data-lang]')) {
        if (link.dataset.lang === lang) {
            link.setAttribute('aria-current', 'true');
        } else {
            link.removeAttribute('aria-current');
        }
    }
    for (const element of document.querySelectorAll('[data-i18n]')) {
        element.textContent = t(element.dataset.i18n);
    }
    for (const element of document.querySelectorAll('[data-lang-only]')) {
        element.hidden = element.dataset.langOnly !== lang;
    }
    for (const element of document.querySelectorAll('[data-i18n-aria]')) {
        element.setAttribute('aria-label', t(element.dataset.i18nAria));
    }
    for (const element of document.querySelectorAll('[data-i18n-label]')) {
        const label = t(element.dataset.i18nLabel);
        element.setAttribute('aria-label', label);
        element.title = label;
    }
}
