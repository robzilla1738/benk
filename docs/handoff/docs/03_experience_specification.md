# 03 — Information architecture and interaction specification

## Primary navigation
The native shell has Workspace switcher, Inbox, Channels, Projects, Work, and Search. Settings, identity, connection state, and running-work indicators remain reachable but quiet. The default visual hierarchy privileges people and decisions over machine traces. Use system fonts initially; ship no copied commercial font files. Design tokens for spacing, typography, focus, contrast, and density are explicit and shared conceptually with the browser.

**Inbox:** decisions awaiting the current user, blocked delegated work, mentions, and materially changed followed work. Keep chronological/unfiltered access. Ranking cannot be the only way to find important content. Each row explains why it is present and provides a clear action.

**Channels:** familiar conversation timeline with thread navigation and a compact task/run summary card. Agent activity appears as meaningful state changes. Token streams, tool calls, and logs are inside the run view.

**Projects:** approved context, repositories, environments, team decisions, and work. Project membership does not magically authorize private source material attached by an individual.

**Work:** tasks across permitted projects, filterable by owner, state, due date, agent, environment, and review status. Start with list and detail; defer elaborate customizable dashboards.

## Three-pane behavior
A narrow navigation pane sits left; the primary conversation/work list is central; a contextual detail pane opens right. On smaller windows, detail replaces the main pane with a clear back action. Do not squeeze critical approval content into an unreadable sliver. Resizable panes remember state per device. A separate window may show a review without sharing mutable UI entities unsafely across windows.

## Message composer
Use a structured document model, not untrusted HTML. Support paragraphs, basic emphasis, inline code, code blocks, links, lists, mentions, and attachments through an allowlisted schema. Preserve a plain-text fallback for accessibility, notifications, and export. Mentions are stable IDs with display labels, not textual authority. Paste sanitizes content and discloses oversized attachments. Enter-to-send is configurable; Shift+Enter inserts a newline. Never intercept Enter while an input-method composition is active.

Drafts autosave locally on change, with debounced persistence off the UI thread. Switching channels retains separate drafts. An offline send creates a durable queued operation and immediately visible pending message. Failed sends expose retry/edit/discard. A cross-device edit conflict retains both texts rather than silently overwriting.

## Create work from conversation
The context action "Create task" preserves source message references. The proposed contract shows goal, criteria, owner, sources, allowed actions, environment/model destination, budget, and approval points. Advanced fields collapse but remain inspectable. The user confirms scope; the agent does not infer authorization from the wording of a request. Source links have access-aware placeholders for viewers lacking permission.

## Workroom layout
Header: task title, accountable owner, contract revision, task state. Summary: outcome, relevant changes, current blocker. Tabs or sections: Discussion, Artifacts, Evidence, Activity, Context. Environment and budget indicators are visible near execution controls. A terminal is an optional section, not the default home screen.

## Review package
Show artifact version, summary, comparison baseline, checks, unresolved questions, and requested decision. A test result includes invocation, environment, timestamp, exit result, and linked log reference; do not imply all possible behavior was verified. Review comments attach to artifact version and stable position/anchor. Changing an artifact marks earlier approvals stale when their binding no longer matches. Approval chrome is trusted UI and visually separated from agent-generated content.

## Error and empty states
No project: connect or create one, without forcing organization-wide import. No agent: the communication application still works. Runner offline: show last observed state and unavailable execution; offer a supported checkpoint destination, not magical continuation. Budget limit: explain reservation/spend and pause new dispatch. Access revoked: show that the resource is no longer available, clear cached content after processing, and never leak its title in an error. Unknown external result: display reconciliation state and disable blind retry.

## Keyboard and accessibility
Implement semantic roles, labels, focus order, announced state changes, and full keyboard reachability before calling a screen complete. The focused message can be opened, replied to, copied, and acted on without a pointer. Restore focus after closing dialogs. Escape closes the nearest transient surface without losing drafts. Shortcuts vary by platform and avoid IME/system conflicts. See `29_accessibility_and_internationalization.md`.

## Visual direction
Calm, dense enough for professional work, clear enough for nontechnical teammates. Avoid generic AI gradients, decorative agent avatars dominating the timeline, constant animations, and fake progress. Use text labels plus state icons rather than color alone. Provide comfortable and compact density modes, reduced motion, and light/dark themes after the foundational interaction surfaces work. The handoff does not include an approved visual mockup; the design owner should review the actual native composer/workroom before broad implementation.
