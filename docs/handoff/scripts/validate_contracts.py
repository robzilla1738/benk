#!/usr/bin/env python3
"""Validate local JSON Schemas and positive/negative fixtures; never fetch schemas."""
from pathlib import Path
import json
import sys
try:
    from jsonschema import Draft202012Validator, FormatChecker
except ImportError:
    raise SystemExit("Install requirements-qa.txt before running contract validation.")
ROOT = Path(__file__).resolve().parents[1]

def main() -> int:
    schema = json.loads((ROOT / "contracts/schemas/domain.schema.json").read_text())
    ipc = json.loads((ROOT / "contracts/schemas/ipc.schema.json").read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator.check_schema(ipc)
    index = json.loads((ROOT / "contracts/examples/index.json").read_text())
    outcomes = []
    for item in index:
        selected = {"$schema": schema["$schema"], "$defs": schema["$defs"],
                    "$ref": "#/$defs/" + item["definition"]}
        validator = Draft202012Validator(selected, format_checker=FormatChecker())
        value = json.loads((ROOT / item["path"]).read_text())
        errors = list(validator.iter_errors(value))
        observed = not errors
        ok = observed == item["valid"]
        outcomes.append({"path": item["path"], "expected_valid": item["valid"],
                         "observed_valid": observed, "passed": ok,
                         "validation_errors": [e.message for e in errors]})
    result = {"check": "contract_fixture_validation", "schemas_checked": 2,
              "fixtures": len(outcomes), "passed": sum(x["passed"] for x in outcomes),
              "failed": sum(not x["passed"] for x in outcomes), "results": outcomes,
              "limit": "Structural fixtures only; not runtime authorization or API implementation."}
    print(json.dumps(result, indent=2))
    return 0 if result["failed"] == 0 else 1

if __name__ == "__main__":
    sys.exit(main())
