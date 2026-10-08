# Disposable local infrastructure — not started or verified

The compose template requires explicit image/password inputs and binds only to loopback. Select a supported PostgreSQL version, record a real immutable image digest, and verify that version's data-directory mount convention. No image tag/digest is invented here. The mounted parent path accommodates versions with versioned subdirectories; check the selected image documentation before first use or upgrade.

From this directory, create a private `.env` from `.env.example`, fill actual local values, then run `docker compose config` to inspect the resolved configuration before `docker compose up -d`. Do not share the rendered configuration because it contains a password. Run the server DDL only in this disposable database and capture actual SQL test results. The DDL is a reference baseline, not a production migration or complete permission system.

Do not expose this database publicly, reuse the owner account as the application role, or enable real customer data. Separate a restricted application role, migration role, authenticated session scope, backups, current resource authorization, and secret handling are required. `docker compose down` stops it; `docker compose down -v` deletes its data and requires an explicit intentional decision. No command in the handoff automatically runs the destructive form.

Temporal, object storage, identity, and provider services must be chosen and configured through their tickets. The template deliberately does not pretend that a collection of containers constitutes production readiness.
