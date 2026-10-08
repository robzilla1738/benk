export function facts() {
  const digest="sha256:"+"a".repeat(64);
  return {
    nowMs:1000,
    subject:{workspaceId:"ws_alpha",principalId:"agt_runner",active:true,ownerActive:true,
      allowedActions:["pull_request.create"],allowedResources:["repo_checkout"],
      allowedDestinations:["repo_checkout"],allowedModelDestinations:["simulator:no-network"]},
    run:{workspaceId:"ws_alpha",id:"run_checkout",taskId:"task_checkout",agentId:"agt_runner",
      contractRevision:1,generation:1,state:"running",cancellationRequested:false,modelDestination:"simulator:no-network"},
    authority:{workspaceId:"ws_alpha",runId:"run_checkout",generation:1,leaseExpiresAtMs:2000,
      policyRevision:1,contractRevision:1,budgetReserved:true},
    action:{workspaceId:"ws_alpha",id:"act_openpr",runId:"run_checkout",taskId:"task_checkout",generation:1,
      contractRevision:1,policyRevision:1,kind:"pull_request.create",resourceIds:["repo_checkout"],
      destinationId:"repo_checkout",artifactVersionId:"av_version01",digest,state:"ready",requiresApproval:true,expiresAtMs:1900},
    approval:{workspaceId:"ws_alpha",actionId:"act_openpr",decision:"approved",deciderKind:"human",
      deciderCurrentlyAuthorized:true,separationOfDutySatisfied:true,quorumSatisfied:true,actionDigest:digest,
      artifactVersionId:"av_version01",contractRevision:1,policyRevision:1,expiresAtMs:1800,consumed:false}
  };
}
export const event=(sequence,patch={})=>({eventId:`evt_${sequence}`,workspaceId:"ws_alpha",sequence:String(sequence),
  messageId:"msg_first",revision:Number(sequence),kind:"upsert",body:`body ${sequence}`,...patch});
