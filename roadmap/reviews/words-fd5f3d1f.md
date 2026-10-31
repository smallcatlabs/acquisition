bbd3023e^..fd5f3d1f — `roadmap/briefs/words.md`’s done-when does not fully hold: Windows commands still fail to round-trip.

- **P1 — [plan_cmd.rs:102](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/plan_cmd.rs:102), shapes 2 and 6:** The Windows policy pipe can silently corrupt persisted intent. A prior policy containing league `Café` produces an undo that stock PowerShell 5.1 pipes as `Caf?`, because native-program stdin defaults to ASCII; the valid JSON can then replace the policy successfully. The harness masks this by setting `$OutputEncoding` to UTF-8. Emit ASCII-escaped JSON for the pipe, and pin a non-ASCII policy undo without changing the shell’s input encoding. [PowerShell’s documented default](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_preference_variables?view=powershell-5.1#outputencoding)

- **P2 — [answer.rs:3237](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-search/src/answer.rs:3237), shape 6:** Single-quoting still loses embedded double quotes when PowerShell 5.1 invokes `acq`. Client input such as `--trade '{"query":{"type":"Chaos Orb"}}'` therefore reaches the JSON reader without its required quotes; ordinary quoted search expressions fail similarly. The commit acknowledges this exception while closing #16, although the brief includes it. Route these payloads through the existing `--query-file -` and `--trade-file -` forms using lossless encoding, and retain execution checks through `powershell.exe`. [Native argument behavior](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_parsing?view=powershell-7.5#passing-arguments-that-contain-quote-characters)

- Builds, tests, gate status and Windows execution: **not verified**, as requested for this read-only review.
- Neither finding is a nitpick.
- Brief: explicitly include a non-ASCII policy round trip in #16’s acceptance cases.
- Plan: no separate plan was supplied.
- Prompt: no changes suggested.
