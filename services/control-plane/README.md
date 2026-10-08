# Effect control-plane seed
Read-only loopback HTTP adapter over an Effect service. No authentication,
production persistence, mutations, billing, model or external-action routes.
Run `npm run dev:control-plane`, then query 127.0.0.1:4317/healthz or
/v1/demo/snapshot. Public binding cannot be enabled by configuration. Browser
Origin headers and unexpected Host headers are rejected. Do not deploy this
adapter publicly or treat these development controls as production identity.
