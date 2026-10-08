import test from "node:test";
import assert from "node:assert/strict";
import {emptySync,applyBatch} from "../dist/sync.js";
import {event} from "./helpers.mjs";
test("apply ordered batch atomically",()=>{const r=applyBatch(emptySync("ws_alpha"),[event(1),event(2)]);assert.equal(r.value.cursor,"2");assert.equal(r.value.messages.get("msg_first").revision,2);});
test("identical event delivery deduplicated",()=>assert.equal(applyBatch(emptySync("ws_alpha"),[event(1),event(1)]).value.cursor,"1"));
test("duplicate event with changed payload rejected",()=>assert.equal(applyBatch(emptySync("ws_alpha"),[event(1),event(1,{body:"tampered"})]).error,"ConflictingDuplicate"));
test("gap does not advance source cursor",()=>{const s=emptySync("ws_alpha");assert.equal(applyBatch(s,[event(1),event(3)]).error,"Gap");assert.equal(s.cursor,"0");assert.equal(s.messages.size,0);});
test("cross tenant batch rejected",()=>assert.equal(applyBatch(emptySync("ws_alpha"),[event(1,{workspaceId:"ws_beta"})]).error,"TenantMismatch"));
test("new sequence with older revision cannot resurrect a tombstone",()=>{
 const r=applyBatch(emptySync("ws_alpha"),[event(1),event(2,{kind:"delete",body:""}),event(3,{revision:1,body:"old"})]);
 assert.deepEqual(r.value.messages.get("msg_first"),{revision:2,deleted:true,body:""});
});
test("unknown old event requires reconciliation instead of silent skip",()=>{const s=applyBatch(emptySync("ws_alpha"),[event(1)]).value;assert.equal(applyBatch(s,[event(1,{eventId:"evt_other"})]).error,"UnknownOldEvent");});
test("sequence precision above safe integer",()=>{const s={...emptySync("ws_alpha"),cursor:"9007199254740992"};assert.equal(applyBatch(s,[event("9007199254740993",{revision:1})]).value.cursor,"9007199254740993");});
test("invalid numeric wire sequence rejected",()=>assert.equal(applyBatch(emptySync("ws_alpha"),[event(1,{sequence:"-1"})]).error,"InvalidEvent"));

test("numeric primitive wire sequence rejected",()=>assert.equal(applyBatch(emptySync("ws_alpha"),[event(1,{sequence:1})]).error,"InvalidEvent"));
