#!/usr/bin/env python3
"""seat.py — a CLI seat's one door (the seat skill): one `acq` command, logged; and any seat's world.

usage (WORLD is the world's root, BIN the directory holding the binaries it runs):
  seat.py WORLD BIN setup [--facts FILE.db | --store DIR] [--daemon] [--archive FILE]
                                          make the world; --facts copies that file of the owner's store, --store an
                                          earlier world's pristine store; --daemon lets this world's commands start one;
                                          --archive unpacks a release archive's binaries into WORLD/bin (BIN is then
                                          WORLD/bin), as a person installing from it would, and records its sha256
  seat.py WORLD BIN login [--t T2] [--env ACQ_IDLE_SHUTDOWN=S] <user>
                                          a scripted mock login (the mock-session skill's recipe; a person uses a browser),
                                          logged as a call of its own with its knob, exit and output
  seat.py WORLD BIN run --t T3 [--note '...'] [--timeout S] [--stdin FILE|-] [--out NAME] [--env ACQ_IDLE_SHUTDOWN=S] -- <acq arguments>
  seat.py WORLD BIN report [--new]        append stdin to WORLD/REPORT.md (--new starts it): a runner's harness may refuse it file tools
  seat.py WORLD BIN debrief [--new]       the same, to WORLD/DEBRIEF.md
  seat.py WORLD BIN teardown              stop the world's daemon, if it has one, and remove the world's store

The world is set here and nowhere else: the mock provider, no keyring, a store and a log directory under
WORLD, and no daemon unless setup said --daemon (ACQ_NO_SPAWN=1 on every command). Nothing here can reach
GGG or write the owner's store: `setup --facts` reads the owner's files once, through sqlite's online backup,
and setup records the world in WORLD/world.json with the sha256 of every file in BIN. Every `run` is a row
of WORLD/calls.csv and a file under WORLD/calls/. Output is printed whole up to SHOWN bytes; past that its
head is printed and the row says `head`. `--out NAME` also writes stdout to WORLD/work/NAME, as a shell's
`>` would: the one way a verb's output becomes a file another verb reads (`refresh --apply=FILE`). `--env` sets
the one knob a README line puts before a command, the daemon's idle time, on that command alone.
The MCP door, seat-mcp.py, takes the same WORLD and BIN and runs in a world this script made.
"""
import argparse, csv, datetime, hashlib, json, os, re, shutil, sqlite3, subprocess, sys, time, urllib.parse, urllib.request

OWNER = os.path.expanduser("~/Library/Application Support/com.smallcatlabs.acq/store/ggg")
# Set by `place`, from the command line.
WORLD = BIN = STORE = CSV = OUT = None


def place(world, bin_, making_bin=False):
    global WORLD, BIN, STORE, CSV, OUT
    WORLD, BIN = os.path.abspath(world), os.path.abspath(bin_)
    if making_bin:
        if BIN != os.path.join(WORLD, "bin"):
            sys.exit(f"seat: setup --archive unpacks into {WORLD}/bin, so BIN is that directory")
        os.makedirs(BIN, exist_ok=True)
    for d, what in ((WORLD, "the world's root"), (BIN, "the directory of the binaries")):
        if not os.path.isdir(d):
            sys.exit(f"seat: {d} is not a directory — it is {what}")
    STORE = os.path.join(WORLD, "store")
    CSV = os.path.join(WORLD, "calls.csv")
    OUT = os.path.join(WORLD, "calls")


def exe(name):
    path = os.path.join(BIN, name)
    if not os.access(path, os.X_OK):
        sys.exit(f"seat: no {name} in {BIN}")
    return path


def daemon():
    try:
        return json.load(open(os.path.join(WORLD, "world.json")))["daemon"]
    except (OSError, ValueError, KeyError):
        sys.exit(f"seat: {WORLD} has no world.json — `setup` makes the world")


FIELDS = ["n", "task", "args", "exit", "stdout_bytes", "stderr_bytes", "wall_ms", "shown", "note"]
SHOWN = 60_000
HEAD = 3_000
DROPPED = ("ACQ_GGG", "ACQ_TRIPWIRE", "ACQ_MAX_SENDS", "ACQ_ACCOUNT", "ACQ_JOURNAL", "ACQ_NO_SPAWN")


ALLOWED_ENV = ("ACQ_IDLE_SHUTDOWN",)


