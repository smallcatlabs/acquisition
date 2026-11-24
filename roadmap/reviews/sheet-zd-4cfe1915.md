4cfe1915 — The issue’s completion conditions are not met: this commit prepares the sitting; the Rust composer and answer link remain unbuilt.

Findings, most severe first:

- **P2 — tools/trade-sheet.py:2785, shape 1:** The probes interpret an empty result as proof that the site accepts only its own encoding, although the control searches for a different item. Concrete upstream input: zd05 successfully executes and returns zero Redbeaks while zd01 finds Tabula Rasas; the sheet directs an unsupported conclusion about encoding. Treat zero results as inconclusive, and require a successful canonical link for the same query before attributing failure to encoding. Apply that rule to the generated CSV wording too.

- **P2 — tools/trade-sheet.py:2811, shapes 2 and 8:** The encoding guard checks freshly composed links, while verification accepts committed links solely by decoded query equality, so it does not protect the bytes the sitting actually opens. Concrete persisted input: replace zd02’s committed URL with the default encoding of the same query; `same()` accepts it, `kept()` preserves it, and the new guard still checks the independently generated stored encoding. The control encoding can therefore masquerade as the experimental one. Validate the required encoding properties on the committed URLs, and pin rejection of that substitution; likewise preserve the serialized key order that zd06 tests.

What I did not check and why:

- Execution results, gate status, and live-site acceptance: **not verified**, as requested.
- Unchanged daemon internals were outside this diff; the changed tool composes links without sending requests.

Nitpicks for unlikely cases: neither finding; both affect whether the planned sitting supports its conclusions.

Brief: separate completion conditions for sitting preparation would clarify this commit’s scope.

Plan: using another Python encoding supplies preliminary evidence, but cannot establish acceptance of the eventual Rust composer’s output.

Prompt: no obstacle.
