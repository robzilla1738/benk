# Native Human–Agent Workspace

Start with [00_START_HERE.md](00_START_HERE.md), or open [INDEX.html](INDEX.html).

The handoff preserves the full ambition: Slack-level communication, shared human–agent execution, local development capabilities, cloud runners, excellent native performance, explicit accountability, inspectable context, versioned review, and a practical adoption path.

| Directory | Purpose |
|---|---|
| `docs/` | Product, UX, architecture, security, operations, and delivery specifications. |
| `adrs/` | Baseline architecture decisions and their tradeoffs. |
| `contracts/` | JSON Schema, OpenAPI, event and IPC contracts, and example payloads. |
| `data/` | Reference PostgreSQL design and executable SQLite cache schema. |
| `planning/` | Requirements, implementation backlog, parity register, scenarios, risks, and gates. |
| `reference/core/` | Dependency-free TypeScript domain reference, tests, and a simulated walkthrough. |
| `scaffolds/` | GPUI, Rust-domain, Effect-service, and local-infrastructure starting points. |
| `prompts/` | Coordinator, implementation, review, and resume instructions for build agents. |
| `templates/` | Evidence, task, decision, incident, and handoff templates. |
| `scripts/` | Verification, fixture generation, and integrity utilities. |
| `verification/` | Recorded results and explicit validation limits. |

Do not mistake passing reference tests for an implemented or secure product. The release gates require evidence from the real application.
