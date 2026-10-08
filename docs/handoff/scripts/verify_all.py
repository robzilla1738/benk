#!/usr/bin/env python3
"""Run available handoff tests; a missing command is a failure, never a silent pass."""
from pathlib import Path
import shutil
import subprocess
import sys
ROOT=Path(__file__).resolve().parents[1]
npm=shutil.which('npm')
if not npm:
    raise SystemExit('npm/Node is required for the TypeScript reference checks.')
commands=[([sys.executable,'scripts/verify_handoff.py'],ROOT),
          ([sys.executable,'scripts/validate_contracts.py'],ROOT),
          ([sys.executable,'scripts/test_sqlite.py'],ROOT),
          ([npm,'run','build'],ROOT/'reference/core'),
          ([npm,'test'],ROOT/'reference/core')]
for command,where in commands:
    print('\nRUN:', ' '.join(command), 'IN', where, flush=True)
    result=subprocess.run(command,cwd=where,check=False)
    if result.returncode:
        raise SystemExit(result.returncode)
print('\nAll specified handoff checks passed. GPUI/Effect integration and PostgreSQL are not covered.')
