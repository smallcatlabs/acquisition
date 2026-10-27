//! The integration tests of the MCP server, one binary (C122): each
//! file under `tests/` is a module here, so the shared modules and
//! the crate's own dependencies are compiled and linked once. A file
//! that sets the process environment is its own binary instead
//! (`tools/arch-check.sh`, the layout rule), and is not listed here.

mod harness;

mod daemon_status;
mod ggg_refusal;
mod journey;
mod mcp_reference;
mod persisted_queue;
mod pricing_tools;
mod provider_default;
mod search_tools;
mod surface_size;
mod tool_refusals;
