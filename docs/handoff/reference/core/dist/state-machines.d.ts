import { type Result } from "./result.js";
export declare const taskTransitions: {
    readonly draft: readonly ["ready", "cancelled"];
    readonly ready: readonly ["active", "cancelled"];
    readonly active: readonly ["blocked", "awaiting_review", "cancelled"];
    readonly blocked: readonly ["active", "cancelled"];
    readonly awaiting_review: readonly ["active", "accepted", "cancelled"];
    readonly accepted: readonly ["ready"];
    readonly cancelled: readonly [];
};
export declare const runTransitions: {
    readonly queued: readonly ["preparing", "cancelling", "failed"];
    readonly preparing: readonly ["running", "awaiting_approval", "paused", "cancelling", "failed"];
    readonly running: readonly ["awaiting_approval", "paused", "reconciling", "cancelling", "succeeded", "failed"];
    readonly awaiting_approval: readonly ["preparing", "running", "paused", "cancelling", "failed"];
    readonly paused: readonly ["preparing", "cancelling", "failed"];
    readonly reconciling: readonly ["running", "cancelling", "succeeded", "failed"];
    readonly cancelling: readonly ["reconciling", "cancelled", "failed"];
    readonly succeeded: readonly [];
    readonly failed: readonly [];
    readonly cancelled: readonly [];
};
export type TaskState = keyof typeof taskTransitions;
export type RunState = keyof typeof runTransitions;
export type TransitionError = "UnknownState" | "IllegalTransition";
/** Guards such as authorization, evidence, and accepted->ready revision creation are NOT supplied. */
export declare const transitionTask: (from: TaskState, to: TaskState) => Result<"ready" | "cancelled" | "active" | "blocked" | "awaiting_review" | "accepted" | "draft", TransitionError>;
export declare const transitionRun: (from: RunState, to: RunState) => Result<"running" | "cancelled" | "preparing" | "cancelling" | "failed" | "awaiting_approval" | "paused" | "reconciling" | "succeeded" | "queued", TransitionError>;
