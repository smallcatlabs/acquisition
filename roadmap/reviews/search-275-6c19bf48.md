b325f812^..6c19bf48 — functional done-when holds by inspection; execution-dependent criteria remain open.

- **P3 — `crates/acquisition-daemon/src/mockggg.rs:1310`, shape 3:** `fixture_faults` still promises a frame from “N43’s set,” although this round admits N54’s two additional frames. Concrete reading: a `Viscera` fixture passes despite falling outside the documented set. Smallest fix: say “a frame word from `FRAME_WORDS`.” This concerns documentation, not client, persisted, or upstream input handling.

- **Not checked:** Gate results, copy replay, and measured outcomes were not verified because execution was prohibited; the private survey’s underlying observations were unavailable.
- **Nitpicks:** The P3 documentation finding above. No behavioural or invariant defects found.
- **Brief:** Name the intended existing vocabulary test explicitly; its current description leaves the required test location ambiguous.
- **Plan and prompt:** Nothing else impeded the review.
