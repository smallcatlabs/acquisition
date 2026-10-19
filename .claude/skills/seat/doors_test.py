#!/usr/bin/env python3
"""doors_test.py — the seat's doors against a tool that misbehaves (the seat skill).

A fake acq-mcp that stalls, writes half a line, answers something that is not JSON, exits before answering,
or answers and never exits; a fake acq whose login prints no URL or half of one. Each runs under a
temporary world's bin/ with the deadlines cut short. Every case must end within its deadline, leave no
process behind, and leave one row in calls.csv saying what happened — and the one that answers must be
logged with the server's text and size alone, its host configuration kept beside the row.

  python3 -I .claude/skills/seat/doors_test.py        # before a seat, and after any change to a door
"""
import argparse, contextlib, csv, importlib.util, io, json, os, stat, sys, tempfile, textwrap, time

HERE = os.path.dirname(os.path.abspath(__file__))

FAKE_MCP = textwrap.dedent('''\
    #!/usr/bin/env python3
    import json, os, sys, time
    mode = os.environ["FAKE_MODE"]
    open(os.environ["FAKE_PID"], "w").write(str(os.getpid()))
    def answer(req):
        if req.get("method") == "initialize":
            return {"jsonrpc": "2.0", "id": req["id"], "result": {"instructions": "hi", "capabilities": {}}}
        if req.get("method") == "tools/list":
            return {"jsonrpc": "2.0", "id": req["id"], "result": {"tools": []}}
        return {"jsonrpc": "2.0", "id": req["id"], "result": {"content": [{"type": "text", "text": "ok"}]}}
    for line in sys.stdin:
        req = json.loads(line)
        if "id" not in req:
            continue
        if mode == "stall":
            time.sleep(30)
        elif mode == "partial":
            sys.stdout.write('{"jsonrpc":')
            sys.stdout.flush()
            time.sleep(30)
        elif mode == "malformed":
            print("not json")
            sys.stdout.flush()
        elif mode == "eof":
            sys.exit(0)
        else:
            print(json.dumps(answer(req)))
            sys.stdout.flush()
    if mode == "lingers":
        time.sleep(30)
    ''')

FAKE_ACQ = textwrap.dedent('''\
    #!/usr/bin/env python3
    import os, sys, time
    mode = os.environ["FAKE_MODE"]
    open(os.environ["FAKE_PID"], "w").write(str(os.getpid()))
    if mode == "silent":
        time.sleep(30)
    elif mode == "partial":
        sys.stdout.write('{"authorize_url": "http://127.0.0.1:1/authorize?x')
        sys.stdout.flush()
        time.sleep(30)
    ''')


