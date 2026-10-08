# Development guide
## Toolchains and locks
Active direct pins: TypeScript 5.9.3, Effect 4.0.2, GPUI 0.2.2; Node 24 runtime
line; Rust 1.99.0 toolchain. These are seed choices, not proof of compatibility.
The preparation environment had Node 22.16 and TypeScript 5.8.3 but no Cargo or
package-network access. Pure core tests were run there; dependency-backed Effect
and native compilation were not. Use the verification report for actual evidence.

Generate and review genuine package-lock.json and Cargo.lock on a connected host:

```sh
npm install --ignore-scripts
rustup toolchain install 1.99.0 --profile minimal --component rustfmt --component clippy
cargo generate-lockfile
npm run check
cargo test --locked
cargo check --locked -p benk-desktop
cargo fmt --all
```

Commit those authentic locks after validation. CI uses install fallback until
locks exist, then npm ci/Cargo --locked. Never fabricate integrity hashes. Select
an exact Node patch and immutable workflow/container pins when establishing the
release toolchain. Inspect any required dependency install hook before enabling it.

## Native platform
macOS is the initial GPUI compile target. Install Xcode command-line tools and
platform components required by the exact published GPUI release. Upstream Zed
main may differ from that release. Run cargo check, then launch cargo run -p
benk-desktop on an actual graphical host. Linux and Windows desktop readiness is
not claimed: follow that release's platform prerequisites and test independently.
Do not replace GPUI because an SDK/library is missing.

Known macOS specifics on the current development host:
- GPUI 0.2.2 compiles Metal shaders via `xcrun metal`, which ships with full
  Xcode, not Command Line Tools alone. If `xcrun metal` is missing, set
  `DEVELOPER_DIR=/Applications/Xcode-beta.app/Contents/Developer` (or the
  installed Xcode path) before cargo builds of `benk-desktop`.
- `npm run dev:desktop` / `cargo run -p benk-desktop` launches the shell.

Headless UI tests: `cargo test -p benk-desktop` uses GPUI `test-support`
(`TestAppContext`/`VisualTestContext`; dev-dependency feature). `simulate_input`
and `simulate_keystrokes` drive the real dispatch path — keymap, actions, focus
and the platform input handler — without a real window server. Prefer them over
OS-level keystroke injection (assistive access/frontmost-app state makes
synthetic input nondeterministic).

GPUI dispatch reentrancy: while a window dispatches an event, GPUI checks it out
of `cx.windows`; `AnyWindowHandle::update` inside an `App::on_action` handler
then fails with "window not found". Handlers that need `&mut Window` (e.g.
focus) must defer via `cx.defer` + handle update — see `deferred_window_update`
in `apps/desktop/src/main.rs`. Element-level `.on_action` handlers are a valid
alternative but only receive actions on the focused dispatch path.

## Command scope
npm run demo: pure TypeScript fixture; no model, tools, database or cloud service.
npm run demo:reference: original standalone demo using its preserved built files.
npm run dev:control-plane: read-only loopback server; health and synthetic snapshot
only. Use numeric 127.0.0.1, not a public bind. BENK_DEV_PORT selects another port.
Origin/Host rejection is development protection, not production authentication.
npm run dev:agent-engine: separate Effect simulation process.
cargo run -p benk-broker -- --capabilities: disabled/empty capability report.
Every other broker invocation exits 77 without invoking a tool.

## Python verification
```sh
python3 -m venv .venv
. .venv/bin/activate
python3 -m pip install -r docs/handoff/requirements-qa.txt
npm run check:handoff
```
On Windows use the platform activation script and python where python3 is absent.
The historical verifier rebuilds its reference core; verify historical integrity
before running a compiler upgrade that might change generated files. Do not
silently rewrite historical evidence to match new outputs.

## Data and credentials
The simulator needs none. PostgreSQL is optional for future persistence work;
see infra/README.md. Historical SQL is a reference, not a migration chain. Add
versioned migrations and transactional authorization tests before real writes.
Never put production data, provider keys or credentials into the demo.

## Troubleshooting
Module not found: install workspaces and build domain before simulator before
services; root scripts enforce order. Effect errors: check the pinned v4 APIs
and TypeScript >=5.9 guidance rather than copying v3 Context.Tag examples. Keep
strict typing. GPUI errors: compare the published pinned release, not unpinned
main. Missing tools are failures/unverified status, not passing checks.

Composer IME, selection, accessibility, focus, notification behavior, sleep/wake
and scroll position require real platform evidence. None is complete because a
window compiles. Record those separately from unit tests and benchmarks.
Headless tests cover the logical input path; assistive-technology behavior
(VoiceOver etc.) still needs live evidence.
