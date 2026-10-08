# GPUI native bootstrap — not compiled here

This is original small scaffold code targeting published `gpui = 0.2.2`, not current upstream main. Verify the exact APIs and supported native SDK on the first chosen platform. A current-main example may use a different application initializer; do not combine dependency generations.

On a machine with the required Rust/native platform toolchain:

```sh
cargo generate-lockfile
cargo check
cargo run
cargo build --release
```

Record `rustc -Vv`, Cargo version, OS, SDK, resolved lockfile, and results in T003 evidence. Commit the authentic lockfile for the application. Dependency-resolution or compile errors are possible because this scaffold could not be checked here. Fix API drift without replacing GPUI. Do not assert screen-reader/input/media support merely because a window opens.

The static colors/layout are placeholders, not the final design system. Before general product UI, implement and test the native composer/IME/focus/accessibility risk surface. No credentials, provider calls, shell execution, persistence, or messaging are implemented here.
