# Dependency-ordered implementation backlog

All 72 tickets are unimplemented product work. Reference tests do not complete them. Ticket numbers are identifiers, not a strict execution order; follow dependencies.

## T001 — Repository and toolchain audit

**Phase:** F0 · **Owner:** lead · **Status:** todo
**Dependencies:** None

Inventory existing code; record selected exact toolchains and package-install capability.

**Definition of done:**

- A new engineer can reproduce the selected baseline from recorded commands and lockfiles.
- Missing tools or external dependencies are reported as unavailable, never represented as tested.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-001, REQ-002. Acceptance scenarios: AT-001, AT-002.
Evidence destination: `implementation/evidence/T001.md`.

## T002 — Contract and fixture ownership

**Phase:** F0 · **Owner:** backend · **Status:** todo
**Dependencies:** T001

Establish schema/golden-fixture generation and compatibility checks.

**Definition of done:**

- Rust/TypeScript serialization uses the same IDs, revisions, decimal sequences, and error contract.
- A breaking payload or inconsistent generated OpenAPI fails the contract check.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-003, REQ-004. Acceptance scenarios: AT-003, AT-004.
Evidence destination: `implementation/evidence/T002.md`.

## T003 — GPUI bootstrap and native package pin

**Phase:** F0 · **Owner:** native · **Status:** todo
**Dependencies:** T001

Build and run a minimal native application on the first supported platform.

**Definition of done:**

- The selected GPUI release, compiler, and native SDK produce a reproducible windowed release build.
- Current-main and published-crate API differences cannot be silently mixed.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-005, REQ-006. Acceptance scenarios: AT-005, AT-006.
Evidence destination: `implementation/evidence/T003.md`.

## T004 — Composer, IME, and accessibility risk spike

**Phase:** F0 · **Owner:** native · **Status:** todo
**Dependencies:** T003

Implement the hard native input and focus test surface before general UI polish.

**Definition of done:**

- IME composition, selection, clipboard, undo, and keyboard/screen-reader traversal work on the chosen matrix.
- Enter during active IME composition never sends a message or loses input.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-007, REQ-008. Acceptance scenarios: AT-007, AT-008.
Evidence destination: `implementation/evidence/T004.md`.

## T005 — SQLite cache, drafts, and migrations

**Phase:** F0 · **Owner:** native · **Status:** todo
**Dependencies:** T002, T003

Create the real Rust-owned cache and migration layer with a single logical writer.

**Definition of done:**

- Drafts and message/search projections survive process restart and migrate without loss.
- Account/workspace mixing, failed transactions, and revoked-content search leakage are rejected.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-009, REQ-010. Acceptance scenarios: AT-009, AT-010.
Evidence destination: `implementation/evidence/T005.md`.

## T006 — Threat model and authority boundaries

**Phase:** F0 · **Owner:** security · **Status:** todo
**Dependencies:** T001, T002

Approve trust boundaries, source-use/publication rules, and execution disable gates.

**Definition of done:**

- Every privileged surface has an owner, threat model, policy source, and negative test plan.
- A separate but unrestricted process is not accepted as a sandbox.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-011, REQ-012. Acceptance scenarios: AT-011, AT-012.
Evidence destination: `implementation/evidence/T006.md`.

## T007 — Verified identity, tenancy, and memberships

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T002, T006

Integrate selected authentication and enforce current workspace/resource membership.

**Definition of done:**

- An authenticated human sees and mutates only permitted workspace resources across clients.
- Cross-tenant IDs, revoked sessions, and forged actor/role fields cannot disclose or modify data.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-013, REQ-014. Acceptance scenarios: AT-013, AT-014.
Evidence destination: `implementation/evidence/T007.md`.

## T008 — Effect service runtime and typed errors

**Phase:** F0 · **Owner:** backend · **Status:** todo
**Dependencies:** T001, T002

Implement process-level Effect runtime, configuration, services, and adapter boundaries.

**Definition of done:**

- Startup validates configuration and services expose typed failures with scoped resource cleanup.
- Missing secrets, interrupted work, and malformed requests fail closed without content leakage.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-015, REQ-016. Acceptance scenarios: AT-015, AT-016.
Evidence destination: `implementation/evidence/T008.md`.

## T009 — Native timeline and pending sends

**Phase:** F1 · **Owner:** native · **Status:** todo
**Dependencies:** T004, T005

Implement channel navigation, composer, timeline, thread view, and pending-operation display.

**Definition of done:**

- Cached navigation and durable local sends work while the agent engine is stopped.
- Slow network, oversized content, variable row heights, and failed sends do not freeze input or discard drafts.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-017, REQ-018. Acceptance scenarios: AT-017, AT-018.
Evidence destination: `implementation/evidence/T009.md`.

