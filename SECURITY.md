# Security Policy

## Supported versions

| Version | Supported |
| --- | --- |
| 2.x (current) | Yes |
| 1.x | No |

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security
vulnerabilities. Instead, email
**israel.alberto.rv@gmail.com** with:

1. A description of the vulnerability and its impact.
2. Steps to reproduce.
3. The version of `long-multiplication-calculator` affected.
4. (Optional) A suggested fix or patch.

You should receive an acknowledgement within 72 hours. We will keep
you informed of the progress and credit you in the fix's release
notes unless you prefer to remain anonymous.

## Scope

This project has a small attack surface:

- The CLI binary has no network access and reads only its command
  line arguments.
- The WASM module runs in the user's browser. It performs only
  string manipulation; it does not access the network, the DOM, or
  any browser API beyond what `wasm-bindgen` requires.
- The Cloudflare Pages deployment serves only static files.

Vulnerabilities of interest:

- Crashes or panics in `core` triggered by adversarial inputs.
- Memory unsafety in `core` (the project forbids `unsafe`).
- XSS in the web frontend.
- Vulnerable transitive dependencies reported by `cargo audit`.
