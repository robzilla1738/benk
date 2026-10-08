# 09 — Synchronization, offline behavior, and conflict handling

## Authority and local optimism
PostgreSQL owns shared accepted state. SQLite owns local drafts, cached authorized content, and pending operations. A local write can be visible immediately but must carry `pending` until accepted. Resource permissions, shared approvals, membership changes, and consequential cloud actions require current authorization; they are not authoritative offline operations.

Persist an operation before transmission. Every mutation includes an operation ID, expected revision where needed, and stable request digest. Reusing an idempotency key with different content is a conflict. A timeout does not authorize a new key for the same uncertain operation.

## Event model
Durable collaboration events, ephemeral presence, and high-volume execution logs are separate streams. Event payloads have IDs, workspace scope, aggregate identity/revision, type, schema version, correlation/causation IDs, and a sequence. Clients deduplicate before applying. Unknown mandatory protocol versions require upgrade/rebootstrap rather than silent misinterpretation.

The initial server can allocate ordered workspace sequences by locking/updating a workspace counter in the same transaction as the event. This serializes allocation and avoids a naive sequence watermark skipping late commits. Do not implement catch-up as `max(bigserial)` over independently committing transactions and assume every lower event has committed. Profile the counter's hot-workspace contention; partition later only with explicit ordering semantics.

## Scoped subscriptions
An event subscription is bound to a verified principal, workspace, filter set, and authorization epoch. Clients receive only events they are allowed to see. A scoped stream may omit global events, so gaps in global sequence numbers are not automatically data loss. The server provides an opaque scoped cursor with a stable catch-up contract. The reference sequence reducer demonstrates a contiguous stream only; it must not be misapplied to filtered global sequences.

Revocation increments the relevant authorization epoch and sends a purge instruction with allowed metadata. A reconnect rechecks identity and scopes before resuming. Avoid leaking private channel names, source titles, counts, or event bodies through errors or cursor metadata.

## Reconnect algorithm
Authenticate; refresh authorization; reconcile server acknowledgments for pending operations; request the scoped delta or snapshot; apply each batch plus cursor atomically in SQLite; resume live delivery from the accepted cursor. If the cursor expired, perform a bounded snapshot and replay pending local intent against it. Never overwrite drafts during a cache rebuild. Deduplicate the boundary between snapshot and live events.

Apply privacy invalidations before showing newly loaded content. If a sync batch fails, roll it back and preserve the old cursor. If a message event is stale relative to an already-known revision/tombstone, ignore it. Do not resurrect deleted content because an old create event arrives late.

## Conflict rules
| Object | Conflict rule |
|---|---|
| New messages | Stable client ID + operation key; server accepts once. |
| Message edits | Compare expected revision; preserve rejected draft and show conflict. |
| Reactions | Set semantics keyed by actor/message/reaction. |
| Read progress | Monotonic acknowledged cursor; separate mark-unread marker. |
| Drafts | Device-local initially; optional explicit cross-device draft conflict model later. |
| Task/contract edits | Compare-and-set revision; no silent merge of authority or acceptance criteria. |
| Approvals/actions | Transactional guards; never a CRDT merge. |
| Collaborative documents | CRDT only for eligible document body; permission and approval ledger remain authoritative. |
| Deletes | Tombstones dominate older writes; physical purge follows retention rules. |

## Offline policy
Reading cached permitted content, editing drafts, and queuing ordinary sends can work offline. Display last successful sync and queued status. Offline access expiry is an organization policy; encrypted caches and key expiry can reduce exposure but cannot guarantee deletion from a disconnected or compromised device. Do not promise immediate remote wipe. When access is revoked, reject queued sends on reconnect, preserve user-owned drafts according to policy, and purge revoked source content.

## Storage and search consistency
Each sync transaction updates message/cache rows and their search projection together. The supplied SQLite schema includes FTS maintenance and purge tests. It does not implement encryption or full sync. In the real native store use a single logical writer, WAL, bounded read tasks, and planned checkpointing. WAL permits concurrent readers but retains one writer at a time. [S09, S10.]

## Failure tests
Duplicate/reordered events; crash after local commit but before network send; crash after server commit before response; expired cursor; malformed event; tenant mismatch; permission loss mid-page; clock skew; sleep/wake; concurrent edits; deleted thread root; partial attachment upload; full disk; corrupt cache rebuild; two accounts open at once. Each must end in an understandable user state, not just a handled exception.
