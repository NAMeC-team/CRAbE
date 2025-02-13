//! # CRABE_io - mod tool
//! This module contains the data emitted to external tools,
//! and an implementation that handles connections to those tools.

/// Defines configuration used for setup
mod config;

/// Data and implementation for external tools using a TCP socket
mod server;

pub use config::ToolConfig;
pub use server::ToolServer;
