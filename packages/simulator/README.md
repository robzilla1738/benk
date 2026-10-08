# Deterministic review simulator
A safe fixture that produces a versioned artifact and requires a distinct
simulated human to accept it. It never calls a model or tool. Its review inputs
are not authenticated production facts. Do not expose reviewDemo as a real
approval endpoint. `npm run demo` runs the complete synthetic flow.
