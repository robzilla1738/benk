# 13 — Integrations, interoperability, and migration

## Initial integration boundary
Support the minimum sources required by the first partner workflow: one repository provider, one issue source, and one coding-agent adapter. Keep identity and authorization centrally owned. Integration-specific IDs, webhook formats, or model session representations do not become the product's task schema.

Repository actions should begin with read, working-copy preparation, patch generation, and proposed pull-request publication. Production deployment and customer communication are separate later capabilities with their own approvals. Avoid asking for organization-wide administrative scopes when a selected repository installation suffices.

## Connector installation
Show publisher, version, scope, data destination, and accountable installer. Store tokens through a secrets provider and reference them by handle. Separate user-delegated credentials from organization service credentials. A connector can never use a service identity to expand the acting user's authority without explicit organizational policy. Revocation stops new use, invalidates caches, and identifies dependent workflows.

Verify OAuth/redirect behavior against the selected provider's current documentation. Webhooks require signature validation, replay protection, duplicate handling, delivery ledgers, bounded payloads, and a dead-letter/replay path. Keep external source versions and modification timestamps as provenance. Normalize safely without inventing missing data.

## ACP, MCP, and future agent protocols
ACP can normalize supported coding-agent sessions; MCP can expose or consume tools. Maintain tested adapter/version capability matrices rather than promising universal interchangeability. Protocol-defined permissions do not override task grants or source ACLs. A2A-style independent-agent delegation can be evaluated later; it is not needed to prove the initial workflow. [S12, S13.]

An installed agent or connector may ship executable code. Version pinning, publisher trust, sandboxing, and update review remain necessary. An agent marketplace without these controls is a liability, not a launch milestone.

## Slack coexistence
Start with an explicit one-way or narrowly scoped bidirectional bridge for selected task requests and outcomes. Show the authoritative home of each task and conversation. Record origin IDs to suppress loops. An externally mirrored message retains provenance and should not masquerade as locally authored. Permission revocation and edits/deletes must have a documented cross-system behavior.

Do not attempt full two-way history mirroring before the team has a useful workflow. API rate limits, app distribution, export eligibility, and file availability can constrain migration; check the current source rather than hard-code an assumed allowance. [S16, S17.]

## Import protocol
Run a preflight showing source workspace, requested scope, administrative authority, expected volume, privacy mappings, identity mappings, attachments, unsupported features, and retention implications. Import into a staged workspace or isolated batch. Preserve original IDs/times in provenance while assigning valid local identities and ordering. Private channels stay private. Unmapped people are historical identities, not invited active users.

Support restart from a checkpoint without duplicate rows. Validate counts and sample content, then produce a reconciliation report: imported, skipped, inaccessible, malformed, deduplicated, and failed. Never call the import complete because the API stopped returning pages without checking pagination and rate-limit state.

## Export and portability
Provide structured workspace/task exports with messages, threads, actors, permissions metadata where authorized, contracts, artifact references, approvals, evidence, and provenance. Export is itself a privileged action and may require audit or policy approval. Redact secrets and honor scope; use expiring protected downloads. Customer data should remain portable even as the product becomes valuable.

## Integration release criteria
A connector is enabled only after least-privilege install, revoke/reinstall, duplicate/out-of-order webhook, rate limit, retry/unknown-outcome, token expiry, permission loss, and data-deletion scenarios are tested. Count completed useful work, not number of logos on an integration page.
