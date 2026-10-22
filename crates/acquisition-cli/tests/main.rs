//! The integration tests of the CLI, one binary (C122): each
//! file under `tests/` is a module here, so the shared modules and
//! the crate's own dependencies are compiled and linked once. A file
//! that sets the process environment is its own binary instead
//! (`tools/arch-check.sh`, the layout rule), and is not listed here.

mod harness;

mod apply_loop;
mod characters_wire;
mod cli_reference;
mod daemon_observe;
mod daemon_signal_exit;
mod daemon_stop_checkpoint;
mod error_kind;
mod journey;
mod new_league;
mod no_ids;
mod normaliser;
mod persisted_queue;
mod plan_json;
mod price_json;
mod price_story;
mod provider_default;
mod readme_tour;
mod realm_wire;
mod reference_currency;
mod search_json;
mod seat_faults;
mod shop_json;
mod world;
