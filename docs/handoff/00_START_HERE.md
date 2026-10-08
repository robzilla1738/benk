# Human–Agent Workspace — build handoff
**Prepared for Robert · 8 October 2026 · Version 1.0**

## The assignment
Build a native, exceptionally responsive collaboration workspace in which humans and agents discuss work, execute it in approved local or cloud environments, inspect evidence, and accept outcomes. The destination includes Slack-level communication breadth. The initial wedge is a cross-functional software/product team completing a real request together.

**Fixed technology decisions:** Rust + GPUI for the desktop; TypeScript + Effect for application services and agent orchestration. SQLite is the local collaboration store. PostgreSQL is the shared authority. A narrow Rust execution broker enforces tool capabilities. Temporal is the initial cloud durability choice, subject to a documented foundation-stage decision.

**Do not replace GPUI with Electron, Tauri, or a webview shell. Do not replace Effect with an unrelated backend framework.** Propose evidence-backed changes through an architecture decision record rather than silently changing the brief.

## Start in this order
1. Read `AGENTS.md` and `01_BUILD_AGENT_PROMPT.md`.
2. Read `docs/00_decisions_and_assumptions.md`, `docs/01_product_strategy.md`, and `docs/22_first_vertical_slice.md`.
3. Read `docs/05_architecture.md`, `docs/08_domain_and_state_machines.md`, and `docs/11_permissions_security_and_privacy.md`.
4. Inspect `planning/backlog.json`, `planning/requirements.json`, `planning/release_gates.json`, and `contracts/README.md`.
5. Run the handoff verification commands below. Then execute the first ready backlog ticket; do not attempt the whole platform in one pass.

For a human-readable browser index, open `INDEX.html`. It contains an embedded, searchable reading view of the narrative documents and links to the machine-readable files. It needs no internet connection.

## What is actually supplied
This is a specification and implementation handoff, plus a tested **reference core** and clearly labeled **unverified integration scaffolds**. It is not a completed application, deployed service, certified sandbox, or benchmarked GPUI build.

The reference TypeScript modules model state transitions, dispatch policy checks, budgets, synchronization ordering, idempotency, and coordinator handoff. They are executable examples for testing invariants, not a substitute for transactional persistence, identity verification, or operating-system isolation.

The GPUI and Effect scaffolds show the intended boundaries. Neither dependency-backed scaffold was compiled in the preparation environment: Rust/Cargo were absent, and external package installation was unavailable. Their READMEs identify the exact follow-up commands. See `verification/RESULTS.md` for what was and was not run.

## Verification commands
From this directory:

```sh
python scripts/verify_handoff.py
python scripts/test_sqlite.py
cd reference/core
npm run build
npm test
cd ../..
python scripts/validate_contracts.py
```

The first two Python commands use only the standard library. Contract validation needs the packages in `requirements-qa.txt`. The reference core build needs TypeScript; the preparation environment used TypeScript 5.8.3 and Node 22.16.0. `npm install` in `reference/core` supplies the declared compiler on a connected machine. No product credentials are needed for these checks.

## The first deliverable
A user opens a cached native workspace, discusses a customer checkout issue, proposes a bounded task, runs a deterministic **simulated** agent, receives a review package, and has a second human review the specific artifact version. The UI clearly labels simulation. Real tools, credentials, and external writes remain disabled until their gates pass.

Then replace the simulator with **one** supported adapter behind the same contracts. Do not expand into a marketplace, autonomous swarms, unrestricted computer use, or broad enterprise functionality before the vertical slice is reliable.

## Source-of-truth hierarchy
The user's fixed decisions override recommendations. Within this package, the decisions document and accepted ADRs govern architecture; domain contracts govern wire shapes; the state-machine document governs behavior; requirements and release gates govern acceptance; backlog tickets govern sequence. A scaffold never overrides a requirement. Log contradictions in `templates/IMPLEMENTATION_REPORT.md` and resolve before changing a shared contract.

## Important corrections and limits
Effect 4.0 was released on 30 September 2026; the checked API reference showed 4.0.2. This package supersedes earlier beta-era Effect guidance. GPUI's published 0.2.2 documentation and upstream main are not identical API surfaces; use the selected release consistently and verify platform capabilities. Sources and verification dates are recorded in `docs/25_sources_and_verification.md`.

Performance numbers, staffing bands, and product hypotheses are **targets and planning assumptions**, not established results. Industry leadership must be earned through measured quality, retention, and accepted outcomes. No package can guarantee product-market fit.
