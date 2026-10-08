# 15 — Browser participation, mobile, and media

## Browser reviewer
Deliver an authenticated browser path early: open a shared task, inspect permitted artifacts, comment, review evidence, and approve or request changes. Include selected conversation context and visible version bindings. The browser client must never depend on the initiator's desktop being online to read already-shared artifacts.

Use a focused React/TypeScript application with shared wire contracts and semantic design tokens. It is not a replacement for GPUI desktop. Avoid premature attempts to share every rendering component across web and native. Share validation fixtures, permission behavior, accessibility semantics, and domain rules where appropriate.

A review link conveys routing, not unrestricted data access. Authenticate the recipient; restrict guest scope; revalidate access at each sensitive read/action; expire invitations. Prevent forwarded links from broadening access. Approval UI must display the bound artifact/action version and refresh if it changes while a dialog is open.

## Mobile essentials
Mobile initially supports messages, notifications, task status, compact artifact previews, and decisions that can be safely understood on a small screen. Complex code review may require opening desktop/browser rather than encouraging blind approval. Mobile execution environments and full developer tooling are out of early scope.

Push payloads default to minimal metadata when content could be sensitive. Notification settings, logout, account switching, and device token revocation must be correct. A mobile approval is still a real authenticated, version-bound decision; biometric unlock alone does not authorize an otherwise forbidden action.

## Media plan
Perform a native media spike during foundations, even if calls ship later. LiveKit's Rust SDK is a candidate for real-time media; the application still owns device permissions, controls, rendering, and lifecycle behavior. Verify the selected SDK version/platform support rather than treating repository availability as product readiness. [S15.]

Calls require join/leave, mute, device selection, reconnect, network degradation, active participants, and clear sharing indicators. Screen sharing requires source selection, permission handling, revocation, and a visible stop control. Handle machine sleep, device changes, closed windows, and browser/native participants. Bound media CPU/memory impact so messaging remains responsive.

Recording/transcription is disabled until explicit visible consent, organization policy, storage access, retention, and regional handling are implemented. Do not create an always-recording meeting product accidentally through agent participation.

## Release gates
Browser: current authorization, version-bound review, keyboard/screen-reader flow, expired/revoked links, slow-network usability. Mobile: deduplicated notification routing, protected payloads, session revocation, small-screen decision clarity. Media: supported device/OS matrix, reconnect, privacy indicators, accessible controls, and no unexplained local capture. These are separate deliverables; "desktop-first" does not justify blocking all collaborators behind an install.