## T010 — Shared messaging transaction and outbox

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T007, T008

Implement authorized message create/edit/delete with durable idempotency and transactional outbox.

**Definition of done:**

- One operation produces one accepted message/revision and one consistent event history.
- Lost responses, duplicate requests, changed idempotency payloads, and concurrent edits do not duplicate or overwrite work.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-019, REQ-020. Acceptance scenarios: AT-019, AT-020.
Evidence destination: `implementation/evidence/T010.md`.

## T011 — Resumable selective synchronization

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T009, T010

Connect native cache to authorized snapshots, scoped cursors, and live events.

**Definition of done:**

- Reconnect converges messages, read state, and pending intent without losing local drafts.
- Expired cursors, revoked membership, late commits, duplicates, and tombstones cannot leak or resurrect content.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-021, REQ-022. Acceptance scenarios: AT-021, AT-022.
Evidence destination: `implementation/evidence/T011.md`.

## T012 — Task and versioned work-contract model

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T002, T006, T008

Implement task lifecycle, immutable contract revisions, owners, criteria, and source references.

**Definition of done:**

- A message becomes a bounded task with a human-confirmed current contract.
- Changing acceptance criteria or authority cannot mutate an already-bound contract invisibly.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-023, REQ-024. Acceptance scenarios: AT-023, AT-024.
Evidence destination: `implementation/evidence/T012.md`.

## T013 — Authoritative policy and dispatch guards

**Phase:** F1 · **Owner:** security · **Status:** todo
**Dependencies:** T007, T012

Implement server/broker policy facts and command-specific guards beyond the pure reference.

**Definition of done:**

- Current owner, agent, source, action, environment, audience, and policy permissions intersect before dispatch.
- Client-supplied approval flags or permission facts never become trusted authority.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-025, REQ-026. Acceptance scenarios: AT-025, AT-026.
Evidence destination: `implementation/evidence/T013.md`.

## T014 — Durable coordinator baseline and replay

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T008, T012

Select one shared durability backend and implement run coordination/versioning tests.

**Definition of done:**

- Run state survives worker restarts and durable approval waits with one current coordinator.
- History replay, worker upgrade, and duplicate delivery cannot create a second authoritative run.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-027, REQ-028. Acceptance scenarios: AT-027, AT-028.
Evidence destination: `implementation/evidence/T014.md`.

## T015 — Deterministic simulated agent

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T012, T013

Implement a no-network/no-shell simulator with success, blocked, failed, and cancelled cases.

**Definition of done:**

- The simulator produces stable artifacts/evidence through the real task/run interfaces.
- Simulation is visible and cannot accept credentials or perform actual external actions.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-029, REQ-030. Acceptance scenarios: AT-029, AT-030.
Evidence destination: `implementation/evidence/T015.md`.

## T016 — Native workroom and progress cards

**Phase:** F1 · **Owner:** native · **Status:** todo
**Dependencies:** T009, T012, T015

Build task/workroom views with compact channel updates and optional detailed activity.

**Definition of done:**

- A teammate can see outcome, owner, environment, state, blockers, and artifacts without opening a terminal.
- High-volume activity never floods the human timeline or steals scroll/focus.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-031, REQ-032. Acceptance scenarios: AT-031, AT-032.
Evidence destination: `implementation/evidence/T016.md`.

## T017 — Protected artifact upload and versions

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T007, T012

Implement upload tickets, byte/digest verification, immutable versions, and audience policy.

**Definition of done:**

- Permitted users can inspect exact versioned artifacts and protected derivatives.
- Incomplete, oversized, wrong-digest, cross-task, and unauthorized uploads do not become accepted artifacts.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-033, REQ-034. Acceptance scenarios: AT-033, AT-034.
Evidence destination: `implementation/evidence/T017.md`.

## T018 — Version-bound review and acceptance

**Phase:** F1 · **Owner:** backend · **Status:** todo
**Dependencies:** T013, T016, T017

Implement review comments, evidence states, request-changes, and immutable human acceptance.

**Definition of done:**

- Acceptance names exact contract/artifact versions and preserves historical decisions.
- Agent success, stale artifact versions, or unauthorized reviewers cannot auto-accept work.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-035, REQ-036. Acceptance scenarios: AT-035, AT-036.
Evidence destination: `implementation/evidence/T018.md`.

## T019 — Authenticated browser reviewer

**Phase:** F1 · **Owner:** web · **Status:** todo
**Dependencies:** T007, T018

Create a focused browser experience for permitted task context, artifacts, comments, and decisions.

**Definition of done:**

