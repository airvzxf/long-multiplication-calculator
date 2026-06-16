# Hallazgos Pendientes por Implementar

**Fecha:** 2026-06-16
**Origen:** Síntesis de las 4 propuestas de análisis (`01-analisis-*.md`) y las 4 propuestas mejoradas (`03-mejorada-*.md`).
**Propósito:** Backlog priorizado de mejoras que añadirían valor real al proyecto, ordenado por impacto y esfuerzo.

> Cada hallazgo indica: dónde se mencionó, qué valor aporta, esfuerzo estimado, y guía de implementación concreta.

---

## Tier 1 — Valor alto, recomendado para el próximo ciclo

### H1. `proptest` para invariante del algoritmo

**Donde se mencionó:** 6soS, 8kjc, K3LK, Q0ER, p6gp, R7mK (6 docs).

**Valor:** Aporta confianza algorítmica continua. Los golden tests cubren casos específicos, pero `proptest` valida una propiedad formal sobre el espacio de inputs:

> ∀ a, b ∈ ℕ. producto(⟨a⟩, ⟨b⟩) == a × b

donde `⟨x⟩` es la conversión a string y `producto(...)` es la salida del producto en la tabla renderizada.

**Esfuerzo:** ~30 LOC + 1 dev-dependency.

**Implementación:**

```toml
# crates/core/Cargo.toml
[dev-dependencies]
proptest = "1"
```

```rust
// crates/core/src/multiplication.rs (al final, en #[cfg(test)] mod tests)
use proptest::prelude::*;

proptest! {
    /// Invariante fundamental: la tabla generada por get_table contiene
    /// el producto correcto, y el último dígito del producto (que aparece
    /// en la columna 1 de la fila P) es el último dígito de a*b.
    #[test]
    fn product_in_final_row_matches_native(
        a in 1u64..1_000_000,
        b in 1u64..1_000_000,
    ) {
        let a_str = a.to_string();
        let b_str = b.to_string();
        let table = get_table(&a_str, &b_str);
        let expected_product = (a * b).to_string();

        // Localizar la fila "P" (la fila del producto final).
        let p_row = table
            .lines()
            .rev()
            .find(|line| line.contains("┃ P") || line.contains("| P"))
            .expect("the table must end with a 'P' row");

        // Todos los dígitos del producto esperado deben aparecer en la fila P
        // (puede haber espacios/separadores entre los dígitos).
        for digit in expected_product.chars() {
            prop_assert!(
                p_row.contains(digit),
                "expected digit {digit} of product {expected_product} not found in P row: {p_row}"
            );
        }
    }

    /// Sanity: la tabla es idéntica byte-a-byte para el mismo input.
    #[test]
    fn determinism(a in 0u64..10_000, b in 0u64..10_000) {
        let a_str = a.to_string();
        let b_str = b.to_string();
        prop_assert_eq!(get_table(&a_str, &b_str), get_table(&a_str, &b_str));
    }
}
```

**Por qué importa:** detecta regresiones algorítmicas sutiles que los golden tests pasarían por alto. Si alguien cambia `break_down_addition` y por error invierte el orden de las filas, los 3 golden fixtures pasan, pero `proptest` lo cacha en milisegundos.

---

### H2. `criterion` benchmark

**Donde se mencionó:** 6soS, 8kjc, K3LK, Q0ER, p6gp, R7mK (6 docs).

**Valor:** Tracking de regresiones de performance. Hoy el algoritmo corre en microsegundos para 100 dígitos, pero no hay forma de detectarlo automáticamente.

**Esfuerzo:** ~30 LOC + 1 dev-dependency.

**Implementación:**

```toml
# Cargo.toml (workspace.dependencies)
criterion = { version = "0.5", default-features = false }

# crates/core/Cargo.toml
[dev-dependencies]
criterion.workspace = true

[[bench]]
name = "multiplication_bench"
harness = false
```

```rust
// crates/core/benches/multiplication_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use long_multiplication_core::get_table;

fn bench_cases() -> Vec<(&'static str, String, String)> {
    vec![
        ("10x10", "1234567890".into(), "9876543210".into()),
        ("100x100", "1".repeat(100), "9".repeat(100)),
        ("500x500", "1".repeat(500), "2".repeat(500)),
    ]
}

fn bench_get_table(c: &mut Criterion) {
    for (name, a, b) in bench_cases() {
        c.bench_function(&format!("get_table/{name}"), |c| {
            c.iter(|| get_table(&a, &b));
        });
    }
}

criterion_group!(benches, bench_get_table);
criterion_main!(benches);
```

