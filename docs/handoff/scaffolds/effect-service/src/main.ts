/** UNCOMPILED Effect 4 scaffold. This is a simulated service composition, not a server. */
import { Context, Effect, Layer } from "effect";

type SimulationError = { readonly _tag: "SimulationError"; readonly reason: string };
type Proposal = {
  readonly taskId: string;
  readonly mode: "simulation";
  readonly summary: string;
  readonly externalActionsPerformed: 0;
};

class ProposalService extends Context.Service<ProposalService, {
  readonly propose: (taskId: string) => Effect.Effect<Proposal, SimulationError>;
}>()("workspace/ProposalService") {}

const SimulatedProposalService = Layer.succeed(ProposalService, {
  propose: (taskId: string) => taskId.length < 6
    ? Effect.fail({ _tag: "SimulationError" as const, reason: "Synthetic task ID is too short" })
    : Effect.succeed({
        taskId,
        mode: "simulation" as const,
        summary: "Produce a synthetic review package; no model or tool is invoked.",
        externalActionsPerformed: 0 as const,
      }),
});

const program = Effect.gen(function* () {
  const service = yield* ProposalService;
  const proposal = yield* service.propose("task_checkout");
  yield* Effect.log("SIMULATION ONLY", proposal);
});

await Effect.runPromise(program.pipe(Effect.provide(SimulatedProposalService)));
