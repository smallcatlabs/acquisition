05fcb1d9^..1a6f1695 — implementation done-when holds on static inspection; two evidence summaries need correction.

- **[P3] [transport.rs:267](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/tests/transport.rs:267):** A raw response can panic because an HTTP byte length is used to slice UTF-8 text. Concrete simulated upstream input: `HTTP/1.1 200 OK\r\nContent-Length: 1\r\n\r\né`. This is fixture-authored input, not client or persisted input. Slice bytes before decoding and add this case to the existing component test.
- **[P3, shape 1] [spec.rs:181](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/ratelimit/tests/spec.rs:181), `c27557ea` message:** The inventory summary says four binds remain and nine move, but six remain and seven move. The retained sites are the raw helper, preparation table, counting helper, redirect target, and two origin-check listeners. Correct the summary in a follow-up message.
- **[P3, shape 1] [door.rs:305](/Users/tom/Development/GitHub/smallcatlabs/acquisition/crates/acquisition-daemon/src/daemon/tests/door.rs:305), `1a6f1695` message:** `held_by_transport=1` does not establish that only one park encountered a held request, because `quiet` and scenario holds take precedence. The new boundary rows themselves call `quiet` while requests remain held. Say “one park selected the transport-hold branch.” Both message findings concern summaries wider than their evidence.

Not verified: gate/CI results, measured counters and test counts, breakers, and deliberate-failure experiments; execution was prohibited and the worker’s report was withheld.

Nitpicks: all three are nonblocking; the UTF-8 case is unlikely with the current fixtures, and the other two correct evidence wording.

Brief: distinguish selected hook branches from total parks with a condition present. Plan: no change suggested. Prompt: no change suggested.
