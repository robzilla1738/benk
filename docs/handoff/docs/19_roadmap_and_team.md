# 19 — Roadmap, sequencing, and team ownership

## Gate-driven roadmap
**F0 — Foundations.** Audit repository/toolchain, pin compatible dependencies, prove GPUI text/input/accessibility risks, establish contracts, threat model, local cache, and Effect service boundaries. Decide the durable backend with evidence. Exit with a reproducible native bootstrap and agreed invariants, not a polished marketing screen.

**F1 — Complete simulated vertical slice.** Cached native conversation, work contract, simulated run, artifact/evidence package, version-bound human review, browser participation, and reliable pending/reconnect behavior. Exit when two people can complete the workflow without a narrated demonstration. Simulation remains visibly labeled.

**F2 — Trusted real team execution.** Verified identity/tenancy, real scoped adapter, broker enforcement, transactional approvals/actions, budgets, cancellation, hosted runner, supported checkpoint handoff, context permissions, and recovery. Exit after real work, independent security review, and fault tests.

**F3 — Daily workspace replacement.** Close critical communication gaps, deliver reliable calls/screen sharing, mobile participation, migration, notifications, updates, and platform support required by target teams. Exit when representative pilot teams can stop using the previous workspace without critical blockers.

**F4 — Enterprise and developer platform.** Company-managed runners, provisioning, policy controls, retention/discovery support, external spaces, workflow SDK, and operational scale. These features are demand-driven; do not claim enterprise certification or full legal compliance from their presence.

**F5 — Ecosystem and advanced workflows.** Reviewed marketplace, richer delegation, advanced orchestration, broader work types, and proven infrastructure expansion. Expand only when simpler workflows show retained value and acceptable economics.

## First 90-day planning envelope
This is an illustrative prioritization window, not a delivery guarantee. First focus on native risk, permission semantics, and the exact request-to-review path. Then complete the two-human simulator workflow, robust local persistence, and browser review. Then replace one simulator boundary with one real adapter only after controls are ready, and run paid/design-partner pilots with measured outcomes. Do not force the calendar to imply broad Slack parity or secure cloud execution before the gates pass.

## Parallel work without integration chaos
Native and backend engineers can work in parallel once contracts and golden fixtures are stable. Security/broker work starts early and is not deferred behind UI polish. A design/product owner validates composer, inbox, contract, and review usability. A quality/reliability owner builds fault tests and collects evidence. An integration owner maintains provider/version conformance. The lead integrator alone resolves cross-boundary contract changes after review.

## Staffing assumptions
A small founding team can build a narrow vertical slice. Simultaneous native cross-platform polish, full communication breadth, secure execution, browser/mobile, and enterprise operations needs dedicated ownership rather than optimistic part-time coverage. Plan at least distinct accountable leads for native engineering, Effect/backend, execution/security, product/design, and quality/release. Team size is a budgeting decision based on actual skills and scope; agent throughput does not remove review or operational responsibility.

## Backlog use
`planning/backlog.json` contains bounded tickets with dependencies, linked requirements, acceptance scenarios, deliverables, and risk notes. Work the first safe unblocked ticket, not the most exciting later feature. Use one coordinator, clear file/module ownership, and independent reviews for security-critical changes. Do not have several agents concurrently edit shared schema/state-machine files without coordination.

## Scope control
A new feature needs a user job, target milestone, cost/complexity assessment, and testable success criterion. Adding a general marketplace, editor, bespoke database, custom version-control system, or large service fleet requires an ADR and explicit owner approval. Maintain both communication completeness and differentiated execution tracks; neither should consume the other invisibly.
