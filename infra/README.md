# Optional local PostgreSQL
Not needed for the simulator. Copy .env.example to an ignored .env, set a
non-production password and run:

    docker compose --env-file .env -f infra/compose.yaml up -d

No migrations are automatically applied. The historical SQL is a reference,
not a production migration chain. Add migration and transactional authorization
tests before real writes. The port binds to loopback. `down -v` destroys local
data and must be deliberate. Pin the image digest before release.
