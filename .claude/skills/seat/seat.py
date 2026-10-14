#!/usr/bin/env python3
"""seat.py — a CLI seat's one door (the seat skill): one `acq` command, logged.

usage, from the repository root (AT is the seat's record directory, where its BRIEF.md is):
  seat.py AT setup [--facts FILE.db]      build the world; with --facts, over a copy of that file of the owner's store
  seat.py AT login <user>                 a scripted mock login (the mock-session skill's recipe; a person uses a browser)
  seat.py AT run --t T3 [--note '...'] [--timeout S] [--stdin FILE|-] [--out NAME] -- <acq arguments>
  seat.py AT report [--new]               append stdin to AT/REPORT.md (--new starts it): a runner's harness may refuse it file tools
  seat.py AT teardown                     stop the world's daemon

The world is set here and nowhere else: the mock provider, no keyring, a store and a log directory
under runs/seat-<the name of AT's parent>/ (gitignored). Nothing here can reach GGG or the owner's
store: every command carries ACQ_PROVIDER=mock and ACQ_STORE_DIR, and `setup --facts` reads the owner's
files once, through sqlite's online backup. Every `run` is a row of AT/calls.csv and a file under the
world's calls/. Output is printed whole up to SHOWN bytes; past that its head is printed and the row
says `head`. `--out NAME` also writes stdout to the world's work/NAME, as a shell's `>` would: the one
way a verb's output becomes a file another verb reads (`refresh --apply=FILE`).
"""
import argparse, csv, json, os, re, sqlite3, subprocess, sys, time, urllib.parse, urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
EXE = os.path.join(ROOT, "target", "release", "acq")
COPY = os.path.join(ROOT, "target", "release", "examples", "copy-as-store")
OWNER = os.path.expanduser("~/Library/Application Support/com.smallcatlabs.acq/store/ggg")
# Set by `place`, from the record directory the command line names.
HERE = RUN = STORE = CSV = OUT = WORLD = None


def place(at):
    global HERE, RUN, STORE, CSV, OUT, WORLD
    HERE = os.path.abspath(at)
    if not os.path.isdir(HERE):
        sys.exit(f"seat: {at} is not a directory — the first argument is the seat's record directory")
    WORLD = "runs/seat-" + os.path.basename(os.path.dirname(HERE))
    RUN = os.path.join(ROOT, WORLD)
    STORE = os.path.join(RUN, "store")
    CSV = os.path.join(HERE, "calls.csv")
    OUT = os.path.join(RUN, "calls")


FIELDS = ["n", "task", "args", "exit", "stdout_bytes", "stderr_bytes", "wall_ms", "shown", "note"]
SHOWN = 60_000
HEAD = 3_000
DROPPED = ("ACQ_GGG", "ACQ_TRIPWIRE", "ACQ_MAX_SENDS", "ACQ_ACCOUNT", "ACQ_JOURNAL", "ACQ_NO_SPAWN")


def env():
    e = {k: v for k, v in os.environ.items() if k not in DROPPED}
    e.update(ACQ_PROVIDER="mock", ACQ_NO_KEYRING="1", ACQ_IDLE_SHUTDOWN="1800",
             ACQ_STORE_DIR=STORE, ACQ_LOG_DIR=os.path.join(RUN, "logs"))
    return e


def backup(src, dst):
    # A WAL-mode file with no `-shm` cannot be opened `mode=ro` (sqlite would have to create one);
    # with an empty or absent WAL the file is whole and `immutable=1` reads it without touching anything.
    wal = src + "-wal"
    if os.path.exists(wal) and os.path.getsize(wal) > 0:
        sys.exit(f"seat: {wal} holds pages — a daemon is running or left one; stop it and start it once more before the copy")
    a = sqlite3.connect(f"file:{urllib.parse.quote(src)}?immutable=1", uri=True)
    b = sqlite3.connect(dst)
    a.backup(b)
    b.close()
    a.close()


def setup(owner_facts):
    if os.path.exists(STORE):
        sys.exit(f"seat: {STORE} exists — a world is made once; move {WORLD} aside (a rerun keeps the first run's) to start over")
    os.makedirs(STORE)
    if not owner_facts:
        print(f"seat: an empty world at {STORE}")
        return
    facts = os.path.join(RUN, "copy.db")
    backup(os.path.join(OWNER, owner_facts), facts)
    placed = subprocess.run([COPY, facts, STORE], check=True, capture_output=True, text=True).stdout.strip()
    print(f"seat: facts copy at {placed}")
    for f in os.listdir(OWNER):
        if f.endswith(".annotations.db"):
            uuid = f.split(".")[0]
            index = open(os.path.join(STORE, "mock", "accounts.json")).read()
            if uuid in index:
                backup(os.path.join(OWNER, f), os.path.join(STORE, "mock", f))
                print(f"seat: intent copy at {os.path.join(STORE, 'mock', f)}")


