"""Save a reader's final report from its transcript, whole.

usage: python3 -I save_report.py <transcript.jsonl> <out.md>
A subagent returns text and cannot write files. Its report is the
`message` of its last hand-back call, or failing that its last text;
this copies it so no report is retyped.
"""
import json, sys
src, out = sys.argv[1], sys.argv[2]
last = None
with open(src, encoding="utf-8") as fh:
    for line in fh:
        try:
            ev = json.loads(line)
        except ValueError:
            continue
        content = (ev.get("message") or {}).get("content")
        if ev.get("type") != "assistant" or not isinstance(content, list):
            continue
        for block in content:
            if block.get("type") == "tool_use" and block.get("name") == "SubagentHandback":
                text = (block.get("input") or {}).get("message", "")
            elif block.get("type") == "text":
                text = block.get("text", "")
            else:
                continue
            if len(text.strip()) > 400:
                last = text
if last is None:
    sys.exit("no report found")
with open(out, "w", encoding="utf-8") as fh:
    fh.write(last.rstrip() + "\n")
print(len(last), "chars ->", out)