def env(extra=()):
    e = {k: v for k, v in os.environ.items() if k not in DROPPED}
    e.update(ACQ_PROVIDER="mock", ACQ_NO_KEYRING="1", ACQ_IDLE_SHUTDOWN="1800",
             ACQ_STORE_DIR=STORE, ACQ_LOG_DIR=os.path.join(WORLD, "logs"))
    for item in extra:
        name, _, value = item.partition("=")
        if name not in ALLOWED_ENV or not value:
            sys.exit(f"seat: --env takes {', '.join(f'{n}=value' for n in ALLOWED_ENV)} and nothing else")
        e[name] = value
    if not daemon():
        e["ACQ_NO_SPAWN"] = "1"
    return e


def unpack(archive):
    """A release archive's one top-level directory, unpacked into BIN as a person would unpack it."""
    import tarfile, tempfile, zipfile
    with tempfile.TemporaryDirectory() as tmp:
        if archive.endswith(".zip"):
            with zipfile.ZipFile(archive) as z:
                z.extractall(tmp)
        else:
            with tarfile.open(archive) as t:
                t.extractall(tmp, filter="data")
        # a tar made on macOS carries AppleDouble `._` entries beside each file, which tar itself
        # merges on macOS and never lists; they are not the archive's contents
        tops = [t for t in os.listdir(tmp) if not t.startswith("._")]
        if len(tops) != 1 or not os.path.isdir(os.path.join(tmp, tops[0])):
            sys.exit(f"seat: {archive} does not hold one top-level directory")
        for f in sorted(os.listdir(os.path.join(tmp, tops[0]))):
            if not f.startswith("._"):
                shutil.copy2(os.path.join(tmp, tops[0], f), os.path.join(BIN, f))
        return tops[0]


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


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def setup(a):
    if os.path.exists(STORE):
        sys.exit(f"seat: {STORE} exists — a world is made once; move {WORLD} aside (a rerun keeps the first run's) to start over")
    if a.facts and a.store:
        sys.exit("seat: --facts or --store, not both")
    os.makedirs(STORE)
    placed = []
    unpacked = None
    if a.archive:
        unpacked = unpack(os.path.abspath(a.archive))
    if a.facts:
        facts = os.path.join(WORLD, "copy.db")
        backup(os.path.join(OWNER, a.facts), facts)
        subprocess.run([exe("copy-as-store"), facts, STORE], check=True, capture_output=True, text=True)  # moves the copy into STORE
        index = open(os.path.join(STORE, "mock", "accounts.json")).read()
        for f in sorted(os.listdir(OWNER)):
            if f.endswith(".annotations.db") and f.split(".")[0] in index:
                backup(os.path.join(OWNER, f), os.path.join(STORE, "mock", f))
    elif a.store:
        src = os.path.abspath(a.store)
        for d, _, files in os.walk(src):
            os.makedirs(os.path.join(STORE, os.path.relpath(d, src)), exist_ok=True)
            for f in sorted(files):
                s, t = os.path.join(d, f), os.path.join(STORE, os.path.relpath(d, src), f)
                if f.endswith(".db"):
                    backup(s, t)
                elif f.endswith(".json"):
                    shutil.copyfile(s, t)
    for d, _, files in os.walk(STORE):
        placed += [os.path.relpath(os.path.join(d, f), WORLD) for f in sorted(files)]
    record = {"made": datetime.datetime.now().astimezone().isoformat(timespec="seconds"),
              "from": f"owner's {a.facts}" if a.facts else a.store and os.path.abspath(a.store) or "empty",
              "daemon": a.daemon, "store": placed,
              "archive": a.archive and {"path": os.path.abspath(a.archive), "sha256": sha256(os.path.abspath(a.archive)), "top": unpacked},
              "bin": {f: sha256(os.path.join(BIN, f)) for f in sorted(os.listdir(BIN)) if os.path.isfile(os.path.join(BIN, f))}}
    with open(os.path.join(WORLD, "world.json"), "w") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    print(f"seat: the world at {WORLD}")
    print(json.dumps(record, indent=2))


LOGIN_DEADLINE = 60.0


