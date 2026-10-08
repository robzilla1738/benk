# Separate Effect engine process
Deterministic simulation only. No provider, shell, model credentials, MCP server,
runner lease or external action is wired. Next: authenticated typed IPC and a
durable execution journal; then one supported adapter after the security gates.
Keep one coordinator per run. Never place execution privileges in the renderer.