**Por qué importa:** cuando alguien optimice el algoritmo o cambie la estructura de datos, los criterios detectan si la mejora realmente mejora. Sin esto, las optimizaciones son "feel-based".

---

### H3. Job de CI con `rustc 1.85.0` (MSRV test)

**Donde se mencionó:** 6soS, Q0ER, p6gp, R7mK (4 docs).

**Valor:** El `rust-version = "1.85.0"` está pineado en `Cargo.toml` y `.clippy.toml`, pero **nadie verifica** que el código compile con esa versión exacta. CI corre con `stable` (1.88+ en 2026), así que un dev podría usar una feature de Rust 1.86 sin darse cuenta y romper el MSRV silenciosamente.

**Esfuerzo:** ~15 líneas de YAML.

**Implementación:**

```yaml
# .github/workflows/ci.yml (agregar como nuevo job)
  msrv:
    name: MSRV build (rustc 1.85.0)
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - name: Install MSRV toolchain
        uses: dtolnay/rust-toolchain@1.85.0
        with:
          components: [clippy, rustfmt]
      - uses: Swatinem/rust-cache@v2
      - name: cargo build (MSRV)
        run: cargo build --workspace --all-targets
      - name: cargo test (MSRV)
        run: cargo test --workspace
```

**Por qué importa:** cualquier feature post-1.85 rompe el MSRV. El clippy.toml dice `msrv = "1.85.0"` pero clippy sólo verifica uso de features deprecated, no features nuevas. Sin este job, el MSRV es una mentira.

---

### H4. CI matrix multi-OS

**Donde se mencionó:** 6soS, K3LK, p6gp, R7mK (4 docs).

**Valor:** Rust es multiplataforma, pero el proyecto no se valida en macOS ni Windows. Bugs específicos de plataforma pasarían inadvertidos (e.g., line endings en fixtures, diferencias de `usize` en arquitecturas 32-bit, paths en Windows).

**Esfuerzo:** ~10 líneas de YAML, ~5-10 min extra de CI.

**Implementación:**

```yaml
# .github/workflows/ci.yml (modificar el job `lint-test`)
  lint-test:
    name: Lint, test, build
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v5
      # ... resto igual
```

**Por qué importa:** el CLI se publica como `cargo install` y los usuarios pueden tener cualquier OS. Si el binario rompe en Windows, los golden tests actuales no lo detectarían (usan paths de Unix).

**Caveat:** los golden fixtures usan line endings de Unix (`\n`). En Windows el binario de test podría convertir a `\r\n` y romper los byte-by-byte. Hay que verificar. Si rompe, agregar `assert_cmd` con `.success()` y no comparar stdout byte-a-byte, sólo `predicate::str::contains(...)`.

---

## Tier 2 — Valor real, prioridad media

### H5. Cron workflow mensual con `cargo upgrade --compatible`

**Donde se mencionó:** 6soS, Q0ER, p6gp (3 docs).

**Valor:** Mantiene las dependencias al día automáticamente. Sin esto, las deps se van quedando atrás hasta que haya un CVE y se tenga que hacer un upgrade urgente.

**Esfuerzo:** ~25 líneas de YAML.

**Implementación:**

```yaml
# .github/workflows/dependency-update.yml
name: Monthly dependency update

on:
  schedule:
    - cron: '0 6 1 * *'  # primer día del mes a las 06:00 UTC
  workflow_dispatch:

permissions:
  contents: write
  pull-requests: write

jobs:
  upgrade:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: [clippy, rustfmt]
      - uses: Swatinem/rust-cache@v2
      - name: Install cargo-edit
        run: cargo install cargo-edit --locked
      - name: Upgrade compatible deps
        run: cargo upgrade --compatible
      - name: Run Gauntlet
        run: |
          cargo fmt --all
          cargo clippy --workspace --all-targets -- -D warnings
          cargo test --workspace
      - name: Create PR
        uses: peter-evans/create-pull-request@v6
        with:
          commit-message: 'chore(deps): monthly cargo upgrade --compatible'
          title: 'Monthly dependency update'
          body: |
            Automated monthly update of compatible dependency versions.
            See [cargo-edit](https://github.com/killercup/cargo-edit) for details.
          branch: deps/monthly-upgrade
```

**Por qué importa:** evita el "dependency rot" y reduce la superficie de CVEs acumulados. Compatible-only es seguro (no breaking changes).

---

### H6. `release.yml` con binarios pre-compilados

