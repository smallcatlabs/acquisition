//! The integration tests of the protocol crate, one binary (C122): each
//! file under `tests/` is a module here, so the shared modules and
//! the crate's own dependencies are compiled and linked once. A file
//! that sets the process environment is its own binary instead
//! (`tools/arch-check.sh`, the layout rule), and is not listed here.

mod line_endings;
mod wire;
