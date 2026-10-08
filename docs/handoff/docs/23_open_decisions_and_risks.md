# 23 — Open decisions and risk management

## Decisions that should not block safe implementation
Brand/name, production domain, billing vendor, exact cloud provider, identity-provider vendor, first non-simulated adapter, and initial partner list remain unselected. Use reserved identifiers and dependency adapters. Do not create real accounts or buy infrastructure on the user's behalf merely to fill a field.

The first shipping platform defaults to macOS Apple Silicon, but Windows/Linux risks are probed during foundations. This is a reversible planning choice, not a user instruction to exclude other platforms. A partner device survey can change release order without changing the fixed GPUI architecture.

## Foundation decisions requiring evidence
Select exact GPUI/Rust/native SDK versions through a reproducible build. Confirm Effect package/API compatibility and lockfiles. Compare the initial Temporal baseline with an Effect-native durable engine only through the required failure/versioning suite. Choose the local sandbox implementation and its threat boundary before executing untrusted tools. Define supported model-data destinations and permissions for the first real integration.

## Highest risks
Scope expansion can produce a broad but unreliable clone; gate the first complete workflow. Native text, accessibility, and cross-platform APIs can consume unexpected engineering effort; spike them before polishing the shell. Shared execution can leak private context; separate read/use/publish permissions and test every surface. Local/cloud handoff can create double authority; use generations, leases, CAS, and reconciliation. Agents can generate high cost/noise without useful work; reserve budgets and measure accepted outcomes/review effort.

Switching friction can overwhelm the feature advantage; allow project-level coexistence and browser review. Framework churn can break reproducibility; pin and test dependencies. Provider protocols can expose inconsistent capabilities; maintain adapter conformance. A reference policy function can be mistaken for actual security; keep untrusted facts away from it and enforce the real broker/transaction boundaries.

## Risk register use
`planning/risk_register.json` records likelihood, impact, owner discipline, trigger, mitigation, fallback, and release effect. Update it after spikes and incidents. Do not hide a risk by marking a feature "done" when only a mock path works.

## Escalation rules
Escalate decisions that change the fixed stack, authorize new external side effects, weaken isolation, create material spending, affect customer retention/deletion, or expand legal/commercial obligations. Routine reversible coding choices follow existing ADRs and defaults. An unresolved platform capability can be staged behind an honest support matrix; an unresolved cross-tenant read cannot be staged into production.

## Exit from uncertainty
Each experiment should have a question, fixture, success threshold, evidence path, owner, and decision deadline relative to its milestone. A spike ending "seems fine" is not enough. Record what was tested, what failed, the chosen baseline, and the next validation needed.
