import { err, ok, type Result } from "./result.js";
export interface Lease { readonly ownerId:string; readonly generation:number; readonly expiresAtMs:number }
export interface TransferFacts {
  readonly lease:Lease; readonly expectedGeneration:number; readonly newOwnerId:string;
  readonly nowMs:number; readonly ttlMs:number; readonly checkpointVerified:boolean;
  readonly pendingExternalActions:number; readonly cancelled:boolean;
}
export type TransferError="InvalidTransfer"|"StaleGeneration"|"CheckpointNotReady"|"ReconciliationRequired"|"RunStopped";
/** Pure proposal. The authoritative store must apply this with CAS and fence all old dispatches. */
export function proposeTransfer(f:TransferFacts):Result<Lease,TransferError> {
  if (!Number.isSafeInteger(f.expectedGeneration)||f.expectedGeneration<1
      ||!Number.isSafeInteger(f.lease.generation)||f.lease.generation<1
      ||f.lease.generation>=2147483647||!f.newOwnerId||f.newOwnerId===f.lease.ownerId
      ||!Number.isSafeInteger(f.nowMs)||f.nowMs<0||!Number.isSafeInteger(f.ttlMs)
      ||f.ttlMs<=0||f.ttlMs>300000||!Number.isSafeInteger(f.pendingExternalActions)
      ||f.pendingExternalActions<0||!Number.isSafeInteger(f.nowMs+f.ttlMs)) return err("InvalidTransfer");
  if (f.expectedGeneration!==f.lease.generation) return err("StaleGeneration");
  if (f.cancelled) return err("RunStopped");
  if (!f.checkpointVerified) return err("CheckpointNotReady");
  if (f.pendingExternalActions>0) return err("ReconciliationRequired");
  return ok({ownerId:f.newOwnerId,generation:f.lease.generation+1,expiresAtMs:f.nowMs+f.ttlMs});
}
