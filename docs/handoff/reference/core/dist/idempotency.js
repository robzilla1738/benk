export class InMemoryIdempotencyLedger {
    entries = new Map();
    key(scope, key) { return JSON.stringify([scope.workspaceId, scope.actorId, scope.operation, key]); }
    claim(scope, key, digest, owner) {
        if (!key || !digest || !owner)
            throw new Error("Empty idempotency input");
        const k = this.key(scope, key), old = this.entries.get(k);
        if (!old) {
            this.entries.set(k, { digest, owner, state: "pending" });
            return { kind: "acquired" };
        }
        if (old.digest !== digest)
            return { kind: "conflict" };
        if (old.state === "complete")
            return { kind: "replay", result: old.result };
        return { kind: old.state };
    }
    complete(scope, key, owner, result) {
        const e = this.entries.get(this.key(scope, key));
        if (!e || e.owner !== owner || e.state === "complete")
            return false;
        e.state = "complete";
        e.result = result;
        return true;
    }
    markUnknown(scope, key, owner) {
        const e = this.entries.get(this.key(scope, key));
        if (!e || e.owner !== owner || e.state !== "pending")
            return false;
        e.state = "unknown";
        return true;
    }
}
//# sourceMappingURL=idempotency.js.map