# 02 — Product requirements and release boundaries

The machine-readable requirements are in `planning/requirements.json`. Each has a priority, phase, owner discipline, measurable acceptance statement, and linked implementation/test evidence. This document explains the product contract behind those rows.

## People and identities
Support humans, agents, integrations, and system actors as distinct principal kinds. A shared agent has an accountable active human owner, explicit grants, provider/runtime identity, and visible execution location. Deactivated owners trigger an administrative reassignment or stop; they do not leave orphaned autonomous authority. An agent never impersonates a human message author.

A workspace is the principal tenancy boundary. Public channels are public only within their permitted workspace audience. Private channels, direct messages, group messages, projects, and externally shared spaces have explicit access rules. Guests have narrower visibility. Notification snippets, search results, previews, artifacts, exported content, and agent context obey the same rules as their source resources.

## Communication requirements
Provide public/private channels, direct/group conversations, threads, rich text, code blocks, mentions, reactions, files, edits, deletes, drafts, read state, search, and reliable notifications. Preserve keyboard behavior, selection, scroll anchoring, copy/paste, navigation history, and offline draft durability. Expose clear local/pending/accepted/failed message states. A failed send retains recoverable content and does not masquerade as a delivered message.

Unread and mention counts must not double count reconnects. Editing a message must not create a new ordinary unread item unless a defined notification rule requires it. Deleted and revoked content must stop appearing in local search after the deletion/revocation is processed. Cross-device read convergence uses monotonic cursors, with a separate user-facing "mark unread" marker if supported.

## Delegated work requirements
A task has an owner, acceptance criteria, scope, contract revision, budget, and execution constraints. A run is one attempt. Multiple attempts may exist without duplicating accepted work. Run status is separate from task status. Proposed changes to scope are reviewed before they become authority. Bounded parallel runs may compare alternatives, but they must not concurrently mutate the same shared workspace without ownership rules.

A review package contains versioned artifacts, relevant evidence, unresolved issues, provenance, and a precise proposed next action. Users can accept, request changes, pause, cancel, or take over. Acceptance identifies exact contract and artifact versions. Publication and deployment can require approvals distinct from task acceptance.

## Local/cloud requirements
The native app renders cached collaboration immediately. Local execution is a separate subsystem with a visible lifecycle. A suspended laptop is unavailable, even if its last presence said online. Cloud work continues under a shared coordinator and scoped identity. A company-managed runner is a later deployment option using the same capability protocol.

Handoff transfers supported checkpoints and references, not arbitrary process memory. Credentials are re-resolved in the destination environment. A failed handoff preserves artifacts and exposes the current authoritative owner. Network partitions must not authorize two coordinators to perform new external writes.

## Non-functional requirements
Use measured responsiveness budgets, bounded queues, privacy-aware observability, authenticated updates, schema-versioned protocols, recovery tests, account separation, and accessible native interaction. No real model-generated code executes until isolation and permission tests pass. No public deployment occurs until tenancy and identity are real. No enterprise certification is implied by implementing a checklist.

## Scope priority
P0 means a safety/foundation/vertical-slice blocker. P1 means needed for team adoption and workspace replacement. P2 means advanced platform or enterprise breadth. A late-phase P0 gate is a blocker for that release, not a command to build all enterprise features before the first local prototype. Phase and priority must be considered together.

## Definition of product completion
The current milestone is complete only when its product demo can be performed without manual database edits, its positive and negative acceptance cases pass, the implementation has actual evidence, and its known limitations are visible. A static screen, simulated agent, or passing reference library does not satisfy a production workflow requirement on its own.
