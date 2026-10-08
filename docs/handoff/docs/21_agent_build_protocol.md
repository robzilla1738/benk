# 21 — Build-agent collaboration and session protocol

## Coordinator responsibility
One lead agent maintains the source-of-truth backlog, accepted ADRs, contract versions, and integration branch. It delegates bounded modules with explicit inputs, file ownership, acceptance tests, and forbidden side effects. It does not ask subagents to "build the whole app" independently.

## Session start
Read `AGENTS.md`, the current implementation ledger, the selected ticket, and its linked specs. Inspect repository status and existing code before generating files. Identify tools, supported platforms, network/package availability, and credentials without exposing secret values. Run relevant existing checks before changing behavior. Preserve unrelated user work.

## Task brief
A delegated task names the goal, linked requirement/test IDs, allowed files, interfaces that cannot change, expected artifact, test commands, and stop conditions. A native task should not silently change cloud authorization. An integration task should not add a broad shell capability to make an adapter work. Contract changes require coordinator review and updated cross-runtime fixtures.

## Implementation rhythm
Build a narrow working increment, run tests, inspect behavior, and record evidence. A failing dependency or unavailable tool does not justify an unlabeled mock success. Use a simulator only where the product explicitly supports simulation, and keep its status visible. Do not spend the whole session creating TODO files or rewriting the plan when an unblocked implementation task exists.

## Security-sensitive work
Require an independent reviewer for identity, grants, broker enforcement, connector installation, preview isolation, approval consumption, publication, budgets, sync visibility, and retention. The reviewer reads the actual code and negative tests, not only the implementing agent's summary. A reviewer cannot certify unexecuted native or cloud behavior.

## Evidence discipline
Save command, working directory, environment/version, exit code, test count, artifact path, and known limitations. A test suite that passes only because real execution is disabled should be described that way. State whether outputs are implemented, simulated, stubbed, specified, or externally verified. Keep sensitive logs out of the repository.

## Session end
Update task status and implementation report. List changed files and remaining risks. Record the next unblocked ticket and exact resume commands. Preserve migration/version decisions and unresolved questions. The next agent should not have to infer progress from chat history.

## Guardrails for autonomy
Do not publish deployments, grant accounts, send external communications, install unreviewed executable connectors, rotate credentials, delete data, or incur material costs unless authorized. Do not bypass tests, hide failures, or weaken permission checks to finish a demo. Ordinary reversible code choices can follow this package's defaults; reserve questions for genuinely missing irreversible decisions.

## Definition of done
A ticket is done when its required behavior works in its intended environment, linked positive/negative tests pass, evidence is saved, docs/contracts are updated, and an appropriate reviewer accepts it. A scaffold or reference test alone cannot mark the corresponding product ticket done. See the role prompts and report templates for reusable handoff formats.
