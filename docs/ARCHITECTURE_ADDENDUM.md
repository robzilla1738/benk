# ADR-015 — Benk scaffold and repository boundaries
Date: 2026-10-08. Status: accepted for the foundation.

The product is Benk. Preserve the 203-file original handoff byte-for-byte in
docs/handoff; its old generic title is historical. Active packages use Benk.
The download contains expanded files, not the GitHub transfer encoding used
during the interrupted upload. No restoration workflow is required.

GPUI owns interaction. Rust owns local state/cache. Effect runs in separate
application and engine processes. PostgreSQL/durable cloud workflows remain
future authority components. The broker denies all execution and is not yet a
sandbox. No component silently inherits renderer or user-machine privileges.

The promoted TypeScript domain core tests shared historical transitions. The
Rust reducer mirrors task edges; generated cross-language wire/conformance
fixtures are still required before enabling real IPC. The public OpenAPI is a
target specification; the development adapter implements only health/snapshot.

The pure demo progresses draft -> ready -> active -> awaiting_review. A run can
succeed without accepting the task. TypeScript review checks exact artifact ID,
version and digest plus distinct simulated-human facts. Those facts are test data,
not identity. The native canned review is an interaction seed only, not wired to
Effect, persistence or a real second user. Do not disguise it as production review.

The historical Effect seed's TypeScript 5.8.3 choice is superseded by active
5.9.3. All native/Effect pins need connected build evidence. Actual lockfiles,
accessible composition, durable cache, authenticated IPC, identity and transactional
authority precede real execution. No safety gate is waived by a successful demo.
