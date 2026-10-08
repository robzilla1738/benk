# Benk
**A native collaboration workspace where humans and agents turn requests into reviewed outcomes.**

Rust + GPUI desktop. TypeScript + Effect application services and orchestration.
SQLite local state, PostgreSQL intended shared authority, and a separate
fail-closed Rust execution broker.

This is a simulation-first development foundation, not a completed Slack
replacement or hardened execution service. No model, credentials, external tools,
real teammates, or production approval system are connected.

## Start
The complete original handoff is already expanded in `docs/handoff/`. Begin with
[START_HERE.md](START_HERE.md). Give another agent [docs/BUILD_AGENT.md](docs/BUILD_AGENT.md)
and have it read [AGENTS.md](AGENTS.md) before implementing anything.

```sh
cd benk
npm install --ignore-scripts
npm run doctor
npm run demo
```

Use Node 24 for active development. The original standalone reference demo can
also be run without installation using Node 22 or newer:

```sh
node docs/handoff/reference/core/demo.mjs
```

## Application surfaces
```sh
npm run dev:desktop         # GPUI shell; Rust and native platform prerequisites required
npm run dev:control-plane   # Read-only Effect adapter: 127.0.0.1:4317
npm run dev:agent-engine    # Separate Effect simulation process
```

The server implements only `GET /healthz` and `GET /v1/demo/snapshot`. Do not
expose it publicly. Docker and API keys are not needed for the simulation.

## Layout
| Path | Purpose |
| --- | --- |
| apps/desktop | Native GPUI shell and explicitly simulated task interaction. |
| apps/reviewer-web | Reserved browser participation boundary; not implemented. |
| crates/domain | Rust task reducer and exhaustive edge assertions. |
| crates/local-store | Scoped SQLite/FTS seed using the original schema. |
| crates/broker | Deny-all execution process boundary. |
| packages/domain | Tested TypeScript reference invariants. |
| packages/simulator | Request-to-review fixture, exact-artifact review and tests. |
| packages/contracts | Rules for promoting the shared contract baseline. |
| services/control-plane | Effect service composition and read-only development HTTP adapter. |
| services/agent-engine | Separate Effect process seed; no real adapter enabled. |
| docs/handoff | All original specs, ADRs, tickets, requirements, schemas and reference evidence. |
| docs | Current agent instructions, setup, addendum and implementation sequence. |
| infra | Optional local PostgreSQL. |
| verification | Actual local results and unverified integration boundaries. |

## Validate
```sh
python3 -m venv .venv
. .venv/bin/activate
python3 -m pip install -r docs/handoff/requirements-qa.txt
npm run check
npm run check:handoff
cargo test
cargo test -p benk-desktop   # headless UI tests; desktop is not a default member
cargo check -p benk-desktop
```

Read [DEVELOPMENT.md](docs/DEVELOPMENT.md) for exact command scope, lockfile policy
and platform requirements (including the `DEVELOPER_DIR` Metal toolchain note).
The Rust/GPUI desktop and Effect services now compile and test on macOS arm64 —
see [docs/PROGRESS.md](docs/PROGRESS.md) for the evidence ledger. Latency numbers
in the handoff are targets, not measured results. No release gate is marked
complete.

## Product priority
A customer request becomes a bounded task, a reviewable artifact and an accepted
outcome with another human. Build ordinary communication, fast local interaction
and understandable review before marketplaces or autonomous swarms. Full Slack
communication breadth remains the long-term replacement roadmap.

The historical handoff intentionally keeps its original generic title and
checksums. Active application code and instructions use **Benk**.

No shell in the renderer. No agent-granted permissions. No self-approved external
actions. No private-context publication without authority. See SECURITY.md.
The owner has not selected a product-code license; packages are non-publishable.
