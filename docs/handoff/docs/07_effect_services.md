# 07 — Effect application and orchestration specification

## Version and runtime
Use Effect 4 as the baseline. The checked reference was 4.0.2; the scaffold pins it exactly. Keep platform/provider package releases compatible and commit an actual lockfile after installation. Review unstable-module annotations separately. The supplied core reference does not require Effect because it is intentionally usable as cross-runtime domain fixtures. Product application services should use Effect rather than reimplementing ad hoc cancellation/resource/error behavior. [S01–S03.]

Use one managed runtime per process with deliberate lifetime. Initialize configuration and service Layers at startup. Validate environment variables and redact secrets. Dispose runtimes on shutdown. Do not instantiate a runtime per HTTP request or attach unbounded detached fibers to user activity.

## Service boundaries
| Service | Responsibility | Important typed failures |
|---|---|---|
| Identity | Resolve verified principal and membership. | Unauthenticated, MembershipRevoked. |
| Authorization | Decide resource/action/audience policy. | PermissionDenied, PolicyChanged. |
| TaskRepository | Read revisions; compare-and-set changes. | NotFound, RevisionConflict, StorageUnavailable. |
| RunCoordinator | Own generation, lease, cancellation, progress. | StaleGeneration, LeaseExpired, RunStopped. |
| BudgetLedger | Reserve, settle, and reconcile usage. | BudgetExceeded, ReservationConflict. |
| ArtifactStore | Version/upload/read authorized artifacts. | DigestMismatch, UploadExpired, ContentRejected. |
| AgentAdapter | Normalize capabilities and bounded work. | UnsupportedCapability, ProviderRateLimited, ProviderUnavailable. |
| BrokerClient | Dispatch scoped tool requests. | CapabilityDenied, SandboxUnavailable, ExecutionUnknown. |
| NotificationService | Deliver deduplicated attention events. | DeliveryDeferred, DestinationUnavailable. |
| Audit | Append redacted security-relevant events. | AuditUnavailable. |

Keep expected domain errors in the error channel. Unexpected defects are reported with correlation IDs and safe diagnostics. Do not expose stack traces, source contents, or credentials to arbitrary clients. Where a user lacks visibility, NotFound may intentionally conceal existence; keep detailed reason codes inside authorized audit tooling.

## Concurrency and backpressure
Set per-workspace active-run quotas, per-provider request concurrency, bounded log buffers, and queue depth limits. Separate interactive API work from background indexing and agent streaming. Cancellation must propagate to child work and supported provider requests, followed by explicit broker/process cleanup. Use timeouts for every remote operation and reserve time for cleanup.

Retries are categorized by effect semantics. Safe reads may retry with bounded jitter. Idempotent writes may retry under the same key. Authorization failures, stale grants, exhausted budgets, and validation errors do not retry automatically. A timeout after an externally consequential request may mean the effect happened; enter reconciliation.

## Temporal boundary
Workflow definitions use the Temporal SDK's deterministic subset. Import Activity types, not implementations or an Effect runtime into workflow code by accident. Activities execute Effect services and map typed failures to retryable or nonretryable outcomes. Heartbeat long activities and checkpoint known work. Cancellation of an Activity is not proof that an external process stopped. [S07, S08.]

Represent approval waits as durable state, not a long-lived HTTP request. Bind workflow identity to workspace/run/generation. Version workflow code and test replay before upgrades. Migration from Temporal to another engine requires an execution-history/ownership plan, not just an interface change.

## Effect Workflow evaluation
An Effect-native durable backend is a valid foundation-stage experiment. It must pass the same crash, duplicate, approval-wait, version-upgrade, database-interruption, and reconciliation cases. Evaluate operational tooling, retention, replay/versioning, support, and team expertise. Do not operate both engines as competing authorities for the same run. Core Effect concurrency alone is not durable persistence.

## API composition
Keep HTTP/IPC adapters thin: parse and authenticate, call a service, encode a response. Request actor identity is derived from the session; run configuration cannot override it. Transactions are explicit and short. The transaction that mutates a shared domain row also appends the outbox event and idempotency result. No model calls, approval waits, or remote tool execution happen inside it.

## Testing
Provide deterministic fake services and controlled clocks. Test typed failures, interruption cleanup, bounded concurrency, repeated requests, stale state, and provider uncertainty. Reference tests only cover selected pure rules. The scaffold must be dependency-installed, typechecked, and integration-tested before it becomes product code. Do not replace a missing real adapter with an unlabeled fake success path.
