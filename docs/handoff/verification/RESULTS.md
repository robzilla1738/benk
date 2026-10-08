# Verification results and limits

**Preparation date:** 8 October 2026  
**Scope:** reference code, executable local schema, and handoff contracts—not a finished application.

## Actually run

| Check | Result | Evidence |
|---|---|---|
| TypeScript reference compilation | Passed with TypeScript 5.8.3 | `reference/core/tsconfig.json`; source and compiled output included |
| Domain reference tests | **232 passed; 0 failed** | [Node test log](reference-core-tests.tap) |
| SQLite cache/search/isolation tests | **16 passed; 0 failed** | [SQLite test log](sqlite-tests.log) |
| JSON Schema positive/negative fixtures | **22 passed; 0 failed** | [Contract results](contract-fixture-results.json) |

The 232 domain tests include exhaustive task/run edge combinations and selected permission, approval, budget, idempotency, synchronization, and coordinator handoff cases. Many are deliberately small invariant checks. They do not represent 232 completed features or integration scenarios.

The 16 SQLite tests execute the local schema and test search changes, deletion/tombstones, rollback, account/workspace isolation, visibility, foreign-key behavior, exact sequence ordering, and pending-operation uniqueness. They do not test a native client or production synchronization server.

The 22 contract fixtures include structurally valid samples and expected rejections such as nonhuman approvals, invalid generations, unsafe links, malformed timestamps/digests, deleted content retaining its body, and imprecise numeric wire sequences. Schema validation is not current authorization or cryptographic verification.

## Environment
Python 3.13.5; Node v22.16.0; Version 5.8.3; SQLite 3.46.1. No Rust/Cargo. External package installation was unavailable. The reference TypeScript core has no runtime dependencies and was built using the installed compiler.

## Explicitly not verified
The GPUI application scaffold, Rust domain seed, and Effect integration scaffold were **not compiled**. No authentic lockfiles could be generated for those dependency-backed integrations. Their READMEs contain the required follow-up steps and disclose possible API/environment adjustments.

The PostgreSQL schema was **not executed against PostgreSQL**. It is a reviewed design reference with deliberately documented missing production enforcement and role setup. No HTTP service, native collaboration application, browser reviewer, broker sandbox, model integration, media integration, signed release, CI deployment, cloud infrastructure, or compliance certification is claimed complete.

Performance numbers throughout the handoff are **engineering targets**, not benchmark results. The supplied simulator is not a model or a real tool invocation. A task reducer accepting a state transition does not constitute a human review.

## Reproduction
Install `requirements-qa.txt` for contract checks and the declared compiler in `reference/core` when needed. Run `python scripts/verify_all.py` from the package root. `scripts/check_integrity.py` verifies original-file hashes after extraction; editing original files naturally changes those hashes.

Final static coherence, fixture-generation, and extraction checks are recorded in [package checks](package-checks.json). These results are separate from application release gates, all of which remain unpassed.

## Final package checks
Static coherence passed for all ticket dependencies, requirements/scenario traces, gate membership, source references, JSON/OpenAPI definitions, and Markdown links. A deterministic 100,000-message fixture was generated and contained 100,000 indexed rows. The offline index has unique navigation IDs and no external asset dependencies; its visual/browser behavior was not manually tested. See `package-checks.json` and associated logs.
