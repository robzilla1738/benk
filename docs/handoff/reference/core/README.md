# Executable domain reference

This package is deliberately small and dependency-free at runtime. TypeScript compiles pure domain helpers; Node's built-in test runner checks explicit invariants and negative cases. It is not a cloud API, durable coordinator, authorization database, sandbox, agent runtime, or native application.

```sh
npm install
npm run check
npm run demo
```

On a machine with TypeScript already installed, `npm run build && npm test` requires no package download. The handoff includes compiled `dist/` output so `npm test` and `npm run demo` can also run with Node alone. Rebuild after source changes.

## Important boundaries
`policy.ts` evaluates authoritative preflight facts. It does not authenticate those facts, compute a cryptographic action digest, consume approvals, or prevent a race between check and use. Production must revalidate and claim action/approval/lease/budget state atomically and independently enforce tool capability at the broker.

`sync.ts` models one contiguous, already-authorized stream. Filtered global sequences may contain legitimate gaps; production uses scoped cursors and a snapshot/recovery protocol. Its maps are unbounded for readability and must not become the production cache strategy.

`idempotency.ts` uses a single-process Map. Production requires durable atomic claims, ownership/recovery, retention, and provider reconciliation. `budget.ts` uses exact BigInt arithmetic but not a reservation ledger; settle each real reservation once transactionally. `handoff.ts` proposes a state update, not a distributed compare-and-set or process migration. State reducers validate edges only, not human approval or acceptance guards.

Tests demonstrate these specific rules. They do not prove the complete product meets security, durability, usability, or performance requirements.
