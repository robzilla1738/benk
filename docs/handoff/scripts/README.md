# Verification and fixture scripts

`verify_handoff.py` checks JSON, ID references, ticket dependencies/cycles/phases, requirements traceability, gates, schema/OpenAPI equality, local references, and Markdown links. It is standard-library only.

`validate_contracts.py` uses `requirements-qa.txt` to validate the positive and negative fixtures, including timestamp formats. It does not call the network or implement runtime authorization.

`test_sqlite.py` executes the supplied local schema against fresh SQLite fixtures and tests search/deletion/isolation behavior. It is not a benchmark or a full sync implementation.

`verify_all.py` runs the above checks plus the TypeScript reference build and Node tests. Install the declared compiler in `reference/core` on a connected environment when no compatible compiler is available. A missing command or failure stops the run.

`generate_fixture.py --output /path/to/new/workload.sqlite --messages 100000 --channels 50 --seed 4108` builds synthetic content and a fixture manifest. It refuses existing paths and does not overwrite data. Choose an output outside this immutable handoff when testing it. Creating a workload is not a performance result.

`check_integrity.py` checks original payload hashes against the final manifest. New implementation files are reported separately; edited originals naturally fail the original integrity check. Integrity checks are not digital publisher signatures.

No script deploys, calls a model, uses credentials, or intentionally deletes a user's files.
