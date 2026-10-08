# Integration scaffolds — UNCOMPILED

These intentionally small starting points express architectural boundaries. They are not a runnable product and have not been dependency-installed or compiled in the preparation environment. Rust/Cargo were absent; external package installation was unavailable. No lockfile was invented.

- `gpui-desktop/`: native window baseline for the selected published GPUI API, not a composer or collaboration client.
- `effect-service/`: Effect service composition and an explicitly simulated proposal. No HTTP server, authenticated session, database, or real execution.
- `rust-domain/`: minimal typed task reducer and tests to seed native parity; not yet compiled.
- `infrastructure/`: disposable local PostgreSQL configuration requiring explicit image/password inputs; no production deployment.

The build agent must run each README's commands, resolve actual compatibility, commit authentic lockfiles, and record evidence before promoting a scaffold into product code. Specifications and contracts override shortcuts in scaffolds.
