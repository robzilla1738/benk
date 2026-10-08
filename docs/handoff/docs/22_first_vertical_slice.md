# 22 — First vertical slice: checkout request to accepted change

## Goal
Demonstrate the product's distinguishing workflow with two humans and one visibly simulated agent, then replace the simulator with one real adapter after the execution gates. This is the first cohesive product milestone, not a miniature of every future feature.

## Synthetic fixture
Workspace: `ws_alpha`. Channel: `chn_checkout`. Product owner: `usr_product`. Engineering reviewer: `usr_reviewer`. Agent: `agt_simulator`. Repository resource: `repo_checkout`. Task: `task_checkout`. No actual company data, provider credentials, or production access is involved. Contract examples live in `contracts/examples/valid/`.

The reported problem is ambiguous checkout error wording. The requested result is a proposed wording/code change with defined checks and human review. The task allows selected repository inspection/change and tests, not deployment or customer communication. Opening a pull request is a later separately permitted action; the structurally valid action fixture is not itself authorized by the initial contract.

## Sequence and expected behavior
**1. Open native application.** Show cached channel messages immediately, a connection indicator, and a working composer. The user can type and navigate while the engine is absent. Save a draft, close/reopen the window, and recover it.

**2. Discuss the request.** Post the customer report, reply in a thread, and turn the source message into a task. A queued send is distinguishable from a server-accepted message. Retry after a forced lost acknowledgment without duplication.

**3. Confirm work contract.** Display goal, owner, criteria, approved repository, permitted actions, execution/model location, budget, and approval policy. Product confirms revision 1. A request to broaden scope becomes a new proposal; it does not silently change permissions.

**4. Start simulated run.** The agent announces one compact status card. It emits known progress, a small synthetic patch, a preview description, and a test log with an explicit simulated marker. It cannot run a real shell, access the user's filesystem, call a model, or open a pull request.

**5. Open review package.** Show exact artifact version, patch summary, defined checks, limitations, and requested review. The timeline remains readable. Detailed activity is available on demand. A failed-check variation remains reviewable but is not described as verified success.

**6. Invite second reviewer.** The engineering reviewer opens an authenticated browser route or a second native session, reads permitted context, and comments on the artifact. A forwarded/revoked link cannot reveal private content.

**7. Change and re-review.** Produce version 2 after requested changes. The version-1 approval is visibly stale and cannot authorize a version-2 action. The reviewer accepts the specific current artifact and criteria; record immutable acceptance history.

**8. Propose publication separately.** Opening a pull request requires an allowed contract/policy action and an exact approval binding. In the simulator this shows a clearly simulated receipt. In the later real adapter slice, it uses an idempotent provider operation and reconciliation logic.

**9. Inspect history.** A third permitted participant can identify the initial request, accepted change, checks, owner, reviewer, and any external action without reconstructing a long chat transcript.

## Required variations before calling this slice complete
Offline draft/send and reconnect; application restart; lost server acknowledgment; permission revoked during review; changed artifact after approval; failed check; agent blocked requesting access; budget reservation failure; cancellation while running; and simulated unknown publication result. Each has an understandable UI and no silent authority expansion.

## Minimal architecture in this slice
GPUI desktop, Rust cache/core, Effect service boundary, structured task/run/artifact contracts, deterministic simulator, and focused browser reviewer. Actual identity/tenancy is required for any multi-user network deployment. A fully local two-person fixture can be used for early interaction testing only if it is explicitly not presented as deployed authentication.

## Not in this slice
Marketplace, multiple model providers, general terminal-as-product, full editor, swarms, screen sharing, mobile execution, enterprise provisioning, transparent process migration, or broad import. The roadmap retains them where relevant, but they are not allowed to obscure the core product test.

## Demo acceptance
A participant who did not build the product can complete the flow without manual database edits or hidden scripts. The second reviewer can explain what was verified and what was not. Native interaction remains responsive during synthetic streams. All simulated elements are labeled. There is a recorded walkthrough, linked automated evidence, known limitations, and a list of real-execution gates still pending.

## Transition to real work
Keep the same contracts and screens. Replace one simulator adapter only after broker isolation, credentials, authorization, budgets, cancellation, approval consumption, and unknown-outcome recovery are implemented. Begin with a disposable repository and read-only actions; then allow bounded working-copy changes; finally enable separately approved external publication. Do not jump from a simulator to an unrestricted agent with home-directory access.