- A second person reviews work without installing the desktop app or relying on the initiator being online.
- Forwarded/expired/revoked links and changed approval bindings cannot broaden access.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-037, REQ-038. Acceptance scenarios: AT-037, AT-038.
Evidence destination: `implementation/evidence/T019.md`.

## T020 — First complete two-human vertical slice

**Phase:** F1 · **Owner:** product · **Status:** todo
**Dependencies:** T011, T015, T019

Integrate and record the complete synthetic checkout request-to-accepted-result workflow.

**Definition of done:**

- Two users complete the flow without narrated assistance or manual database edits.
- Offline/restart, failed-check, and changed-artifact variations remain clear and safe.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-039, REQ-040. Acceptance scenarios: AT-039, AT-040.
Evidence destination: `implementation/evidence/T020.md`.

## T021 — Authenticated broker capability protocol

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T003, T006, T013

Implement narrowly scoped broker grants, transport authentication, and request validation.

**Definition of done:**

- Every tool request is bound to run generation, permitted resources, expiry, and action intent.
- Guessing a socket path, replaying a grant, or forging a session frame cannot execute a tool.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-041, REQ-042. Acceptance scenarios: AT-041, AT-042.
Evidence destination: `implementation/evidence/T021.md`.

## T022 — Local sandbox and process enforcement

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T021

Enforce filesystem, process, environment, network, output, and resource boundaries.

**Definition of done:**

- A disposable local run executes permitted commands without host-wide authority.
- Symlink traversal, malicious repository scripts, secret inheritance, and broker bypass attempts are contained.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-043, REQ-044. Acceptance scenarios: AT-043, AT-044.
Evidence destination: `implementation/evidence/T022.md`.

## T023 — Connector credentials and installation

**Phase:** F2 · **Owner:** integrations · **Status:** todo
**Dependencies:** T007, T013

Implement a selected provider install with secret handles, scopes, rotation, and revocation.

**Definition of done:**

- Users can install the least-privilege integration required for one project and see data destinations.
- Token expiry, webhook replay, revoked scopes, and unintended token audiences fail safely.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-045, REQ-046. Acceptance scenarios: AT-045, AT-046.
Evidence destination: `implementation/evidence/T023.md`.

## T024 — First real coding-agent adapter

**Phase:** F2 · **Owner:** integrations · **Status:** todo
**Dependencies:** T015, T022, T023

Replace one simulated adapter with one tested real provider on a disposable project.

**Definition of done:**

- The real adapter produces bounded changes/artifacts through the same task/run contract.
- Unsupported capabilities, malformed tool output, and requested privilege expansion are explicit and denied by default.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-047, REQ-048. Acceptance scenarios: AT-047, AT-048.
Evidence destination: `implementation/evidence/T024.md`.

## T025 — Transactional action intents and approvals

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T013, T014, T017

Implement exact action hashing, human decisions, atomic claim/consume, and provider reconciliation.

**Definition of done:**

- A valid current decision authorizes one specific publication/action with a stable operation ID.
- Concurrent consumption, replay, changed destination/artifact, and uncertain provider results cannot duplicate or broaden effects.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-049, REQ-050. Acceptance scenarios: AT-049, AT-050.
Evidence destination: `implementation/evidence/T025.md`.

## T026 — Usage reservations and spending limits

**Phase:** F2 · **Owner:** backend · **Status:** todo
**Dependencies:** T008, T013

Implement durable budget accounts, unique reservations, charge settlement, and cost visibility.

**Definition of done:**

- Budget checks reserve before dispatch and reconcile actual usage, including failed attempts.
- Concurrent reservations, duplicate settlements, and estimate overruns cannot hide spend or continue unlimited work.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-051, REQ-052. Acceptance scenarios: AT-051, AT-052.
Evidence destination: `implementation/evidence/T026.md`.

## T027 — Cancellation and process lifecycle

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T014, T024, T026

Implement requested/acknowledged/stopped cancellation with child process cleanup.

**Definition of done:**

- Users can stop new work and understand which in-flight operations are still reconciling.
- A provider timeout or process signal does not falsely report already-completed external effects as undone.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-053, REQ-054. Acceptance scenarios: AT-053, AT-054.
Evidence destination: `implementation/evidence/T027.md`.

## T028 — Isolated hosted execution runner

**Phase:** F2 · **Owner:** infrastructure · **Status:** todo
**Dependencies:** T014, T021, T025, T026

Implement disposable cloud environments with bounded resources, scoped secrets, and network controls.

**Definition of done:**

- Shared work can continue independently of a laptop under one durable coordinator.
- Host sockets, cloud metadata, broad credentials, and noisy-neighbor resource escapes are unavailable.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-055, REQ-056. Acceptance scenarios: AT-055, AT-056.
Evidence destination: `implementation/evidence/T028.md`.

