"""Keyword themes over the owner's typed messages (origin=human, under 2500 chars, no pasted content).
Prints counts per theme, per week, and samples to themes_samples.txt. Keyword matching is a proxy; samples were read by hand."""
import json, os, re, collections, datetime as dt
D = os.path.dirname(os.path.abspath(__file__))
H = [json.loads(l) for l in open(os.path.join(D, "msgs.jsonl"))]
H = [m for m in H if m["origin"] == "human" and "<pasted_content" not in m["text"] and len(m["text"]) < 2500]
TH = {
 "simplicity/elegance/idiomatic": r"\b(simpl\w*|elegan\w*|idiomatic|complexit\w*|reframe|reframing|root cause|band.?aid|patching|edge cases?)\b",
 "explain/plain words/understand": r"(help me understand|plain (language|terms|words)|simpler terms|explain|what does .* mean|i don'?t (fully )?understand|in simple)",
 "your opinion/recommendation": r"(your (professional |objective )?(opinion|recommendation|judge?ment|assessment|thinking)|what do you (recommend|think|advise)|would you (advise|recommend|prefer)|do you (think|advise|recommend)|you yourself)",
 "step back/stop/redirect": r"(step back|stop (here|now|and)|before we (continue|rule|move|fold|push|commit|go)|don'?t (stop|do|start|use|launch)|that'?s not|not what i|i don'?t (want|like)|instead of|wait\b|hold on)",
 "autonomy grant/trust": r"(i trust your|your (own )?judge?ment|act autonomously|you (are|have) (in charge|the authority|my explicit permission)|use your (own )?judge?ment|as much as you want|you decide)",
 "session/context/model mgmt": r"(new session|fresh session|context|compact|subagent|sub ?session|worktree|fable|opus|sonnet|codex|usage|tokens?)\b",
 "status/what's left": r"(where are we|what('s| is) left|how much (planned )?work|what work is left|what'?s next|status)",
 "see/run it himself": r"(sit(ting)?|i (ran|tried|clicked|opened|saw|see|looked)|in (my|the) browser|brave|acq web|screenshot)",
 "review request": r"\b(review|codex|cold|blind)\b",
}
C = collections.Counter(); W = collections.defaultdict(collections.Counter); S = collections.defaultdict(list)
for m in H:
    wk = m["ts"][:10]
    for k, rx in TH.items():
        if re.search(rx, m["text"], re.I):
            C[k] += 1; W[k][dt.date.fromisoformat(wk).isocalendar()[1]] += 1; S[k].append(m)
print("n", len(H))
for k, v in C.most_common(): print(f"{v:4d} {k:35s}", dict(sorted(W[k].items())))
with open(os.path.join(D, "themes_samples.txt"), "w") as o:
    for k in TH:
        o.write(f"\n########## {k} ({C[k]})\n")
        for m in S[k]: o.write(f"- {m['ts'][:10]} {m['sid']}: {m['text'][:260]!r}\n")
