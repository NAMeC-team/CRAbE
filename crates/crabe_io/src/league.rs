/// This module provides an interface and a struct for communicating with the
/// SSL Game Controller.
pub mod game_controller;

pub mod real;
pub mod simulator;
/// This module provides an interface and a struct for communicating with SSL
/// Vision or the Simulator vision module.
pub mod vision;

mod utils;