## T029 — Checkpoint handoff and generation fencing

**Phase:** F2 · **Owner:** backend · **Status:** todo
**Dependencies:** T027, T028

Implement verified checkpoints, quiescence, ownership CAS, generation changes, and continuation.

**Definition of done:**

- Supported work resumes locally/cloud-side from a known checkpoint with resolved destination credentials.
- Delayed old-runner output and unresolved external actions cannot create split authority or blind retries.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-057, REQ-058. Acceptance scenarios: AT-057, AT-058.
Evidence destination: `implementation/evidence/T029.md`.

## T030 — Inspectable context and source permissions

**Phase:** F2 · **Owner:** backend · **Status:** todo
**Dependencies:** T007, T012, T017

Implement source manifests, use-for-model/publish distinctions, and invalidation dependencies.

**Definition of done:**

- Users inspect/correct the sources a run uses and their model/output destinations.
- Private source summaries, stale ACLs, and hidden source titles cannot leak into broader audiences.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-059, REQ-060. Acceptance scenarios: AT-059, AT-060.
Evidence destination: `implementation/evidence/T030.md`.

## T031 — Local and server permission-aware search

**Phase:** F2 · **Owner:** backend · **Status:** todo
**Dependencies:** T011, T030

Implement fast exact/keyword/scoped search with protected snippets and clear partial-result states.

**Definition of done:**

- Authorized cached results arrive promptly and server results extend them without disrupting selection.
- Revoked/deleted content, autocomplete, result counts, and cross-account caches do not reveal private sources.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-061, REQ-062. Acceptance scenarios: AT-061, AT-062.
Evidence destination: `implementation/evidence/T031.md`.

## T032 — Decision-focused inbox and notifications

**Phase:** F2 · **Owner:** native · **Status:** todo
**Dependencies:** T011, T016

Implement deduplicated mentions, review requests, blockers, and grouped run milestones.

**Definition of done:**

- Users can identify what needs action and still access chronological/unfiltered activity.
- Repeated agent updates, reconnects, and muted channels do not create notification floods.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-063, REQ-064. Acceptance scenarios: AT-063, AT-064.
Evidence destination: `implementation/evidence/T032.md`.

## T033 — Adapter and model-destination conformance

**Phase:** F2 · **Owner:** integrations · **Status:** todo
**Dependencies:** T024, T030

Create versioned capability tests and clear provider/model destination selection.

**Definition of done:**

- Supported adapters declare and pass their actual read/write/cancel/checkpoint/reporting capabilities.
- An unsupported provider capability or unapproved model destination cannot silently fall back to broader behavior.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-065, REQ-066. Acceptance scenarios: AT-065, AT-066.
Evidence destination: `implementation/evidence/T033.md`.

## T034 — Isolated generated previews

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T017, T021

Implement protected preview origins, access controls, lifecycle, and isolated rendering.

**Definition of done:**

- Permitted reviewers inspect a generated preview without exposing privileged application interfaces.
- Preview scripts cannot access broker routes, app cookies, arbitrary local ports, or forbidden network destinations.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-067, REQ-068. Acceptance scenarios: AT-067, AT-068.
Evidence destination: `implementation/evidence/T034.md`.

## T035 — Cross-component observability and safe diagnostics

**Phase:** F2 · **Owner:** infrastructure · **Status:** todo
**Dependencies:** T014, T026

Implement correlated metadata, metrics, alerts, and opt-in redacted diagnostics.

**Definition of done:**

- Operators trace a run/action and identify queues, costs, failures, and recovery state.
- Default logs/telemetry never include raw prompts, source text, credentials, or sensitive URLs.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-069, REQ-070. Acceptance scenarios: AT-069, AT-070.
Evidence destination: `implementation/evidence/T035.md`.

## T036 — Native and system performance regression suite

**Phase:** F2 · **Owner:** quality · **Status:** todo
**Dependencies:** T020, T024, T031, T032

Build named-hardware release-mode benchmarks and streaming/load fixtures.

**Definition of done:**

- Launch, cached switch/search, input, and concurrent streams are measured against declared budgets.
- Idle redraw loops, unbounded caches, log floods, and background work cannot hide behind average latency.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-071, REQ-072. Acceptance scenarios: AT-071, AT-072.
Evidence destination: `implementation/evidence/T036.md`.

## T037 — Native media risk spike

**Phase:** F0 · **Owner:** native · **Status:** todo
**Dependencies:** T003, T004

Verify selected media SDK, device permissions, and platform integration feasibility.

**Definition of done:**

