/**
 * Preflight reference, NOT a production authorization system.
 * Every fact must be derived from verified server/broker state, not accepted from a client.
 * Revalidate and atomically claim approval, action, lease, and reservation at dispatch.
 */
export interface DispatchFacts {
    readonly nowMs: number;
    readonly subject: {
        readonly workspaceId: string;
        readonly principalId: string;
        readonly active: boolean;
        readonly ownerActive: boolean;
        readonly allowedActions: readonly string[];
        readonly allowedResources: readonly string[];
        readonly allowedDestinations: readonly string[];
        readonly allowedModelDestinations: readonly string[];
    };
    readonly run: {
        readonly workspaceId: string;
        readonly id: string;
        readonly taskId: string;
        readonly agentId: string;
        readonly contractRevision: number;
        readonly generation: number;
        readonly state: string;
        readonly cancellationRequested: boolean;
        readonly modelDestination: string;
    };
    readonly authority: {
        readonly workspaceId: string;
        readonly runId: string;
        readonly generation: number;
        readonly leaseExpiresAtMs: number;
        readonly policyRevision: number;
        readonly contractRevision: number;
        readonly budgetReserved: boolean;
    };
    readonly action: {
        readonly workspaceId: string;
        readonly id: string;
        readonly runId: string;
        readonly taskId: string;
        readonly generation: number;
        readonly contractRevision: number;
        readonly policyRevision: number;
        readonly kind: string;
        readonly resourceIds: readonly string[];
        readonly destinationId: string;
        readonly artifactVersionId: string | null;
        readonly digest: string;
        readonly state: string;
        readonly requiresApproval: boolean;
        readonly expiresAtMs: number;
    };
    readonly approval: null | {
        readonly workspaceId: string;
        readonly actionId: string;
        readonly decision: string;
        readonly deciderKind: string;
        readonly deciderCurrentlyAuthorized: boolean;
        readonly separationOfDutySatisfied: boolean;
        readonly quorumSatisfied: boolean;
        readonly actionDigest: string;
        readonly artifactVersionId: string | null;
        readonly contractRevision: number;
        readonly policyRevision: number;
        readonly expiresAtMs: number;
        readonly consumed: boolean;
    };
}
export type Denial = "InvalidFacts" | "TenantMismatch" | "IdentityInactive" | "PrincipalMismatch" | "RunMismatch" | "StaleGeneration" | "LeaseExpired" | "RunStopped" | "RevisionChanged" | "ActionNotReady" | "ActionExpired" | "CapabilityDenied" | "ResourceDenied" | "DestinationDenied" | "ModelDestinationDenied" | "BudgetNotReserved" | "ApprovalRequired" | "ApprovalInvalid" | "ApprovalStale" | "ApprovalConsumed";
export type Decision = {
    readonly allowed: true;
} | {
    readonly allowed: false;
    readonly reason: Denial;
};
export declare function evaluateDispatch(f: DispatchFacts): Decision;
