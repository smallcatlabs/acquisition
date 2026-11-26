"""Classify every wrap_up ask the owner answered (turns.jsonl) by what it asked, using keywords on the ask text,
and cross with how he answered (button1 / button2 / typed). Keyword classes are a proxy."""
import json, os, re, collections
D = os.path.dirname(os.path.abspath(__file__))
T = [json.loads(l) for l in open(os.path.join(D, "turns.jsonl"))]
CL = [
 ("go-ahead: commit/push/merge/close", r"\b(push|commit|merge|close|file (the|it|them|an)|post)\b"),
 ("go-ahead: start/launch/continue a step or session", r"\b(start|launch|carry on|continue|resume|next|send|run|build|go on|proceed|fix|cut|draft|write|archive|clean)\b"),
 ("his act: click/sit/stop/tell me", r"\b(click|sit|tell me|will you|stop your|adjust the ruleset)\b"),
 ("rule/approve a design or ruling", r"\b(rule|ruling|approve|accept|drafted|as designed|stand|recommend|which|what should|should)\b"),
]
def cls(a):
    a = a or ""
    for k, rx in CL:
        if re.search(rx, a, re.I): return k
    return "other"
X = collections.defaultdict(collections.Counter)
for t in T:
    if t["kind"] not in ("button1", "button2", "typed-to-ask"): continue
    X[cls(t.get("ask"))][t["kind"]] += 1
for k, v in X.items(): print(f"{sum(v.values()):4d} {k:50s} {dict(v)}")