def load(name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, name + ".py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def fake(path, text):
    with open(path, "w") as f:
        f.write(text)
    os.chmod(path, os.stat(path).st_mode | stat.S_IXUSR)


def rows(world):
    with open(os.path.join(world, "calls.csv"), newline="") as f:
        return [r for r in csv.reader(f) if r and not r[0].startswith("#")][1:]


def gone(pid_file, within=3.0):
    """The fake's process is gone within `within` seconds of the door returning."""
    pid = int(open(pid_file).read())
    until = time.monotonic() + within
    while time.monotonic() < until:
        try:
            os.kill(pid, 0)
        except ProcessLookupError:
            return True
        time.sleep(0.05)
    return False


def world():
    w = tempfile.mkdtemp(prefix="acq-doors-")
    os.makedirs(os.path.join(w, "bin"))
    os.makedirs(os.path.join(w, "work"))
    with open(os.path.join(w, "world.json"), "w") as f:
        json.dump({"daemon": True}, f)
    return w


def check(cond, what):
    if not cond:
        sys.exit(f"doors_test: FAIL — {what}")
    print(f"doors_test: ok — {what}")


def mcp_cases():
    w = world()
    fake(os.path.join(w, "bin", "acq-mcp"), FAKE_MCP)
    pid_file = os.path.join(w, "fake.pid")
    os.environ["FAKE_PID"] = pid_file
    door = load("seat-mcp")
    door.place(w, os.path.join(w, "bin"))
    door.EXIT_GRACE = 0.5
    with open(os.path.join(w, "work", "acq.json"), "w") as f:
        json.dump({"mcpServers": {"acquisition": {"command": os.path.join(w, "bin", "acq-mcp")}}}, f)
    cases = [
        ("stall", "did not answer within"),
        ("partial", "did not answer within"),
        ("malformed", "is not JSON"),
        ("eof", "closed its stdout"),
    ]
    for i, (mode, said) in enumerate(cases, 1):
        os.environ["FAKE_MODE"] = mode
        a = argparse.Namespace(t="T0", note="", host_config=None, timeout=0.5, tool="x", args="{}", draft=None)
        t0 = time.monotonic()
        with contextlib.redirect_stdout(io.StringIO()):
            door.call_cmd(a)
        took = time.monotonic() - t0
        row = rows(w)[-1]
        text = open(os.path.join(w, "calls", f"{i:03d}.json")).read()
        check(took < 3.0 and row[0] == str(i) and row[4] == "1" and said in text and "killed" in text and gone(pid_file),
              f"a server that {mode}s: one error row within the deadline saying so, the server gone ({took:.2f} s)")
    # answers, through a host configuration: the text and size are the server's, the configuration beside the row
    os.environ["FAKE_MODE"] = "ok"
    a = argparse.Namespace(t="T1", note="", host_config="acq.json", timeout=5, tool="x", args="{}", draft=None)
    with contextlib.redirect_stdout(io.StringIO()):
        door.call_cmd(a)
    n = len(cases) + 1
    row = rows(w)[-1]
    kept = os.path.join(w, "calls", f"{n:03d}.host-config.json")
    check(row[4] == "0" and row[5] == "2" and open(os.path.join(w, "calls", f"{n:03d}.json")).read() == "ok"
          and os.path.exists(kept) and json.load(open(kept))["mcpServers"]["acquisition"]["command"].endswith("acq-mcp")
          and "kept beside this row" in row[9] and gone(pid_file),
          "a server that answers through a host configuration: the text is its 2 bytes, the configuration kept beside the row")
    # answers and never exits: the row is an answer, the note says it was killed
    os.environ["FAKE_MODE"] = "lingers"
    a = argparse.Namespace(t="T1", note="", host_config=None, timeout=5, tool="x", args="{}", draft=None)
    t0 = time.monotonic()
    with contextlib.redirect_stdout(io.StringIO()):
        door.open_cmd(a)
    took = time.monotonic() - t0
    row = rows(w)[-1]
    check(took < 3.0 and row[4] == "0" and "did not exit" in row[9] and gone(pid_file),
          f"a server that answers and lingers: an answer row whose note says it was killed ({took:.2f} s)")


def login_cases():
    w = world()
    fake(os.path.join(w, "bin", "acq"), FAKE_ACQ)
    pid_file = os.path.join(w, "fake.pid")
    os.environ["FAKE_PID"] = pid_file
    door = load("seat")
    door.place(w, os.path.join(w, "bin"))
    door.LOGIN_DEADLINE = 0.5
    for i, (mode, said) in enumerate([("silent", "no authorize_url within"), ("partial", "no authorize_url within")], 1):
        os.environ["FAKE_MODE"] = mode
        t0 = time.monotonic()
        with contextlib.redirect_stdout(io.StringIO()):
            door.login("u", (), "T2")
        took = time.monotonic() - t0
        row = rows(w)[-1]
        text = open(os.path.join(w, "calls", f"{i:03d}.txt")).read()
        check(took < 3.0 and row[0] == str(i) and row[3] == "timeout" and said in text and gone(pid_file),
              f"a login whose acq is {mode}: one timeout row within the deadline saying so, acq gone ({took:.2f} s)")


if __name__ == "__main__":
    mcp_cases()
    login_cases()
    print("doors_test: every door ends within its deadline and leaves a row")
