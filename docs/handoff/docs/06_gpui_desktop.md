# 06 — GPUI desktop engineering specification

## Dependency discipline
GPUI is a fixed user choice. The included scaffold targets published `gpui = "=0.2.2"`, whose checked documentation exposes `Application::new`. Current upstream main uses a different bootstrap path in its example. Select a tested release or exact revision and keep documentation, platform crate versions, examples, and patches consistent. Do not use `*` dependencies or unpinned main. Record the Rust compiler and native SDKs in a lock/compatibility report after the first build. [S04, S05.]

The scaffold was not compiled here. It is deliberately a small bootstrap surface, not a claim that native messaging is implemented. Do not import editor code from the wider Zed repository without individual license review.

## Native module design
`desktop-shell` owns windows, menus, tray/dock behavior, deep links, notifications, and update UI. `client-core` owns cache/query/sync APIs. `composer` owns the structured document and input-method integration. `timeline` owns virtualized message rows and measurement caches. `workroom` coordinates task/artifact/review views. `platform` wraps OS identity storage, background availability, permissions, media devices, and diagnostics. `broker-client` speaks the narrow execution protocol.

Use explicit ownership and lifetime boundaries. A closed window releases observers and heavyweight views. Multi-window state references shared immutable/domain state rather than passing mutable UI contexts across threads. Backend completions update view state only through the selected GPUI execution pattern.

## Composer risk spike
Before polishing navigation, implement composition-safe input, cursor movement, selection, clipboard, undo, and draft persistence. Test Chinese/Japanese/Korean IMEs, combining characters, right-to-left text, grapheme deletion, multi-line paste, code blocks, mentions, long URLs, and attachment drag/drop. Text shaping, selection, and accessibility must agree about offsets. Store textual anchors in a defined coordinate system; never confuse UTF-8 bytes, UTF-16 code units, and graphemes.

For structured content, serialize allowlisted nodes from `contracts/schemas/domain.schema.json`. Display user input safely without interpreting HTML as privileged UI. Spellcheck and OS text services may be staged, but broken composition is a ship blocker.

## Timeline architecture
Use stable message IDs and cached layout measurements keyed by message revision, width, font scale, and density. Invalidate only affected rows. Preserve the top visible item's ID and intra-item offset when older messages load or an image acquires dimensions. When at the bottom, new content can follow; otherwise show a new-message affordance without moving the reader. Thread panels maintain independent anchors.

Virtualize offscreen rows and attachments. Avoid re-parsing all Markdown or recomputing all message heights on each token update. Task cards update a bounded summary state; logs have a separate viewer and ring buffer. Link/image previews load lazily and can be disabled on constrained connections.

## Local data path
Single logical writer per SQLite collaboration store, with priority for drafts, sends, revocations, and accepted sync batches. Index maintenance is incremental and cancellable. Query results contain immutable view data, not live database handles. Separate account caches and encryption-key references. Purge the selected account's cached data on logout according to policy without deleting other accounts.

The Effect engine must not open the collaboration database. It can submit explicit commands through the Rust client or shared API. Its own execution journal is independent.

## Platform lifecycle
Distinguish app window closed, application running in background, machine locked, machine suspended, and machine offline. Default: closing the window does not silently create a persistent execution daemon. Any background-runner option requires an explicit setting and status indicator. Reconcile leases on resume before dispatching more work. Never assume wall-clock continuity after sleep.

Handle signed deep links through strict route parsing and user confirmation for sensitive actions. Store tokens through OS credential services; no long-lived secret in localStorage-style files. Updates must be signed and staged with rollback/recovery. A failed update preserves drafts and data schema compatibility.

## Accessibility and media
Build semantic labels, roles, focus behavior, and state announcements for the actual selected GPUI version. Test VoiceOver, NVDA, and Linux assistive technology on the supported matrix rather than extrapolating from framework support. Screen sharing and calls need an independent platform spike; a media SDK does not supply product-quality native device permissions and controls automatically.

## Performance instrumentation
Instrument launch, first cached render, channel switch, input-to-frame, search, memory, cache/index work, and idle wakeups. Run release-mode benchmarks on declared hardware. Count the whole application process tree and separately identify execution/model workloads. Native framework choice is not benchmark evidence.
