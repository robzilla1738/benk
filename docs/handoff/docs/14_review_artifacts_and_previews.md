# 14 — Artifacts, review, acceptance, and previews

## Artifact model
An artifact groups immutable versions. Versions have content digest, producer/run, task/contract context, protected blob reference, media type, size, audience restrictions, and provenance. Uploads use short-lived tickets, size ceilings, and server-side verification of actual bytes. An agent claiming a digest or test result does not establish its truth.

Artifacts include patches, documents, reports, datasets, preview bundles, test logs, and external receipts. Keep binaries/logs out of the ordinary message record. Generate thumbnails and rendered derivatives in a constrained pipeline with inherited access rules. Do not execute attachment content during indexing.

## Review package composition
The package opens with the requested outcome and current contract revision. It identifies the comparison baseline, concise proposed changes, artifact versions, checks, known limitations, and exact next decision. A reviewer can drill into the evidence without reading every agent activity message. Show "not run" and "inconclusive" distinctly from "passed". Use explicit scope: a unit-test pass does not mean a deployment or security review occurred.

For code, capture repository/commit, patch digest, file summary, build/test commands, environment version, exit results, relevant logs, preview link, and unresolved issues. For a document, capture source references, assumptions, reviewer comments, and unresolved factual questions. For external actions, capture destination, payload summary/digest, authorized actor, provider receipt, and uncertainty if confirmation is missing.

## Comment anchors and revisions
Comments attach to a specific artifact version and stable range/element anchor. A later version can map a comment when the anchor is still meaningful, but must not pretend a moved/deleted line is the same review context. Keep earlier discussions accessible. Show which comments were resolved and by whom.

A materially changed artifact invalidates approval bindings that name the old version. Small changes are not automatically harmless: a destination address or permission field can be a critical one-character edit. Evaluate binding changes semantically rather than only by diff size.

## Acceptance versus external authorization
Task acceptance is a human decision that named criteria are met for exact versions. Opening a pull request, sending a message outside the task audience, merging code, or deploying may require separate authorization. The UI must not combine unrelated actions under a vague "Looks good" button. Labels should say "Accept this result", "Approve opening this pull request", or "Request changes".

An acceptance record is immutable history. Reopening work creates another contract/review cycle without erasing the old acceptance. A workflow can automate checks, but its own success cannot manufacture a human acceptance when the task policy requires one.

## Preview isolation
Initially prefer an isolated browser origin. Generated previews have no native broker bridge, application authentication cookie, unrestricted localhost access, or unreviewed network egress. Use content security policy and sandboxing appropriate to the actual deployment. Authenticate preview access separately from the generated application. A public-looking URL is not permission.

Screen captures, downloads, and preview-generated artifacts can contain sensitive source data; inherit access and retention rules. Expire environments and release resources after a defined idle/lifetime limit. Stop/revoke a preview explicitly when the task or source access requires it.

## Review usability gate
A teammate who did not initiate the task must identify the goal, important change, evidence, risk, and next action without a narrated walkthrough. Measure review time, requests for missing context, rework after acceptance, and mistaken assumptions about what was verified. The product succeeds when review is easier and more reliable—not merely when it has a sophisticated diff viewer.
