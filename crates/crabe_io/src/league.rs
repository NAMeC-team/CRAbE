/// This module provides an interface and a struct for communicating with the
/// SSL Game Controller.
pub mod game_controller;

pub mod real;
pub mod simulator;
/// This module provides an interface and a struct for communicating with SSL
/// Vision or the Simulator vision module.
pub mod vision;

/// Listens to the tracker port for filtered vision packets,
/// with addtional data such as velocity, better ball position, etc...
/// Refer to the [ssl-vision-tracker](https://ssl.robocup.org/league-software/#ssl-vision-tracker-protocol)
/// protocol for more details
pub mod tracker;

mod utils;