**Donde se mencionó:** 6soS, K3LK, Q0ER, p6gp, R7mK (5 docs).

**Valor:** Hoy el usuario final tiene que tener Rust toolchain instalado para usar el CLI. Con binarios pre-compilados, `long-multiplication` está disponible con un download.

**Esfuerzo:** ~40 líneas de YAML (sin `cargo-dist`) o ~15 líneas con `cargo-dist`.

**Implementación (opción simple, sin cargo-dist):**

```yaml
# .github/workflows/release.yml
name: Release CLI binaries

on:
  push:
    tags: ['v*']

permissions:
  contents: write

jobs:
  build:
    name: Build (${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            asset: long-multiplication-linux-x86_64
          - os: macos-latest
            target: x86_64-apple-darwin
            asset: long-multiplication-macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            asset: long-multiplication-macos-aarch64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            asset: long-multiplication-windows-x86_64.exe
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - uses: Swatinem/rust-cache@v2
      - name: Build release
        run: cargo build --release --target ${{ matrix.target }} -p long-multiplication
      - name: Package
        shell: bash
        run: |
          mkdir -p dist
          cp target/${{ matrix.target }}/release/long-multiplication${{ matrix.os == 'windows-latest' && '.exe' || '' }} \
             dist/${{ matrix.asset }}
      - name: Upload
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset }}
          path: dist/${{ matrix.asset }}

  release:
    name: Create GitHub Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
        with: { path: artifacts }
      - name: Create release
        uses: softprops/action-gh-release@v2
        with:
          files: artifacts/**/*
          generate_release_notes: true
```

**Por qué importa:** bajar la barrera de entrada. Un profesor que quiera usar esto en clase no necesita instalar Rust.

**Alternativa más simple:** `cargo-dist` hace todo esto en ~15 líneas, vale la pena considerarlo si se quiere seguir ese camino.

---

### H7. Compartir cálculos vía URL con query params

**Donde se mencionó:** 6soS, 8kjc, K3LK, Q0ER (4 docs).

**Valor:** Permite compartir un cálculo específico vía link. "Mira cómo se desglosa 13597 × 8642" se vuelve un clickeable. **Aporta valor de uso real al usuario final**, sin tocar arquitectura.

**Esfuerzo:** ~15 LOC en `app.js` + ajustes menores en `index.html`.

**Implementación:**

```javascript
// web/asset/js/app.js (al inicio del archivo, antes de run())

function readQueryParams() {
    const params = new URLSearchParams(window.location.search);
    return {
        multiplier: params.get('m') || '',
        multiplicand: params.get('c') || '',
    };
}

function writeQueryParams(multiplier, multiplicand) {
    const url = new URL(window.location.href);
    url.searchParams.set('m', multiplier);
    url.searchParams.set('c', multiplicand);
    window.history.replaceState({}, '', url);
}

// Inicializar inputs desde query params al cargar
(function init() {
    const { multiplier: a, multiplicand: b } = readQueryParams();
    if (a && b && DIGIT_RE.test(a) && DIGIT_RE.test(b)) {
        document.getElementById('multiplier').value = a;
        document.getElementById('multiplicand').value = b;
        // Auto-calcular si ambos inputs son válidos
        queueMicrotask(run);
    }
})();

// Llamar a writeQueryParams al final de run() exitoso:
function run() {
    // ... validación y cálculo como antes ...
    if (lastResult) {
        writeQueryParams(
            document.getElementById('multiplier').value,
            document.getElementById('multiplicand').value
        );
    }
}
```

**Uso resultante:** `multiplication.rovisoft.net?m=13597&c=8642` carga y calcula automáticamente. Se puede compartir por email, mensajería, etc.

**Por qué importa:** sin esto, cada vez que alguien quiere mostrar un cálculo a otro, tiene que copiar/pegar los números. Con esto, comparte un link. Es el feature más "humilde" de la lista pero el de uso más inmediato.

---

### H8. Mapeo de errores al frontend en español

**Donde se mencionó:** K3LK, p6gp, R7mK (3 docs).

**Valor:** El frontend actual muestra el mensaje literal de `JsError` en inglés (`"Invalid character 'a' at position 2"`). Para usuarios hispanohablantes, traducir mejora UX sin tocar arquitectura.

**Esfuerzo:** ~15 LOC en `app.js`.

**Implementación:**

