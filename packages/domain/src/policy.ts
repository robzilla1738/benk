/**
 * Preflight reference, NOT a production authorization system.
 * Every fact must be derived from verified server/broker state, not accepted from a client.
 * Revalidate and atomically claim approval, action, lease, and reservation at dispatch.
 */
export interface DispatchFacts {
  readonly nowMs: number;
  readonly subject: {
    readonly workspaceId: string; readonly principalId: string;
    readonly active: boolean; readonly ownerActive: boolean;
    readonly allowedActions: readonly string[]; readonly allowedResources: readonly string[];
    readonly allowedDestinations: readonly string[]; readonly allowedModelDestinations: readonly string[];
  };
  readonly run: {
    readonly workspaceId: string; readonly id: string; readonly taskId: string;
    readonly agentId: string; readonly contractRevision: number; readonly generation: number;
    readonly state: string; readonly cancellationRequested: boolean; readonly modelDestination: string;
  };
  readonly authority: {
    readonly workspaceId: string; readonly runId: string; readonly generation: number;
    readonly leaseExpiresAtMs: number; readonly policyRevision: number;
    readonly contractRevision: number; readonly budgetReserved: boolean;
  };
  readonly action: {
    readonly workspaceId: string; readonly id: string; readonly runId: string; readonly taskId: string;
    readonly generation: number; readonly contractRevision: number; readonly policyRevision: number;
    readonly kind: string; readonly resourceIds: readonly string[]; readonly destinationId: string;
    readonly artifactVersionId: string | null; readonly digest: string; readonly state: string;
    readonly requiresApproval: boolean; readonly expiresAtMs: number;
  };
  readonly approval: null | {
    readonly workspaceId: string; readonly actionId: string; readonly decision: string;
    readonly deciderKind: string; readonly deciderCurrentlyAuthorized: boolean;
    readonly separationOfDutySatisfied: boolean; readonly quorumSatisfied: boolean;
    readonly actionDigest: string; readonly artifactVersionId: string | null;
    readonly contractRevision: number; readonly policyRevision: number;
    readonly expiresAtMs: number; readonly consumed: boolean;
  };
}
export type Denial = "InvalidFacts" | "TenantMismatch" | "IdentityInactive" | "PrincipalMismatch"
  | "RunMismatch" | "StaleGeneration" | "LeaseExpired" | "RunStopped" | "RevisionChanged"
  | "ActionNotReady" | "ActionExpired" | "CapabilityDenied" | "ResourceDenied"
  | "DestinationDenied" | "ModelDestinationDenied" | "BudgetNotReserved"
  | "ApprovalRequired" | "ApprovalInvalid" | "ApprovalStale" | "ApprovalConsumed";
export type Decision = { readonly allowed:true } | { readonly allowed:false; readonly reason:Denial };
const deny=(reason:Denial):Decision=>({allowed:false,reason});
const isPositiveInt=(n:number)=>Number.isSafeInteger(n)&&n>0&&n<=2147483647;
const isTime=(n:number)=>Number.isSafeInteger(n)&&n>=0;
export function evaluateDispatch(f:DispatchFacts):Decision {
  const {subject:s,run:r,authority:l,action:a,approval:p}=f;
  if (![f.nowMs,l.leaseExpiresAtMs,a.expiresAtMs].every(isTime)
      || ![r.contractRevision,r.generation,l.generation,l.policyRevision,l.contractRevision,
        a.generation,a.contractRevision,a.policyRevision].every(isPositiveInt)
      || !/^sha256:[a-f0-9]{64}$/.test(a.digest)) return deny("InvalidFacts");
  if ([r.workspaceId,l.workspaceId,a.workspaceId].some(x=>x!==s.workspaceId)) return deny("TenantMismatch");
  if (!s.active||!s.ownerActive) return deny("IdentityInactive");
  if (s.principalId!==r.agentId) return deny("PrincipalMismatch");
  if (a.runId!==r.id||l.runId!==r.id||a.taskId!==r.taskId) return deny("RunMismatch");
  if (r.generation!==l.generation||a.generation!==l.generation) return deny("StaleGeneration");
  if (f.nowMs>=l.leaseExpiresAtMs) return deny("LeaseExpired");
  if (r.cancellationRequested||r.state!=="running") return deny("RunStopped");
  if (a.contractRevision!==r.contractRevision||a.contractRevision!==l.contractRevision
      ||a.policyRevision!==l.policyRevision) return deny("RevisionChanged");
  if (a.state!=="ready") return deny("ActionNotReady");
  if (f.nowMs>=a.expiresAtMs) return deny("ActionExpired");
  if (!s.allowedActions.includes(a.kind)) return deny("CapabilityDenied");
  if (a.resourceIds.length===0||a.resourceIds.some(x=>!s.allowedResources.includes(x))) return deny("ResourceDenied");
  if (!s.allowedDestinations.includes(a.destinationId)) return deny("DestinationDenied");
  if (!s.allowedModelDestinations.includes(r.modelDestination)) return deny("ModelDestinationDenied");
  if (!l.budgetReserved) return deny("BudgetNotReserved");
  if (a.requiresApproval) {
    if (!p) return deny("ApprovalRequired");
    if (!isTime(p.expiresAtMs)||![p.contractRevision,p.policyRevision].every(isPositiveInt)) return deny("InvalidFacts");
    if (p.workspaceId!==s.workspaceId||p.actionId!==a.id||p.decision!=="approved"
        ||p.deciderKind!=="human"||!p.deciderCurrentlyAuthorized||!p.separationOfDutySatisfied
        ||!p.quorumSatisfied) return deny("ApprovalInvalid");
    if (p.consumed) return deny("ApprovalConsumed");
    if (f.nowMs>=p.expiresAtMs||p.actionDigest!==a.digest||p.artifactVersionId!==a.artifactVersionId
        ||p.contractRevision!==a.contractRevision||p.policyRevision!==a.policyRevision) return deny("ApprovalStale");
  }
  return {allowed:true};
}
