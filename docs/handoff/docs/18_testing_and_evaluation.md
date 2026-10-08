# 18 — Testing, evaluation, and evidence requirements

## Test hierarchy
Pure unit tests cover state transitions, permissions facts, budget arithmetic, idempotency decisions, and cursor handling. Contract tests cover valid/invalid payloads, versioning, serialization, and Rust/TypeScript parity. Database integration tests cover constraints, transactions, tenant isolation, concurrent approval consumption, and outbox consistency. System tests exercise the real API, workflow service, runner, and native/browser clients. Human usability and accessibility testing covers what automated checks miss.

The supplied reference core and SQLite tests belong to the first layers only. Passing them does not implement a sandbox, authenticate users, or prove real recovery. The acceptance catalog includes planned application scenarios with implementation status separate from reference coverage.

## Required negative cases
Reject cross-tenant IDs and private-context publication. Reject stale/replayed approvals, wrong artifact versions, stale coordinator generations, expired leases, revoked identities, unreserved spending, malformed frames, oversized content, unsafe links/preview access, unapproved model destinations, and unauthorized filesystem/network operations. Test simultaneous races, not only sequential calls.

A fail-closed message on one code path is not enough: test the UI, public API, agent API, background worker, broker, and reconnect path. They must not offer alternative routes around the same policy.

## Deterministic simulation
Use a simulator that produces a known patch, check result, optional blocker, and artifact package. It performs no shell, provider, or network actions. Scenarios include success, failed check, request for access, budget exhaustion, cancellation, and unknown external receipt simulation. Label synthetic evidence. This enables product-flow iteration before real agent integration and provides reproducible tests after providers change.

## Real adapter evaluation
Select a fixed suite of representative tasks with explicit acceptance criteria and allowed resources. Record agent/provider versions, environment definition, context package, budgets, outputs, evidence, human interventions, and outcomes. Include failures, not just showcase tasks. Evaluate correctness, safety, review effort, latency, and total cost. Do not reward task splitting or verbose tool activity as productivity.

## Fault injection
Kill the client after pending-operation persistence; disconnect after server commit; restart workflow workers while waiting for approval; terminate a runner mid-command; expire credentials; revoke source access; duplicate webhook delivery; reorder event batches; fill local disk; interrupt artifact upload; force provider timeouts after possible acceptance; suspend/resume the laptop; race two approvals/dispatchers; and switch coordinator ownership during delayed output.

Expected outcomes are explicit: recoverable draft, one accepted mutation, no unauthorized dispatch, visible reconciliation, protected evidence, or safe stop. "The process did not crash" is not the acceptance criterion.

## Native and browser evidence
Collect release-mode traces, keyboard-only recordings, screen-reader walkthroughs, and platform/input matrices. Use synthetic data. Screenshots demonstrate layout only; they do not prove interaction or access control. Browser review must be tested with a user who did not initiate the task and with expired/revoked links.

## Release gate evidence
Each gate names required requirements/scenarios, evidence paths, reviewer, and actual result. Mark not-run as not-run. A release candidate cannot infer passed status from a spec document or TODO comment. Independent review is required for execution security and approval/authorization changes.

## Preparation verification scope
See `verification/RESULTS.md` for the actual commands run while building this handoff. At preparation time the available environment can compile the dependency-free TypeScript reference, execute Node tests, validate schemas, and run SQLite tests. Rust/GPUI, external Effect packages, PostgreSQL, Temporal, cloud providers, browser/native interaction, sandbox enforcement, performance targets, and security audits require follow-up validation in a suitable environment.