```javascript
// web/asset/js/app.js (función helper)

function translateError(msg) {
    if (msg.includes('Invalid character') && msg.includes('at position')) {
        return 'Solo se permiten dígitos 0-9. Revisa que no haya letras ni símbolos.';
    }
    if (msg.includes('Input cannot be empty')) {
        return 'Ambos campos son obligatorios.';
    }
    if (msg.includes('exceeds the maximum length')) {
        return 'Los números son demasiado grandes (máximo 1000 dígitos).';
    }
    return msg; // fallback al mensaje original
}

// En run(), reemplazar:
//   const message = err?.message ?? String(err);
//   showError(message);
// por:
function run() {
    // ... validación ...
    try {
        const result = calculate(a, b);
        lastResult = result;
        solution.textContent = result;
        copyBtn.disabled = false;
    } catch (err) {
        const raw = err?.message ?? String(err);
        showError(translateError(raw));
    }
}
```

**Caveat:** inconsistente con el HTML que está en inglés (`Solution`, `Multiplier`, etc.). Si se traducen los errores, considerar traducir también los labels — pero eso es un scope mayor.

**Por qué importa:** micro-cambio, cero riesgo, mejora perceptible para usuarios en español.

---

## Tier 3 — Cosmético pero valioso cuando haya tiempo

### H9. Refactor de `generate.rs` (1775 líneas) en submódulos

**Donde se mencionó:** 6soS, 8kjc, p6gp, R7mK (4 docs).

**Valor:** Mantenibilidad. El archivo es difícil de navegar aunque funcione perfectamente. La firma pública no cambia.

**Esfuerzo:** ~2-3 horas (refactor cuidadoso, mover tests con sus funciones).

**Estructura objetivo:**

```
crates/core/src/generate/
├── mod.rs          # re-exports + get_table interno
├── header.rs       # symbols, position_title, top_border
├── operations.rs   # multiplication, operations
├── sum.rs          # sum_title, long_sum
├── border.rs       # bottom_border
└── footer.rs       # author
```

**Caveat:** los tests viven dentro de cada archivo con `#[cfg(test)] mod tests`. Si se mueven las funciones, hay que mover los tests también. **No es trivial** pero tampoco es riesgoso si se hace paso a paso con `cargo test` después de cada movimiento.

**Por qué importa:** cuando alguien necesite entender o modificar el formato de la tabla, va a sufrir recorriendo 1775 líneas. Submódulos bajan la curva de entrada.

**Estado actual:** la sección 1.1 de `03-mejorada-CuzL.md` argumentaba que "funciona correctamente, no reescribir masivamente". Esa postura es defendible — los tests golden cubren el output. Pero la deuda técnica sigue ahí.

---

### H10. Reemplazar `&mut String` con `String` returns en `get_table`

**Donde se mencionó:** 6soS (originalmente), pero con baja prioridad.

**Valor:** Cosmético, idiomático. `multiplication.rs:27-39` muta un buffer y lo pasa a cada generador. Una versión más limpia devolvería `String` desde cada generador y haría concatenación.

**Esfuerzo:** ~1 hora (cambio mecánico, no toca lógica).

**Implementación (esquema):**

```rust
// Antes (en multiplication.rs):
pub fn get_table(multiplicand: &str, multiplier: &str) -> String {
    let mut content = String::new();
    generate::symbols(&mut content);
    generate::top_border(multiplicand, multiplier, &mut content);
    // ... 8 generadores más con &mut
    content
}

// Después:
pub fn get_table(multiplicand: &str, multiplier: &str) -> String {
    let mut parts: Vec<String> = Vec::with_capacity(11);
    parts.push(generate::symbols());
    parts.push(generate::top_border(multiplicand, multiplier));
    parts.push(generate::position_title(multiplicand, multiplier));
    parts.push(generate::operation_title(multiplicand, multiplier));
    parts.push(generate::multiplication(multiplicand, multiplier));
    parts.push(generate::operations(multiplicand, multiplier));
    parts.push(generate::sum_title(multiplicand, multiplier));
    parts.push(generate::long_sum(multiplicand, multiplier));
    parts.push(generate::bottom_border(multiplicand, multiplier));
    parts.push(generate::author());
    parts.join("\n")
}
```

**Por qué importa:** reduce side-effects, mejor composición. El performance es equivalente (mismo número de allocations). El output es idéntico (los golden tests pasan).

**Tradeoff:** el `&mut` actual permite a cada función hacer append incremental sin asignar el `String` completo de una vez. La versión `join("\n")` hace 11 allocations en vez de 1. Para 1775 líneas de Unicode, negligible.

---

### H11. Texto "Inicializando calculadora..." mientras WASM carga

