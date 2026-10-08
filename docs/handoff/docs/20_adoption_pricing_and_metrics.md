# 20 — Adoption, pricing hypotheses, and product metrics

## Onboarding path
A new user creates or joins a workspace, chooses one project, connects a selected repository/source with explicit scopes, and completes a bounded task. Do not require a company-wide import or ten integrations. Invite one reviewer through a protected browser link. Let the team keep its editor, issue tracker, and existing chat during evaluation.

Onboarding should explain execution location, model destination, what access was granted, how to stop work, and where to review results. Do this in the moment of use, not as a wall of setup policy. Ordinary messaging remains usable with agents disabled.

## Pilot design
Recruit several teams from one coherent segment and observe real recent workflows. Capture their current baseline: coordination steps, review time, rework, cost, and context loss. Run the same class of task in the product. Use independent acceptance criteria and avoid founder-driven demos. Review reasons users return to their previous system; those are prioritized switching blockers.

## Core metrics
North-star candidate: human-accepted shared outcomes per active team per week. Define an eligible outcome before measuring, so trivial task splitting cannot inflate it. Activation candidate: two humans and one agent complete and accept a real bounded task. Pair these with team retention, paid conversion, review minutes, rework, interventions, safety incidents, and cost per accepted outcome.

Operational quality metrics include crash-free sessions, draft loss, notification delay/duplication, reconnect success, stale-approval rejection, permission incidents, budget reconciliation delay, and unknown external outcomes. Do not replace these with model token counts or number of agent messages.

## Event analytics
Collect content-free events such as `workspace_joined`, `project_connected`, `task_contract_confirmed`, `run_started`, `review_opened`, `review_decision_recorded`, and `task_accepted`. Include opaque IDs, milestone state, elapsed time, environment class, and safe outcome codes. Do not record prompts, private message bodies, repository contents, or full URLs as analytics properties. Track anonymous/cohort metrics only under the organization's privacy settings.

## Pricing hypothesis
Start by testing human collaboration seats plus transparent execution usage. Temporary agents are not automatically human-priced seats. Support workspace budgets and approved customer-provided credentials where practical. Separate model usage, compute, storage, and premium administration. Do not promise unlimited costly execution before observing the economics.

Cost accounting includes failed runs, retries, idle previews, artifact storage, and human review effort. Usage estimates are labeled; final charges reconcile to provider/billing records. Real invoicing, tax, refunds, and legal terms require the selected billing/legal setup and are not implemented by the usage-ledger reference.

## Defensibility and portability
The potential advantage is excellent coordination around permission-aware context, reliable execution, and accepted evidence. Make customer data exportable. Openness of the runner/SDK is a strategic option to improve inspectability and integrations, not an automatically chosen license or a substitute for product value. Keep provider choice real through tested adapters, not a cosmetic dropdown.

## Leadership scorecard
Call the product industry-leading only after representative comparisons show less human effort at equal or better accepted quality, strong repeated preference, dependable native behavior, and understandable control over agents. A feature checklist is an input to that evaluation, not its conclusion.
