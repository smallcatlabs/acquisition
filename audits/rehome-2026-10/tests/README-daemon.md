# Test read — daemon group

144 rows read of 144 (`data/read-daemon.tsv`), every body read, none filled from a name.
boundary 124 · weak 10 · mechanism 8 · harness 2 · cannot-fail 0 · duplicate 0.
Confidence: high 102, medium 42, low 0.
`data/cases-daemon.tsv` is header-only: the crate has no proptest and no generated-case loop; its loops run over fixed tables written in the test.
Rule used for `mechanism`: the verdict depends on private state a behaviour-preserving rewrite need not keep; a private read beside sufficient boundary asserts stays `boundary` at medium, with the lines in `note`.

Paths below are under `crates/acquisition-daemon/src/`.

## Weak (10)

- `artifact.rs:75` `c84_the_artifact_is_this_executable_identified_and_hashed` — the hash is compared with the same function's output (`artifact.rs:81`); a wrong digest passes.
- `daemon.rs:8195` `a_failed_relogin_keeps_the_old_session_and_reports_the_failure` — the failure is injected by calling `abort_login` (`daemon.rs:8212`); a `login_with_profile` that drops the old session on a real failed re-login passes.
- `daemon.rs:8348` `dispatcher_exhausts_on_third_429_without_a_fourth_send_or_final_sleep` — "no final sleep" is unread: the clock passed at `daemon.rs:8357` is never consulted; a dispatcher that sleeps out the last hold passes.
- `daemon.rs:8632` `ceiling_halts_after_n_sends_and_does_not_persist` — one lifetime, no restart (`daemon.rs:8632-8669`); a persisted ceiling passes. `rails.rs:736` holds that half.
- `daemon.rs:9354` `rejected_refresh_grant_disables_further_refreshes_until_login_or_logout` — only logout is exercised (`daemon.rs:9401`); a login that leaves the dead-grant mark passes.
- `daemon.rs:9487` `whoami_is_refused_in_real_mode_without_a_token_request` — the harness session has no refresh token, so no send is possible either way (`daemon.rs:9488-9489`, `9500`); only the refusal text can fail.
- `rails.rs:712` `ceiling_is_per_lifetime_and_counts_every_method` — "per lifetime" is not exercised; only `reset_tripwire` clears it (`rails.rs:731`). `rails.rs:736` holds that half.
- `rails.rs:812` `journal_writes_one_flushed_line_without_secrets` — the leak scan (`rails.rs:854-855`) runs over reports that carry no secret (`rails.rs:820-830`); a writer copying a real Authorization value passes. The line-format half is sound.
- `rails.rs:911` `iso_timestamp_is_well_formed` — length 24, a `20` prefix and the `T` on `SystemTime::now()` (`rails.rs:912-915`); a wrong month, day or hour passes. The exact stamp is held at `daemon.rs:6877`.
- `ratelimit.rs:4366` `ip_rules_share_state_across_accounts` — both "accounts" send on the same bare key (`ratelimit.rs:4378`, `4383`); a limiter splitting an Ip-only policy per account passes.

## Mechanism (8, most disposable first)

- `daemon.rs:5811` `bounded_429_requeue_keeps_the_existing_two_retry_contract` — private predicate `may_requeue_429`; the bound is held on the wire at `daemon.rs:8313` and `8348`. Only the cancel arm is unique.
- `daemon.rs:9155` `a_halted_daemon_with_only_waiting_jobs_is_idle` — private `has_live_jobs`; the halted-and-waiting arm is held through `is_idle` at `daemon.rs:9192`. The running-under-halt arm is only here.
- `daemon.rs:5736` `session_keys_follow_renames_so_replace_finds_the_right_session` — the private `Sessions` map and its generation counter (`daemon.rs:5757-5761`).
- `daemon.rs:5440` `successful_refresh_rotation_advances_token_generations_and_persists_new_token` — the subject is the private generation counters (`daemon.rs:5463-5465`); the keyring save is the observable half.
- `daemon.rs:5394` `abandoned_refresh_owner_without_waiters_allows_retry` — private `refresh_flight` and generations (`daemon.rs:5411-5412`); the retry's second POST is observable.
- `daemon.rs:5326` `abandoned_refresh_owner_completes_all_waiters_and_allows_retry` — polls `receiver_count` to proceed (`daemon.rs:5347`); the retry's second POST is observable and has no other pin.
- `daemon.rs:7586` `dispatcher_preserves_priority_within_a_scheduling_key` — private `pick_runnable` and `active_jobs` (`daemon.rs:7596-7607`). C5's order has no wire pin in this group: rewrite at the wire before deleting.
- `daemon.rs:7191` `dispatcher_auth_waits_do_not_cap_independent_job_progress` — gated on `receiver_count == 1` (`daemon.rs:7211-7220`). The C21 property under it is boundary and has no other pin for auth waits: rewrite, do not delete.

## Notes for the reviewer

- `auth.rs:225` `keyring_round_trip_survives_a_fresh_entry` is classed `boundary` at medium: it returns before any assertion unless `ACQ_KEYRING_ROUNDTRIP` is set (`auth.rs:226-230`), so in the local gate it cannot fail; CI is where it can.
- The two `mockggg.rs` rows are `harness`: the mock provider's path reading and fixtures. The mock is also what `ACQ_PROVIDER=mock` serves, which is the argument for `boundary`.
- No row is an exact duplicate. Nearest: `ratelimit.rs:3806` (choke point) is subsumed by `daemon.rs:7615` (same headerless 503, with the 16 s hold exact) but not equal to it; the gate's unit tests (`gate.rs`) and the HTTP-level gate tests (`ratelimit.rs:3988`, `4055`, `4121`) hold the same properties at two rungs.
- Three `response_tests` rows (`daemon.rs:5854`, `5872`, `5889`) pin substrings of a job's failure text through private `classify_api_body`; classed `boundary` at medium because the text travels on the protocol's `Outcome::Failure`.
