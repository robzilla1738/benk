# CI/build plan — specification, not a successful pipeline

## Every change
Run the package verifier, JSON Schema fixtures, TypeScript reference build/tests, SQLite checks, and actual product unit tests. Once product services exist, add PostgreSQL transactions/RLS tests and generated Rust/TypeScript wire compatibility. Reject schema divergence and unreviewed migration changes.

## Native matrix
Use actual supported macOS/Windows/Linux runners with recorded SDKs. Compile release targets, run platform tests where automation supports them, and retain manual IME/accessibility/media evidence where not. Do not claim those checks from a headless pure-domain run.

## Security-sensitive changes
Run broker deny cases, authorization/concurrency tests, failure injection, preview isolation, dependency/license scanning, and independent review. Keep secrets restricted to explicitly authorized jobs; never give untrusted pull-request code distribution or production credentials.

## Distribution
Pin and review CI actions with real resolved commits; do not insert fabricated hashes. Sign/release only after gate approval. Store artifact digests, SBOM, migration/rollback notes, and staged update evidence. This package does not claim CI, signing, or deployment was run.
