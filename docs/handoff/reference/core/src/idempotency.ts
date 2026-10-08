/** Single-process demonstration only. Real persistence needs atomic claim/compare-and-set and recovery. */
export interface Scope { readonly workspaceId:string; readonly actorId:string; readonly operation:string }
type Entry = { digest:string; owner:string; state:"pending"|"unknown"|"complete"; result?:string };
export type Claim = {kind:"acquired"}|{kind:"pending"}|{kind:"unknown"}|{kind:"conflict"}|{kind:"replay";result:string};
export class InMemoryIdempotencyLedger {
  private readonly entries=new Map<string,Entry>();
  private key(scope:Scope,key:string):string {return JSON.stringify([scope.workspaceId,scope.actorId,scope.operation,key]);}
  claim(scope:Scope,key:string,digest:string,owner:string):Claim {
    if (!key||!digest||!owner) throw new Error("Empty idempotency input");
    const k=this.key(scope,key), old=this.entries.get(k);
    if (!old) {this.entries.set(k,{digest,owner,state:"pending"});return {kind:"acquired"};}
    if (old.digest!==digest) return {kind:"conflict"};
    if (old.state==="complete") return {kind:"replay",result:old.result!};
    return {kind:old.state};
  }
  complete(scope:Scope,key:string,owner:string,result:string):boolean {
    const e=this.entries.get(this.key(scope,key));
    if (!e||e.owner!==owner||e.state==="complete") return false;
    e.state="complete";e.result=result;return true;
  }
  markUnknown(scope:Scope,key:string,owner:string):boolean {
    const e=this.entries.get(this.key(scope,key));
    if (!e||e.owner!==owner||e.state!=="pending") return false;
    e.state="unknown";return true;
  }
}
