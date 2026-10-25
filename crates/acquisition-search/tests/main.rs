//! The integration tests of the search crate, one binary (C122): each
//! file under `tests/` is a module here, so the shared modules and
//! the crate's own dependencies are compiled and linked once. A file
//! that sets the process environment is its own binary instead
//! (`tools/arch-check.sh`, the layout rule), and is not listed here.

mod common;

mod acceptance;
mod against;
mod answer;
mod build_name;
mod category;
mod class;
mod counts;
mod depth;
mod derive;
mod explain;
mod generated_completion;
mod generated_cross_checks;
mod generated_equivalence;
mod generated_explain;
mod generated_routes;
mod generated_rows;
mod language;
mod membership;
mod moved_world;
mod next;
mod offers;
mod percentile;
mod price;
mod properties;
mod pseudo;
mod ranged;
mod refusal;
mod rows;
mod seat_faults;
mod sockets;
mod surface_faults;
mod tabs;
mod trade_catalogue;
mod trade_decode;
mod trade_dps;
mod trade_forms;
mod trade_oracle;
mod trade_stats;
mod trade_translate;
mod unread;
