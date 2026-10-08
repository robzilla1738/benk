import test from "node:test";
import assert from "node:assert/strict";
import {reserveBudget,settleBudget} from "../dist/budget.js";
const b=()=>({limit:"1000",spent:"100",reserved:"200"});
test("reserve exactly remaining allowance",()=>assert.equal(reserveBudget(b(),"700").value.reserved,"900"));
test("reject over reservation",()=>assert.equal(reserveBudget(b(),"701").error,"BudgetExceeded"));
for (const value of ["-1","1.1","01","1e2","","NaN","100000000000000000000"]) test(`reject invalid amount ${JSON.stringify(value)}`,()=>assert.equal(reserveBudget(b(),value).error,"InvalidAmount"));
test("exact integers beyond JS safe integer",()=>{
  const r=reserveBudget({limit:"9007199254740995",spent:"9007199254740993",reserved:"0"},"2");
  assert.equal(r.ok,true);assert.equal(r.value.reserved,"2");
});
test("settlement releases reservation and records actual usage",()=>assert.deepEqual(settleBudget(b(),"200","150").value,{budget:{limit:"1000",spent:"250",reserved:"0"},overLimit:false,exceededReservation:false}));
test("overage is recorded rather than hidden",()=>{
  const r=settleBudget(b(),"200","1200");assert.equal(r.value.budget.spent,"1300");assert.equal(r.value.overLimit,true);assert.equal(r.value.exceededReservation,true);
});
test("cannot settle more than reserved aggregate",()=>assert.equal(settleBudget(b(),"201","50").error,"ReservationMismatch"));
test("no new work after actual overage",()=>assert.equal(reserveBudget({limit:"100",spent:"101",reserved:"0"},"0").error,"BudgetExceeded"));
