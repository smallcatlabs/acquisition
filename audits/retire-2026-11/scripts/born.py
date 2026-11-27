"""When each entry was entered: the first commit whose diff adds its bullet.

usage, from the tree's root:
  python3 -I audits/retire-2026-11/scripts/born.py > born.tsv     id, date, commit, subject (about two seconds an id)

`git log --reverse -S'**C<n> —'` over the whole history, so an entry
written before the root swap is found under `spikes/rust-playground/`.
C1 to C63 were entered together when the registry was made, so their
date is the registry's and says nothing of when each was ruled.
Reads only.
"""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import registry

for cid in registry.REG.order:
    out = subprocess.run(["git", "log", "--reverse", "--format=%ad\t%h\t%s", "--date=short", f"-S**{cid} —"],
                         capture_output=True, text=True, cwd=registry.ROOT).stdout.splitlines()
    print(cid + "\t" + (out[0][:160].rstrip() if out else "?\t?\t?"), flush=True)
