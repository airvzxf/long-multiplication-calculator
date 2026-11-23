// Long Multiplication Calculator - interface language.
//
// The page is written in English. When the browser prefers Spanish,
// every element with `data-i18n="key"` gets the Spanish text, and the
// messages built by app.js use the same dictionary. No reload and no
// network request: both languages ship in this file.

const MESSAGES = {
    en: {
        'page.title': 'Long Multiplication Calculator',
        'page.heading': 'Long Multiplication Calculator',
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
        'solution.heading': 'Solution',
        'solution.empty': 'The table will appear here.',
        'footer.author': 'Author:',
        'footer.license': 'License:',
        'footer.source': 'Source:',
        'footer.siteVersion': 'Site version:',
        'footer.appVersion': 'App version:',
        'status.loading': 'Loading calculator…',
        'status.loadFailed': 'Failed to load the calculator: {message}',
        'status.calculating': 'Calculating… press Clear to cancel.',
        'status.ready': 'Table ready: {rows} rows.',
        'status.copyFailed': 'Could not copy to clipboard: {message}',
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
        'solution.heading': 'Solución',
        'solution.empty': 'La tabla aparecerá aquí.',
        'footer.author': 'Autor:',
        'footer.license': 'Licencia:',
        'footer.source': 'Código fuente:',
        'footer.siteVersion': 'Versión del sitio:',
        'footer.appVersion': 'Versión de la aplicación:',
        'status.loading': 'Cargando la calculadora…',
        'status.loadFailed': 'No se pudo cargar la calculadora: {message}',
        'status.calculating': 'Calculando… pulsa Limpiar para cancelar.',
        'status.ready': 'Tabla lista: {rows} filas.',
        'status.copyFailed': 'No se pudo copiar al portapapeles: {message}',
        'error.digitsOnly': 'Los dos valores solo pueden contener dígitos 0-9.',
        'error.tooLong':
            'La calculadora web acepta hasta {max} dígitos por número. ' +
            'Usa la versión de línea de comandos para números más grandes.',
        'error.tooLarge':
            'La tabla ocuparía unos {estimated} (límite {max}). ' +
            'Usa la versión de línea de comandos para tablas más grandes.',
    },
};

/** The interface language: `es` when the browser prefers Spanish, else `en`. */
export const lang = pickLanguage(navigator.languages ?? [navigator.language]);

function pickLanguage(preferred) {
    for (const tag of preferred) {
        const base = String(tag ?? '').toLowerCase().split('-')[0];
        if (base in MESSAGES) {
            return base;
        }
    }
    return 'en';
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

/** Translate the static page: `lang`, the title and every `[data-i18n]` element. */
export function translatePage() {
    document.documentElement.lang = lang;
    if (lang === 'en') {
        return;
    }
    document.title = t('page.title');
    for (const element of document.querySelectorAll('[data-i18n]')) {
        element.textContent = t(element.dataset.i18n);
    }
}
