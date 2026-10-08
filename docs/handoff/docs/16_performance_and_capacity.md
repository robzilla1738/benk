# 16 — Performance budgets, capacity, and benchmarking

## Targets, not results
The following are initial product targets. No GPUI implementation or distributed system was benchmarked during handoff creation. Set a hardware/OS fixture before accepting or revising them. Record release build, dependency versions, dataset, warm/cold cache conditions, sample count, median, p95, p99, memory, and CPU.

| Experience | Initial target | Test fixture |
|---|---|---|
| Warm launch to interactive cached workspace | p95 < 750 ms. | Named supported laptop; no agent/media engine preloaded. |
| Cached channel switch | p95 < 50 ms. | 100,000 cached messages across 100 channels. |
| Local search initial results | p95 < 150 ms. | Defined keyword/ID/filter set over the same dataset. |
| Composition and scroll | Meet display frame budget without recurring input stalls. | Long messages, IME input, attachments, varying row heights. |
| Agent stream isolation | No material chat input regression with 10 concurrent synthetic streams. | Defined 20 updates/second/stream before batching. |
| Idle application | No continuous redraw or polling loop; explicit wakeup/CPU target set after platform baseline. | No calls/runs, one open workspace. |
| Interactive API | Provisional p95 < 200 ms excluding network/client and external providers. | Declared load, database size, and region. |

Do not treat cold launch as warm launch, omit indexing workloads from memory, or compare release and debug builds. Whole-process-tree resource use matters; separately report optional agent/media/model workloads without hiding them.

## Workload envelope
Initial load fixture: 100 active humans in a workspace, 100,000 cached messages on a test client, ten concurrent run streams, a 10,000-line diff, several attachment thumbnails, and two open native windows. Server testing includes multiple independent workspaces to detect noisy-neighbor effects. These are validation scenarios, not hard product limits or guaranteed production capacity.

## Native controls
Virtualize timelines and large artifact lists. Cache layout by stable ID/revision/width/font scale. Move SQL, search parsing, large Markdown processing, and indexing off the UI path. Batch run summaries at a bounded display cadence; do not redraw the full workspace for every token. Lazy-load terminals, diff internals, and media. Bound attachment caches and use explicit eviction. Low disk space produces a recoverable warning and reduced cache behavior, not silent draft loss.

## Server controls
Separate queues for interactive commands, delivery, search indexing, and execution. Per-workspace/provider concurrency limits and admission control prevent one agent loop from saturating the fleet. Bound request/response sizes and subscription fanout. Keep slow external operations outside database transactions. Use query plans, indexes, connection-pool limits, and backpressure before adding services.

## Streaming controls
Durable events cannot be dropped; buffer or force resumable catch-up. Presence may be dropped/expired. Logs may be batched, sampled for display, and truncated under an explicit quota while preserving relevant artifact references and a visible truncation notice. A UI ring buffer cannot become the only audit record of consequential actions.

## Benchmark method
Generate synthetic fixtures using `scripts/generate_fixture.py`, keeping seeds/counts explicit. Run at least 30 launch/search/navigation repetitions for an initial distribution, then use a larger controlled suite for release decisions. Warm-up runs are labeled and excluded consistently. Profile before optimizing. Record actual traces and regression comparisons; a single screenshot of a fast result is insufficient.

## Product efficiency
Measure time to first useful output, time to accepted outcome, human review minutes, number of interventions, and rework. A faster model response with more human cleanup is not necessarily a faster product. Compare the same workflow and acceptance standards against the team's previous process. Report quality and cost alongside latency.

## Release policy
A new feature cannot consume an unbounded portion of the interaction budget. Establish an explicit regression threshold and reviewer for each benchmark. Accessibility fixes must be engineered together with performance, not sacrificed to meet a frame-time graph. Serious input stalls, repeated notification delays, lost drafts, or unreliable reconnects block the corresponding release even when the average latency looks good.
