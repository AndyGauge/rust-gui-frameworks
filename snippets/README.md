# Snippets

Every code block in the book is a real file here, compiled against the crate
versions in each package's `Cargo.lock`. The pages load these files directly
(`src/lib/snippets.js`), so what you read is exactly what compiled.

- One standalone package per topic (separate lockfiles, because frameworks pin
  conflicting dependencies).
- `python3 verify.py` re-checks every package (plus `wasm32-unknown-unknown` for
  the web ones), re-runs the headless examples to capture their output, and
  regenerates `meta.json` and `extra/*`.
- Blocks marked "illustrative" in the book are not here; they are inline in
  `content/*.md`.
- Some examples are macOS-specific (glutin's display, font paths). Everything
  was checked on macOS, rustc 1.95.0.

Shared build cache: `CARGO_TARGET_DIR=/some/dir python3 verify.py`
