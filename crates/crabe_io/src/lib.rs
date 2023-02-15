//! # CRAbE_IO
//!
//! This crate handles all I/O operations on the project such as :
//! - USB Serial communication (CRAbE <> MainBoard)
//! - UDP Socket communication (CRAbE <> Vision / Simulation / Python)
//! - Websockets communication (CRAbE <> Viewer)
//!
//! This crate produce some tools to debug I/O operations.
//! - `vision_log` : Log packets receive of the vision project.

/// TODO: Make network documentation
pub mod network;
/// Modules which some constants related to the package `CRAbE_io`.
mod constants;
