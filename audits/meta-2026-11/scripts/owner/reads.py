"""What the owner refers to (reads), and how many of his messages are relayed agent-drafted prompts."""
import json, os, re, collections
D = os.path.dirname(os.path.abspath(__file__))
H = [json.loads(l) for l in open(os.path.join(D, "msgs.jsonl"))]
H = [m for m in H if m["origin"] == "human"]
REF = {
 "a commit hash": r"\b[0-9a-f]{8}\b",
 "an issue #N": r"#\d{2,3}\b",
 "a ruling id Cnnn": r"\bC\d{1,3}\b",
 "the roadmap/README": r"roadmap|road\b",
 "a review (read it)": r"(i'?ve read|i have read|read the).{0,40}(review|brief|findings|packet|file|issue)",
 "a brief file path": r"briefs?/",
 "a ledger": r"ledger",
 "HISTORY/close commit": r"close commit|HISTORY",
 "GitHub web (issues page, ruleset, CI)": r"(github|ruleset|\bCI\b|actions)",
}
RELAY = re.compile(r"(^(Take #|Orchestrate |You are the |Read (AGENTS|roadmap)|Task:|Resume )|by one rule|Read roadmap/README\.md|its cold review|write its close|Your scratch)", re.I)
C = collections.Counter()
for m in H:
    for k, rx in REF.items():
        if re.search(rx, m["text"], re.I if k not in ("a commit hash",) else 0): C[k] += 1
print(len(H), C.most_common())
R = [m for m in H if RELAY.search(m["text"])]
print("relay-like", len(R))
for m in R[:60]: print(" ", m["ts"][:10], repr(m["text"][:110]))
