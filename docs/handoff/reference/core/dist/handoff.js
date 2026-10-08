import { err, ok } from "./result.js";
/** Pure proposal. The authoritative store must apply this with CAS and fence all old dispatches. */
export function proposeTransfer(f) {
    if (!Number.isSafeInteger(f.expectedGeneration) || f.expectedGeneration < 1
        || !Number.isSafeInteger(f.lease.generation) || f.lease.generation < 1
        || f.lease.generation >= 2147483647 || !f.newOwnerId || f.newOwnerId === f.lease.ownerId
        || !Number.isSafeInteger(f.nowMs) || f.nowMs < 0 || !Number.isSafeInteger(f.ttlMs)
        || f.ttlMs <= 0 || f.ttlMs > 300000 || !Number.isSafeInteger(f.pendingExternalActions)
        || f.pendingExternalActions < 0 || !Number.isSafeInteger(f.nowMs + f.ttlMs))
        return err("InvalidTransfer");
    if (f.expectedGeneration !== f.lease.generation)
        return err("StaleGeneration");
    if (f.cancelled)
        return err("RunStopped");
    if (!f.checkpointVerified)
        return err("CheckpointNotReady");
    if (f.pendingExternalActions > 0)
        return err("ReconciliationRequired");
    return ok({ ownerId: f.newOwnerId, generation: f.lease.generation + 1, expiresAtMs: f.nowMs + f.ttlMs });
}
//# sourceMappingURL=handoff.js.map