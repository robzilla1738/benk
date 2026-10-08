# Package contents and reading routes

## Build immediately
Read `00_START_HERE.md`, `AGENTS.md`, `01_BUILD_AGENT_PROMPT.md`, and `docs/22_first_vertical_slice.md`. Run the reference checks. Then start T001/T002 and the native platform-risk work in dependency order.

## Product and design review
Read `docs/01_product_strategy.md`, `02_product_requirements.md`, `03_experience_specification.md`, `04_communication_and_parity.md`, `14_review_artifacts_and_previews.md`, `15_browser_mobile_and_media.md`, and `20_adoption_pricing_and_metrics.md`.

## Technical architecture review
Read docs 05–12, 16–18, 24, and the accepted ADRs. Inspect `contracts/`, `data/`, `reference/core/`, and the distinctly labeled `scaffolds/`. Confirm the chosen durable engine and native platform baseline at the foundation gate.

## Security and operational review
Read docs 11, 12, 17, 27, 28, 30, and 31; the risk register; exact-action approval/handoff ADRs; and `templates/THREAT_REVIEW.md`. Real execution and external publication are disabled by default.

## Program management
Use `planning/backlog.json`, `requirements.json`, `acceptance_catalog.json`, `traceability.json`, `milestones.json`, `release_gates.json`, `risk_register.json`, and the Markdown backlog. All product tickets start as unimplemented. The 143-row parity register is an auditable candidate inventory, not a claim of achieved or exhaustively verified Slack equivalence.

## Evidence and independent QA
Read `verification/RESULTS.md`. Reproduce checks with `scripts/verify_all.py` and inspect original-file hashes with `scripts/check_integrity.py`. The synthetic fixture generator can create 100,000 messages on demand; that large database is intentionally not embedded in the ZIP.

## Source verification
The primary-source ledger records URLs, access date, supported assumptions, and limitations. Re-check unstable APIs and exact dependency compatibility before implementation. No provider credentials, production accounts, or commercial font files are included.
