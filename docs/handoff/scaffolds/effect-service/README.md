# Effect service composition — not installed or compiled here

Targets exact `effect@4.0.2` and demonstrates a service, a Layer, a typed failure, and an explicitly simulated proposal. Verify the chosen release's `Context.Service`/Layer APIs against the source register before relying on the code.

```sh
npm install
npm run typecheck
npm run build
npm start
```

Commit the authentic package lock after successful resolution. No lockfile or dependency-install success is claimed in this handoff. This executable, once built, prints simulation output only. It does not implement an authenticated API, broker, Temporal worker, server, provider integration, or database. Use the domain contracts and services specification when extending it.

Production services require a process-lifetime runtime, validated configuration, explicit cleanup, authenticated actor resolution, transactions, and integration tests. Keep Effect runtime code out of Temporal deterministic workflow definitions; it belongs inside Activities. Do not expand this demonstration into an insecure mock success server.
