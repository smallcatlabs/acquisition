57713465 — the fix round’s done-when does not fully hold: the three prior findings are addressed, but the new recovery check has a scheduling race.

- **P2 — [browser.rs:722](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/browser.rs:722):** The retry scroll is sent before the test establishes that the failed fetch’s catch handler has removed its block. For this simulated loopback transport failure—not client, persisted, or GGG input—the browser can acknowledge `Fetch.failRequest`, process the scroll’s fill while the block remains pending, then deliver the rejection; removal queues no further fill, so the correctly fixed script never retries and the test hangs until the runner kills it. Wait for the failed block’s element to disappear before recording the request count and issuing the retry scroll.

- Execution-dependent checks, including browser behavior and gate/doc results: **not verified**, because execution was prohibited.
- Other scope not checked: unrelated daemon, persistence, and upstream GGG paths; this diff changes neither their code nor their callers.
- Nitpicks for unlikely cases: none.
- Brief: nothing impeded this fix-round review.
- Plan: the commit’s recovery-test sequence needs an explicit failure-observed barrier.
- Prompt: no changes needed.
