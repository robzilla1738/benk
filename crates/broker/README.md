# Fail-closed broker
`cargo run -p benk-broker -- --capabilities` reports an empty capability set.
All other invocations exit 77 without executing a tool. Isolation, authenticated
IPC, verified grants, credential brokering, budgets, fencing, cancellation and
audit controls remain unimplemented. Do not remove default denial for a demo.
