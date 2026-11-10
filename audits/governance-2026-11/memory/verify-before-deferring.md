---
name: verify-before-deferring
description: "Before deferring a gap as rare or cheap, check its cost and timing in the code; prefer a small fix that keeps a promise over a caveat"
metadata:
  type: feedback
---

On 2026-10-14 (external review of note 36 phase 1, C111 replay guard) I twice justified a deferral with an unchecked claim: a park said concurrent applies cost "one repeated request" (it is the whole unspent set), and a commit said "a replay follows the first apply's submit" (the guard saw only children, not the accepted apply parent). The reviewer caught both. The fix that ended it was ~15 lines in `unspent`, not the help-text caveat the reviewer first proposed; Tom and the reviewer accepted it.

**Why:** Tom wants design simplicity and doesn't want rare edge cases chased, but a deferral argued from a wrong cost is the shortcut he warns against; and agents skip caveats like "wait before retrying".

**How to apply:** before writing "rare", "cheap" or "can't happen" into a park, ruling or commit, read the code path that bounds it (dispatch, fan-out, who holds what). When a gap breaks an agent-facing promise and the fix is small, close it instead of documenting it. Disagreeing with a reviewer is fine; Tom relays between agents and wants a reasoned position. Related: [[owner-delegates-agent-facing-design]].

**Instance 2026-10-26 (r2 design):** called "whether the keychain read at start prompts" a fact to establish; `tools/sign-acqd.sh`'s header had recorded it since 2026-09-12. Before writing "a fact to establish", grep the tools' and skills' headers for the finding: the owner's scripts carry dated findings in their comments.

**Counterweight 2026-11-07 (4-full-1-saved fix rounds):** a reviewer called a legacy-file case P2 and offered a "small fix"; I took it under this rule without weighing the case's reach, and the small fix became 361 lines, then two more findings on its remedy's edges. The case was mock-only: a real GGG username carries `#discriminator`, so no real account file can collide with `user.db`. The owner asked "are we adding complexity to handle an obscure and unlikely scenario?" and the honest answer was yes. **Before taking a reviewer's small fix, read who can meet the case** (a real world, or only a scratch mock one) and size the fix to that; a structural one-line escape was right, a hand-rename remedy was not. Related: [[triage-now-vs-after-alpha]], [[simplicity-over-mechanism]].
