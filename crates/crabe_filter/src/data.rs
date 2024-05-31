pub mod camera;

use crate::constant;
use crate::data::camera::{CamBall, CamGeometry, CamRobot};
use chrono::{DateTime, Utc};
use constant::PACKET_BUFFER_SIZE;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot};
use ringbuffer::ConstGenericRingBuffer;
use std::collections::HashMap;
use std::time::Instant;
use crabe_framework::data::referee::Referee;

#[derive(Clone, Debug)]
pub struct FrameInfo {
    pub camera_id: u32,
    pub frame_number: u32,
    pub t_capture: DateTime<Utc>,
}

pub type TrackedRobotMap<T> = HashMap<u8, TrackedRobot<T>>;

/// Collection of transformed data from external
/// sources into our own structures
pub struct FilterData {
    /// Map associating a robot id to its data, for allies only
    pub allies: TrackedRobotMap<AllyInfo>,
    /// Map associating a robot id to its data, for enemies only
    pub enemies: TrackedRobotMap<EnemyInfo>,
    /// Data about the ball on the field
    pub ball: TrackedBall,
    /// Information on the field geometry
    pub geometry: CamGeometry,
    /// Game controller events
    pub referee: Vec<Referee>,
}

impl Default for FilterData {
    fn default() -> Self {
        FilterData {
            allies: Default::default(),
            enemies: Default::default(),
            ball: Default::default(),
            geometry: Default::default(),
                referee: vec![],
        }
    }
}

/// Contains data bout a robot detected on the field
pub struct TrackedRobot<T> {
    pub packets: ConstGenericRingBuffer<CamRobot, PACKET_BUFFER_SIZE>,
    pub data: Robot<T>,
    pub last_update: DateTime<Utc>,
}

impl<T: Default> Default for TrackedRobot<T> {
    fn default() -> Self {
        TrackedRobot {
            packets: ConstGenericRingBuffer::new(),
            data: Robot::<T>::default(),
            last_update: Utc::now(),
        }
    }
}

/// Contains data about the ball detected on the field
/// Note that there might be multiple instances of this
/// struct in the system (the vision is able to watch
/// multiple balls on the field)
pub struct TrackedBall {
    pub packets: ConstGenericRingBuffer<CamBall, PACKET_BUFFER_SIZE>,
    pub data: Ball,
    pub last_update: Instant,
}

impl Default for TrackedBall {
    fn default() -> Self {
        Self {
            packets: ConstGenericRingBuffer::new(),
            last_update: Instant::now(),
            data: Default::default(),
        }
    }
}
