use crate::data::output::FeedbackMap;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::vision_packet::{SslWrapperPacket, TrackerWrapperPacket};
use std::fmt::Debug;

/// Represents the data received by the software from external sources and
/// passed through the filters.
#[derive(Debug, Default)]
pub struct InboundData {
    /// Vision packet received by the software. This can come from a simulator
    /// or SSL-Vision.
    pub vision_packet: Vec<SslWrapperPacket>,
    /// Game controller packet received by the software. This can come from
    /// SSL-Game-Controller.
    pub gc_packet: Vec<Referee>,
    /// Filtered vision data provided by tracker implementations
    /// with the ssl-vision-tracker. At the moment, providers
    /// are AutoReferee implementations
    pub tracker_packet: Vec<TrackerWrapperPacket>,
    /// Simulator or USB Packet that provides feedback from the robot,
    /// such as odometry or infrared data.
    pub feedback: FeedbackMap,
}
