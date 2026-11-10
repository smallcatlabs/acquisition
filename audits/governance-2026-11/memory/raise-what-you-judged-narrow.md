---
name: raise-what-you-judged-narrow
description: "a risk noticed while building goes in the report's \"Raised for review\" even when judged narrow; r2 left one unraised and it came back as a P2"
metadata:
  type: feedback
---

A risk a building session notices and judges unlikely is still raised, in the report and the commit message, never decided away silently.

**Why:** in r2 (2026-10-26) the build saw that a token rejected during a held keyring save could be handed out again through the open refresh flight, judged the window narrow, and said nothing; Codex's review found it as a P2 because a keychain prompt makes the window long. The fix round cost more than one sentence in the report would have.

**How to apply:** when the build accepts an edge case, write it under the report's "Raised for review" with why it was accepted, so the owner and the reviewer judge it. Related: [[claims-match-evidence]], [[verify-before-deferring]].
