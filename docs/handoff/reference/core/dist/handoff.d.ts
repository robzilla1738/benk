import { type Result } from "./result.js";
export interface Lease {
    readonly ownerId: string;
    readonly generation: number;
    readonly expiresAtMs: number;
}
export interface TransferFacts {
    readonly lease: Lease;
    readonly expectedGeneration: number;
    readonly newOwnerId: string;
    readonly nowMs: number;
    readonly ttlMs: number;
    readonly checkpointVerified: boolean;
    readonly pendingExternalActions: number;
    readonly cancelled: boolean;
}
export type TransferError = "InvalidTransfer" | "StaleGeneration" | "CheckpointNotReady" | "ReconciliationRequired" | "RunStopped";
/** Pure proposal. The authoritative store must apply this with CAS and fence all old dispatches. */
export declare function proposeTransfer(f: TransferFacts): Result<Lease, TransferError>;