**Donde se mencionó:** 6soS, CuzL, p6gp, R7mK.

**Valor:** UX. Hoy el botón "Calculate" sólo se deshabilita (`app.js:28`); el usuario puede no darse cuenta de que el sitio está cargando.

**Esfuerzo:** ~5 LOC en `app.js` + 1 línea en `index.html`.

**Implementación:**

```html
<!-- web/index.html, junto al botón: -->
<span id="loader" hidden>Inicializando calculadora…</span>
```

```javascript
// web/asset/js/app.js, reemplazar:
calculateBtn.disabled = false;
// por:
document.getElementById('loader').hidden = true;
calculateBtn.disabled = false;
```

```javascript
// Al inicio, antes de await init():
document.getElementById('loader').hidden = false;
```

**Por qué importa:** el sitio es estático pero la primera carga del `.wasm` toma 50-200ms. Sin feedback, el usuario podría pensar que el sitio está roto.

---

### H12. `aria-invalid` dinámico en inputs

**Donde se mencionó:** CuzL, p6gp (2 docs).

**Valor:** Accesibilidad con screen readers. Hoy `styles.css:127-129` usa `:invalid` (estilo visual), pero `aria-invalid` no se setea dinámicamente.

**Esfuerzo:** ~10 LOC en `app.js`.

**Implementación:**

```javascript
// web/asset/js/app.js, en run() durante validación:
const setInvalid = (el, isInvalid) => {
    if (isInvalid) el.setAttribute('aria-invalid', 'true');
    else el.removeAttribute('aria-invalid');
};

function run() {
    hideError();
    const a = multiplier.value.trim();
    const b = multiplicand.value.trim();
    
    const aValid = DIGIT_RE.test(a);
    const bValid = DIGIT_RE.test(b);
    setInvalid(multiplier, !aValid);
    setInvalid(multiplicand, !bValid);
    
    if (!aValid || !bValid) {
        showError('Both values must contain digits 0-9 only.');
        return;
    }
    // ... resto igual
}
```

**Por qué importa:** los screen readers anuncian "invalid" cuando encuentran `aria-invalid="true"`. Mejora a11y para usuarios con discapacidad visual.

---

### H13. Sugerencia `mkdir -p` en errores de escritura del CLI

**Donde se mencionó:** 8kjc, p6gp, R7mK (3 docs).

**Valor:** UX de terminal. Cuando `std::fs::write` falla porque el directorio padre no existe, el error actual dice "Cannot write to 'foo.txt'" sin contexto accionable.

**Esfuerzo:** ~5 LOC en `cli/src/main.rs`.

**Implementación:**

```rust
// crates/cli/src/main.rs, reemplazar la función write_table:

fn write_table(path: &std::path::Path, table: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(anyhow::anyhow!(
                "Cannot write to '{}' (parent directory '{}' does not exist).\n  \
                 Hint: create it with `mkdir -p {}`",
                path.display(),
                parent.display(),
                parent.display()
            ));
        }
    }
    std::fs::write(path, table)
        .with_context(|| format!("Cannot write to '{}'", path.display()))?;
    Ok(())
}
```

**Por qué importa:** baja la fricción para usuarios no técnicos. Hoy el mensaje genérico de `with_context` no sugiere qué hacer.

---

### H14. OG / Twitter meta tags

**Donde se mencionó:** 6soS, Q0ER (2 docs).

**Valor:** Previsualización al compartir en redes sociales. Sin OG tags, Facebook/Twitter/LinkedIn muestran sólo el título.

**Esfuerzo:** ~8 líneas en `index.html`.

**Implementación:**

```html
<!-- web/index.html, dentro de <head>, después del title: -->
<meta property="og:title" content="Long Multiplication Calculator">
<meta property="og:description" content="Multiplication step-by-step, as taught in primary school.">
<meta property="og:type" content="website">
<meta property="og:url" content="https://multiplication.rovisoft.net">
<meta property="og:image" content="https://multiplication.rovisoft.net/android-chrome-512x512.png">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="Long Multiplication Calculator">
<meta name="twitter:description" content="Multiplication step-by-step, as taught in primary school.">
<meta name="twitter:image" content="https://multiplication.rovisoft.net/android-chrome-512x512.png">
```

**Por qué importa:** si alguien comparte el link, la previsualización es presentable. Bajo valor para una herramienta educativa, pero estándar en sitios web modernos.

---

### H15. Documentar decisiones YAGNI en el CHANGELOG

