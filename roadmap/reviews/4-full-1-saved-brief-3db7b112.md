[4-full-1-saved.md](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md) at `3db7b112`, unchanged in the working tree, is **not buildable as drawn**.

Third review’s five findings:

1. **Answered:** clear returns `no_undo` when another row holds the prior’s name, with that case pinned.
2. **Answered:** the kind admits `all`; explicit and zero-realm resolution are pinned.
3. **Answered:** `User::at_revision` is explicitly delegated.
4. **Answered:** `--trade-file` joins the exclusion rule and its pins; the remaining count error is noted below.
5. **Answered:** done-when explicitly names the `doors` regex and `User::open_for`.

Findings 1–2 share one cause: the admitted duplicate-name state is handled for clears and third-row creates, but not for other generated writes.

1. **Do 3/5 — Renaming a duplicate can still produce an undo guaranteed to refuse.** The set receipt promises restoration of a readable prior ([brief:203](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:203)), while the courtesy check rejects its name when another live row holds it ([brief:230](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:230)). With A and B both named `rings`, rename A to `helmets`; its prescribed undo restores `rings` and immediately meets B, without any intervening write. **Smallest change:** apply the courtesy-check fallback to every receipt’s undo, including replacements, and extend the existing two-row pin with this rename returning `no_undo`.

2. **Do 3/5 — One conflicting row does not always make the re-aimed `name_taken` offer executable.** The single-match rule unconditionally offers the set against that other row ([brief:231](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:231)). With A and B named `rings`, edit A’s text while retaining its name: B is the one other holder, so the offer targets B; replaying it now meets A and refuses again. The prescribed duplicate-name pin exercises a third create, which misses this case ([brief:313](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:313)). **Smallest change:** offer a re-aimed set only where its resulting name would pass the same courtesy check; otherwise offer the holders’ show/run requests. Add this edit-and-replay case to that pin.

3. **Do 5 — “Five words” understates the exclusion cases now prescribed.** Do 4 excludes six inputs: positional query, `--query-file`, `--trade`, `--trade-file`, `--realm`, and `--membership` ([brief:273](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:273)); Do 5 still requests five ([brief:327](/Users/tom/Development/GitHub/smallcatlabs/acquisition/roadmap/briefs/4-full-1-saved.md:327)). **Smallest change:** require “each excluded input,” removing the duplicated count.

Not checked: builds, tests, gate execution or current CI conclusions; execution-dependent acceptance claims are **not verified**, as requested. GitHub was unreachable, so issue #42’s live body was not checked.

Nitpicks: finding 3 only. Findings 1–2 concern uncommon but explicitly supported persisted state.

The step should **not start as drawn**.
