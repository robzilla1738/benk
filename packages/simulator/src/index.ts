import { transitionTask, type TaskState } from "@benk/domain";
export const DEMO_ARTIFACT_CONTENT = "SIMULATION: Replace the checkout error with \"We could not complete your payment. Please check your details and try again.\" No repository was modified.";
export const DEMO_ARTIFACT_DIGEST = "sha256:4f39d52eea05a574e4e1e74cceb74dadb9f6b927ab6a32bd09f87b45442f4bcb";
export type SimulationErrorCode = "IllegalTransition" | "ReviewNotReady" | "HumanRequired" | "SeparationOfDuty" | "StaleArtifact" | "ReviewerNotAuthorized";
export class SimulationError extends Error {
  readonly _tag = "SimulationError";
  constructor(readonly code: SimulationErrorCode) { super(code); this.name = "SimulationError"; }
}
export interface Artifact {
  readonly id: string; readonly version: number; readonly digest: string;
  readonly content: string; readonly verifiedChecks: readonly string[];
}
export interface DemoState {
  readonly mode: "simulation"; readonly taskId: string; readonly taskState: TaskState;
  readonly requesterId: string; readonly runState: "not_started" | "running" | "succeeded";
  readonly contract: { readonly revision: 1; readonly allowedCapabilities: readonly []; readonly externalActionsAllowed: false };
  readonly artifact: Artifact | null; readonly acceptedBy: string | null;
  readonly events: readonly string[]; readonly externalActionsPerformed: 0;
}
export function createDemo(): DemoState {
  return { mode: "simulation", taskId: "task_demo_checkout", taskState: "draft", requesterId: "human_requester",
    runState: "not_started", contract: { revision: 1, allowedCapabilities: [], externalActionsAllowed: false },
    artifact: null, acceptedBy: null, events: [], externalActionsPerformed: 0 };
}
function move(s: DemoState, to: TaskState): DemoState {
  if (!transitionTask(s.taskState, to).ok) throw new SimulationError("IllegalTransition");
  return { ...s, taskState: to, events: [...s.events, `task:${to}`] };
}
/** Fixture only: no models, networking, credentials, shell or external writes. */
export function advanceDemo(s: DemoState): DemoState {
  switch (s.taskState) {
    case "draft": return move(s, "ready");
    case "ready": return { ...move(s, "active"), runState: "running" };
    case "active": return { ...move(s, "awaiting_review"), runState: "succeeded", artifact: {
      id: "artifact_demo_checkout", version: 1, digest: DEMO_ARTIFACT_DIGEST, content: DEMO_ARTIFACT_CONTENT,
      verifiedChecks: ["Synthetic fixture produced; no real tests executed"] } };
    default: throw new SimulationError("IllegalTransition");
  }
}
export interface DemoReview {
  readonly reviewerId: string; readonly reviewerKind: "human" | "agent";
  readonly reviewerAuthorized: boolean; readonly artifactId: string;
  readonly artifactVersion: number; readonly artifactDigest: string;
}
/** Caller-supplied review facts are test data, not authenticated identity. */
export function reviewDemo(s: DemoState, r: DemoReview): DemoState {
  if (s.taskState !== "awaiting_review" || !s.artifact) throw new SimulationError("ReviewNotReady");
  if (r.reviewerKind !== "human") throw new SimulationError("HumanRequired");
  if (!r.reviewerAuthorized || !r.reviewerId.trim()) throw new SimulationError("ReviewerNotAuthorized");
  if (r.reviewerId === s.requesterId) throw new SimulationError("SeparationOfDuty");
  if (r.artifactId !== s.artifact.id || r.artifactVersion !== s.artifact.version || r.artifactDigest !== s.artifact.digest)
    throw new SimulationError("StaleArtifact");
  return { ...move(s, "accepted"), acceptedBy: r.reviewerId };
}
export function demoAwaitingReview(): DemoState { return advanceDemo(advanceDemo(advanceDemo(createDemo()))); }
