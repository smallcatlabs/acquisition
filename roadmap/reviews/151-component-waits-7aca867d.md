514fdeb9..7aca867d — Do 4’s implementation holds on inspection; the done-when is incomplete because the final measure’s failure inventory omits three tests.

- **P3, shape 1 — 7aca867d commit message:155 and :202.** Both `policy_state_key` mutants claim “46 failing,” but the saved logs report 49 failures: 46 `FAIL` outcomes and three `SIGABRT` outcomes. The omitted tests are `sessions::authentication_completes_before_the_api_send_takes_a_gate_slot`, `spec::common_gate_caps_gets_until_response_bodies_complete`, and `spec::waiting_head_has_writer_preference_over_later_mixed_policy_send`. This concerns saved review evidence, not client, persisted-store or upstream input. Smallest fix: correct both entries to distinguish the 46 assertion failures and three aborts, and name the omitted tests. Evidence: `runs/151-component-waits/mutants-151-after/mutants.out/log/crates__acquisition-daemon__src__ratelimit__limiter.rs_line_112_col_5.log:2434`, and its `_001.log` sibling.

- **P3, shape 10 — crates/acquisition-daemon/src/daemon/tests/stress.rs:143.** The retained `assert_whole(ends)` duplicates the fixture-completion and fault checks now performed by `Door::finish` at `door.rs:1511`. For the same scripted upstream refresh, both inspect the same `Ends`; the final check also catches later faults. The complete-script method assertions at `dispatch.rs:417` and `:589` share this cause: the script and `Door::finish` already enforce those complete sequences. Smallest fix: retire these duplicate assertions with the required retirement lines; retain the intermediate pre-reset `[HEAD]` assertion.

Gate and fresh execution: **not verified**, as requested; I inspected the saved measure without executing code. Do 2’s tools commit and the future Weekly comparison were outside this review.

No production defect or invalid ownership declaration was found among the declaring waits, including stress’s terminal waits and exit’s arrival wait. The duplicate assertions are nitpicks; the inventory discrepancy is an observed reporting error.

The brief and plan introduced no additional obstacle. The prompt could clarify whether “tests that now fail” explicitly includes aborted test processes.
