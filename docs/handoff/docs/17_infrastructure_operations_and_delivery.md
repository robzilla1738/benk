# 17 — Infrastructure, operations, and delivery

## Initial deployment topology
Use one regional collaboration deployment with managed PostgreSQL, private object storage, a durable workflow service, and separately controlled execution workers. Prefer managed primitives until measured scale or customer requirements justify more operations. Kubernetes, a service mesh, Kafka, a graph database, and multi-region active-active writes are not foundation requirements.

Every workspace has a home region. Data routing, artifact storage, workflow histories, logs, backups, and model-provider destinations must be considered together before offering regional guarantees. Company-managed runners do not automatically keep all context/model traffic inside the company network.

## Environments and configuration
Separate development, test, staging, and production credentials/resources. A simulation flag is visible in product UI and cannot be toggled into real execution without configured enforcement and passing gates. Validate configuration at startup. Fail closed when secrets, signing keys, sandbox support, identity settings, or required policy are missing.

The supplied Compose template is local infrastructure only. It requires an explicitly reviewed image digest and credentials through environment variables. It binds database access to loopback and does not expose an application or initialize production security. There is no default real agent, billing integration, or public deployment target.

## Database operations
Version migrations and test forward/backward compatibility for supported client/server versions. Back up before destructive migrations. Favor expand/contract changes and dual-read compatibility where appropriate. Avoid running application migrations as the runtime role. Verify tenant context cleanup in pooled connections. Rehearse restore into an isolated environment and validate both relational state and artifact references.

Set provisional recovery objectives with the owner before a paid SLA. A starting engineering objective might be an RPO of 15 minutes and RTO of 4 hours for shared collaboration, but those are not contractual promises until backup frequency, restore exercises, staffing, and actual data-loss behavior support them. Artifact and workflow consistency matter as much as the database restore.

## Native packaging and updates
Produce signed platform-specific packages using dedicated credentials. Pin dependencies, create an SBOM/license inventory, scan dependencies, verify update signatures, and use staged rollout rings. Test update interruption and schema compatibility. A corrupt update must not strand drafts or silently roll data backwards. Windows/macOS/Linux support claims require their own build and behavior evidence.

Never package live provider credentials. The Effect engine runtime ships with the application and uses a reviewed security-update process. A separate engine process is not permission to lag indefinitely on runtime patches.

## Observability
Correlate workspace-safe request IDs, run IDs, action IDs, and provider operation references across API, workflow, runner, and UI diagnostics. Record latency, error class, queue depth, retries, interruptions, resource use, and usage settlement. Logs omit user content/secrets by default. Content-bearing diagnostics require explicit opt-in and redaction.

Key alerts: authorization anomaly, approval replay, stale-generation dispatch attempt, unexpected external-action uncertainty, stalled run lease, queue saturation, failed notification delivery, sync cursor errors, data/index purge failures, budget reconciliation lag, and signed-update verification failure. Route alerts to an accountable operator with a runbook; do not collect metrics without response ownership.

## Incident handling
Disable affected capabilities first when safety is uncertain. Preserve minimal evidence securely, identify impacted tenants/resources, stop credential use where appropriate, and reconcile external actions. Communicate known impact and uncertainty without guessing. Use `docs/30_failure_and_recovery_playbook.md` and the incident template. Restore service only after proving the relevant invariant and recording follow-up prevention work.

## CI and release evidence
Build/typecheck/lint/test all changed modules; validate schemas/golden fixtures; execute database migrations and isolation tests; run agent-adapter conformance; exercise native input/accessibility/performance on supported platforms; scan and sign artifacts; rehearse rollback. The handoff verification script checks this package's structure and references, not the complete future release pipeline.

## Supportability
Provide safe export of diagnostic metadata, visible run/activity state, explicit unknown outcomes, and recoverable drafts. Prefer a clear degraded mode over endless spinning. A support operator needs scoped, audited access; default internal tooling must not expose every customer's chat or secrets.
