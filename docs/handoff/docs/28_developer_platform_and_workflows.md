# 28 — Developer platform, recipes, and advanced orchestration

## Stable platform surface
Expose typed task, artifact, event, approval-request, and context operations under the same permissions as the product UI. Third-party applications never receive an administrative back door. Version APIs, provide scoped installations, rate limits, webhook delivery/replay semantics, and deprecation notices. The first OpenAPI file covers only the initial vertical slice; platform breadth expands through explicit contracts.

## Reusable workflows
A successful run can be proposed as a recipe with owner, version, declared inputs, permitted sources/tools, environment requirements, budget ceiling, approval points, and evaluation cases. Installing a recipe does not auto-grant its requested scopes. Changes to tools, destinations, or executable instructions trigger re-review. Scheduled runs revalidate membership, credentials, policy, and budget at execution time.

## Workflow composition
Start with a small set of bounded templates: reproduce and propose a fix; summarize a permitted project with citations; generate a release-review package; collect evidence for a defined investigation. Prefer declarative inputs/outputs and durable checkpoints. Do not use an arbitrary scripting escape hatch as the normal workflow language before the security model is mature.

## Multi-agent work
Begin with bounded patterns such as implementer plus reviewer, or two independent proposals compared by a human. Each child run has its own identity, budget, scope, and artifacts. A parent cannot transfer privileges it does not possess. Define mutation ownership to prevent two agents writing to the same working copy. Measure improved accepted quality after review time and cost; more parallel agents are not automatically better.

## Marketplace
A marketplace is late-phase work. Require publisher identity, package/version integrity, permission manifests, review, isolation, update/revocation behavior, abuse reporting, and a clear trust indicator. Reviews or popularity are not a security boundary. A listing should disclose execution/model locations and data destinations. Disable or quarantine compromised integrations without corrupting customer task history.

## SDK experience
Provide a local simulator, golden payloads, contract validator, test workspace, deterministic webhook replay, and examples of safe approval requests. Developers should see typed denied/stale/unsupported states and clear rate-limit behavior. SDKs map into stable language-neutral contracts; they do not leak GPUI, Effect, or workflow-engine internals into the public interface.

## Deliberate exclusions
No universal app compatibility claim, unrestricted computer-control API, arbitrary public tool execution, or autonomous recursive delegation without limits. Advanced capabilities are introduced only when the product can explain and enforce their authority and measure their usefulness.
