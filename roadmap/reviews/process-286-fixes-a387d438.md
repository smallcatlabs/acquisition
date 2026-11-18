a387d438 — the fix-round done-when does not hold: receiptless recovery can overwrite another client’s policy, and the recovery-command pin remains incomplete.

- **P1 — tools/tracer-rung.sh:170:** Cleanup treats revision `BEFORE_REVISION + 1` as proof that this run wrote it, allowing loss of another client’s persisted intent. Concrete interleaving: the driver reads revision n and arms cleanup; another client writes n+1; interruption leaves the driver without a receipt before its write lands or its refusal is recorded; cleanup then replaces the other client’s n+1 with the old policy. Require evidence attributing the write to this run before restoring; if ownership is uncertain, leave it and fail explicitly. Pin this interleaving alongside the interrupted-write case.

- **P2 — tools/tracer-rung-breakers.sh:89 — shape 2:** The printed-command case still does not pin the account qualification and revision protection required by the preceding review. Its mock world has one account, the run supplies no `--account`, and the revision stays unchanged before replay, so removing the account selector or revision guard from the printed command would leave this case green. Stage two accounts with an explicit selector, replay the command successfully, then separately verify that replay after another policy write refuses without changing that write.

Execution—gate, verifier self-test, mock variants, breakers and terminal behavior—**not verified**; this review read committed source and affected callers only, as requested. Real-provider behavior was not exercised.

Nitpicks: none; the narrow interruption window can lose stored intent, and the missing regression coverage was explicitly required.

Brief: no changes suggested. Plan: none supplied separately. Prompt: no changes suggested.
