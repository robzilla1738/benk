import { err, ok, type Result } from "./result.js";
export interface SyncEvent {
  readonly eventId:string; readonly workspaceId:string; readonly sequence:string;
  readonly messageId:string; readonly revision:number; readonly kind:"upsert"|"delete"; readonly body:string;
}
export interface CachedMessage { readonly revision:number; readonly deleted:boolean; readonly body:string }
export interface SyncState {
  readonly workspaceId:string; readonly cursor:string;
  readonly messages:ReadonlyMap<string,CachedMessage>; readonly seen:ReadonlyMap<string,string>;
}
export type SyncError="TenantMismatch"|"InvalidEvent"|"Gap"|"ConflictingDuplicate"|"UnknownOldEvent";
export const emptySync=(workspaceId:string):SyncState=>({workspaceId,cursor:"0",messages:new Map(),seen:new Map()});
const decimal=/^(0|[1-9][0-9]{0,18})$/;
/**
 * Contiguous-stream reference. A filtered GLOBAL sequence is NOT contiguous; use a scoped server cursor.
 * Clones simulate batch atomicity. Production uses a SQLite transaction, bounded retention, and snapshots.
 */
export function applyBatch(s:SyncState,events:readonly SyncEvent[]):Result<SyncState,SyncError> {
  if (typeof s.cursor!=="string" || !decimal.test(s.cursor)) return err("InvalidEvent");
  let cursor=BigInt(s.cursor);const messages=new Map(s.messages),seen=new Map(s.seen);
  for (const e of events) {
    if (e.workspaceId!==s.workspaceId) return err("TenantMismatch");
    if (typeof e.sequence!=="string"||!decimal.test(e.sequence)||!Number.isSafeInteger(e.revision)||e.revision<1
      ||!(e.kind==="upsert"||e.kind==="delete")||!e.eventId||!e.messageId) return err("InvalidEvent");
    const fingerprint=JSON.stringify([e.workspaceId,e.sequence,e.messageId,e.revision,e.kind,e.body]);
    const prior=seen.get(e.eventId);
    if (prior!==undefined) {if(prior!==fingerprint)return err("ConflictingDuplicate");continue;}
    const seq=BigInt(e.sequence);
    if (seq<=cursor) return err("UnknownOldEvent");
    if (seq!==cursor+1n) return err("Gap");
    const old=messages.get(e.messageId);
    if (!old||e.revision>old.revision) messages.set(e.messageId,{revision:e.revision,deleted:e.kind==="delete",body:e.kind==="delete"?"":e.body});
    seen.set(e.eventId,fingerprint);cursor=seq;
  }
  return ok({workspaceId:s.workspaceId,cursor:cursor.toString(),messages,seen});
}
