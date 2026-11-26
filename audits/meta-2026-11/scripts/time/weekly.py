import subprocess, re
W = [(42,"2026-10-12","2026-10-19"),(43,"2026-10-19","2026-10-26"),(44,"2026-10-26","2026-11-02"),(45,"2026-11-02","2026-11-09"),(46,"2026-11-09","2026-11-16"),(47,"2026-11-16","2026-11-23"),(48,"2026-11-23","2026-11-26")]
for w,a,b in W:
    out = subprocess.run(["python3","-I","wallclock.py","events_dedup.json",a,b],capture_output=True,text=True).stdout
    parts = re.findall(r"^  ([a-e]) \S+\s+([\d.]+) h", out, re.M)
    un = re.search(r"in previous 60 min: ([\d.]+) h of ([\d.]+) h", out)
    pres = re.search(r"owner-present clock hours ([\d.]+)", out)
    print(f"W{w}", " ".join(f"{k}={v}" for k,v in parts), f"unatt={un.group(1)}/{un.group(2)}", f"present/day={pres.group(1)}")
