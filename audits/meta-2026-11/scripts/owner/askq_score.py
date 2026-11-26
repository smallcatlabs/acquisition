"""Score each AskUserQuestion sub-question: recommended (marked or first) vs the answer. Prints a table and a full dump to askq_dump.txt."""
import json, os, re, collections
D = os.path.dirname(os.path.abspath(__file__))
rows = []
dump = open(os.path.join(D, "askq_dump.txt"), "w")
for l in open(os.path.join(D, "askq.jsonl")):
    v = json.loads(l)
    r = v.get("result") or {}
    txt = r.get("text") or ""
    ans = {}
    tur = r.get("tur")
    if isinstance(tur, dict) and isinstance(tur.get("answers"), dict): ans = tur["answers"]
    dump.write(f"===== {v.get("ts")} {v['sid']} -> {r.get('ts')}\n{txt}\n")
    for q in v["input"].get("questions", []):
        opts = [o.get("label", "") for o in q.get("options", [])]
        rec = next((o for o in opts if "(Recommended)" in o), opts[0] if opts else "")
        a = ans.get(q["question"])
        if a is None:
            m = re.search(re.escape('"' + q["question"] + '"="') + r'(.*?)"(?:[,.] |\s+selected| You)', txt, re.S)
            a = m.group(1) if m else None
        if isinstance(a, list): a = ", ".join(a)
        if a is None: kind = "unanswered/rejected"
        elif a == rec or a.startswith(rec): kind = "recommended"
        elif a in opts: kind = "other offered"
        else: kind = "not offered"
        rows.append(((v["ts"] or r.get("ts") or "?")[:10], v["sid"], kind, q["question"][:90], rec[:40], (a or "")[:90]))
        dump.write(f"Q: {q['question']}\n OPTS: {opts}\n REC: {rec}\n ANS: {a}\n KIND: {kind}\n")
for r in rows: print(" | ".join(r))
print(collections.Counter(r[2] for r in rows), len(rows))
