# Actual local verification — Benk download
Date: 8 October 2026.

The application source, expanded original handoff and build-agent guidance are
included. This package is a development foundation, not a completed product.

## Executed and passed
- Active pure TypeScript domain and simulator build: TypeScript 5.8.3, Node 22.16.0.
- Promoted reference-core tests: 232 passed.
- New simulator tests: 17 passed, including exact artifact digest/version review,
  rejected agent/self/stale review, immutable transitions and zero external actions.
- Original handoff coherence, 22 contract fixtures and 16 SQLite tests passed.
- All 203 original files compared byte-for-byte against the provided ZIP.
- JavaScript syntax checks for the development server, server tests and demo.
- Seed repository structure checks.

The historical reference tests were also rerun by the original verifier; those
are the same 232 tests, not another independent 232-test suite.

## Supplied, not verified here
Effect service dependency compilation and its seven HTTP tests; Rust reducers,
SQLite integration and broker tests; GPUI native compilation/rendering; PostgreSQL
execution; accessibility, media and performance benchmarks. No successful GitHub
CI run or completed repository publication is claimed.

Generate authentic lockfiles on a connected host, then run BENK-001. Declared
dependencies do not prove compatibility. The active workspace targets Node 24,
TypeScript 5.9.3 and Effect 4.0.2; the local pure-core check used the available
older Node/TypeScript toolchain and does not certify those dependency integrations.

Logs are included here; original historical verification remains under
`docs/handoff/verification/`. No production credentials or real execution are enabled.

Later evidence (8 October 2026, development host): the items listed above as
"supplied, not verified" have since been compiled and tested — Effect build +
HTTP tests, Rust crates, GPUI 0.2.2 compile/launch, and headless UI tests of
the native composer. See docs/PROGRESS.md for the dated ledger; this file
continues to describe only the original download verification.
