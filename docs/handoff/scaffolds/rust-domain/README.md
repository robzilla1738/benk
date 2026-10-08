# Rust domain seed — not compiled here

Run `cargo test` with an installed Rust toolchain. This crate has no third-party dependencies but was not compiled because Cargo was absent. It illustrates only task edges. Port the full required domain semantics and compare against `contracts/state-machines.json` and shared golden fixtures. Do not interpret an allowed reducer edge as authentication, authorization, atomic persistence, or actual human approval.
