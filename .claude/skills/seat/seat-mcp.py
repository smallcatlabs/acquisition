#!/usr/bin/env python3
"""seat-mcp.py — an MCP seat's one door (the seat skill): one call to `acq-mcp` over stdio, logged.

usage (WORLD and BIN as seat.py takes them; the world is made by `seat.py WORLD BIN setup`):
  seat-mcp.py WORLD BIN open --t T0 [--host-config NAME]    what a host shows before any call: the server's instructions and its tools
  seat-mcp.py WORLD BIN call --t T1 <tool> '<json arguments>' [--note '...']
  seat-mcp.py WORLD BIN call --t T1 <tool> - [--note '...'] the arguments read from stdin
  seat-mcp.py WORLD BIN call --t T1 <tool> --from NAME      the arguments read from WORLD/drafts/NAME
  --host-config NAME, on open or call: the door starts the server as a host would from WORLD/work/NAME, the
                                                            configuration the README shows (its one server's command, args and env)
  seat-mcp.py WORLD BIN report [--new]                      append stdin to WORLD/REPORT.md (--new starts it)
  seat-mcp.py WORLD BIN debrief [--new]                     the same, to WORLD/DEBRIEF.md

Every call is a row of WORLD/calls.csv and a file under WORLD/calls/. The world is set here and nowhere
else: the mock provider, no keyring, no daemon (ACQ_NO_SPAWN=1: a tool that wants one says so), the store
and log directory under WORLD. One server process per call; a request is whole, so nothing is held between.
One deadline (--timeout, 300 s) covers a call whole; a server that does not answer within it, closes its
stdout, answers something that is not JSON or loses its stdin is killed and the call is logged as an error
saying which; one that answers and does not exit when its stdin closes is killed after 30 s and the row's
note says so. With --host-config the configuration as the call read it is kept beside the call's file, as
calls/NNN.host-config.json; the call's text and sizes are the server's alone.

bytes: `text_bytes` is the result's text block; `result_bytes` is the whole JSON-RPC line, which carries
the document a second time as structured content. A result is printed whole up to SHOWN bytes of text;
past that its head is printed and the row says `head`. `open` is printed whole: it is what a host shows.
"""
import argparse, csv, json, os, queue, shutil, subprocess, sys, threading, time

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


def host_config(name):
    """The command line and environment a host would start the server with, from WORLD/work/NAME as the
    README shows the configuration: one server under `mcpServers`, its `command` the acq-mcp in BIN."""
    if not name:
        return [EXE], {}
    if "/" in name or name.startswith("."):
        sys.exit("seat-mcp: --host-config takes a bare file name under WORLD/work/")
    try:
        doc = json.load(open(os.path.join(WORLD, "work", name)))
        servers = doc["mcpServers"]
        (label, spec), = servers.items()
        command = spec["command"]
    except (OSError, ValueError, KeyError, TypeError) as e:
        sys.exit(f"seat-mcp: {name} is not a host configuration with one server under mcpServers: {e}")
    if os.path.realpath(command) != os.path.realpath(EXE):
        sys.exit(f"seat-mcp: {name} starts {command}; the door runs only {EXE}")
    args = spec.get("args") or []
    env = spec.get("env") or {}
    if not isinstance(args, list) or not isinstance(env, dict):
        sys.exit(f"seat-mcp: {name}: args is a list and env an object")
    return [EXE] + [str(x) for x in args], {str(k): str(v) for k, v in env.items()}


class Transport(Exception):
    """The server did not hold up its end: no answer within the deadline, its stdout closed, a line that is
    not JSON, or its stdin gone. Carries the process and its stderr so the caller can end it and say why."""

    def __init__(self, why, p, errs):
        super().__init__(why)
        self.p, self.errs = p, errs


EXIT_GRACE = 30.0


