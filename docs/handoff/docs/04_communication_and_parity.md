# 04 — Communication completeness and Slack parity

## What parity means
The target is functional breadth and dependable behavior, not pixel copying, trademark imitation, or automatic compatibility with every Slack application. Maintain `planning/slack_parity.json` as a living register of candidate parity obligations and release blockers. Its initial rows are an implementation inventory, not a claim of exhaustive verified equivalence to every Slack plan on the preparation date. Reconcile it against actual customer requirements and current first-party documentation before any parity claim. [S16, S17.]

## Layer 1: trustworthy daily communication
Channels, private channels, direct/group messages, threads, mentions, reactions, rich text, files, search, local drafts, edit/delete, read state, notification settings, and connection/retry states. Include operational edges: closed windows, multiple accounts, sleeping devices, revoked membership, large channels, long messages, unavailable attachments, and server rejection after optimistic display.

Define thread behavior explicitly. A reply belongs to one root in one channel. A deleted root retains a tombstone if replies remain. Sharing a thread elsewhere creates an access-checked reference or authorized copy; it does not grant access. Thread subscriptions and mentions have deduplicated notification keys. A group DM participant change should not silently reveal historical content to a new person: default to a new conversation unless a specific history-sharing rule is approved.

## Layer 2: everyday polish
Saved items, reminders, scheduled messages, profile/status, availability, multiple workspaces, command navigation, preferences, custom reactions, channel organization, bookmarks/pins, notification schedules, exports, and link previews. Scheduled sends revalidate authorization at actual send time. Previews are fetched through a constrained service, not a privileged browser that can read internal networks. Typing/presence is ephemeral and can be dropped under pressure.

Read state is user-private by default. Distinguish an unread cursor from optional public read receipts. A user choosing "mark unread" does not move the authoritative acknowledged cursor backwards; it sets a local/synchronized attention marker. Mention edits, deleted messages, and bot updates have explicit count semantics.

## Layer 3: broader collaboration
Calls, screen sharing, clips, collaborative documents, lightweight lists, guest access, externally shared channels, and workflow surfaces. Calls need device selection, permissions, reconnect, echo handling, active sharing indicators, and accessible controls. Recording/transcription is off by default until visible consent, policy, and retention behavior are implemented. Collaborative documents may use CRDTs; approval/action ledgers do not.

## Layer 4: ecosystem and administration
APIs, webhooks, OAuth-style installations, interactive cards, slash-like commands, workflow builders, migration, provisioning, audit exports, retention controls, discovery workflows, and regional options. A webhook endpoint is not Slack-app compatibility. Scope any compatibility layer explicitly: accepted payload versions, unsupported features, permission mapping, retries, and deprecation rules.

## Migration behavior
An importer previews data volume, permissions, author mappings, files, expected omissions, and legal/administrative authority. Never turn imported private channels public to "simplify" mapping. Imported users with no linked account remain attributed historical identities, not active accounts. Original IDs and timestamps are preserved as provenance; application ordering uses its own consistent rules. Report unavailable attachments and omitted fields rather than inventing content.

## Replacement gate
A pilot team must use the product for normal work across a representative period, not just a staged demo. Record every reason it returns to its previous tool. Critical gaps in notification reliability, mobile participation, search, calls, or external collaboration block a replacement claim for that segment. Breadth can be staged, but the claim must match the shipped scope.
