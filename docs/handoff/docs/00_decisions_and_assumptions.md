# 00 — Decisions, assumptions, and scope control

## User-authoritative decisions
The product is a Slack-type collaboration workspace for humans and agents together. It starts as a desktop application. It must be exceptionally fast and powerful, cover local development and cloud execution, and ultimately pursue 1:1 Slack functional breadth. Rust/GPUI and TypeScript/Effect are fixed selections. Industry-leading quality is the ambition; no competitive outcome is presumed.

## Baseline decisions to implement
Desktop: GPUI rendering with a Rust-owned local collaboration store and narrow background workers. TypeScript/Effect handles application services, provider adapters, and local/cloud orchestration. A separate restricted Rust broker enforces local execution capabilities. PostgreSQL is the authoritative shared database; object storage carries larger artifacts. SQLite caches authorized collaboration data. Temporal coordinates shared cloud workflows by default. A lightweight React/TypeScript browser client supports early reviewers.

Start as a modular application, not a microservice fleet. Separately schedule interactive APIs, execution workers, media, and indexing so they cannot starve each other. Share public wire contracts and fixtures across runtimes, not framework internals. Use a provider-neutral agent capability model while testing only a small initial adapter set.

## Reversible defaults
| Question | Working default | Revisit when |
|---|---|---|
| Initial audience | Software/product teams of 5–30 people. | Five design partners show a different coherent segment. |
| First complete workflow | Customer checkout report to reviewed code/preview. | Observed partner work supports a better repeatable workflow. |
| First shipping native platform | macOS Apple Silicon. | Founder/design-partner device mix requires another order. |
| Cross-platform obligation | Compile/input/accessibility probes on Windows and Linux during foundations; supported releases later. | A platform-specific dependency invalidates the approach. |
| Browser | Review, comments, approvals, and selected messages early. | Repeated workflows require broader participation. |
| Runtime | Packaged supported Node LTS for Effect engine; exact release selected in foundation. | Compatibility/performance evidence favors another runtime. |
| Authentication | Managed OIDC-compatible identity adapter initially. | Enterprise tenancy or purchasing needs demand changes. |
| Cloud | Single region per workspace; managed database/object storage/workflow service preferred. | Residency or measured scale requires expansion. |
| Agent adapter | Simulator first, then one partner-preferred supported coding agent. | Conformance tests show another adapter is needed. |
| Brand, billing vendor, production domain | Unselected. | Explicit owner decision; these do not block core implementation. |

These are defaults, not additional promises from Robert. A build agent should not repeatedly ask for them before beginning reversible work.

## Explicit exclusions from the first release
No general-purpose IDE replacement, arbitrary computer use, unrestricted agent-to-agent messaging, model training pipeline, full self-hosted control plane, universal Slack-app compatibility, transparent live-process migration, or claims of cryptographic end-to-end encryption. Mobile execution environments and broad autonomous business actions are later work.

## Dependency correction
Effect's official release post dated 30 September 2026 supersedes beta-era advice. The API reference checked on 8 October showed 4.0.2. The scaffold selects that baseline, but its dependency install was not verified here. Modules with unstable annotations still need version-specific review. GPUI's published 0.2.2 examples and upstream main can differ. Do not combine the published crate with main-branch bootstrap APIs. [S01–S05 in `25_sources_and_verification.md`.]

## Decision status vocabulary
**Fixed** means required by the user. **Accepted baseline** means implement unless an ADR changes it. **Experiment** means compare against a measurable gate. **Deferred** means intentionally not in the current build. **Unverified scaffold** means supplied code not compiled against its external dependencies. This vocabulary must appear in implementation reports and avoid accidental promotion of an idea into a guarantee.

## Leadership criteria
Leadership requires retained teams, useful accepted outcomes, accessible and dependable daily communication, lower review effort, clear permissions, and trustworthy recovery. An attractive demo, large roadmap, or fast rendering framework alone proves none of these.
