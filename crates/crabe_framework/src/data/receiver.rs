use crate::data::output::{Feedback, FeedbackMap};
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use std::fmt::Debug;
use crate::constant::MAX_ROBOTS;

/// Represents the data received by the software from external sources and passed through the filters.
#[derive(Debug)]
pub struct InboundData {
    /// Vision packet received by the software. This can come from a simulator or SSL-Vision.
    pub vision_packet: Vec<SslWrapperPacket>,
    /// Game controller packet received by the software. This can come from SSL-Game-Controller.
    pub gc_packet: Vec<Referee>,
    /// TODO:
    pub feedback: FeedbackMap,
}

impl Default for InboundData {
    fn default() -> Self {
        Self {
            vision_packet: Vec::new(),
            gc_packet: Vec::new(),
            feedback: FeedbackMap{ feedbacks: Default::default() }
        }
    }
}
