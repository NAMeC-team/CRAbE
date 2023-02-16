//! # CRAbE_IO
//!
//! This crate handles all I/O operations on the project such as :
//! - USB Serial communication (CRAbE <> MainBoard)
//! - UDP Socket communication (CRAbE <> Vision / Simulation / Python)
//! - Websockets communication (CRAbE <> Viewer)
//!
//! This crate produce some tools to debug I/O operations.
//! - `vision_log` : Log packets receive of the vision project.

/// This module provides constants related to the `crabe_io` package.
pub mod constants;
/// The `network` module provides a set of types and functions for network communication.
pub mod network;
