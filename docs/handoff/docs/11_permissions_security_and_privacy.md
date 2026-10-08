# 11 — Security, authorization, and privacy specification

This is a threat-informed design requirement, not a certification or claim that the supplied code enforces operating-system isolation. Real execution and public deployment remain disabled until implementation evidence exists.

## Threats and protected assets
Protect conversations, local files, repositories, credentials, private sources, artifacts, organization money, publication authority, account identity, and audit integrity. Threat actors include malicious content authors, compromised connectors, adversarial model output, unauthorized workspace members, compromised devices/runners, hostile preview code, and accidental cross-tenant software defects.

Treat user messages, retrieved documents, repository instructions, tool outputs, connector descriptions, and generated content as untrusted data. None can redefine system policy or confer authority. Do not depend on a model to reliably ignore prompt injection; bound the consequences through permission and execution controls.

## Effective authority
An action is permitted only when it fits the current human owner's authority, the agent's grants, workspace policy, task contract, environment capability, destination/audience policy, and required approval. Deny if any necessary decision is missing or stale. Caller-provided role, actor ID, workspace ID, cost, or approval state is untrusted until resolved by authoritative services.

Separate read permission, use-as-model-context permission, transform/derive permission, and publish permission. An employee being able to read a private document does not authorize an agent to summarize it into a public team channel. Default derived artifact visibility is no broader than the intersection of source restrictions; explicit authorized declassification is a separate audited operation, not a checkbox the agent can set.

## Approval protocol
Persist the proposed action before requesting approval. Canonicalize the full semantic payload with a reviewed implementation; use a domain-separated cryptographic digest. RFC 8785 is a candidate canonical JSON standard, not a reason to roll a casual key-sorting substitute. Bind workspace, task contract revision, run/action ID, artifact version/digest, destination, capabilities, policy revision, expiry, and relevant resource versions. [S19.]

The trusted UI displays the same data that is bound. Human identity and allowed reviewer role are verified server-side. Apply any quorum or separation-of-duty policy. At dispatch, revalidate all bindings and current permissions, atomically consume the single-use approval/intent, reserve budget, and create the durable dispatch record. A changed diff/destination must not reuse an old decision.

A preflight policy function cannot prevent a time-of-check/time-of-use race by itself. The reference function is illustrative; production consumes approvals under transaction/locking or equivalent serializable constraints. External execution still needs idempotency and reconciliation.

## Identity and tenancy
Use a verified identity provider adapter; validate token signature, issuer, audience, expiry, nonce/session binding, and revocation policy as applicable. Browser session protection includes secure cookies, CSRF/origin defenses, and step-up controls where needed. Native login uses a standards-appropriate external browser flow and secure OS credential storage. Do not invent a password system for the first release. [S20.]

Every shared relational reference carries workspace scope or an equivalent enforced tenant key. Never authorize merely because an ID exists. PostgreSQL RLS can add tenant defense in depth; the application role must not bypass it, and connection-pool tenant context must be transaction-local. Table-owner/bypass behavior requires careful configuration. Tenant RLS alone does not implement private-channel ACLs. [S11.]

## Runner and connector defenses
Broker grants are short-lived, scoped, audience-bound, and checked on every action. Restrict filesystem roots, child environment, network destinations, process count, output volume, and runtime. Avoid arbitrary shell string interpolation; use argument arrays and explicit working directories. Validate archives against traversal, symlink escapes, decompression bombs, and unexpected executable content. Do not expose Docker sockets, host credentials, or broad cloud metadata endpoints.

Connector installs need publisher/version identity, requested scopes, consent, revocation, and audit. OAuth tokens are stored as secret handles, not passed through arbitrary tools. Verify webhook signatures and replay windows. Prevent SSRF, DNS rebinding, redirect-based bypass, and token forwarding to an unintended audience. MCP security guidance identifies these classes of risk; protocol adoption is not itself a control. [S13.]

## Privacy and retention
Separate operational metadata from user content. Default telemetry records timing, counts, safe status/error codes, and opaque IDs, not source text or prompts. Diagnostics require explicit redaction and a visible export step. Model/data destinations are shown before sensitive work. Do not claim zero retention or no training without verifying the provider/account-specific terms.

Store audit content as minimal structured facts and protected references, not immutable copies of everything users delete. Define retention and legal-hold precedence with qualified legal review. Purge messages, artifacts, indexes, context derivatives, cached thumbnails, and downstream copies under policy; backups and offline devices have explicit limitations. Do not promise instant deletion everywhere.

## Release-blocking tests
Cross-workspace ID substitution; private-to-public context laundering; changed action after approval; approval replay/concurrent consumption; revoked membership mid-run; stale coordinator dispatch; token/secret leakage; symlink escape; malicious dependency install; preview-origin abuse; SSRF; poisoned connector output; unsafe retry after timeout; budget races; unsigned update; account-cache mixing. Any exploitable unauthorized read/write blocks release even when the happy path works.
