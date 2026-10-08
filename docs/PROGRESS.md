# Progress ledger
## 2026-10-08 — Downloadable Benk foundation
Supplied native/Effect boundaries, promoted reference invariants, deterministic
review simulator, loopback read-only API seed/tests, scoped SQLite crate and a
deny-all broker. Full original handoff is expanded and preserved. GitHub publishing
was stopped at the user's request; this folder makes no repository/CI success claim.

Read verification/LOCAL_RESULTS.md for current evidence. Next: BENK-001, then
native composer/accessibility work. No historical release gate is marked complete.
For later entries record ticket, files, commands/results, evidence, limits and
next ready task. Separate compile, unit, platform, accessibility and production.

## 2026-10-08 — BENK-001 genuine build evidence (macOS arm64)
Tools: node 24.21.0 (`/opt/homebrew/opt/node@24`, keg-only; system node is 26.7.0
and out of the `>=24 <25` engines range — prefix PATH or use a version manager),
npm 11.19.0, rustc/cargo 1.99.0 via rust-toolchain.toml, python 3.14.7,
`DEVELOPER_DIR=/Applications/Xcode-beta.app/Contents/Developer` (CLT alone lacks
`xcrun metal`; GPUI 0.2.2 shader compilation requires it).

Files changed: `tsconfig.base.json` (lib += `ESNext.Disposable` — effect 4.0.2
types require `Disposable`/`Symbol.asyncDispose`); `crates/broker/src/main.rs`
(println! format-string fix, then clippy `print_literal` suggestion applied);
`cargo fmt --all` repaired seed formatting across crates/apps. Generated
authentic `package-lock.json` and `Cargo.lock` (713 packages); no invented
integrity hashes.

Commands actually run, all passing:
- `npm install --ignore-scripts`, `npm run doctor`, `npm run check`
  (build + 232 domain + 17 simulator + 7 control-plane node tests +
  scripts/check_repo.py)
- `python3 -m venv .venv` + `pip install -r docs/handoff/requirements-qa.txt`,
  `npm run check:handoff` ("All specified handoff checks passed")
- `cargo generate-lockfile`, `cargo test --locked` (7 tests: broker deny-all,
  domain exhaustive edges, local-store scoping/deletion), `cargo clippy
  --locked --all-targets` (clean), `cargo fmt --all -- --check` (clean)
- `cargo check --locked -p benk-desktop` — GPUI 0.2.2 + benk-desktop compile
  clean on macOS arm64 (1m35s with warm deps)
- `cargo run -p benk-broker -- --capabilities` → JSON, exit 0; other args →
  stderr "execution disabled", exit 77
- `node services/control-plane/src/server.mjs` on 127.0.0.1:4317: `/healthz`
  and `/v1/demo/snapshot` 200; POST → 405; foreign Origin → 403. Server stopped.
- `npm run demo` prints full request→review→accept simulation JSON.

- `cargo build --locked -p benk-desktop` produced `target/debug/benk-desktop`
  (28M); smoke launch stayed alive 9s with empty stderr and exited cleanly on
  SIGTERM — window creation works, but no interaction/navigation/close UX was
  exercised or observed.

Evidence boundaries (unchanged): compile + brief launch ≠ rendering,
navigation or accessibility evidence. IME/focus/scroll/close-menu behavior
unverified. No Linux/Windows claim. Future-incompat warnings on transitive
deps (`block 0.1.6`, `proc-macro-error2 2.0.1`) noted, not failures.

Next ready task: finish BENK-001 launch evidence by running `npm run
dev:desktop` interactively and recording navigation/close behavior; then
BENK-002 composer/accessibility.
