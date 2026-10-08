#!/usr/bin/env python3
"""Standard-library package coherence checks. Not a product/security certification."""
from pathlib import Path
from urllib.parse import unquote
import json
import re
import sys
ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    errors: list[str] = []
    warnings: list[str] = []
    def check(condition: bool, message: str) -> None:
        if not condition:
            errors.append(message)
    def load(path: str):
        return json.loads((ROOT / path).read_text(encoding="utf-8"))
    all_json = list(ROOT.rglob("*.json"))
    for path in all_json:
        if "node_modules" not in path.parts:
            try:
                json.loads(path.read_text(encoding="utf-8"))
            except (ValueError, OSError) as exc:
                errors.append(f"Invalid JSON {path.relative_to(ROOT)}: {exc}")
    required = ["00_START_HERE.md", "01_BUILD_AGENT_PROMPT.md", "AGENTS.md", "INDEX.html",
                "contracts/schemas/domain.schema.json", "contracts/openapi.json", "data/server-schema.sql",
                "data/local-schema.sql", "verification/RESULTS.md", "reference/core/dist/policy.js",
                "scaffolds/gpui-desktop/README.md", "scaffolds/effect-service/README.md",
                "prompts/FIRST_SESSION.md", "templates/IMPLEMENTATION_REPORT.md"]
    for p in required:
        check((ROOT / p).is_file() and (ROOT / p).stat().st_size > 0, f"Missing/empty required file: {p}")
    backlog = load("planning/backlog.json")
    requirements = load("planning/requirements.json")
    scenarios = load("planning/acceptance_catalog.json")
    trace = load("planning/traceability.json")
    parity = load("planning/slack_parity.json")
    gates = load("planning/release_gates.json")
    milestones = load("planning/milestones.json")
    def index(rows, label):
        result = {r["id"]: r for r in rows}
        check(len(result) == len(rows), f"Duplicate {label} IDs")
        return result
    tickets = index(backlog, "ticket")
    reqs = index(requirements, "requirement")
    ats = index(scenarios, "acceptance")
    pars = index(parity, "parity")
    sources = index(load("planning/source_register.json"), "source")
    for t in backlog:
        for dep in t["depends_on"]:
            check(dep in tickets, f"Unknown dependency {dep} on {t['id']}")
        for p in t["specs"]:
            check((ROOT / p).is_file(), f"Missing spec {p} on {t['id']}")
        for r in t["requirement_ids"]:
            check(r in reqs and reqs[r]["ticket_id"] == t["id"], f"Requirement mismatch on {t['id']} / {r}")
        for a in t["acceptance_ids"]:
            check(a in ats and ats[a]["ticket_id"] == t["id"], f"Scenario mismatch on {t['id']} / {a}")
    visiting, visited = set(), set()
    order = []
    def visit(tid):
        if tid in visiting:
            errors.append(f"Dependency cycle at {tid}"); return
        if tid in visited or tid not in tickets:
            return
        visiting.add(tid)
        for dep in tickets[tid]["depends_on"]:
            visit(dep)
        visiting.remove(tid); visited.add(tid); order.append(tid)
    for tid in tickets:
        visit(tid)
    for t in backlog:
        for dep in t["depends_on"]:
            if dep in tickets:
                check(tickets[dep]["phase"] <= t["phase"], f"Later-phase dependency: {t['id']} {t['phase']} -> {dep} {tickets[dep]['phase']}")
    for r in requirements:
        check(r["acceptance_id"] in ats, f"Missing acceptance for {r['id']}")
        if r["acceptance_id"] in ats:
            a = ats[r["acceptance_id"]]
            check(a["requirement_id"] == r["id"] and a["ticket_id"] == r["ticket_id"], f"Broken trace for {r['id']}")
        check((ROOT/r["spec"]).is_file(), f"Missing requirement spec {r['spec']}")
    check({r["id"] for r in requirements} == {x["requirement_id"] for x in trace}, "Traceability coverage mismatch")
    for x in trace:
        check(x["ticket_id"] in tickets and x["acceptance_id"] in ats, f"Unknown trace link: {x}")
    for p in parity:
        check(p["owner_ticket"] in tickets, f"Parity owner missing for {p['id']}")
    for g in gates:
        expected = {t["id"] for t in backlog if t["phase"] == g["phase"]}
        check(set(g["required_ticket_ids"]) == expected, f"Gate ticket coverage mismatch: {g['id']}")
    for m in milestones:
        check(set(m["ticket_ids"]) == {t["id"] for t in backlog if t["phase"] == m["id"]}, f"Milestone coverage mismatch {m['id']}")
    for d in load("planning/dependency_decisions.json"):
        for source in d.get("source_ids", []):
            check(source in sources, f"Unknown source ID {source}")
    schema = load("contracts/schemas/domain.schema.json")
    api = load("contracts/openapi.json")
    def convert(value):
        if isinstance(value, dict):
            return {k: (v.replace("#/$defs/", "#/components/schemas/") if k == "$ref" else convert(v)) for k,v in value.items()}
        if isinstance(value, list): return [convert(v) for v in value]
        return value
    check(convert(schema["$defs"]) == api["components"]["schemas"], "OpenAPI schemas diverge from domain definitions")
    def inspect_refs(value, defs, prefix):
        if isinstance(value, dict):
            if "$ref" in value:
                ref = value["$ref"]
                check(ref.startswith(prefix) and ref[len(prefix):] in defs, f"Unresolved/local-contract reference {ref}")
            for v in value.values(): inspect_refs(v, defs, prefix)
        elif isinstance(value, list):
            for v in value: inspect_refs(v, defs, prefix)
    inspect_refs(schema, schema["$defs"], "#/$defs/")
    inspect_refs(api, api["components"]["schemas"], "#/components/schemas/")
    ipc = load("contracts/schemas/ipc.schema.json")
    inspect_refs(ipc, ipc["$defs"], "#/$defs/")
    ops = [op["operationId"] for path in api["paths"].values() for method,op in path.items() if isinstance(op,dict) and "operationId" in op]
    check(len(ops) == len(set(ops)), "Duplicate OpenAPI operation IDs")
    for fixture in load("contracts/examples/index.json"):
        check((ROOT/fixture["path"]).is_file(), f"Missing fixture {fixture['path']}")
        check(fixture["definition"] in schema["$defs"], f"Missing fixture definition {fixture['definition']}")
    broken_links = 0
    for path in ROOT.rglob("*.md"):
        if "node_modules" in path.parts: continue
        text = path.read_text(encoding="utf-8")
        for target in re.findall(r"\[[^\]]+\]\(([^)]+)\)", text):
            target = target.split(' "')[0]
            if "://" in target or target.startswith(("#","mailto:")): continue
            local = unquote(target.split("#")[0])
            if not local: continue
            resolved=(path.parent/local).resolve()
            if not resolved.exists():
                errors.append(f"Broken Markdown link {path.relative_to(ROOT)} -> {target}"); broken_links += 1
    forbidden=[p for p in ROOT.rglob('*') if p.is_file() and (p.name == '.env' or p.suffix.lower() in ('.pem','.key','.ttf','.otf'))]
    check(not forbidden, "Unexpected credentials/font-like files: " + ', '.join(str(p.relative_to(ROOT)) for p in forbidden))
    result = {"check": "handoff_coherence", "passed": not errors, "errors": errors, "warnings": warnings,
              "counts": {"json_files": len(all_json), "tickets": len(tickets), "requirements": len(reqs),
                         "acceptance_scenarios": len(ats), "parity_candidates": len(pars), "release_gates": len(gates),
                         "schema_definitions": len(schema["$defs"]), "api_operations": len(ops), "source_records": len(sources)},
              "topological_ticket_order": order,
              "limit": "Static handoff consistency, not implemented feature correctness or security certification."}
    print(json.dumps(result, indent=2))
    return 0 if not errors else 1

if __name__ == "__main__":
    sys.exit(main())