def server(config=None, timeout=300.0):
    """Start the server as a host would and bring it to the initialized state. One deadline covers the whole
    exchange, a reader thread feeds its lines, and every way it can fail is one `Transport`."""
    argv, from_config = host_config(config)
    env = {k: v for k, v in os.environ.items() if k not in DROPPED}
    env.update(from_config)
    env.update(ACQ_PROVIDER="mock", ACQ_NO_KEYRING="1", ACQ_NO_SPAWN="1",
               ACQ_STORE_DIR=os.path.join(WORLD, "store"), ACQ_LOG_DIR=os.path.join(WORLD, "logs"))
    p = subprocess.Popen(argv, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, env=env)
    lines, errs = queue.Queue(), []

    def pump(stream, sink):
        for line in stream:
            sink(line)
        sink(None)

    threading.Thread(target=pump, args=(p.stdout, lines.put), daemon=True).start()
    threading.Thread(target=pump, args=(p.stderr, lambda l: errs.append(l) if l is not None else None), daemon=True).start()
    deadline = time.monotonic() + timeout

    def rpc(o):
        try:
            p.stdin.write(json.dumps(o) + "\n")
            p.stdin.flush()
        except (BrokenPipeError, OSError) as e:
            raise Transport(f"the server's stdin is gone ({e})", p, errs)
        if "id" not in o:
            return None, 0.0
        t0 = time.perf_counter()
        try:
            line = lines.get(timeout=max(0.0, deadline - time.monotonic()))
        except queue.Empty:
            raise Transport(f"the server did not answer within {timeout:g} s", p, errs)
        if line is None:
            raise Transport("the server closed its stdout before answering", p, errs)
        try:
            json.loads(line)
        except ValueError as e:
            raise Transport(f"the server's answer is not JSON ({e}): {line[:300]!r}", p, errs)
        return line, (time.perf_counter() - t0) * 1000

    first, _ = rpc({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
        "protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "seat-mcp.py", "version": "0"}}})
    rpc({"jsonrpc": "2.0", "method": "notifications/initialized"})
    return p, rpc, first, errs


def finish(p):
    """The server exits when its stdin closes; one that has not within EXIT_GRACE is killed, and the row says so."""
    try:
        p.stdin.close()
    except OSError:
        pass
    try:
        p.wait(timeout=EXIT_GRACE)
        return ""
    except subprocess.TimeoutExpired:
        p.kill()
        p.wait()
        return f"the server did not exit within {EXIT_GRACE:g} s of its stdin closing; killed"


def stderr_tail(errs):
    text = "".join(errs)
    return f"\n--- stderr\n{text[-2000:]}" if text.strip() else ""


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


def noted(a, *more):
    parts = [f"--host-config {a.host_config}, kept beside this row"] if a.host_config else []
    parts += [m for m in more if m]
    if a.note:
        parts.append(a.note)
    return "; ".join(parts)


def keep_config(a, n):
    """The host configuration as this call read it, beside the call's file: a rewritten file is still on
    record, and the call's text and size are the server's alone."""
    if a.host_config:
        shutil.copyfile(os.path.join(WORLD, "work", a.host_config), os.path.join(OUT, f"{n:03d}.host-config.json"))


def exchange(a, tool, sent, request):
    """One server, one request, logged whatever happens. Returns (first, line, wall, left), or None after
    logging a transport failure as an error row — the server killed, its stderr's tail in the text."""
    t0 = time.perf_counter()
    try:
        p, rpc, first, errs = server(a.host_config, a.timeout)
        line, wall = rpc(request)
    except Transport as e:
        e.p.kill()
        e.p.wait()
        text = f"{e}; killed" + stderr_tail(e.errs)
        n = log(a.t, tool, sent, True, text, 0, (time.perf_counter() - t0) * 1000, "whole", noted(a))
        keep_config(a, n)
        show(n, tool, True, text, 0, (time.perf_counter() - t0) * 1000, "whole")
        return None
    left = finish(p)
    return first, line, wall, left


def open_cmd(a):
    done = exchange(a, "initialize + tools/list", "", {"jsonrpc": "2.0", "id": 2, "method": "tools/list"})
    if done is None:
        return
    first, line, wall, left = done
    init = json.loads(first)["result"]
    tools = json.loads(line)["result"]["tools"]
    text = json.dumps({"instructions": init.get("instructions", ""), "tools": tools}, ensure_ascii=False, indent=1)
    size = len(first.encode()) + len(line.encode())
    n = log(a.t, "initialize + tools/list", "", False, text, size, wall, "whole", noted(a, left))
    keep_config(a, n)
    show(n, "initialize + tools/list", False, text, size, wall, "whole")


def call_cmd(a):
    if a.draft:
        # a harness that parses a heredoc's body as shell refuses some JSON: the arguments as a file
        if "/" in a.draft or a.draft.startswith(".") or a.args is not None:
            sys.exit("seat-mcp: --from takes a bare file name under WORLD/drafts/, in place of the arguments")
        raw = open(os.path.join(WORLD, "drafts", a.draft)).read()
    elif a.args is None:
        sys.exit("seat-mcp: no arguments: '<json>', - for stdin, or --from NAME")
    else:
        raw = sys.stdin.read() if a.args == "-" else a.args
    try:
        args = json.loads(raw)
    except ValueError as e:
        sys.exit(f"seat-mcp: the arguments are not JSON ({e}); nothing was sent and nothing logged")
    sent = json.dumps(args, separators=(",", ":"))
    done = exchange(a, a.tool, sent, {"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": a.tool, "arguments": args}})
    if done is None:
        return
    _, line, wall, left = done
    resp = json.loads(line)
    if "error" in resp:
        text, is_error = json.dumps(resp["error"]), True
    else:
        r = resp["result"]
        text = "".join(c.get("text", "") for c in r.get("content", []))
        is_error = bool(r.get("isError"))
    shown = "whole" if len(text.encode()) <= SHOWN else "head"
    n = log(a.t, a.tool, sent, is_error, text, len(line.encode()), wall, shown, noted(a, left))
    keep_config(a, n)
    show(n, a.tool, is_error, text, len(line.encode()), wall, shown)


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
    # what the door did, not a size alone: a runner could not tell an append
    # from nothing (the re-sit, cli-sonnet fault 6)
    print(f"seat-mcp: {'started' if new else 'appended to'} {name}, now {os.path.getsize(path)} bytes")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("world", help="the world's root, made by seat.py setup")
    ap.add_argument("bin", help="the directory holding acq-mcp")
    sub = ap.add_subparsers(dest="cmd", required=True)
    o = sub.add_parser("open")
    o.add_argument("--t", required=True)
    o.add_argument("--note", default="")
    o.add_argument("--host-config", help="start the server from this host configuration under WORLD/work/")
    o.add_argument("--timeout", type=float, default=300)
    c = sub.add_parser("call")
    c.add_argument("--t", required=True)
    c.add_argument("--note", default="")
    c.add_argument("--host-config", help="start the server from this host configuration under WORLD/work/")
    c.add_argument("--timeout", type=float, default=300)
    c.add_argument("tool")
    c.add_argument("args", nargs="?")
    c.add_argument("--from", dest="draft", help="the arguments from a file under WORLD/drafts/, by bare name")
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