- A reproducible test demonstrates basic native media capture/render/lifecycle on the first platform.
- SDK availability alone is not accepted as proof of screen-sharing or accessibility support.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-073, REQ-074. Acceptance scenarios: AT-073, AT-074.
Evidence destination: `implementation/evidence/T037.md`.

## T038 — Calls and audio collaboration

**Phase:** F3 · **Owner:** native · **Status:** todo
**Dependencies:** T007, T037

Implement join/leave, mute, device selection, reconnect, and accessible call controls.

**Definition of done:**

- Native/browser participants maintain usable audio with clear device/capture state.
- Device changes, sleep, network loss, and permission revocation do not leave hidden capture active.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-075, REQ-076. Acceptance scenarios: AT-075, AT-076.
Evidence destination: `implementation/evidence/T038.md`.

## T039 — Screen sharing and media privacy

**Phase:** F3 · **Owner:** native · **Status:** todo
**Dependencies:** T038

Implement source selection, visible sharing/stop, capture permissions, and degraded states.

**Definition of done:**

- Users know what is shared and can reliably stop it across the supported platform matrix.
- Recording/transcription remains off without implemented consent, access, and retention controls.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-077, REQ-078. Acceptance scenarios: AT-077, AT-078.
Evidence destination: `implementation/evidence/T039.md`.

## T040 — Signed desktop updates and rollback

**Phase:** F2 · **Owner:** infrastructure · **Status:** todo
**Dependencies:** T003, T007

Build signed packages, update verification, staged rollout, and data-compatible recovery.

**Definition of done:**

- A client upgrades reproducibly while preserving drafts and compatible cache data.
- Unsigned/corrupt updates and interrupted migrations do not execute or destroy user work.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-079, REQ-080. Acceptance scenarios: AT-079, AT-080.
Evidence destination: `implementation/evidence/T040.md`.

## T041 — Windows compatibility and native acceptance

**Phase:** F3 · **Owner:** native · **Status:** todo
**Dependencies:** T004, T040

Complete Windows build/input/accessibility/notification/update acceptance for the supported baseline.

**Definition of done:**

- Published Windows support matches actual recorded build and behavior evidence.
- Framework source support is not substituted for tested end-user workflows.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-081, REQ-082. Acceptance scenarios: AT-081, AT-082.
Evidence destination: `implementation/evidence/T041.md`.

## T042 — Linux compatibility and native acceptance

**Phase:** F3 · **Owner:** native · **Status:** todo
**Dependencies:** T004, T040

Complete selected Linux display/input/accessibility/package acceptance.

**Definition of done:**

- Published Linux support names tested distributions/backends and known limitations.
- Untested compositor/assistive-technology behavior is not presented as supported parity.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-083, REQ-084. Acceptance scenarios: AT-083, AT-084.
Evidence destination: `implementation/evidence/T042.md`.

## T043 — Staged migration and structured export

**Phase:** F3 · **Owner:** integrations · **Status:** todo
**Dependencies:** T011, T023, T031

Implement import preflight, permission/identity mapping, checkpoints, and reconciliation report.

**Definition of done:**

- Authorized content imports/export reproducibly with provenance and explicit omissions.
- Private channels, inaccessible files, repeated pages, and malformed identities cannot silently leak or duplicate.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-085, REQ-086. Acceptance scenarios: AT-085, AT-086.
Evidence destination: `implementation/evidence/T043.md`.

## T044 — Narrow Slack coexistence bridge

**Phase:** F3 · **Owner:** integrations · **Status:** todo
**Dependencies:** T023, T025, T043

Bridge selected task requests/outcomes with origin tracking and loop prevention.

**Definition of done:**

- Teams can evaluate one project without moving all daily communication.
- Mirrored edits/deletes, retries, and source revocation do not create loops or broaden access.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-087, REQ-088. Acceptance scenarios: AT-087, AT-088.
Evidence destination: `implementation/evidence/T044.md`.

## T045 — Saved items, reminders, and scheduled sends

**Phase:** F3 · **Owner:** backend · **Status:** todo
**Dependencies:** T011, T025, T032

Implement durable attention utilities and scheduling with named-timezone semantics.

**Definition of done:**

- Users can save work and schedule permitted messages/reminders across devices.
- Revoked access, DST changes, duplicate scheduling, and stale permissions cannot send unauthorized messages.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-089, REQ-090. Acceptance scenarios: AT-089, AT-090.
Evidence destination: `implementation/evidence/T045.md`.

## T046 — Profiles, status, and notification preferences

**Phase:** F3 · **Owner:** native · **Status:** todo
**Dependencies:** T011, T032

Implement status/profile, focus schedules, per-channel preferences, and multi-workspace routing.

**Definition of done:**