def login(user, extra=(), task="login"):
    """The scripted login, logged as a call of its own: the command as a person would type it, the knob
    it carried, its exit and output, so a report can cite it. One deadline covers the wait for the
    authorize URL and, after the approval, the login's finish; a reader thread feeds the lines, so half a
    line or no line ends the same way — the login killed, and the row saying why."""
    if not daemon():
        sys.exit("seat: this world has no daemon (setup without --daemon), so it has no login")
    import queue, threading
    words = list(extra) + ["acq", "auth", "--no-browser", "--json"]
    t0 = time.perf_counter()
    p = subprocess.Popen([exe("acq"), "auth", "--no-browser", "--json"], env=env(extra), stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    lines = queue.Queue()

    def pump():
        for line in p.stdout:
            lines.put(line)
        lines.put(None)

    threading.Thread(target=pump, daemon=True).start()
    deadline = time.monotonic() + LOGIN_DEADLINE
    url, out, why, ended = None, "", "", False
    while url is None and not ended:
        try:
            line = lines.get(timeout=max(0.0, deadline - time.monotonic()))
        except queue.Empty:
            why = f"no authorize_url within {LOGIN_DEADLINE:g} s; killed"
            break
        if line is None:
            ended, why = True, "acq auth ended without printing an authorize_url"
            break
        out += line
        m = re.search(r'"authorize_url"\s*:\s*"([^"]+)"', line)
        if m:
            url = m.group(1)
    if url:
        try:
            urllib.request.urlopen(url.replace("/authorize?", "/approve?") + f"&user={user}", timeout=30).read()
            p.wait(timeout=LOGIN_DEADLINE)
        except (OSError, subprocess.TimeoutExpired) as e:
            why = f"the login did not finish ({e}); killed"
    if p.poll() is None:
        p.kill()
        p.wait()
    while True:  # the rest of stdout, which the pump ends with None once the process has exited
        try:
            line = lines.get(timeout=5)
        except queue.Empty:
            break
        if line is None:
            break
        out += line
    err = p.stderr.read() + (f"\nseat: {why}" if why else "")
    code = "timeout" if why else p.returncode
    wall = round((time.perf_counter() - t0) * 1000)
    ob, eb = len(out.encode()), len(err.encode())
    n = log([task, json.dumps(list(extra) + ["auth", "--no-browser", "--json"]), code, ob, eb, wall, "whole", f"login {user}: the door approved the mock's page"],
            f"$ {' '.join(words)}   # login {user}: the door approves at the mock\n--- stdout\n{out}\n--- stderr\n{err}")
    print(f"[call {n}] {' '.join(words)}   # login {user}\n[exit {code} · stdout {ob} bytes · stderr {eb} bytes · {wall} ms]")
    if out.strip():
        print("--- stdout\n" + out.rstrip("\n"))
    if err.strip():
        print("--- stderr\n" + err.rstrip("\n"))


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
            f.write("# written by seat.py during the run, one row per `acq` command in this world; row n's output is calls/NNN.txt\n")
            csv.writer(f, lineterminator="\n").writerow(FIELDS)
        csv.writer(f, lineterminator="\n").writerow([n] + row)
    return n


def run(a):
    stdin = None
    if a.stdin == "-":
        stdin = sys.stdin.read()
    elif a.stdin:
        stdin = open(a.stdin).read()
    # What the command read, as it read it: its stdin, and each work file an argument names,
    # so a call's file holds its input even after the file is rewritten.
    inputs = "" if stdin is None else f"--- stdin\n{stdin}\n"
    for path in sorted({m for arg in a.args for m in re.findall(re.escape(os.path.join(WORLD, "work") + os.sep) + r"[^\s,=]+", arg)}):
        try:
            inputs += f"--- {path}\n{open(path).read()}\n"
        except OSError as e:
            inputs += f"--- {path}\n(unread: {e.strerror})\n"
    t0 = time.perf_counter()
    try:
        p = subprocess.run([exe("acq")] + a.args, env=env(a.env), input=stdin, capture_output=True, text=True, timeout=a.timeout,
                           stdin=None if stdin is not None else subprocess.DEVNULL)
        code, out, err = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        dec = lambda b: b.decode(errors="replace") if isinstance(b, bytes) else (b or "")
        code, out, err = "timeout", dec(e.stdout), dec(e.stderr)
    if a.out:
        if "/" in a.out or a.out.startswith("."):
            sys.exit(f"seat: --out takes a bare file name; it is written under {WORLD}/work/")
        os.makedirs(os.path.join(WORLD, "work"), exist_ok=True)
        with open(os.path.join(WORLD, "work", a.out), "w") as f:
            f.write(out)
    wall = round((time.perf_counter() - t0) * 1000)
    ob, eb = len(out.encode()), len(err.encode())
    whole = ob + eb <= SHOWN
    said = " ".join(a.env + ["acq"] + a.args)
    n = log([a.t, json.dumps(a.env + a.args), code, ob, eb, wall, "whole" if whole else "head", a.note or ""],
            f"$ {said}\n{inputs}--- stdout\n{out}\n--- stderr\n{err}")
    print(f"[call {n}] {said}\n[exit {code} · stdout {ob} bytes · stderr {eb} bytes · {wall} ms]")
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


def append(name, new, draft):
    # A part arrives on stdin, or from WORLD/drafts/DRAFT, which the runner's file tool may write:
    # a harness that parses a heredoc's body as shell refuses some prose (`?` before an open quote).
    path = os.path.join(WORLD, name)
    if draft:
        if "/" in draft or draft.startswith("."):
            sys.exit(f"seat: --from takes a bare file name, read from {WORLD}/drafts/")
        text = open(os.path.join(WORLD, "drafts", draft)).read()
    else:
        text = sys.stdin.read()
    with open(path, "w" if new else "a") as f:
        f.write(text if text.endswith("\n") else text + "\n")
    # what the door did, not a size alone: a runner could not tell an append
    # from nothing (the re-sit, cli-sonnet fault 6)
    print(f"seat: {'started' if new else 'appended to'} {name}, now {os.path.getsize(path)} bytes")


def teardown():
    if daemon():
        print(subprocess.run([exe("acq"), "daemon", "stop"], env=env(), capture_output=True, text=True).stdout.strip())
    if os.path.exists(STORE):
        shutil.rmtree(STORE)
        print(f"seat: removed {STORE}; the calls, the report and the debrief stay")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("world", help="the world's root: its store, calls.csv, calls/, work/, REPORT.md, DEBRIEF.md")
    ap.add_argument("bin", help="the directory of the binaries the world runs: acq, and copy-as-store for setup --facts")
    sub = ap.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("setup")
    s.add_argument("--facts", help="a facts file of the owner's store to copy, by name")
    s.add_argument("--store", help="an earlier world's store directory, never run in, to copy")
    s.add_argument("--daemon", action="store_true", help="let this world's commands start a mock daemon (a seat with a login)")
    s.add_argument("--archive", help="a release archive (.tar.gz or .zip) whose binaries are unpacked into WORLD/bin")
    sub.add_parser("teardown")
    l = sub.add_parser("login")
    l.add_argument("user")
    l.add_argument("--env", action="append", default=[], help="as run's: the daemon this login starts reads ACQ_IDLE_SHUTDOWN")
    l.add_argument("--t", default="login", help="the task the login is for; the login is a numbered call")
    r = sub.add_parser("report")
    r.add_argument("--new", action="store_true")
    r.add_argument("--from", dest="draft", help="a file under WORLD/drafts/, by bare name, in place of stdin")
    d = sub.add_parser("debrief")
    d.add_argument("--new", action="store_true")
    d.add_argument("--from", dest="draft", help="a file under WORLD/drafts/, by bare name, in place of stdin")
    r = sub.add_parser("run")
    r.add_argument("--t", required=True)
    r.add_argument("--note")
    r.add_argument("--timeout", type=float, default=300)
    r.add_argument("--stdin")
    r.add_argument("--out")
    r.add_argument("--env", action="append", default=[], help="NAME=value for the one knob a README line sets, ACQ_IDLE_SHUTDOWN")
    r.add_argument("args", nargs=argparse.REMAINDER)
    a = ap.parse_args()
    place(a.world, a.bin, making_bin=a.cmd == "setup" and bool(a.archive))
    if a.cmd == "setup":
        setup(a)
    elif a.cmd == "login":
        login(a.user, a.env, a.t)
    elif a.cmd in ("report", "debrief"):
        append(a.cmd.upper() + ".md", a.new, a.draft)
    elif a.cmd == "teardown":
        teardown()
    else:
        if a.args and a.args[0] == "--":
            a.args = a.args[1:]
        if not a.args:
            sys.exit("seat: no acq arguments")
        run(a)


if __name__ == "__main__":
    main()
