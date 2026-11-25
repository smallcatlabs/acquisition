# A job whose realm does not parse runs as pc

Labels: area:daemon trigger:step

`crates/acquisition-daemon/src/daemon/kinds.rs:606`: `Realm::from_params(&params).unwrap_or(Realm::DEFAULT)`. Malformed input should be a refusal with a stable kind (C47), not a default.
