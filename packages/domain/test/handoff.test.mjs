import test from "node:test";
import assert from "node:assert/strict";
import {proposeTransfer} from "../dist/handoff.js";
const f=()=>({lease:{ownerId:"local_runner",generation:1,expiresAtMs:2000},expectedGeneration:1,newOwnerId:"cloud_runner",nowMs:1000,ttlMs:1000,checkpointVerified:true,pendingExternalActions:0,cancelled:false});
test("transfer increments generation",()=>assert.deepEqual(proposeTransfer(f()).value,{ownerId:"cloud_runner",generation:2,expiresAtMs:2000}));
for(const [name,patch,error] of [["stale ownership",{expectedGeneration:2},"StaleGeneration"],["unverified checkpoint",{checkpointVerified:false},"CheckpointNotReady"],["unknown action",{pendingExternalActions:1},"ReconciliationRequired"],["cancelled run",{cancelled:true},"RunStopped"],["unbounded lease",{ttlMs:999999},"InvalidTransfer"],["same owner",{newOwnerId:"local_runner"},"InvalidTransfer"]]) test(name,()=>assert.equal(proposeTransfer({...f(),...patch}).error,error));
