use crate::data::FilterData;
use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;

/// Filters vision data coming from
/// the cameras, into structures of our own
pub mod vision;

/// Filters data from the game controller, who decides
/// the current game state, fouls, half times and timeouts
pub mod game_controller;

/// Common functions used by both modules
mod common;

/// Responsible for converting incoming data
/// from external sources (vision & game controller),
/// stored in the field `inbound_data`,
/// into data structures of our implementation
/// that can be manipulated further, in the field `filter_data`.
///
/// Similar to an outlet adapter
///
/// Parameters :
/// - inbound_data      Data received from external sources
/// - team_color        The color of our own team
/// - filter_data       Result of the processed incoming data
//                      Must be modified when transforming
//                      the incoming data
pub trait PreFilter {
    fn step(
        &mut self,
        inbound_data: &mut InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    );
}
