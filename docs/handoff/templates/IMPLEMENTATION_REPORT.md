# Implementation report: <ticket ID / title>

## Scope and source requirements
Ticket, requirement IDs, acceptance IDs, relevant ADRs; explicit non-goals.

## Repository state
Base commit, actual branch/worktree, pre-existing changes preserved, toolchain versions.

## Change summary
Files changed and user-observable behavior. Identify mock/simulator/reference code clearly.

## Verification actually performed
| Command / scenario | Environment / fixture | Result | Evidence path |
|---|---|---|---|
| <actual command> | <actual environment> | pass / fail / not run | <path> |

## Security and consistency review
Actor/tenant/resource boundary, stale revisions, retries, budgets, cancellation, deletion, log redaction, and external-action implications. Write not applicable with a reason where appropriate.

## Migration / rollback
Schema/protocol version implications, compatibility window, reversible steps, data protection.

## Limits and blocked checks
What is not implemented or not verified; missing dependencies/credentials; no invented success.

## Review and next step
Reviewer, unresolved findings, gate status, next dependency-ready ticket.
