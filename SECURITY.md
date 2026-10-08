# Development security status
This is a scaffold, not a hardened service or certified agent sandbox. Do not
use it for production secrets or consequential external actions. Broker execution
is denied. Simulated identities are fixtures, not authentication. The local HTTP
adapter is read-only and loopback-bound; never deploy it publicly.

Before real execution complete identity, tenancy, verified grants, credential,
isolation, budget, cancellation, fencing, approval, audit, idempotency and recovery
gates in the original handoff. Process separation/worktrees are not a sandbox.
Treat agent outputs and imported content as untrusted data. Never let agents
approve themselves or silently disclose private context to a wider audience.

Report suspected vulnerabilities privately to the repository owner. Use GitHub
private reporting only when enabled; its availability is not assumed here. Do
not put credentials or sensitive evidence into public issues. No contact address
has been invented for this scaffold.