- Ordinary team communication preferences work predictably across clients.
- Muted/private content does not leak in push previews or cross-account notification routing.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-091, REQ-092. Acceptance scenarios: AT-091, AT-092.
Evidence destination: `implementation/evidence/T046.md`.

## T047 — Mobile messaging and safe review participation

**Phase:** F3 · **Owner:** mobile · **Status:** todo
**Dependencies:** T019, T025, T032

Deliver protected notifications, messaging/status, and understandable mobile decisions.

**Definition of done:**

- A teammate can participate in critical communication/review without the desktop app.
- A small screen cannot obscure important approval details or allow stale/revoked decisions.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-093, REQ-094. Acceptance scenarios: AT-093, AT-094.
Evidence destination: `implementation/evidence/T047.md`.

## T048 — Daily communication edge-case completeness

**Phase:** F3 · **Owner:** native · **Status:** todo
**Dependencies:** T009, T010, T017

Close threads, reactions, files, read state, navigation, formatting, and large-history gaps.

**Definition of done:**

- Target teams complete ordinary daily chat workflows with consistent semantics.
- Deleted thread roots, group membership changes, long content, and concurrent edits preserve privacy and user intent.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-095, REQ-096. Acceptance scenarios: AT-095, AT-096.
Evidence destination: `implementation/evidence/T048.md`.

## T049 — Collaborative documents

**Phase:** F3 · **Owner:** backend · **Status:** todo
**Dependencies:** T011, T017, T018

Add versioned collaborative documents with explicit permissions and CRDT scope.

**Definition of done:**

- Concurrent document editing converges without mixing document content with approval authority.
- Offline merges cannot grant permissions, mutate accepted evidence versions, or bypass deletion policy.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-097, REQ-098. Acceptance scenarios: AT-097, AT-098.
Evidence destination: `implementation/evidence/T049.md`.

## T050 — Lightweight lists and work views

**Phase:** F3 · **Owner:** product · **Status:** todo
**Dependencies:** T011, T012

Add useful list/board views over existing tasks rather than a competing task database.

**Definition of done:**

- Teams organize work with shared task identity and current permissions.
- View-specific edits cannot diverge from task state or expose hidden tasks.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-099, REQ-100. Acceptance scenarios: AT-099, AT-100.
Evidence destination: `implementation/evidence/T050.md`.

## T051 — Versioned reusable workflow recipes

**Phase:** F4 · **Owner:** backend · **Status:** todo
**Dependencies:** T024, T025, T030, T035

Turn reviewed successful work into owned, versioned, permission-declared recipes.

**Definition of done:**

- An installed recipe revalidates input, policy, budget, and approvals on every run.
- Recipe updates and schedules cannot auto-grant new tools or continue under revoked ownership.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-101, REQ-102. Acceptance scenarios: AT-101, AT-102.
Evidence destination: `implementation/evidence/T051.md`.

## T052 — Guests and externally shared spaces

**Phase:** F4 · **Owner:** security · **Status:** todo
**Dependencies:** T007, T025, T043

Implement explicit external membership, history-sharing, artifact, and revocation rules.

**Definition of done:**

- Permitted outside reviewers collaborate without obtaining general workspace access.
- Joining a group or forwarding a link cannot reveal previously private history by accident.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-103, REQ-104. Acceptance scenarios: AT-103, AT-104.
Evidence destination: `implementation/evidence/T052.md`.

## T053 — Enterprise SSO integration

**Phase:** F4 · **Owner:** backend · **Status:** todo
**Dependencies:** T007

Add enterprise identity configuration and audited administrative controls.

**Definition of done:**

- Organization identity policy applies across native/browser/mobile sessions.
- Misconfigured issuer/audience, account linking, and admin role changes do not widen access.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-105, REQ-106. Acceptance scenarios: AT-105, AT-106.
Evidence destination: `implementation/evidence/T053.md`.

## T054 — Provisioning and deprovisioning

**Phase:** F4 · **Owner:** backend · **Status:** todo
**Dependencies:** T025, T030, T053

Implement idempotent provisioning, role/group changes, offboarding, and owned-agent handling.

**Definition of done:**

- Removing a user stops future authority and identifies agents/workflows requiring reassignment.
- Out-of-order provisioning and stale sessions cannot resurrect revoked access.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-107, REQ-108. Acceptance scenarios: AT-107, AT-108.
Evidence destination: `implementation/evidence/T054.md`.

## T055 — Retention, holds, and deletion propagation

**Phase:** F4 · **Owner:** security · **Status:** todo
**Dependencies:** T007, T017, T031, T043

Implement owner-approved content lifecycle and qualified-review boundaries for holds/discovery.

**Definition of done:**

