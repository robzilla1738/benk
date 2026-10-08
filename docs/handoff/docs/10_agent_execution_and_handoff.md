# 10 — Agent execution, environments, and handoff

## Adapter capabilities
An adapter declares support for read-only inspection, structured plans, file changes, command execution, interruption, checkpoints, cost reporting, streaming, and remote sessions independently. Unsupported capabilities are visible and disabled. Provider neutrality does not imply identical behavior or transferable hidden state. A conformance suite runs against each selected adapter/version before enabling it.

Start with a deterministic simulator, then one coding-agent adapter. The simulator uses synthetic artifacts and cannot spawn processes or accept secrets. Label it throughout the UI. Later adapters may use ACP for supported coding-agent sessions and MCP for tool access, but those protocols do not replace the product's authorization model. ACP's checked introduction still flags full remote support as work in progress. [S12, S13.]

## Execution preparation
Resolve the task contract revision, verified owner, permitted sources, environment, model destination, and budget. Build a context manifest of immutable references and ACL information. Prepare an isolated working copy and reproducible environment. Reserve budget and acquire the run generation/lease before dispatch. No task may gain broader access because a prompt says a resource is necessary.

The engine itself must have no route around the broker. Separating processes is insufficient if the model-facing engine still has an unrestricted shell or host filesystem. Constrain the engine runtime and all adapter subprocesses; the broker mediates effectful operations under explicit grants. For an early trusted-local developer experiment, clearly label any weaker isolation and prohibit presenting it as the production security model.

## Local environment
A user chooses approved paths; never default to the home directory. Resolve paths safely, reject traversal, enforce symlink/realpath boundaries at operation time, and handle TOCTOU risk with appropriate OS primitives. Child processes receive an allowlisted environment and bounded CPU, memory, process count, filesystem, output, and network access. Repository scripts and dependency installers are untrusted code too.

A worktree organizes changes; it does not sandbox them. An agent-generated preview runs at an isolated origin/container without privileged application bridges. Opening a preview must not expose local credential endpoints or arbitrary host ports.

## Cloud environment
Create an isolated per-run workload with scoped credentials, resource ceilings, network policy, artifact quotas, and cleanup deadlines. Select an isolation technology with a security review and operational tests; an ordinary container alone is not a blanket assurance. Do not mount the host container socket. Network egress is denied or allowlisted by capability; dependency retrieval uses a reviewed path. Secret injection is short-lived and never included in checkpoint archives.

## Shared coordinator and local runner
The shared workflow owns task progress and action decisions. The local runner receives bounded commands with generation, expiry, operation ID, approved action digest, and capability references. A disconnected local runner may finish only work allowed by its unexpired bounded grant; no further shared external publication or privilege expansion occurs without authority. Lease expiry prevents new dispatch; it cannot magically revoke an already completed external effect.

## Handoff protocol
Quiesce supported work; stop new dispatch; finish or reconcile in-flight actions; create a checkpoint manifest; upload permitted artifact data; compare-and-set coordinator ownership and increment generation; resolve credentials in the target; verify environment compatibility; resume from the documented checkpoint. Handoff completion is acknowledged only after the new owner is ready. The old owner cannot resume on its own if the new owner fails; reacquisition requires another authoritative transition.

Checkpoint fields: task/run IDs and contract revision; source repository/commit; allowed patch/artifact references and digests; environment definition/version; supported continuation point; pending action IDs and outcomes; context manifest; required secret **handles**; cost/reservation state; and adapter compatibility metadata. No raw tokens, hidden model reasoning, OS process memory, or unreviewed private source archive.

Development-container configuration is a candidate portable environment format, not a guarantee of identical execution across architectures or operating systems. [S14.]

## Budget and stopping
Reserve conservatively before work. Set limits for calls, output, wall time, concurrent runs, and money. Actual provider charges may exceed estimates; record the actual amount and pause further work instead of discarding overage. Cancellation has requested/acknowledged/stopped stages. Terminate child process groups where supported and report processes that could not be stopped. Reconciliation may continue after cancellation to identify already-performed effects.

## Completion
A run publishes versioned outputs and check evidence with known limitations. The task enters review when its policy permits, not automatically accepted. A human can take over, create a new attempt, compare bounded alternatives, or accept exact versions. Store reusable workflow definitions only after an owner reviews required permissions, inputs, and evaluation cases.
