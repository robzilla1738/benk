# 05 — System architecture

## Process topology and ownership
| Component | Owns | Must not own |
|---|---|---|
| GPUI application | Rendering, focus, native lifecycle, view models. | Arbitrary generated-code execution, model calls on the UI path. |
| Rust client core | SQLite collaboration cache, pending operations, local search, sync client. | Cloud authority for shared approvals or membership. |
| Effect local engine | Adapter sessions, bounded orchestration, its execution journal. | Unrestricted host filesystem/network/process privileges. |
| Rust execution broker | Validating tool grants, sandbox process lifecycle, resource enforcement. | Agent-controlled policy decisions or approval creation. |
| Effect collaboration API | Verified identity, current authorization, shared commands/queries. | Serving unfiltered private context to models. |
| Temporal worker | Durable shared coordination via deterministic workflows and Activities. | Assuming retries make external effects exactly once. |
| PostgreSQL | Shared relational state, ledgers, revisions, outbox. | High-volume raw token streams as messages. |
| Object store | Immutable artifact/log blobs and controlled derivatives. | Public-by-default download links. |
| Browser reviewer | Permitted review, comments, decisions, lightweight conversation. | Local machine authority or unrestricted preview bridges. |

The cloud starts as a modular codebase with independently scheduled API, delivery, workflow, and indexing processes. This is a deployment boundary for isolation and load, not a requirement to invent many network services.

## Interactive path
Render authorized cached content before requesting fresh data. The GPUI view observes Rust-owned state changes; it does not synchronously ask the Effect engine for a channel. SQLite reads/writes and indexing run off the UI path with bounded work. Batch updates to views and preserve item identity/scroll anchors. Large artifacts are lazily loaded and capped. The collaboration app remains usable when the local agent engine fails or is disabled.

## Shared command path
A client persists an operation ID and canonical request body before submitting. The API authenticates the session, resolves the principal/workspace, validates the schema, authorizes the resource, checks an idempotency record, and executes the domain transaction. Domain row changes and outbox events commit atomically. The response identifies the accepted resource revision and operation. Broadcast is a delivery mechanism, not the source of truth. A disconnected client reconciles via its scoped cursor and pending ledger.

## Durable execution path
Creating a run produces a durable command. The workflow prepares context and environment, reserves budget, asks for necessary approvals, and dispatches bounded operations. Each externally consequential action is a persisted intent. The dispatch transaction checks current identity, resource policy, task revision, action/artifact digest, approval validity, coordinator generation, lease, cancellation, and budget before claiming the intent. Provider execution occurs outside the database transaction. Record success, known failure, or unknown result; reconcile uncertain outcomes.

Never hold a database transaction open during a model call, user approval wait, or remote execution. Use leases and state transitions. Temporal Workflows contain deterministic control flow; Activities bridge to Effect services and external systems. [S07, S08.]

## Communication boundaries
Local IPC uses authenticated OS-local transport, version negotiation, bounded frames, request IDs, cancellation, and streaming sequence numbers. Prefer user-private Unix sockets or Windows named pipes; parent-child stdio is acceptable for the first private session. Do not bind an unauthenticated localhost HTTP command server. The broker authenticates the caller and independently verifies grants; merely knowing a socket path is insufficient.

Cloud uses HTTPS commands/queries and a resumable authorized event transport. Reconnect cannot infer content access from a previously open connection. Resource subscription changes are rechecked, and permission revocations generate explicit client invalidation instructions. Binary artifacts travel through controlled blob endpoints, not oversized event frames.

## State and code boundaries
Keep framework-independent domain rules small: transitions, budget arithmetic, action-binding comparisons, cursor handling, and decisions. Use golden fixtures across Rust and TypeScript. Effect controls typed service boundaries, runtime resources, concurrency, and errors. Rust owns native data paths and execution enforcement. Do not create two business-rule implementations without parity tests.

Public contracts are language-neutral. Do not expose Effect internal serialization, GPUI entity handles, database row layouts, or Temporal execution objects as permanent public API. Each adapter translates into stable task/run/artifact/action concepts.

## Scale posture
A workspace has one write-authority region initially. Use database indexes and bounded selective sync before adding infrastructure. Store raw log streams outside human timelines. Partition execution queues by workspace and workload class; limit per-workspace concurrency. Revisit hot-workspace ordering, search infrastructure, and service extraction only after load tests reveal actual limits.

## Deployment safety
All real adapters, external publication, shared runners, and public invites are feature-gated until their corresponding safety tests pass. A simulation build must visibly identify simulated data and may not accept real credentials. The included SQL and scaffolds are starting points; they do not supply a running, authenticated production service.
