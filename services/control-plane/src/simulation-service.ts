import { Context, Effect, Layer } from "effect";
import { demoAwaitingReview, type DemoState } from "@benk/simulator";
export class SimulationService extends Context.Service<SimulationService, {
  readonly snapshot: Effect.Effect<DemoState>;
}>()("benk/SimulationService") {}
export const SimulationLive = Layer.succeed(SimulationService, { snapshot: Effect.sync(demoAwaitingReview) });
export const snapshotProgram = Effect.gen(function* () {
  const service = yield* SimulationService;
  return yield* service.snapshot;
});
export const getDemoSnapshot = () => Effect.runPromise(snapshotProgram.pipe(Effect.provide(SimulationLive)));
