# 08 — Domain model, state machines, and invariants

## Canonical entities
Workspace, principal, conversation/channel, project, message, task, work-contract revision, run, environment, run lease, context package, artifact/version, action intent, approval/decision, acceptance record, usage reservation/charge, integration, and audit event.

A workroom is a view, not a competing source of task or permission state. A run belongs to one task contract revision. An artifact version is immutable. Acceptance references an exact task contract and one or more artifact versions. Historical acceptance is retained if a task is reopened; reopening creates a new revision/review cycle.

## Task state machine
`draft → ready → active → awaiting_review → accepted` is the primary path.

`active ↔ blocked`; `awaiting_review → active` requests changes; `ready/active/blocked/awaiting_review → cancelled`; `accepted → ready` is permitted only through an explicit reopen command that preserves the historical acceptance and creates the next contract revision. A cancelled task is not silently resumed; create a new task or an explicit audited revision according to the eventual product policy.

The reference reducer checks legal edges. It does not enforce command-specific guards such as human authority, versioned evidence, or reopen revision creation. The real command handler must enforce those guards transactionally.

## Run state machine
`queued → preparing → running` is the primary start. A run may enter `awaiting_approval`, `paused`, or `reconciling`. A run can end `succeeded`, `failed`, or `cancelled`. Cancellation uses `cancelling` before `cancelled`; an outstanding uncertain external action can keep it in reconciliation until the effect is understood.

A successful run means its bounded execution completed and produced its declared output/evidence. It does not accept the task. A failed run may still have useful artifacts or completed external effects. Terminal run state is immutable; retry means a new attempt/run or an explicitly modeled continuation with its own identity, never erasing the previous attempt.

Exact legal transition sets are in the reference core and `contracts/state-machines.json`. Gate changes through shared tests.

## Action intent lifecycle
`proposed → awaiting_approval → ready → dispatching → succeeded/failed/unknown`.

An approval-free permitted action can move proposed to ready. Cancellation before dispatch moves it to cancelled. An unknown result must be reconciled using provider identity/idempotency/evidence. Only a proven no-effect result may permit a safe redispatch of the same intent; do not create a fresh operation ID to bypass uncertainty. Action records are distinct from agent text messages.

## Coordinator ownership
Each run has an authority kind (`local_private` or `cloud_shared`), generation, coordinator ID, lease expiry, and cancellation flag/version. Dispatch requires exact generation equality and a current lease checked against authority time. A transfer increments generation under compare-and-set. Old output can be preserved as quarantined evidence, but cannot be accepted as the new coordinator's live mutation.

For shared work, the cloud owns coordination even when a local runner executes tools. Private local work can continue under a local authority until deliberately published or transferred. Publishing a private task creates or links a shared record after permission and scope review; it is not blind replication of private context.

## Revisions and IDs
Use opaque stable IDs with sufficient collision resistance. Client-generated operation/resource IDs allow offline creation; server constraints remain authoritative. Do not infer permissions, chronology, or tenancy from an ID string. Wire revisions and generations are bounded integers. Durable stream sequence numbers and monetary micro-units are unsigned decimal strings to avoid JavaScript precision loss. Server times use UTC timestamps; schedules also retain a named user timezone.

A task revision change invalidates bindings that name an old revision where the proposed action is no longer applicable. Artifact changes never mutate the old version. An approval digest covers the full action semantics including destination and affected resource versions; a pretty summary is not its binding.

## Required transaction guards
Message edit: verify author/moderation authority, current revision, scope, and tombstone state. Run dispatch: current owner/membership, run/task state, generation/lease, action scope, approval, and budget. Approval consume: valid human decision, required quorum/separation, exact binding, expiry, current policy, and unconsumed state. Acceptance: authorized human reviewer, current review cycle, required evidence, and exact artifact versions. Revocation: invalidate grants/subscriptions and enqueue scoped purge instructions.

## Evidence and provenance
Record what was requested, what context was authorized, which tool/provider version executed, what outputs were produced, what checks ran, and who accepted what. Store concise explanations and tool records; do not require hidden model reasoning. Sensitive source text belongs in protected content stores, not immutable audit payloads.
