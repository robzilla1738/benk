# 25 — Sources and current verification

All sources below are first-party documentation, repositories, or standards and were checked on **8 October 2026**. They support dependency and protocol facts, not the original product recommendations, staffing assumptions, performance targets, or leadership claims. External pages are mutable; recheck them at implementation. No external article, SDK source tree, binary, or font is bundled.

## Important correction
Effect 4.0 was released on 30 September 2026. Earlier beta-era advice in the conversation is superseded. The checked Effect API reference showed 4.0.2. The handoff deliberately distinguishes a stable core release from individual unstable modules and from unverified package installation.

The GPUI scaffold follows the published 0.2.2 API family, not current-main code copied indiscriminately. The preparation environment had no Rust/Cargo and could not install external npm packages, so the dependency-backed scaffolds are not compiled evidence. See `verification/RESULTS.md`.

## Reference ledger

### S01 — Effect 4.0 release
https://effect.website/blog/releases/effect/40

Supports: Release post last updated 2026-09-30; supports stable v4 baseline.

Qualification: Do not reuse vendor benchmark figures as this product’s performance.

### S02 — Effect Context API reference
https://effect.website/docs/v4/api/effect/Context

Supports: Checked page displayed effect 4.0.2 and Context.Service.

Qualification: Observed documentation version; npm installation not verified here.

### S03 — Effect core API
https://effect.website/docs/v4/api/effect/Effect

Supports: Effect programming model and core API reference.

Qualification: Verify exact signatures against installed package.

### S04 — Published GPUI 0.2.2 documentation
https://docs.rs/gpui/0.2.2/gpui/

Supports: Published API and pre-1.0 development caveat.

Qualification: Do not infer all current-main platform behavior from this release page.

### S05 — GPUI upstream bootstrap example
https://raw.githubusercontent.com/zed-industries/zed/main/crates/gpui/examples/hello_world.rs

Supports: Current-main example uses a platform bootstrap different from the published tutorial.

Qualification: Mutable upstream; do not mix with an incompatible pinned release.

### S06 — GPUI package metadata
https://github.com/zed-industries/zed/blob/main/crates/gpui/Cargo.toml

Supports: Checked GPUI package metadata declares Apache-2.0.

Qualification: Does not license all other Zed components under the same terms.

### S07 — Temporal TypeScript developer guide
https://docs.temporal.io/develop/typescript

Supports: Workflow/Activity/worker development guidance.

Qualification: Implementation and replay tests are still required.

### S08 — Temporal Activity execution
https://docs.temporal.io/activity-execution

Supports: Activity execution and retry semantics.

Qualification: Retries are not an exactly-once guarantee for external side effects.

### S09 — SQLite write-ahead logging
https://www.sqlite.org/wal.html

Supports: WAL concurrency and operational characteristics.

Qualification: The product’s persistence/performance still needs testing.

### S10 — SQLite FTS5
https://www.sqlite.org/fts5.html

Supports: Full-text indexing/query/maintenance mechanisms.

Qualification: Permission filtering and deletion consistency are application obligations.

### S11 — PostgreSQL row security
https://www.postgresql.org/docs/current/ddl-rowsecurity.html

Supports: RLS policies and bypass/owner considerations.

Qualification: Tenant filtering is not full resource authorization.

### S12 — Agent Client Protocol introduction
https://agentclientprotocol.com/get-started/introduction

Supports: Local/remote protocol scope; checked text notes full remote support remains in progress.

Qualification: Test adapter/version capabilities; avoid universal compatibility claims.

### S13 — MCP security best practices
https://modelcontextprotocol.io/specification/latest/basic/security_best_practices

Supports: Connector/token/SSRF/local-server security considerations.

Qualification: Protocol adoption alone is not a sandbox or policy engine.

### S14 — Development containers
https://containers.dev/

Supports: Open development-environment configuration specification.

Qualification: Configuration portability is not transparent process migration.

### S15 — LiveKit Rust SDK repository
https://github.com/livekit/rust-sdks

Supports: Rust realtime/server SDK availability.

Qualification: Native media behavior/platform support requires a selected-version spike.

### S16 — Slack feature overview
https://slack.com/features

Supports: Broad feature categories informing parity inventory.

Qualification: Inventory is not a complete audited equivalence claim across all plans.

### S17 — Slack Web API rate limits
https://docs.slack.dev/apis/web-api/rate-limits/

Supports: Current integration rate-limit rules and distribution considerations.

Qualification: Check exact rules again before implementation; avoid hard-coded assumed allowances.

### S18 — WCAG 2.2
https://www.w3.org/TR/WCAG22/

Supports: Web accessibility acceptance reference.

Qualification: Native semantics need platform-specific tests; no conformance claim here.

### S19 — RFC 8785 JSON Canonicalization Scheme
https://datatracker.ietf.org/doc/html/rfc8785

Supports: Candidate canonical serialization for action hashing.

Qualification: Use a reviewed implementation; this package does not implement cryptographic approvals.

### S20 — OpenID Connect Core
https://openid.net/specs/openid-connect-core-1_0.html

Supports: Identity protocol reference for an authentication adapter.

Qualification: No identity provider or authentication service is implemented in the handoff.
