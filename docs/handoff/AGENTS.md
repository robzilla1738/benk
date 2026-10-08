# Instructions for every build agent

## Product and stack constraints
Build the product described in `00_START_HERE.md`. Preserve GPUI + Rust for native desktop and TypeScript + Effect for application/orchestration code. The browser reviewer is complementary, not a desktop substitute. Begin with the first vertical slice and expand through the gates.

## Non-negotiable invariants
- No synchronous network, indexing, model call, or execution-engine round trip on typing, scrolling, cached channel switching, or cached search.
- Every shared write is tenant-scoped and authorized against current membership and resource policy. Caller-supplied actor identity is never authority.
- An agent cannot grant itself privileges, approve its own required human approval, or publish private context into a broader audience.
- One authoritative coordinator owns a run generation. Stale generations cannot commit new actions or artifacts.
- Approval binds a specific immutable action digest, artifact version, audience/destination, policy revision, and expiry. Revalidate at dispatch.
- A successful agent response is not accepted work. Human acceptance names the contract revision and artifact versions.
- Retries must not duplicate external effects. Unknown external outcomes require reconciliation, not blind retries.
- Cancellation requested, acknowledged, and completed are distinct. Completed external effects are not silently undone.
- Local execution and local model processing are separate labels. Offline access, expiry, and revocation limits are disclosed.
- No user content, secrets, private source text, or token stream is placed in product telemetry by default.

## Working method
Read the ticket's linked specs before editing. Claim one bounded task. Keep a small change set with acceptance evidence. Do not change shared contracts incidentally while implementing a screen. Update an ADR for architectural deviations; update contract fixtures for protocol changes. Never remove a failing test to make the build green without documenting a proven test defect.

Inspect an existing repository before scaffolding; preserve user work. Run a non-destructive status check. Do not overwrite configuration, rotate credentials, delete data, change access, deploy publicly, or incur material cost unless the user authorized that action. Work in a separate branch/worktree where available.

## Code expectations
Prefer explicit state machines and typed errors. Parse all untrusted inputs. Make bounds, timeouts, queue capacities, and retry categories explicit. Effect runtimes belong at process boundaries, not per-message or per-widget. Pure domain rules may remain framework-independent so Rust/TypeScript can share golden fixtures. Keep Temporal Workflow code deterministic; Effect integrations belong in Activities unless separately proven safe.

No ambient shell from a model-facing adapter. No default environment-variable inheritance into child processes. No API key in a CLI argument, URL, committed file, or log. The reference code is not a security boundary.

## Completion report
Use `templates/IMPLEMENTATION_REPORT.md`. Report changed files, requirement/test IDs, actual commands and exit codes, screenshots or recordings when a UI was run, untested platforms, residual risks, and next ready ticket. Distinguish implemented, simulated, stubbed, and planned. Do not say "production-ready", "secure", "Slack parity", or "industry-leading" without the relevant evidence.

## When blocked
Resolve ordinary reversible implementation choices using the defaults. Do not stall to ask broad product questions already answered here. Record assumptions. Ask only when a required irreversible decision, credential, legal/commercial choice, or unavailable external permission truly blocks the task. Continue unrelated safe work and identify the exact blocker.

## Security review trigger
Any change affecting auth, grants, executable code, connector installation, credentials, publication, sync visibility, approval consumption, budgets, or retention requires a negative-path test and independent review before enabling that capability.
