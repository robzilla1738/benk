import { err, ok } from "./result.js";
export const taskTransitions = {
    draft: ["ready", "cancelled"],
    ready: ["active", "cancelled"],
    active: ["blocked", "awaiting_review", "cancelled"],
    blocked: ["active", "cancelled"],
    awaiting_review: ["active", "accepted", "cancelled"],
    accepted: ["ready"],
    cancelled: []
};
export const runTransitions = {
    queued: ["preparing", "cancelling", "failed"],
    preparing: ["running", "awaiting_approval", "paused", "cancelling", "failed"],
    running: ["awaiting_approval", "paused", "reconciling", "cancelling", "succeeded", "failed"],
    awaiting_approval: ["preparing", "running", "paused", "cancelling", "failed"],
    paused: ["preparing", "cancelling", "failed"],
    reconciling: ["running", "cancelling", "succeeded", "failed"],
    cancelling: ["reconciling", "cancelled", "failed"],
    succeeded: [], failed: [], cancelled: []
};
function transition(matrix, from, to) {
    if (!Object.hasOwn(matrix, from) || !Object.hasOwn(matrix, to))
        return err("UnknownState");
    return matrix[from].includes(to) ? ok(to) : err("IllegalTransition");
}
/** Guards such as authorization, evidence, and accepted->ready revision creation are NOT supplied. */
export const transitionTask = (from, to) => transition(taskTransitions, from, to);
export const transitionRun = (from, to) => transition(runTransitions, from, to);
//# sourceMappingURL=state-machines.js.map