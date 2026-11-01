68cb9448 — the fix round’s done-when does not fully hold: finding 1 is fixed and pinned; finding 2 remains incomplete.

- **[P2] tools/arch-check.sh:659 — shape 1:** Check 12 still permits production Tokio timers through a crate alias despite claiming enforcement “under any spelling.” Concrete source input `use tokio as runtime;` followed by `runtime::time::sleep(std::time::Duration::ZERO).await` matches none of the three patterns, allowing a timer invisible to the door’s clock accounting. Enforce an unaliased Tokio root for production imports, including grouped crate renames, or resolve aliases before checking; add a breaker for this case.

- **Not checked:** execution—builds, tests, gates and breakers—is **not verified**, because this review was restricted to reading.

- **Nitpicks:** none; crate aliases are uncommon here, but the gap leaves the reviewed structural property unenforced.

- **Brief:** No additional obstacle affected this fix-round review.
- **Plan:** Prefer enforcing a canonical import rule over enumerating timer spellings.
- **Prompt:** No change needed.
