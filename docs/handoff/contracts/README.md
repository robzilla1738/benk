# Contracts and protocol use

`schemas/domain.schema.json` is a JSON Schema 2020-12 definition library for the first vertical slice. `openapi.json` embeds the same definitions as OpenAPI 3.1 components. `examples/index.json` names positive and intentionally invalid cases. `state-machines.json` provides the transition fixture. This is an initial contract, not a deployed API or every eventual Slack-parity endpoint.

Resolve the reserved `.invalid` schema IDs locally; no external schema download is required. Dates are UTC RFC 3339 values. Sequence numbers and USD micro-units are unsigned decimal strings, parsed with exact integer arithmetic. Revisions and generations are bounded integers. Requests never supply trusted actor kind, human approval status, or effective policy; servers derive those values.

Schema validity is not authorization. Relationships such as workspace equality, current membership, source visibility, task/run binding, lease expiry, action digest, artifact version, and approval consumption need domain/transaction tests. The action example deliberately represents a proposed pull-request operation outside the initial contract's allowed actions: a real server must reject or require a reviewed scope revision. It is structurally valid, not dispatch authorization.

## Initial transport requirements
HTTP mutations require `Idempotency-Key`; scope it to verified actor/workspace/operation and canonical payload digest. Authentication and errors must not leak hidden resource existence. GET `/events` represents catch-up semantics; a live subscription uses the same authorization/cursor contract. Pagination and byte limits are enforced independent of schema item counts.

For local IPC, use a user-private authenticated pipe/socket or private parent-child stdio. Authenticate peers outside the JSON frame. A `session_id` alone is not a credential. Frame v1 is length-prefixed UTF-8 JSON with a 1 MiB hard cap; reject oversized length before allocating. Reject incompatible versions, duplicate in-flight request IDs, malformed frames, and unexpected stream sequences. Set finite request deadlines and stream buffer limits. Privileged broker requests require a separately verified capability envelope; this engine IPC schema is **not** the broker authorization protocol.

Large payloads use protected references. Never expose raw filesystem paths or blob URLs as permission. Artifact URLs expire and enforce the current viewer's access where required. Complete-upload processing verifies actual object length/digest and policy; it must not trust the expected digest provided by a client.

## Change process
Add backward-compatible optional fields only through a reviewed contract change. For breaking semantic changes, create a new protocol/schema version with migration and mixed-client tests. The initial objects reject unknown properties to catch drift; adapters must explicitly negotiate versions rather than assuming unknown fields are harmless. Regenerate OpenAPI components after changing definitions. `scripts/verify_handoff.py` checks they remain identical.

## Drafts, tombstones, and upload versioning
A draft task may have no contract yet (`contract_revision: null`). Real execution requires a current immutable contract. Deleted message wire records must contain `body: null`; live messages require structured rich text. Purge also applies to local search and derived context. Upload creation identifies the artifact and its expected previous version (null for a new artifact); completion must compare-and-set against that lineage, validate size/digest/content, and create an immutable version. Stale concurrent upload completion must not overwrite a later version.
