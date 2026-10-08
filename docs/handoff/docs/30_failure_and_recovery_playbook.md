# 30 — Failure and recovery playbook

## General approach
Prefer explicit degraded states over pretending success. Preserve recoverable user work. Disable unsafe new dispatch while determining uncertain external outcomes. Record minimal protected evidence and current authority. The operations owner must be able to explain what happened without relying on an agent's confident narrative.

| Failure | Immediate behavior | Recovery and proof |
|---|---|---|
| Desktop crash while drafting | Draft remains in committed local storage. | Reopen; validate draft/version; report any unsaved interval honestly. |
| Server accepted message but response lost | Keep operation pending/unknown, not duplicate-send with a new ID. | Reconcile the original idempotency key; display one accepted message. |
| Sync cursor expired | Stop applying ambiguous deltas. | Authorized snapshot + boundary cursor + replay pending intent. |
| Permission revoked | Stop new access/dispatch and process purge. | Recheck sessions/subscriptions/context derivatives; record offline limitations. |
| Local machine sleeps | Runner becomes unavailable; no assumed continued capacity. | Refresh lease and ownership before more work; offer supported checkpoint continuation. |
| Agent process crashes | Run fails/pauses with preserved artifacts. | Resume only from supported checkpoint or start a new attempt. |
| Provider timeout after possible write | Mark action unknown; block blind retry. | Query provider receipt/idempotency record or request human reconciliation. |
| Approval changes/stales | Disable dispatch and explain mismatch. | Present exact current action/version for a new valid decision. |
| Budget estimate wrong | Record actual spend and stop new work if needed. | Reconcile charge ledger; change limits only through authorized user action. |
| Coordinator split-brain attempt | Reject old generation. | Establish one owner via authoritative CAS; quarantine stale output. |
| Artifact upload incomplete | Do not publish a valid artifact version. | Retry/expire ticket; verify actual size/digest before finalization. |
| Preview compromise suspected | Stop preview and isolate environment. | Revoke tokens/grants, preserve evidence, assess source/data exposure. |
| Local disk full | Stop risky writes and show recovery choices. | Free cache safely without deleting drafts; retry transaction; verify integrity. |
| Database outage | Accept local drafts; clearly queue eligible sends. | Restore authoritative state, reconcile operations, then resume dispatch. |
| Bad application update | Preserve data; avoid incompatible writes. | Roll back signed binary only under schema compatibility plan. |
| Connector revoked/compromised | Stop new credential use and dependent workflow actions. | Reinstall/review scopes, rotate as authorized, reconcile external actions. |

## Uncertain external effects
The system needs an `unknown`/reconciling state. A network error after a request left the process does not prove failure. Retrying a customer email, purchase-like action, or publication with a new ID can duplicate consequences. Keep the original intent and provider reference; resolve through provider APIs, receipts, idempotency behavior, or explicit human review. Record what remains uncertain.

## Cancellation
Stop scheduling new work immediately when cancellation is accepted. Request supported model/process cancellation, terminate relevant process groups within policy, and record acknowledgments. Do not label "stopped" until enforcement confirms it or the UI explains what could not be stopped. Continue bounded reconciliation for in-flight external actions. Compensation is a new authorized action, not automatic time reversal.

## Restore exercises
Test restoring relational data, artifact availability, workflow histories, pending actions, and authorization state together. A database snapshot alone may not restore a runnable system. Revoke or revalidate credentials and leases after disaster recovery. Old restored workers cannot resume as current authorities without new generation checks.

## Incident documentation
Use the incident template: scope, known impact, uncertainty, containment, timeline, affected resources, evidence, recovery criteria, customer communication owner, and prevention tasks. Avoid recording raw secrets or private source content in general incident logs. An incident is closed only when the relevant invariant is re-established and follow-up work has an owner.
