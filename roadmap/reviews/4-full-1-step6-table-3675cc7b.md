f5f6c542^..3675cc7b — the brief’s done-when does not fully hold.

- **P2 — [window.js:53](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/assets/window.js:53):** A failed fetch leaves its block permanently pending. If a visible window’s loopback fetch rejects without cancellation, the empty block remains in `seg.blocks` with `done=false`; scrolling within that range never retries it and also suppresses prefetching. This is a loopback transport failure, not GGG input. Give non-abort failures an explicit error state with a retry or reload action, and pin that path.

- **P3 — [web/mod.rs:1317](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/src/web/mod.rs:1317):** Client-supplied window bounds can overflow instead of producing the promised refusal. With a valid token, `offset=18446744073709551615&count=1` on a 64-bit build panics with overflow checks or wraps and returns an empty successful window without them. Use checked addition, and add this case to the existing HTML/JSON refusal table.

- **P3 — [browser.rs:605](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-cli/tests/browser.rs:605), shape 1:** The browser check permits an extra block beyond Decision 10’s stated bound. Its fixed inputs—600-pixel viewport, 24-pixel rows, 50-row blocks—give `ceil(75/50)+1 = 150` rows, but the assertion permits 200. A regression retaining a fourth block passes this assertion. Change `+2` to `+1`.

- Not checked: execution-dependent done-when, including browser behavior, gate/doc exits and hosted CI—**not verified**, as execution was prohibited.
- Nitpicks for unlikely cases: the maximum-integer offset and the browser bound’s extra allowance; neither is exercised by ordinary page-generated requests.
- Brief: write the DOM bound as an explicit formula to remove ambiguity.
- Plan: nothing else impeded review.
- Prompt: no changes needed.
