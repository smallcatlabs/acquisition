#!/usr/bin/env python3
"""seat-mcp.py — an MCP seat's one door (the seat skill): one call to `acq-mcp` over stdio, logged.

usage (WORLD and BIN as seat.py takes them; the world is made by `seat.py WORLD BIN setup`):
  seat-mcp.py WORLD BIN open --t T0                         what a host shows before any call: the server's instructions and its tools
  seat-mcp.py WORLD BIN call --t T1 <tool> '<json arguments>' [--note '...']
  seat-mcp.py WORLD BIN call --t T1 <tool> - [--note '...'] the arguments read from stdin
  seat-mcp.py WORLD BIN report [--new]                      append stdin to WORLD/REPORT.md (--new starts it)
  seat-mcp.py WORLD BIN debrief [--new]                     the same, to WORLD/DEBRIEF.md

Every call is a row of WORLD/calls.csv and a file under WORLD/calls/. The world is set here and nowhere
else: the mock provider, no keyring, no daemon (ACQ_NO_SPAWN=1: a tool that wants one says so), the store
and log directory under WORLD. One server process per call; a request is whole, so nothing is held between.

bytes: `text_bytes` is the result's text block; `result_bytes` is the whole JSON-RPC line, which carries
the document a second time as structured content. A result is printed whole up to SHOWN bytes of text;
past that its head is printed and the row says `head`. `open` is printed whole: it is what a host shows.
"""
import argparse, csv, json, os, subprocess, sys, time

# Set by `place`, from the command line.
WORLD = EXE = CSV = OUT = None
FIELDS = ["n", "task", "tool", "args", "is_error", "text_bytes", "result_bytes", "wall_ms", "shown", "note"]
SHOWN = 60_000
HEAD = 3_000
DROPPED = ("ACQ_GGG", "ACQ_TRIPWIRE", "ACQ_MAX_SENDS", "ACQ_IDLE_SHUTDOWN", "ACQ_ACCOUNT", "ACQ_JOURNAL")


def place(world, bin_):
    global WORLD, EXE, CSV, OUT
    WORLD = os.path.abspath(world)
    if not os.path.isfile(os.path.join(WORLD, "world.json")):
        sys.exit(f"seat-mcp: {WORLD} has no world.json — `seat.py {world} {bin_} setup` makes the world")
    EXE = os.path.join(os.path.abspath(bin_), "acq-mcp")
    if not os.access(EXE, os.X_OK):
        sys.exit(f"seat-mcp: no acq-mcp in {os.path.abspath(bin_)}")
    CSV = os.path.join(WORLD, "calls.csv")
    OUT = os.path.join(WORLD, "calls")


