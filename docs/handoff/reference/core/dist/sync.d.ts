import { type Result } from "./result.js";
export interface SyncEvent {
    readonly eventId: string;
    readonly workspaceId: string;
    readonly sequence: string;
    readonly messageId: string;
    readonly revision: number;
    readonly kind: "upsert" | "delete";
    readonly body: string;
}
export interface CachedMessage {
    readonly revision: number;
    readonly deleted: boolean;
    readonly body: string;
}
export interface SyncState {
    readonly workspaceId: string;
    readonly cursor: string;
    readonly messages: ReadonlyMap<string, CachedMessage>;
    readonly seen: ReadonlyMap<string, string>;
}
export type SyncError = "TenantMismatch" | "InvalidEvent" | "Gap" | "ConflictingDuplicate" | "UnknownOldEvent";
export declare const emptySync: (workspaceId: string) => SyncState;
/**
 * Contiguous-stream reference. A filtered GLOBAL sequence is NOT contiguous; use a scoped server cursor.
 * Clones simulate batch atomicity. Production uses a SQLite transaction, bounded retention, and snapshots.
 */
export declare function applyBatch(s: SyncState, events: readonly SyncEvent[]): Result<SyncState, SyncError>;