def login(user):
    p = subprocess.Popen([EXE, "auth", "--no-browser", "--json"], env=env(), stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    url = None
    for line in p.stdout:
        m = re.search(r'"authorize_url"\s*:\s*"([^"]+)"', line)
        if m:
            url = m.group(1)
            break
    if not url:
        sys.exit(f"seat: no authorize_url from `acq auth --no-browser --json`: {p.stderr.read()}")
    urllib.request.urlopen(url.replace("/authorize?", "/approve?") + f"&user={user}").read()
    rest = p.stdout.read()
    code = p.wait(timeout=60)
    print(f"seat: `acq auth --no-browser --json` for {user} exited {code}\n{rest.strip()}")


def log(row, text):
    os.makedirs(OUT, exist_ok=True)
    new = not os.path.exists(CSV)
    n = 1
    if not new:
        with open(CSV, newline="") as f:
            n = sum(1 for r in csv.reader(f) if r and not r[0].startswith("#"))
    with open(os.path.join(OUT, f"{n:03d}.txt"), "w") as f:
        f.write(text)
    with open(CSV, "a", newline="") as f:
        if new:
            f.write(f"# written by seat.py during the run of BRIEF.md, one row per `acq` command in the seat's world; row n's output is {WORLD}/calls/NNN.txt (gitignored)\n")
            csv.writer(f).writerow(FIELDS)
        csv.writer(f).writerow([n] + row)
    return n


def run(a):
    stdin = None
    if a.stdin == "-":
        stdin = sys.stdin.read()
    elif a.stdin:
        stdin = open(a.stdin).read()
    t0 = time.perf_counter()
    try:
        p = subprocess.run([EXE] + a.args, env=env(), input=stdin, capture_output=True, text=True, timeout=a.timeout,
                           stdin=None if stdin is not None else subprocess.DEVNULL)
        code, out, err = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        dec = lambda b: b.decode(errors="replace") if isinstance(b, bytes) else (b or "")
        code, out, err = "timeout", dec(e.stdout), dec(e.stderr)
    if a.out:
        if "/" in a.out or a.out.startswith("."):
            sys.exit(f"seat: --out takes a bare file name; it is written under {WORLD}/work/")
        os.makedirs(os.path.join(RUN, "work"), exist_ok=True)
        with open(os.path.join(RUN, "work", a.out), "w") as f:
            f.write(out)
    wall = round((time.perf_counter() - t0) * 1000)
    ob, eb = len(out.encode()), len(err.encode())
    whole = ob + eb <= SHOWN
    n = log([a.t, json.dumps(a.args), code, ob, eb, wall, "whole" if whole else "head", a.note or ""],
            f"$ acq {' '.join(a.args)}\n--- stdout\n{out}\n--- stderr\n{err}")
    print(f"[call {n}] acq {' '.join(a.args)}\n[exit {code} · stdout {ob} bytes · stderr {eb} bytes · {wall} ms]")
    if whole:
        if out:
            print("--- stdout\n" + out.rstrip("\n"))
        if err:
            print("--- stderr\n" + err.rstrip("\n"))
    else:
        print(f"--- over {SHOWN} bytes; the door prints the first {HEAD} characters of stdout and of stderr")
        print("--- stdout\n" + out[:HEAD])
        if err:
            print("--- stderr\n" + err[:HEAD])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("at", help="the seat's record directory: BRIEF.md, calls.csv, REPORT.md")
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("setup").add_argument("--facts", help="a facts file of the owner's store to copy, by name")
    sub.add_parser("teardown")
    sub.add_parser("login").add_argument("user")
    sub.add_parser("report").add_argument("--new", action="store_true")
    r = sub.add_parser("run")
    r.add_argument("--t", required=True)
    r.add_argument("--note")
    r.add_argument("--timeout", type=float, default=300)
    r.add_argument("--stdin")
    r.add_argument("--out")
    r.add_argument("args", nargs=argparse.REMAINDER)
    a = ap.parse_args()
    place(a.at)
    if a.cmd == "setup":
        setup(a.facts)
    elif a.cmd == "login":
        login(a.user)
    elif a.cmd == "report":
        path = os.path.join(HERE, "REPORT.md")
        text = sys.stdin.read()
        with open(path, "w" if a.new else "a") as f:
            f.write(text if text.endswith("\n") else text + "\n")
        print(f"seat: REPORT.md is {os.path.getsize(path)} bytes")
    elif a.cmd == "teardown":
        print(subprocess.run([EXE, "daemon", "stop"], env=env(), capture_output=True, text=True).stdout.strip())
    else:
        if a.args and a.args[0] == "--":
            a.args = a.args[1:]
        if not a.args:
            sys.exit("seat: no acq arguments")
        run(a)


if __name__ == "__main__":
    main()
