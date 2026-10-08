# Persistence designs

`local-schema.sql` is executable and tested with the preparation environment's SQLite build. It models the collaboration cache, full-text projection, drafts, queued operations, and cursors. It does not encrypt storage, implement a sync client, or secure a compromised device. Every search must constrain account/workspace and join currently visible channel records. Deletion/purge triggers remove search content. The default sample purge deletes channel-bound drafts too, to avoid retaining private source material.

`server-schema.sql` is a reviewed reference DDL design, **not executed against PostgreSQL here**. It deliberately has no application role grants, identity-provider implementation, public bootstrap endpoint, or production secret. Use a disposable database, add actual migration/version tooling, run cross-tenant and concurrency tests, and verify RLS behavior before enabling an API.

Tenant RLS is defense in depth, not private-channel authorization. The selected identity directory/bootstrap flow must implement authenticated workspace discovery and scoped membership resolution; it is not supplied by these tables alone. The runtime connection role cannot be a superuser, owner, or BYPASSRLS role. Set transaction-local tenant context only after authenticating and authorizing the request, and test connection-pool reuse across tenants.

## Dispatch transaction outline
Lock the action, current run/lease, required approval decisions, and budget account in a consistent order. Check current membership, source/destination authority, task revision, generation, expiry, cancellation, exact action/artifact binding, quorum, and funds. Mark the action dispatching; consume the approvals; create/reserve the unique charge allowance; append an outbox record; commit. Execute the provider request outside the transaction using its stable operation/idempotency key. Record confirmed result or unknown outcome in a second transaction. Do not automatically retry an unknown external effect.

## Ordering
Allocate the workspace event counter within the domain transaction and hold its row lock until commit. This simple reference ordering trades hot-workspace throughput for clear catch-up semantics. A raw independent PostgreSQL sequence with `max(sequence)` catch-up can skip late commits. Measure before replacing the ordering scheme, and then specify its exact guarantees.

## Invariants not fully encoded by DDL
Human-only reviewer/owner kinds; cross-task artifact/run checks; source-to-output audience intersection; approval quorum/consumption; legal state transitions; immutable revision rows; cryptographic digest correctness; authorization epoch invalidation; retention/hold behavior; and application identity resolution. These are release-blocking implementation/test work, not assumptions you may ignore because a foreign key exists.
