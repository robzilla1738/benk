# 27 — Enterprise administration and customer-managed execution

## Foundations to design early
Tenant boundaries, scoped principals, audit events, policy revisions, resource ACLs, data lifecycle, and workspace home region are foundational. Full enterprise feature delivery can wait for demand; retrofitting these basic boundaries cannot.

## Administration model
Support owner/admin/member/guest/agent roles with explicit capability mapping. Administrative role does not automatically imply unrestricted access to every private conversation; define customer policy and exceptional access through a visible, audited flow. Separate billing management, identity administration, content administration, and execution-policy management where required.

Agents retain accountable human ownership. Deactivating a person triggers revocation of their sessions/grants, handling of active runs, and reassignment or stop of owned agents/workflows. Revoking one integration does not accidentally revoke unrelated credentials or leave shared service-token access active.

## Identity and provisioning
Add enterprise SSO and provisioning through the selected identity integration. Test invite, join, role change, group mapping, suspension, deprovisioning, reactivation, and ownership transfer. Provisioning events are idempotent and reconciled; they must not widen access due to an out-of-order update. Native/browser/mobile sessions and local caches participate in the revocation policy.

## Retention, discovery, and export
Policies can differ by resource class and workspace. Define legal hold precedence, administrative access, audit trail, export authorization, backup handling, and local-device limitations with appropriate legal review. A retention screen is not compliance certification. Deletion must propagate to search indexes, thumbnails, context packages, and relevant derivatives. Avoid logging raw content into immutable audit records that make deletion promises impossible.

## Company-managed runners
A customer can register a runner in its network with a scoped identity and supported outbound connection. Registration, rotation, revocation, image/version policy, network boundaries, and health are explicit. The runner receives bounded commands and generation-bound grants; it cannot invent shared approvals. Show whether model requests and artifacts leave the company environment. Do not describe the entire product as self-hosted merely because a runner is local.

## Regional and encryption requirements
A workspace home region covers databases, artifacts, workflow histories, backups, telemetry, and model routing considerations. Offer only guarantees that actual infrastructure and provider agreements support. Distinguish transport/storage encryption from end-to-end encryption; ordinary server-side search cannot operate on plaintext the server is architecturally unable to read. Customer-managed keys are a separately designed feature, not a label added to existing encryption.

## Enterprise readiness gate
Require deployment documentation, restore evidence, access reviews, independent security assessment, support ownership, change control, and measured operational targets. Claims about regulatory compliance, certifications, provider data retention, and contract commitments need current evidence and qualified review. Sell a precise supported capability set rather than a generic "enterprise-ready" badge.