- Deletion affects permitted stores, indexes, derivatives, and export rules with documented limitations.
- Immutable logs, offline devices, and backups are not falsely represented as instantly purged.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-109, REQ-110. Acceptance scenarios: AT-109, AT-110.
Evidence destination: `implementation/evidence/T055.md`.

## T056 — Audit exports and privileged support access

**Phase:** F4 · **Owner:** security · **Status:** todo
**Dependencies:** T025, T035, T055

Implement content-minimal audit history, authorized exports, and scoped support workflows.

**Definition of done:**

- Admins can investigate authorized events without default access to every private message.
- Support tools or exports cannot bypass resource policy or leak secrets/source text.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-111, REQ-112. Acceptance scenarios: AT-111, AT-112.
Evidence destination: `implementation/evidence/T056.md`.

## T057 — Company-managed runners

**Phase:** F4 · **Owner:** infrastructure · **Status:** todo
**Dependencies:** T028, T029, T053

Implement registration, outbound control, credential rotation, revocation, and image policy.

**Definition of done:**

- Customer infrastructure executes bounded shared work with clear data/model destinations.
- A local runner is not marketed as full self-hosting or allowed to create its own shared authority.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-113, REQ-114. Acceptance scenarios: AT-113, AT-114.
Evidence destination: `implementation/evidence/T057.md`.

## T058 — Enterprise execution-policy console

**Phase:** F4 · **Owner:** product · **Status:** todo
**Dependencies:** T013, T026, T054

Expose scoped model/tool/destination/budget/approval policies with revisions and audit.

**Definition of done:**

- Admins understand and change enforced execution policy with safe preview and rollout.
- Policy edits cannot silently preserve stale approvals or leave forbidden work running unchecked.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-115, REQ-116. Acceptance scenarios: AT-115, AT-116.
Evidence destination: `implementation/evidence/T058.md`.

## T059 — Regional data placement and restore strategy

**Phase:** F4 · **Owner:** infrastructure · **Status:** todo
**Dependencies:** T028, T055, T057

Design and validate selected region/residency options across all data-bearing systems.

**Definition of done:**

- Claims match databases, artifacts, workflow histories, backups, telemetry, and model routing.
- Single-region labels cannot hide cross-region or external-provider transfers.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-117, REQ-118. Acceptance scenarios: AT-117, AT-118.
Evidence destination: `implementation/evidence/T059.md`.

## T060 — Public SDK and webhook platform

**Phase:** F4 · **Owner:** integrations · **Status:** todo
**Dependencies:** T002, T023, T025

Publish versioned scoped APIs, SDK fixtures, webhook delivery/replay, and developer sandbox.

**Definition of done:**

- Third parties build safe task/artifact/approval integrations using stable contracts.
- API clients have no privileged back door around the same human/agent permissions.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-119, REQ-120. Acceptance scenarios: AT-119, AT-120.
Evidence destination: `implementation/evidence/T060.md`.

## T061 — Reviewed integration marketplace

**Phase:** F5 · **Owner:** security · **Status:** todo
**Dependencies:** T051, T056, T060

Implement publisher/version trust, manifests, review, quarantine, and revocation.

**Definition of done:**

- Installed extensions disclose and stay within their permitted execution/data scope.
- Popularity or connector text cannot substitute for executable-code review and isolation.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-121, REQ-122. Acceptance scenarios: AT-121, AT-122.
Evidence destination: `implementation/evidence/T061.md`.

## T062 — Bounded multi-agent delegation

**Phase:** F5 · **Owner:** backend · **Status:** todo
**Dependencies:** T029, T033, T051

Implement tested implementer/reviewer or alternative-attempt patterns with child budgets.

**Definition of done:**

- Parallel work improves accepted outcomes after accounting for review and cost.
- Recursive delegation and shared working-copy mutations cannot escape authority or budget limits.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-123, REQ-124. Acceptance scenarios: AT-123, AT-124.
Evidence destination: `implementation/evidence/T062.md`.

## T063 — Privacy-aware activation and outcome analytics

**Phase:** F2 · **Owner:** product · **Status:** todo
**Dependencies:** T020, T035

Implement content-free product events and definitions for accepted shared outcomes.

**Definition of done:**

- Teams can measure activation, retention, review effort, rework, and cost consistently.
- Task splitting, failed-run exclusion, or content-bearing telemetry cannot inflate or leak metrics.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-125, REQ-126. Acceptance scenarios: AT-125, AT-126.
Evidence destination: `implementation/evidence/T063.md`.

## T064 — Transparent usage and pricing experiment

**Phase:** F3 · **Owner:** product · **Status:** todo
**Dependencies:** T026, T063

Expose reconciled usage and run controlled pricing tests without unlimited-cost promises.

**Definition of done:**

