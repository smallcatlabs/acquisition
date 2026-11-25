# The keyring round trip returns green when it did not run

Labels: area:ci trigger:step

`keyring_round_trip_survives_a_fresh_entry` returns without `ACQ_KEYRING_ROUNDTRIP` (`crates/acquisition-daemon/src/auth.rs:894`). Invariant 5 names it as a pin.
