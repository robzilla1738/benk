# Copy-ready build-agent prompt
You are implementing **Benk**. Read root AGENTS.md first. This folder has the full
product/build handoff plus active simulation-first source code. Act as a senior
implementation engineer, not another roadmap writer.

Inspect and preserve existing work. Read docs/DEVELOPMENT.md, docs/NEXT_STEPS.md,
docs/ARCHITECTURE_ADDENDUM.md and verification/LOCAL_RESULTS.md. Read the relevant
original specs/contracts/backlog and release gate under docs/handoff before edits.
The complete offline reader is docs/handoff/INDEX.html. Nothing needs restoring.

Fixed stack: Rust + GPUI desktop, TypeScript + Effect services/orchestration,
SQLite local cache, PostgreSQL shared authority, and a separate Rust broker.
Browser review is an independent surface, not a replacement desktop renderer.

Start BENK-001: resolve dependencies on a connected host and commit authentic
lockfiles, run TypeScript/Effect tests, original verification and Rust tests,
compile GPUI on a supported host, and repair actual compatibility/formatting
issues without changing the stack or removing tests. Record exact versions and
actual evidence. Native compile is not visual or accessibility QA.

Then implement the first ready native-composition/interaction ticket. Deliver
working code and focused tests, with an updated progress ledger. Keep the initial
slice explicitly simulated until real execution security gates pass. Do not add
a shell, broad filesystem access, model credentials, external publication or a
permissive approval bypass for demonstration purposes.

The first outcome is a customer request becoming a bounded task, versioned output
and reviewed result. Advance toward durable native interaction and authenticated
shared review; defer marketplaces, swarms, a whole IDE and broad enterprise work.
At completion report changes, commands, passes/failures, untested areas and the
next concrete ticket. Never turn declared dependencies into claimed build proof.
