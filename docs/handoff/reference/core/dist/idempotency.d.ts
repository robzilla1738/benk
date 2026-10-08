/** Single-process demonstration only. Real persistence needs atomic claim/compare-and-set and recovery. */
export interface Scope {
    readonly workspaceId: string;
    readonly actorId: string;
    readonly operation: string;
}
export type Claim = {
    kind: "acquired";
} | {
    kind: "pending";
} | {
    kind: "unknown";
} | {
    kind: "conflict";
} | {
    kind: "replay";
    result: string;
};
export declare class InMemoryIdempotencyLedger {
    private readonly entries;
    private key;
    claim(scope: Scope, key: string, digest: string, owner: string): Claim;
    complete(scope: Scope, key: string, owner: string, result: string): boolean;
    markUnknown(scope: Scope, key: string, owner: string): boolean;
}
