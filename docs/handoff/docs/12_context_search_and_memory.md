# 12 — Context, search, and memory

## Context is an inspectable product object
A task's context package includes goal, contract revision, approved source references and versions, decisions, instructions, artifact references, and unresolved questions. It names what was actually provided to a model and why. Users can remove a source, replace an outdated decision, or compare context revisions. A context package is not a hidden global summary of everything an employee has ever read.

Distinguish four layers: personal scratch context, task-specific authorized context, project-approved knowledge, and organization policy. Personal content does not become shared because an agent helped create it. Project knowledge requires a visible owner and review/promotion step. Organization policy is configured through trusted administration, not learned from a conversation.

## Retrieval pipeline
Resolve the requester, acting agent, task, source-use permission, model destination, and output audience before retrieval. Apply source ACLs at retrieval/index-query time, then recheck returned references before model submission. Do not fetch all organization content and ask the model to avoid leaking it. Cache entries include authorization epoch and source version; stale permissions invalidate affected context and derivatives.

Use exact search first: people, IDs, repository paths, symbols, filenames, channel/project filters, dates, and keywords. Add semantic retrieval when a benchmark demonstrates improved results under the same access constraints. Semantic similarity is not permission. Keep relevance ranking separate from source truth and show provenance when an answer depends on retrieved content.

## Search user experience
The first results come from the authorized local cache and identify whether they are local/partial or server-complete. A background network search can add permitted results without replacing the selected item or stealing focus. Support keyboard navigation, scoped filters, highlighted matches, and search-within-thread/project. Preserve ordinary text search when AI features are disabled or offline.

Search snippets, result counts, suggestions, and autocomplete must not reveal hidden documents or channels. Sensitive sources may have stricter indexing and model-use rules than ordinary reading. The native cache stores only permitted content and removes revoked/deleted entries after processing. Full-text search uses a bounded query parser rather than forwarding arbitrary SQL or allowing an expensive raw query to monopolize the UI.

## Memory lifecycle
A memory record has source references, author/reviewer, scope, version, creation/update times, validity/expiry, and status (`proposed`, `approved`, `superseded`, `retracted`). An agent may propose a durable memory but cannot silently promote it to authoritative policy. Conflicting decisions remain visible and need resolution; the newest summary is not automatically the correct one.

When a source is deleted or access changes, locate dependent packages/answers/artifacts and apply the appropriate invalidation/retention policy. Where complete automatic removal of derived meaning cannot be assured, disclose the limitation rather than claim perfect forgetting. Avoid placing source text in immutable audit records.

## Agent-facing API
Provide typed operations to get a task brief, subscribe to task changes, request source access, propose a plan, publish an artifact version, request a decision, and report a blocker. An agent should not need to scrape thousands of chat messages to infer task state. Agent APIs use the same resource authorization as human APIs and return explicit unsupported/stale/denied states.

## Evaluation
Use synthetic permission graphs and deliberately conflicting/outdated sources. Measure relevant-reference recall, citation accuracy, permission violations, context size, cost, and human correction effort. A single unauthorized disclosure is a release blocker, not an acceptable average. Separately evaluate whether adding more context improves or harms accepted outcomes.
