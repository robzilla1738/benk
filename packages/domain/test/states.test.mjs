import test from "node:test";
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {taskTransitions,runTransitions,transitionTask,transitionRun} from "../dist/state-machines.js";
const golden=JSON.parse(readFileSync(new URL("../../../docs/handoff/contracts/state-machines.json",import.meta.url)));
for (const [name,matrix,fn] of [["task",taskTransitions,transitionTask],["run",runTransitions,transitionRun]]) {
  test(`${name}: exported transitions match independent contract fixture`,()=>assert.deepEqual(matrix,golden[name]));
  for (const from of Object.keys(matrix)) for (const to of Object.keys(matrix)) {
    test(`${name}: ${from} -> ${to}`,()=>assert.equal(fn(from,to).ok,golden[name][from].includes(to)));
  }
  test(`${name}: unknown state is rejected`,()=>assert.equal(fn("not_a_state",Object.keys(matrix)[0]).error,"UnknownState"));
}
test("agent run success does not directly accept a task",()=>assert.equal(transitionTask("active","accepted").ok,false));
