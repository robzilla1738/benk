# Scaffold implementation sequence
These entry tickets complement, not replace, the original 72-ticket backlog.
Map work to its historical requirements and dependencies before implementation.

## BENK-001 — Establish genuine build evidence
Resolve dependencies, generate authentic npm/Cargo locks, run active and historical
tests, compile GPUI on macOS, repair formatting and record exact tool versions.
Launch the shell on a real host and record navigation/close behavior separately.
Acceptance: no fabricated hashes, no removed tests, no replacement stack. No
Linux/Windows/visual/accessibility claim inferred from a macOS compile.

## BENK-002 — Native composer and accessible focus
Implement selection, clipboard, undo/redo, multiline input, IME, keyboard submit,
draft recovery, accessible roles/names and logical focus. Test CJK composition,
right-to-left text, large pastes and keyboard-only navigation. Acceptance: no lost
input or accidental submission during composition; actual assistive-technology
behavior evidence. The seed's composer description is not an input control.

Status (2026-10-08, see docs/PROGRESS.md): the composer is now a real
`EntityInputHandler` — selection, clipboard, undo/redo, multiline, IME marked
text, Enter/Shift+Enter, per-channel drafts — with 4 headless GPUI tests over
the real dispatch path. Still open: soft-wrap for long lines, durable draft
recovery (in-memory only today), CJK/RTL/large-paste tests, keyboard-only
navigation audit, and live assistive-technology evidence.

## BENK-003 — Virtualized timeline and durable local state
Introduce bounded rendering, selective SQLite reads off the interaction path,
schema migrations, account/workspace scope, drafts and outbox. Preserve scroll
position across pagination/arrivals. Test search deletion, reconnect and restart.
Acceptance: draft recovery, no wrong-account leakage, representative measured
latency. The original performance targets are not existing measurements.

## BENK-004 — Typed authenticated IPC and engine lifecycle
Implement framed/versioned messages, caller authentication, bounded streams,
cancellation, crash recovery and error mapping. Separate client/engine databases;
start engine on demand. Acceptance: malformed, oversized and stale sessions are
rejected; engine failure cannot take down chat; renderer never gets a shell.

## BENK-005 — Authenticated shared review
Add real identity/tenancy, transactional task/artifact persistence and a scoped
browser reviewer. Bind approval to exact content and current permissions. Test
two real test accounts, not a canned actor. Another person must understand and
review the request/output/evidence without reconstructing a transcript. Private
context stays private; changed artifacts stale applicable approvals.

## BENK-006 — One real adapter, only after gates
After original security gates: one supported agent and isolated runner. Add
verified grants, credential scope, filesystem/network/process rules, budgets,
cancellation, fencing, audit, idempotency and uncertain-outcome reconciliation.
Acceptance: unauthorized actions fail closed, stopped work cannot keep mutating,
and the product honestly reports external operations whose results are uncertain.

Defer marketplaces, autonomous swarms, a complete IDE, arbitrary process migration
and broad enterprise parity until the narrow vertical slice is reliable.
