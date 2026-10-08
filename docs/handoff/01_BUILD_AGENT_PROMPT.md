# Copy-ready instruction for the lead build agent

You are implementing a native human–agent collaboration product from the attached handoff. Read `00_START_HERE.md` and `AGENTS.md` first. Preserve Rust + GPUI for desktop and TypeScript + Effect for application/orchestration services. Do not replace this with a web chat clone.

Your goal is the complete first vertical slice in `docs/22_first_vertical_slice.md`, not the entire final platform in one pass. Inspect the existing repository and tools, run the supplied reference checks, read the relevant architecture/security/state-machine specs, and use `planning/backlog.json` to identify the first unblocked ticket. Begin with foundation and native-risk tickets. Preserve user files and maintain a progress ledger.

Create a working repository with clear native, contract, service, broker, and integration boundaries. First make cached native communication and a simulated request-to-reviewed-artifact workflow reliable. Then integrate one real agent behind scoped capabilities only after identity, authorization, approvals, and isolation are implemented and tested.

Treat `planning/requirements.json`, `planning/acceptance_catalog.json`, and `planning/release_gates.json` as acceptance inputs. Keep a trace from each change to requirement and test IDs. The supplied TypeScript core is a reference implementation of selected invariants, not a server. The Effect and GPUI scaffolds are uncompiled starting points; verify dependencies and their APIs before relying on them. Do not claim the scaffolds already work.

At each checkpoint, report actual commands and results, useful product behavior completed, remaining risks, and the next bounded task. When a run ends, save enough state for a different agent to resume without repeating discovery. Do not mark planned capabilities complete. Do not deploy, spend, expose credentials, delete data, or change permissions without authorization.

Start by producing a short repository/toolchain assessment and then implement the first safe, unblocked ticket. Avoid spending the whole session rewriting the plan.
