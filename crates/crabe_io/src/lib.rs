//! # CRAbE_IO
//!
//! This crate provides tools for input/output operations on the CRAbE project, including:
//!
//! - `serial`: USB serial communication between CRAbE and the MainBoard.
//! - `network`: UDP and websockets communication between CRAbE and the vision system, game_controller, simulation, and viewer.
//!
//! This crate produce some tools to debug I/O operations.

/// The `communication` module provides a set of generic types and functions for network and usb communication.
pub mod communication;
/// This module provides constants related to the `crabe_io` package.
pub mod constants;
/// This module provide implementations
pub mod ssl;
