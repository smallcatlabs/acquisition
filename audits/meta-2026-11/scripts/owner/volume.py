"""Volume of the owner's typed messages (origin=human) from msgs.jsonl: per week, per day, per session, length, go-aheads."""
import json, os, re, statistics, collections, datetime as dt
D = os.path.dirname(os.path.abspath(__file__))
H = [json.loads(l) for l in open(os.path.join(D, "msgs.jsonl"))]
H = [m for m in H if m["origin"] == "human"]
def local(ts): return dt.datetime.fromisoformat(ts.replace("Z", "+00:00")).astimezone()
for m in H: m["d"] = local(m["ts"])
pasted = [m for m in H if "<pasted_content" in m["text"] or len(m["text"]) > 2500]
print("messages", len(H), "pasted/long", len(pasted))
L = [len(m["text"]) for m in H]
print("median chars", statistics.median(L), "p75", sorted(L)[int(.75*len(L))], "<=40 chars", sum(1 for x in L if x <= 40))
days = collections.Counter(m["d"].date() for m in H)
print("active days", len(days), "median/day", statistics.median(days.values()), "max", max(days.values()))
wk = collections.Counter(m["d"].strftime("%G-W%V") for m in H)
print("per ISO week", sorted(wk.items()))
ss = collections.Counter(m["sid"] for m in H)
print("sessions with a typed message", len(ss), "median/session", statistics.median(ss.values()), "max", max(ss.values()))
GO = re.compile(r"^\W*(yes|yep|yeah|ok|okay|sure|sounds good|go ahead|go|continue|proceed|push|do it|launch|approve[d]?|agreed?|please do|thank you|thanks|keep (it|them)|cut it|start the build|send the fix round|commit|push and close|yes,? (go ahead|push|please|both|all|do it|let'?s do (it|this)|set it|launch)|ok,? go ahead|let'?s do (it|this)|i agree.*|approve all.*|accept.*recommended.*)[\s.!,]*(please|thank you|thanks)?[\s.!]*$", re.I)
go = [m for m in H if len(m["text"]) <= 60 and GO.match(m["text"].strip())]
print("go-ahead (<=60 chars, matches pattern)", len(go), f"{len(go)/len(H):.0%}")
print(collections.Counter(re.sub(r"[^a-z ]", "", m["text"].lower()).strip()[:30] for m in go).most_common(40))
short = [m for m in H if len(m["text"]) <= 60 and not GO.match(m["text"].strip())]
print("other short (<=60) sample:", [m["text"] for m in short][:80])
