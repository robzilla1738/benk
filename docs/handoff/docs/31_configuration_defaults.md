# 31 — Initial configuration defaults

These are conservative development/pilot starting points, not proven production limits. Configuration is versioned, validated at startup, and visible where it affects users. Production values need load/security review and an owner.

| Setting | Initial value or policy |
|---|---|
| Real agent/tool execution | Disabled until execution gates pass. |
| External publication/deployment | Disabled by default; each capability separately approved. |
| Local background runner | Off until the user explicitly enables it. |
| Execution/model location display | Always visible in contract and run details. |
| IPC frame cap | 1 MiB before allocation; large content travels by protected reference. |
| Encoded message body | 64 KiB maximum in the initial API, independent of schema character limits. |
| Initial artifact upload cap | 25 MiB per version; larger workloads require an explicit capability/configuration. |
| UI run-summary update cadence | Up to 5 updates/second/run after batching; durable events are not dropped. |
| Initial retained display log cap | 10 MiB/run with visible truncation and separate evidence policy. |
| Active runs | 2 per person and 3 per workspace initially; use admission control, not silent queue growth. |
| Pilot per-run budget proposal | USD 5 equivalent, explicitly confirmed; not a pricing promise. |
| Run wall-clock limit | 30 minutes initially; approval waits use durable paused state and separate expiry. |
| Individual remote request deadline | 120 seconds unless operation/provider semantics require a reviewed override. |
| Safe-read retry attempts | At most 3 with bounded jitter; non-idempotent unknown writes do not auto-retry. |
| Active run lease | 30 seconds with 10-second heartbeats while running; verify against authority time. |
| Approval proposal expiry | 10 minutes by default; current policy and artifact binding still rechecked. |
| Download access | Authenticated proxy preferred; any signed URL has short expiry and a disclosed revocation window. |
| Product telemetry | Content-free operational metadata; no prompts/source text by default. |
| Recording/transcription | Off until consent, access, and retention behavior are implemented. |
| Production retention | Explicit owner-approved policy required before real customer data. |
| Package/runtime versions | Exact reviewed pins/lockfiles selected by foundation ticket; no mutable latest assumptions. |

Budgets are reservations, not perfect forecasts; actual provider charges are reconciled. Lease expiry stops new authorized dispatch but cannot erase an already-performed external effect. Offline revocation cannot guarantee immediate deletion from a disconnected or compromised device. Defaults must not imply stronger guarantees than the implementation provides.
