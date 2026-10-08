# SQLite cache seed
Uses the full original SQLite schema with scoped FTS and deletion triggers.
Only an in-memory fixture API is exposed. Persistent migrations, encrypted
storage, drafts, identity and complete sync are not implemented. The original
Python SQLite tests provide executable schema evidence; Rust integration requires
Cargo validation on a connected host.