def server():
    env = {k: v for k, v in os.environ.items() if k not in DROPPED}
    env.update(ACQ_PROVIDER="mock", ACQ_NO_KEYRING="1", ACQ_NO_SPAWN="1",
               ACQ_STORE_DIR=os.path.join(WORLD, "store"), ACQ_LOG_DIR=os.path.join(WORLD, "logs"))
    p = subprocess.Popen([EXE], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)

    def rpc(o):
        p.stdin.write(json.dumps(o) + "\n")
        p.stdin.flush()
        if "id" not in o:
            return None, 0.0
        t0 = time.perf_counter()
        line = p.stdout.readline()
        return line, (time.perf_counter() - t0) * 1000

    first, _ = rpc({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
        "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "seat-mcp.py", "version": "0"}}})
    rpc({"jsonrpc": "2.0", "method": "notifications/initialized"})
    return p, rpc, first


def close(p):
    p.stdin.close()
    p.wait(timeout=30)


def log(task, tool, args, is_error, text, result_bytes, wall, shown, note):
    os.makedirs(OUT, exist_ok=True)
    new = not os.path.exists(CSV)
    n = 1
    if not new:
        with open(CSV, newline="") as f:
            n = sum(1 for r in csv.reader(f) if r and not r[0].startswith("#"))  # the header row counts as 1, so this is the next n
    with open(os.path.join(OUT, f"{n:03d}.json"), "w") as f:
        f.write(text)
    with open(CSV, "a", newline="") as f:
        if new:
            f.write("# written by seat-mcp.py during the run, one row per call to acq-mcp in this world; row n's text is calls/NNN.json\n")
        w = csv.DictWriter(f, fieldnames=FIELDS, lineterminator="\n")
        if new:
            w.writeheader()
        w.writerow({"n": n, "task": task, "tool": tool, "args": args, "is_error": int(is_error),
                    "text_bytes": len(text.encode()), "result_bytes": result_bytes, "wall_ms": round(wall),
                    "shown": shown, "note": note})
    return n


def show(n, tool, is_error, text, result_bytes, wall, shown):
    print(f"[call {n}] {tool}\n[{'error' if is_error else 'answered'} · {len(text.encode())} bytes of text · "
          f"{result_bytes} bytes on the wire · {wall:.0f} ms]")
    if shown == "whole":
        print(text)
    else:
        print(f"--- over {SHOWN} bytes; the door prints the first {HEAD} characters")
        print(text[:HEAD])


def open_cmd(a):
    p, rpc, first = server()
    line, wall = rpc({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})
    close(p)
    init = json.loads(first)["result"]
    tools = json.loads(line)["result"]["tools"]
    text = json.dumps({"instructions": init.get("instructions", ""), "tools": tools}, ensure_ascii=False, indent=1)
    size = len(first.encode()) + len(line.encode())
    n = log(a.t, "initialize + tools/list", "", False, text, size, wall, "whole", a.note)
    show(n, "initialize + tools/list", False, text, size, wall, "whole")


def call_cmd(a):
    raw = sys.stdin.read() if a.args == "-" else a.args
    try:
        args = json.loads(raw)
    except ValueError as e:
        sys.exit(f"seat-mcp: the arguments are not JSON ({e}); nothing was sent and nothing logged")
    sent = json.dumps(args, separators=(",", ":"))
    p, rpc, _ = server()
    line, wall = rpc({"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": a.tool, "arguments": args}})
    close(p)
    if not line:
        text, is_error = "the server answered nothing and exited: " + p.stderr.read()[-2000:], True
    else:
        resp = json.loads(line)
        if "error" in resp:
            text, is_error = json.dumps(resp["error"]), True
        else:
            r = resp["result"]
            text = "".join(c.get("text", "") for c in r.get("content", []))
            is_error = bool(r.get("isError"))
    shown = "whole" if len(text.encode()) <= SHOWN else "head"
    n = log(a.t, a.tool, sent, is_error, text, len((line or "").encode()), wall, shown, a.note)
    show(n, a.tool, is_error, text, len((line or "").encode()), wall, shown)


def append(name, new, draft):
    # A part arrives on stdin, or from WORLD/drafts/DRAFT, which the runner's file tool may write:
    # a harness that parses a heredoc's body as shell refuses some prose (`?` before an open quote).
    path = os.path.join(WORLD, name)
    if draft:
        if "/" in draft or draft.startswith("."):
            sys.exit(f"seat-mcp: --from takes a bare file name, read from {WORLD}/drafts/")
        text = open(os.path.join(WORLD, "drafts", draft)).read()
    else:
        text = sys.stdin.read()
    with open(path, "w" if new else "a") as f:
        f.write(text if text.endswith("\n") else text + "\n")
    print(f"seat-mcp: {name} is {os.path.getsize(path)} bytes")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("world", help="the world's root, made by seat.py setup")
    ap.add_argument("bin", help="the directory holding acq-mcp")
    sub = ap.add_subparsers(dest="cmd", required=True)
    o = sub.add_parser("open")
    o.add_argument("--t", required=True)
    o.add_argument("--note", default="")
    c = sub.add_parser("call")
    c.add_argument("--t", required=True)
    c.add_argument("--note", default="")
    c.add_argument("tool")
    c.add_argument("args")
    r = sub.add_parser("report")
    r.add_argument("--new", action="store_true")
    r.add_argument("--from", dest="draft", help="a file under WORLD/drafts/, by bare name, in place of stdin")
    d = sub.add_parser("debrief")
    d.add_argument("--new", action="store_true")
    d.add_argument("--from", dest="draft", help="a file under WORLD/drafts/, by bare name, in place of stdin")
    a = ap.parse_args()
    place(a.world, a.bin)
    if a.cmd == "open":
        open_cmd(a)
    elif a.cmd == "call":
        call_cmd(a)
    else:
        append(a.cmd.upper() + ".md", a.new, a.draft)


if __name__ == "__main__":
    main()
