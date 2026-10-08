#!/usr/bin/env python3
"""Create a deterministic synthetic SQLite workload. No customer data or network access."""
from pathlib import Path
from datetime import datetime, timezone, timedelta
import argparse
import hashlib
import json
import random
import sqlite3
import sys
ROOT = Path(__file__).resolve().parents[1]

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--messages", type=int, default=100000)
    parser.add_argument("--channels", type=int, default=50)
    parser.add_argument("--seed", type=int, default=4108)
    args = parser.parse_args()
    if args.messages < 1 or args.messages > 2000000 or args.channels < 1 or args.channels > 10000:
        parser.error("messages must be 1..2,000,000 and channels 1..10,000")
    target = args.output.resolve()
    if target.exists():
        parser.error("Output already exists; use a new path. This script never overwrites data.")
    target.parent.mkdir(parents=True, exist_ok=True)
    rng = random.Random(args.seed)
    conn = sqlite3.connect(target)
    try:
        conn.executescript((ROOT/"data/local-schema.sql").read_text())
        with conn:
            for i in range(args.channels):
                conn.execute("INSERT INTO cached_channels VALUES (?,?,?,?,?,?,?)",
                             ("acct_synthetic","ws_synthetic",f"chn_{i:06d}",f"Synthetic project {i}","public",1,1))
            base = datetime(2026,10,8,tzinfo=timezone.utc)
            words = ["checkout","review","artifact","offline","permissions","latency","design","test","release","context"]
            rows=[]
            for i in range(args.messages):
                text = f"Synthetic message {i}: " + ' '.join(rng.choices(words,k=18))
                body=json.dumps({"format":"workspace_richtext_v1","blocks":[{"type":"paragraph","content":[{"type":"text","text":text}]}]},separators=(',',':'))
                rows.append(("acct_synthetic","ws_synthetic",f"chn_{i % args.channels:06d}",f"msg_{i:09d}","usr_synthetic",None,body,text,1,str(i+1),(base+timedelta(seconds=i)).isoformat().replace('+00:00','Z'),0))
                if len(rows)==1000:
                    conn.executemany("INSERT INTO cached_messages(account_id,workspace_id,channel_id,message_id,author_id,thread_root_id,body_json,body_text,revision,server_sequence,created_at,deleted) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",rows);rows.clear()
            if rows:
                conn.executemany("INSERT INTO cached_messages(account_id,workspace_id,channel_id,message_id,author_id,thread_root_id,body_json,body_text,revision,server_sequence,created_at,deleted) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",rows)
        counts = conn.execute("SELECT count(*) FROM cached_messages").fetchone()[0]
        indexed = conn.execute("SELECT count(*) FROM message_fts").fetchone()[0]
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")
    finally:
        conn.close()
    result={"synthetic":True,"seed":args.seed,"messages":counts,"indexed_rows":indexed,"channels":args.channels,
            "sqlite_version":sqlite3.sqlite_version,"sha256":hashlib.sha256(target.read_bytes()).hexdigest(),
            "note":"Workload fixture only; no native UI performance measurement is claimed."}
    target.with_suffix(target.suffix+".manifest.json").write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))
    return 0

if __name__ == "__main__":
    sys.exit(main())