- Customers understand human collaboration charges versus execution usage and estimates.
- Provider overages, duplicate charges, and failed attempts are not hidden from accounting.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-127, REQ-128. Acceptance scenarios: AT-127, AT-128.
Evidence destination: `implementation/evidence/T064.md`.

## T065 — Design-partner onboarding and paid pilots

**Phase:** F2 · **Owner:** product · **Status:** todo
**Dependencies:** T019, T020, T063

Run one-project onboarding, reviewer invitations, baseline workflow comparisons, and feedback.

**Definition of done:**

- Teams repeatedly return without founder narration and identify measurable useful outcomes.
- Demo praise alone is not treated as retention or willingness to switch.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-129, REQ-130. Acceptance scenarios: AT-129, AT-130.
Evidence destination: `implementation/evidence/T065.md`.

## T066 — Backup, restore, and disaster rehearsal

**Phase:** F2 · **Owner:** infrastructure · **Status:** todo
**Dependencies:** T010, T014, T017

Implement protected backups and rehearsed recovery of data/artifacts/workflow authority.

**Definition of done:**

- Recovery objectives are supported by actual restore evidence and owner approval.
- Restored old workers and leases cannot resume as current owners without reconciliation.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-131, REQ-132. Acceptance scenarios: AT-131, AT-132.
Evidence destination: `implementation/evidence/T066.md`.

## T067 — Independent execution security review

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T024, T025, T028, T030, T034, T040

Perform cross-surface adversarial review with actual implementation and negative tests.

**Definition of done:**

- Critical authorization, isolation, update, and publication controls have independent evidence.
- Unexecuted tests, reference functions, or simulated paths cannot be marked as production security coverage.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-133, REQ-134. Acceptance scenarios: AT-133, AT-134.
Evidence destination: `implementation/evidence/T067.md`.

## T068 — Replacement release candidate

**Phase:** F3 · **Owner:** lead · **Status:** todo
**Dependencies:** T036, T039, T040, T043, T045, T046, T047, T048, T065, T066, T067, T069, T070

Assemble a supported release candidate with evidence, rollback, support, and known limits.

**Definition of done:**

- Target teams can conduct daily work without critical prior-workspace blockers.
- Marketing does not claim unsupported platforms, full parity, or certifications without evidence.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-135, REQ-136. Acceptance scenarios: AT-135, AT-136.
Evidence destination: `implementation/evidence/T068.md`.

## T069 — Distribution license and dependency audit

**Phase:** F2 · **Owner:** security · **Status:** todo
**Dependencies:** T001, T003, T023, T037

Complete SBOM, license notices, exact pins, supply-chain review, and signing prerequisites.

**Definition of done:**

- Every redistributed component and executable dependency has recorded provenance and permitted use.
- One framework license is not generalized to unrelated repository code or bundled assets.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-137, REQ-138. Acceptance scenarios: AT-137, AT-138.
Evidence destination: `implementation/evidence/T069.md`.

## T070 — Communication replacement gap audit

**Phase:** F3 · **Owner:** product · **Status:** todo
**Dependencies:** T039, T043, T045, T046, T047, T048

Reconcile the parity register against current sources and observed pilot switching blockers.

**Definition of done:**

- Each supported parity claim names tested behavior, plan/platform scope, and evidence.
- A candidate inventory or webhook endpoint is not presented as exhaustive Slack equivalence.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-139, REQ-140. Acceptance scenarios: AT-139, AT-140.
Evidence destination: `implementation/evidence/T070.md`.

## T071 — Independent end-to-end acceptance

**Phase:** F2 · **Owner:** quality · **Status:** todo
**Dependencies:** T020, T067

Have reviewers outside the implementation workstream exercise real workflows and faults.

**Definition of done:**

- A non-initiator confidently reviews current artifacts and understands evidence/limitations.
- A narrated founder demo or manually edited backend is not accepted as a working product flow.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-141, REQ-142. Acceptance scenarios: AT-141, AT-142.
Evidence destination: `implementation/evidence/T071.md`.

## T072 — Leadership benchmark and expansion decision

**Phase:** F5 · **Owner:** product · **Status:** todo
**Dependencies:** T059, T060, T062, T068, T071

Compare retained use, accepted quality, human effort, cost, and operational trust against alternatives.

**Definition of done:**

- Expansion decisions are supported by measured user value and clear economics.
- Feature count, native rendering, or agent activity alone is not treated as industry leadership.
- Record actual implementation evidence and obtain appropriate review; a scaffold/reference alone is insufficient.

Requirements: REQ-143, REQ-144. Acceptance scenarios: AT-143, AT-144.
Evidence destination: `implementation/evidence/T072.md`.
