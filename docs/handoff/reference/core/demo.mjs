import { transitionTask, transitionRun } from "./dist/state-machines.js";
import { evaluateDispatch } from "./dist/policy.js";
import { facts } from "./test/helpers.mjs";
import { reserveBudget, settleBudget } from "./dist/budget.js";
console.log("SIMULATED DOMAIN WALKTHROUGH — no UI, model, shell, credentials, or external action");
let task="draft";
for(const next of ["ready","active"]){
 const r=transitionTask(task,next);if(!r.ok)throw new Error(r.error);task=r.value;console.log(`Task -> ${task}`);
}
let run="queued";
for(const next of ["preparing","running","succeeded"]){const r=transitionRun(run,next);if(!r.ok)throw new Error(r.error);run=r.value;console.log(`Run -> ${run}`);}
for(const next of ["awaiting_review","accepted"]){const r=transitionTask(task,next);if(!r.ok)throw new Error(r.error);task=r.value;console.log(`Task -> ${task}`);}
const dispatch=facts();console.log("Valid dispatch preflight:",evaluateDispatch(dispatch));
dispatch.action.artifactVersionId="av_changed";console.log("Changed artifact preflight:",evaluateDispatch(dispatch));
const reserved=reserveBudget({limit:"5000000",spent:"0",reserved:"0"},"1000000");
if(!reserved.ok)throw new Error(reserved.error);
console.log("Usage settlement:",settleBudget(reserved.value,"1000000","750000"));
console.log("Task acceptance above is only a reducer illustration; real human review is NOT implemented.");
