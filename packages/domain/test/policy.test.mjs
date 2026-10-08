import test from "node:test";
import assert from "node:assert/strict";
import {evaluateDispatch} from "../dist/policy.js";
import {facts} from "./helpers.mjs";
test("all valid authoritative preflight facts permit dispatch proposal",()=>assert.deepEqual(evaluateDispatch(facts()),{allowed:true}));
const cases=[
 ["tenant mismatch",f=>f.action.workspaceId="ws_beta","TenantMismatch"],
 ["inactive agent",f=>f.subject.active=false,"IdentityInactive"],
 ["inactive owner",f=>f.subject.ownerActive=false,"IdentityInactive"],
 ["wrong principal",f=>f.subject.principalId="agt_other","PrincipalMismatch"],
 ["wrong run",f=>f.action.runId="run_other","RunMismatch"],
 ["wrong task",f=>f.action.taskId="task_other","RunMismatch"],
 ["stale runner generation",f=>f.run.generation=2,"StaleGeneration"],
 ["stale action generation",f=>f.action.generation=2,"StaleGeneration"],
 ["lease expired at exact boundary",f=>f.nowMs=2000,"LeaseExpired"],
 ["cancel requested",f=>f.run.cancellationRequested=true,"RunStopped"],
 ["already terminal",f=>f.run.state="succeeded","RunStopped"],
 ["task revision changed",f=>f.authority.contractRevision=2,"RevisionChanged"],
 ["policy changed",f=>f.authority.policyRevision=2,"RevisionChanged"],
 ["already dispatching",f=>f.action.state="dispatching","ActionNotReady"],
 ["action expired",f=>f.action.expiresAtMs=1000,"ActionExpired"],
 ["unsupported capability",f=>f.action.kind="deployment.create","CapabilityDenied"],
 ["unapproved resource",f=>f.action.resourceIds.push("repo_secret"),"ResourceDenied"],
 ["empty resource set",f=>f.action.resourceIds=[],"ResourceDenied"],
 ["private to public destination",f=>f.action.destinationId="chn_public","DestinationDenied"],
 ["unapproved model destination",f=>f.run.modelDestination="unapproved-provider","ModelDestinationDenied"],
 ["no budget reservation",f=>f.authority.budgetReserved=false,"BudgetNotReserved"],
 ["missing approval",f=>f.approval=null,"ApprovalRequired"],
 ["agent tries to approve",f=>f.approval.deciderKind="agent","ApprovalInvalid"],
 ["reviewer access revoked",f=>f.approval.deciderCurrentlyAuthorized=false,"ApprovalInvalid"],
 ["separation of duty not met",f=>f.approval.separationOfDutySatisfied=false,"ApprovalInvalid"],
 ["quorum not met",f=>f.approval.quorumSatisfied=false,"ApprovalInvalid"],
 ["approval wrong tenant",f=>f.approval.workspaceId="ws_other","ApprovalInvalid"],
 ["approval rejected",f=>f.approval.decision="rejected","ApprovalInvalid"],
 ["approval wrong action",f=>f.approval.actionId="act_other","ApprovalInvalid"],
 ["approval consumed",f=>f.approval.consumed=true,"ApprovalConsumed"],
 ["approval expires at boundary",f=>f.approval.expiresAtMs=1000,"ApprovalStale"],
 ["changed action digest",f=>f.action.digest="sha256:"+"b".repeat(64),"ApprovalStale"],
 ["changed artifact version",f=>f.action.artifactVersionId="av_version02","ApprovalStale"],
 ["changed approval contract",f=>f.approval.contractRevision=2,"ApprovalStale"],
 ["invalid clock",f=>f.nowMs=NaN,"InvalidFacts"],
 ["invalid digest",f=>f.action.digest="hash","InvalidFacts"],
 ["invalid approval date",f=>f.approval.expiresAtMs=NaN,"InvalidFacts"]
];
for (const [name,mutate,expected] of cases) test(name,()=>{const f=facts();mutate(f);assert.deepEqual(evaluateDispatch(f),{allowed:false,reason:expected});});
test("policy-derived approval-free action can proceed without a human decision",()=>{
  const f=facts();f.action.requiresApproval=false;f.approval=null;assert.equal(evaluateDispatch(f).allowed,true);
});
test("preflight is intentionally pure and does not consume an approval",()=>{
  const f=facts();evaluateDispatch(f);assert.equal(f.approval.consumed,false);
});
