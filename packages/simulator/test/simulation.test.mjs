import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { createDemo, advanceDemo, demoAwaitingReview, reviewDemo, DEMO_ARTIFACT_CONTENT, DEMO_ARTIFACT_DIGEST } from '../dist/index.js';
const review=(s,o={})=>({reviewerId:'human_reviewer',reviewerKind:'human',reviewerAuthorized:true,artifactId:s.artifact.id,artifactVersion:s.artifact.version,artifactDigest:s.artifact.digest,...o});
test('starts as zero-capability simulation',()=>{const s=createDemo();assert.equal(s.mode,'simulation');assert.deepEqual(s.contract.allowedCapabilities,[]);assert.equal(s.contract.externalActionsAllowed,false);assert.equal(s.externalActionsPerformed,0);});
test('explicit draft-ready-active-review progression',()=>{let s=createDemo();for(const state of ['ready','active','awaiting_review']){s=advanceDemo(s);assert.equal(s.taskState,state);}assert.equal(s.runState,'succeeded');});
test('successful run does not accept task',()=>assert.equal(demoAwaitingReview().taskState,'awaiting_review'));
test('artifact digest matches exact bytes',()=>assert.equal(DEMO_ARTIFACT_DIGEST,'sha256:'+createHash('sha256').update(DEMO_ARTIFACT_CONTENT).digest('hex')));
test('prior state is not mutated',()=>{const s=createDemo();const next=advanceDemo(s);assert.equal(s.taskState,'draft');assert.deepEqual(s.events,[]);assert.notEqual(s,next);});
test('distinct simulated human can review exact artifact',()=>{const s=demoAwaitingReview();const out=reviewDemo(s,review(s));assert.equal(out.taskState,'accepted');assert.equal(out.acceptedBy,'human_reviewer');assert.equal(s.taskState,'awaiting_review');});
for(const [name,override,code] of [
 ['agent',{reviewerKind:'agent'},'HumanRequired'],['self',{reviewerId:'human_requester'},'SeparationOfDuty'],
 ['unauthorized',{reviewerAuthorized:false},'ReviewerNotAuthorized'],['empty identity',{reviewerId:' '},'ReviewerNotAuthorized'],
 ['wrong artifact',{artifactId:'other'},'StaleArtifact'],['stale version',{artifactVersion:2},'StaleArtifact'],['wrong digest',{artifactDigest:'bad'},'StaleArtifact']
]) test(`rejects ${name}`,()=>{const s=demoAwaitingReview();assert.throws(()=>reviewDemo(s,review(s,override)),e=>e.code===code);});
test('review requires output',()=>assert.throws(()=>reviewDemo(createDemo(),{}),e=>e.code==='ReviewNotReady'));
test('accepted task cannot silently advance',()=>{const s=demoAwaitingReview();assert.throws(()=>advanceDemo(reviewDemo(s,review(s))),e=>e.code==='IllegalTransition');});
test('external action count remains zero',()=>{let s=createDemo();for(let i=0;i<3;i++){s=advanceDemo(s);assert.equal(s.externalActionsPerformed,0);}});
test('evidence does not claim real tests ran',()=>assert.match(demoAwaitingReview().artifact.verifiedChecks[0],/no real tests executed/));
