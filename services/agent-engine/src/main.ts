import { Effect } from "effect";
import { demoAwaitingReview } from "@benk/simulator";
const program = Effect.gen(function* () {
  const snapshot = yield* Effect.sync(demoAwaitingReview);
  yield* Effect.log("Benk agent-engine SIMULATION ONLY", {
    taskId: snapshot.taskId, taskState: snapshot.taskState,
    runState: snapshot.runState, externalActionsPerformed: snapshot.externalActionsPerformed,
  });
});
await Effect.runPromise(program);
