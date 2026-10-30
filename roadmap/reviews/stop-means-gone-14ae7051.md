08254cc0^..14ae7051 — the brief’s done-when does not fully hold: `Gone` can win after the bound expires.

- **P2 — crates/acquisition-client/src/client.rs:1682, shape 1.** The biased selection polls `gone` before the bound, allowing endpoint disappearance observed after expiry to return `Gone`. Concrete lifecycle input: the connection ends, the endpoint remains through 20 seconds, then disappears before a delayed client task resumes; both futures are ready and `Gone` wins. Check the absolute deadline before accepting `Gone`, and pin late completion with an injected clock.

- **P3 — crates/acquisition-client/src/client.rs:626, shape 3.** The refusal still says “had not exited within” although the expanded `Acknowledged` outcome can mean an exited daemon whose endpoint remains. Concrete daemon/lifecycle input: mismatch-after-respawn cleanup receives `stopping` and a hang-up, but endpoint removal exceeds the bound. Say that exit and endpoint removal were not confirmed, and pin that wording in the existing renderer test.

Not checked: execution, gate status, test count and Windows runtime behavior — **not verified**, as requested. Reviewed committed content only. Both retirement targets exist and are included by the gate; the new pin forces both outcomes and rejects removal of the endpoint wait. Reset handling landed, and daemon code is unchanged.

Nitpicks: finding 2 is minor and concerns an uncommon cleanup path; finding 1 violates the explicit deadline contract.

Brief: add a forced simultaneous-expiry-and-completion case. Plan: no separate plan was supplied. Prompt: nothing obstructed the review.
