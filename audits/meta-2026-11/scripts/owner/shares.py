"""Mutually exclusive, ordered keyword classification of all owner-typed messages (turns.jsonl), to estimate where his attention goes.
Order matters: first match wins. A proxy; the categories were checked against samples by hand."""
import json, os, re, collections
D = os.path.dirname(os.path.abspath(__file__))
T = [json.loads(l) for l in open(os.path.join(D, "turns.jsonl"))]
first = {}
for t in sorted(T, key=lambda t: t["ts"] or ""):
    first.setdefault(t["sid"], t["ts"])
GO = re.compile(r"^\W*(yes|yep|ok|okay|sure|go ahead|go|continue|proceed|push|approved?|agreed?|confirmed|i agree|accept|launch|start|keep|cut|commit|send|design step|write|run|record|take|fix|thank you|thanks)\b", re.I)
def c(t):
    x = t["text"]
    if "<pasted_content" in x or x.startswith(("Here is the external review", "The external review", "I audited commits")): return "relays a review or prompt (pasted)"
    if t["kind"] in ("button1",): return "go-ahead (recommended button)"
    if t["kind"] == "button2": return "picks the second button"
    if re.search(r"(step back|simpl|elegan|idiomatic|complexit|reframe|band.?aid|instead of|don'?t want|i don'?t like|not what|rather than|let'?s not|no,|not approved)", x, re.I): return "steers: simplicity / redirect / correction"
    if GO.match(x) and len(x) <= 200: return "go-ahead (typed)"
    if t["ts"] == first.get(t["sid"]) and len(x) > 150: return "opens a session with a task"
    if re.search(r"(sitting|i (ran|tried|clicked|opened|saw|pushed|have (stopped|freed|taken|refreshed|read))|captures|done\b|is running|logged in|vm\b)", x, re.I): return "reports what he did / ran / saw"
    if "?" in x: return "asks the session (explain, opinion, status)"
    return "other instruction or context"
C = collections.Counter(c(t) for t in T)
n = sum(C.values())
for k, v in C.most_common(): print(f"{v:5d} {v/n:5.0%} {k}")
print(n)

# sub-classify the questions he asks
SUB = [
 ("how to run the agents (session, subagent, context, model, worktree, usage, series/parallel)", r"(session|subagent|sub-?agent|context|compact|fable|opus|sonnet|codex|worktree|usage|parallel|series|in charge)"),
 ("your opinion / recommendation / priority", r"(recommend|opinion|advise|assessment|would you|do you think|what do you think|prefer|priorit|value of|worth)"),
 ("explain it to me / what does X mean", r"(understand|explain|mean|plain|what is|what does|what are|why )"),
 ("status / what is left / where are we", r"(where are we|left|status|how much|done|finished|next)"),
 ("what would it take (product idea)", r"(what would it take|could we|can we|is there a way|what if)"),
 ("how-to for himself (vm, git, terminal, github)", r"(how do i|which folders|vm|terminal|github|git )"),
]
S = collections.Counter(); ex = collections.defaultdict(list)
for t in T:
    if c(t) != "asks the session (explain, opinion, status)": continue
    k = next((k for k, rx in SUB if re.search(rx, t["text"], re.I)), "other question")
    S[k] += 1; ex[k].append(t["text"][:90].replace("\n", " "))
for k, v in S.most_common(): print(f"{v:4d} {k}"); [print("      ", e) for e in ex[k][:3]]
