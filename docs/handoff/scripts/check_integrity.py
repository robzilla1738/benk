#!/usr/bin/env python3
"""Verify the final package inventory; the manifest excludes only itself and SHA256SUMS.txt."""
from pathlib import Path
import hashlib
import json
import sys
ROOT = Path(__file__).resolve().parents[1]
manifest_path = ROOT / "MANIFEST.json"
if not manifest_path.exists():
    raise SystemExit("MANIFEST.json is absent. This check is for the packaged handoff.")
manifest = json.loads(manifest_path.read_text())
errors = []
for item in manifest["files"]:
    path = ROOT / item["path"]
    if not path.is_file():
        errors.append("Missing: " + item["path"]); continue
    blob = path.read_bytes()
    if len(blob) != item["size_bytes"] or hashlib.sha256(blob).hexdigest() != item["sha256"]:
        errors.append("Changed: " + item["path"])
listed = {x["path"] for x in manifest["files"]}
extras = sorted(str(p.relative_to(ROOT)).replace('\\','/') for p in ROOT.rglob('*') if p.is_file()
                and str(p.relative_to(ROOT)).replace('\\','/') not in listed
                and p.name not in ("MANIFEST.json", "SHA256SUMS.txt")
                and "__pycache__" not in p.parts and "node_modules" not in p.parts)
print(json.dumps({"passed": not errors, "verified_files": len(listed), "errors": errors,
                  "additional_files": extras,
                  "note": "Original-file integrity only; new implementation files are reported but not rejected. Not a publisher signature."}, indent=2))
sys.exit(0 if not errors else 1)
