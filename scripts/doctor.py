#!/usr/bin/env python3
"""Read-only tool report. No installs, network calls or credential inspection."""
import json, shutil, subprocess
from pathlib import Path
R=Path(__file__).resolve().parents[1]
report={'product':'Benk','tools':{}}
for tool in ['node','npm','cargo','rustc','python3','git']:
    exe=shutil.which(tool)
    if not exe: report['tools'][tool]={'available':False};continue
    try:
        p=subprocess.run([exe,'--version'],capture_output=True,text=True,timeout=15,cwd=R)
        report['tools'][tool]={'available':p.returncode==0,'version':(p.stdout or p.stderr).strip()}
    except subprocess.TimeoutExpired: report['tools'][tool]={'available':False,'reason':'timed out'}
report['locks']={name:(R/name).exists() for name in ['package-lock.json','Cargo.lock']}
print(json.dumps(report,indent=2))
