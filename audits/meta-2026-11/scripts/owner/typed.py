"""Classify typed replies to a wrap_up ask (turns.jsonl kind typed-to-ask): accept / accept+more / question back / other. Prints counts, latency, and writes typed_other.txt for hand reading."""
import json, os, re, collections, statistics, datetime as dt
D = os.path.dirname(os.path.abspath(__file__))
T = [json.loads(l) for l in open(os.path.join(D, "turns.jsonl"))]
ACC = re.compile(r"^\W*(yes|yep|yeah|ok|okay|sure|sounds good|go ahead|continue|proceed|push|approved?|agreed?|confirmed|i agree|accept|perfect|great|good|that sounds good|let'?s do|do it|please)", re.I)
def p(ts): return dt.datetime.fromisoformat(ts.replace("Z", "+00:00"))
C = collections.Counter(); lat = []
o = open(os.path.join(D, "typed_other.txt"), "w")
for t in T:
    if t["kind"] in ("button1", "button2", "typed-to-ask") and t.get("ask_ts") and t.get("ts"):
        lat.append((p(t["ts"]) - p(t["ask_ts"])).total_seconds() / 60)
    if t["kind"] != "typed-to-ask": continue
    x = t["text"].strip()
    if ACC.match(x) and len(x) <= 80: k = "accept-short"
    elif ACC.match(x): k = "accept+more"
    elif "?" in x: k = "question-back"
    else: k = "other"
    C[k] += 1
    if k in ("other", "question-back", "accept+more"):
        o.write(f"=== {k} {t['ts'][:16]} {t['sid']}\nASK: {t.get('ask')}\nBUTTONS: {t.get('labels')}\nHE: {x[:700]}\n\n")
print(C)
lat.sort(); print("minutes ask->answer: median", statistics.median(lat), "p75", lat[int(.75*len(lat))], "p90", lat[int(.9*len(lat))], "n", len(lat))
