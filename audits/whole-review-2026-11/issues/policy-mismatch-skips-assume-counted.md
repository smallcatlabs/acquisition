# A counted send whose answer names another policy is not counted

Labels: area:network trigger:step

In `Limiter::observe_impl` the parse-failure arm calls `assume_counted` for a counted send; the `PolicyMismatch` arm beside it returns without (`crates/acquisition-daemon/src/ratelimit/limiter.rs:622-639`). Today this stands as a comment on #66 with no row in `spec.rs`. Fix by structure: 'counted and not reconciled' handled once after the match, so no future arm can skip it.
