import { demoAwaitingReview, reviewDemo } from './dist/index.js';
const pending=demoAwaitingReview();
const accepted=reviewDemo(pending,{reviewerId:'human_reviewer',reviewerKind:'human',reviewerAuthorized:true,artifactId:pending.artifact.id,artifactVersion:pending.artifact.version,artifactDigest:pending.artifact.digest});
console.log(JSON.stringify({product:'Benk',warning:'SIMULATION ONLY — no models, tools, credentials or external actions',pending,accepted},null,2));
