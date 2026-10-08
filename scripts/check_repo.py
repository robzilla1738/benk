#!/usr/bin/env python3
"""Dependency-free structure and seed-boundary checks; not a security audit."""
import json,re,sys
from pathlib import Path
R=Path(__file__).resolve().parents[1]
errors=[]
for name in ['README.md','AGENTS.md','docs/BUILD_AGENT.md','docs/DEVELOPMENT.md','docs/NEXT_STEPS.md','apps/desktop/src/main.rs','services/control-plane/src/simulation-service.ts','crates/broker/src/lib.rs','docs/handoff/00_START_HERE.md']:
    if not (R/name).is_file():errors.append('Missing '+name)
for name in ['package.json','packages/domain/package.json','packages/simulator/package.json','services/control-plane/package.json','services/agent-engine/package.json']:
    if not json.loads((R/name).read_text()).get('private'):errors.append('Package must be private: '+name)
for f in (R/'apps/desktop/src').rglob('*.rs'):
    if re.search(r'std::process::Command|tokio::process::Command',f.read_text()):errors.append('Renderer execution detected: '+str(f))
if errors: print('\n'.join(errors));sys.exit(1)
print('Benk seed structure passed. This is not production/security certification.')