**Donde se mencionó:** Documentos 8kjc, K3LK, Q0ER, p6gp, R7mK; y los `docs/ARCHITECTURE.md:81-95`.

**Valor:** Trazabilidad. Las 4 propuestas `03-mejorada-*.md` proponían abstracciones (`Renderer` trait, `NumericString` newtype, `LongMultiplication` struct, `JsonRenderer`, `--json`) que el implementador rechazó. Esa decisión está documentada en `docs/ARCHITECTURE.md:81-95` pero **no en el CHANGELOG** (el lugar natural donde futuros contribuidores la buscarían).

**Esfuerzo:** ~10 líneas en `CHANGELOG.md`.

**Implementación:**

```markdown
## [Unreleased] - Notas de diseño

Esta versión consolidó las decisiones de las propuestas `6soS`, `8kjc`,
`K3LK`, `Q0ER`, `59bO`, `CuzL`, `p6gp`, `R7mK`. Las siguientes
abstracciones fueron **explícitamente rechazadas** (YAGNI) en favor de
funciones puras simples:

- **Trait `Renderer`** (propuesto por 8kjc, K3LK, p6gp, R7mK): no se
  introdujo porque hoy existe un solo formato de salida (Unicode). Si
  en el futuro se necesita JSON/HTML/SVG, se extraerá el trait entonces.
  Ver `docs/ARCHITECTURE.md:81-88` para el rationale completo.

- **`NumericString` newtype** (propuesto por 6soS, p6gp, R7mK): se
  reemplazó por `validate_input(&str, max_digits) -> Result<&str>`,
  que evita una asignación en el happy path.

- **`LongMultiplication` / `OperationRow` structs** (propuesto por 8kjc,
  K3LK, p6gp, R7mK): no se introdujeron porque `get_table` retorna
  `String` directo. Si se quiere resaltar celdas individuales en CSS
  o implementar animaciones paso a paso, se necesitará el struct.
```

**Por qué importa:** futuros contribuidores (humanos o AI) que lean el CHANGELOG y vean "no se hizo X" tendrán el rationale en el lugar esperado, en vez de tener que buscarlo en 8 documentos de análisis.

---

## Resumen de prioridad

| # | Hallazgo | Esfuerzo | Valor | Docs |
|---|---|---|---|---|
| H1 | `proptest` para invariante | ~30m | **Alto** | 6 |
| H2 | `criterion` benchmark | ~30m | **Alto** | 6 |
| H3 | MSRV test en CI | ~15m | **Alto** | 4 |
| H4 | CI multi-OS matrix | ~15m | **Alto** | 4 |
| H5 | Cron upgrade-compatible | ~30m | Medio | 3 |
| H6 | `release.yml` con binarios | ~1h | Medio | 5 |
| H7 | Compartir por URL | ~30m | **Medio-Alto** | 4 |
| H8 | Errores en español | ~15m | Bajo-Medio | 3 |
| H9 | Subdividir `generate.rs` | ~3h | Bajo (cosmético) | 4 |
| H10 | Reemplazar `&mut String` | ~1h | Bajo (cosmético) | 1 |
| H11 | Texto "Loading..." | ~5m | Bajo (cosmético) | 4 |
| H12 | `aria-invalid` dinámico | ~10m | Bajo (cosmético) | 2 |
| H13 | Sugerencia `mkdir -p` | ~5m | Bajo (cosmético) | 3 |
| H14 | OG / Twitter meta tags | ~5m | Bajo (cosmético) | 2 |
| H15 | Decisiones YAGNI en CHANGELOG | ~10m | Bajo (doc) | 5 |

**Estimación total Tier 1 (H1-H4):** ~90 minutos. Aporta 4 mejoras estructurales reales.

**Estimación total Tier 1+2 (H1-H8):** ~4-5 horas. Cubre todo lo que tiene valor de uso real.

**Estimación total Tier 1+2+3 (todos):** ~9-10 horas. Incluye mejoras cosméticas.

---

## Notas finales

- **H1-H4 son los que más se repitieron** en las 8 propuestas. Son los "consensos" entre todas las visiones, lo que sugiere alto valor.
- **H7 (compartir por URL) es el único que aporta valor directo al usuario final** sin tocar arquitectura. Es el más "humilde" de la lista pero el más visible.
- **H15 (documentar YAGNI)** es barato y previene debates futuros sobre por qué no se adoptó el `Renderer` trait.
- Los hallazgos **H9-H14 son cosméticos** y no urge hacerlos. Se pueden agrupar en un PR "polish" antes de un release mayor.
