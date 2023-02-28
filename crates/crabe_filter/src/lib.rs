use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, World};
use nalgebra::{Point2, Point3};
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Args)]
pub struct FilterConfig {}

pub type TrackedRobotMap<T> = HashMap<u32, TrackedRobot<T>>;

struct CamBall {
    pub camera_id: u32,
    pub position: Point3<f32>,
}

struct CamRobot {
    pub id: usize,
    pub camera_id: u32,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub time: Instant,
    pub frame_number: u32,
}

struct CamField {}

struct TrackedRobot<T> {
    pub packets: ConstGenericRingBuffer<CamRobot, 50>, // TODO: Make circular vector
    pub last_update: Instant,
    pub data: Robot<T>,
}

struct TrackedBall {
    pub packets: ConstGenericRingBuffer<CamBall, 50>, // TODO: Make circular vector
    pub last_update: Instant,
    pub data: Ball,
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

pub struct FilterData {
    allies: TrackedRobotMap<AllyInfo>,
    enemies: TrackedRobotMap<EnemyInfo>,
    ball: TrackedBall,
}

pub trait Filter {}

pub struct FilterPipeline {
    pub filters: Vec<Box<dyn Filter>>,
    pub filter_data: FilterData,
    pub yellow: bool,
}

impl FilterPipeline {
    pub fn with_config_boxed(_config: FilterConfig, common_config: &CommonConfig) -> Box<Self> {
        Box::new(Self {
            filters: vec![],
            filter_data: FilterData {
                allies: Default::default(),
                enemies: Default::default(),
                ball: Default::default(),
            },
            yellow: common_config.yellow,
        })
    }
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut data: InboundData, world: &mut World) -> Option<World> {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let time = detection.t_capture;

                detection.robots_blue.drain(..).for_each(|r| {
                    if let Some(id) = r.robot_id {
                        if self.yellow {
                            self.filter_data
                                .enemies
                                .entry(id)
                                .or_insert_with(|| TrackedRobot {
                                    packets: ConstGenericRingBuffer::new(),
                                    last_update: Instant::now(),
                                    data: Robot {
                                        id,
                                        position: Default::default(),
                                        orientation: 0.0,
                                        has_ball: false,
                                        robot_info: EnemyInfo {},
                                    },
                                });
                        } else {
                            self.filter_data
                                .allies
                                .entry(id)
                                .or_insert_with(|| TrackedRobot {
                                    packets: ConstGenericRingBuffer::new(),
                                    last_update: Instant::now(),
                                    data: Robot {
                                        id,
                                        position: Default::default(),
                                        orientation: 0.0,
                                        has_ball: false,
                                        robot_info: AllyInfo {},
                                    },
                                });
                        }
                    }
                });
                detection.robots_yellow.drain(..).for_each(|r| {
                    if let Some(id) = r.robot_id {
                        if self.yellow {
                            self.filter_data
                                .allies
                                .entry(id)
                                .or_insert_with(|| TrackedRobot {
                                    packets: ConstGenericRingBuffer::new(),
                                    last_update: Instant::now(),
                                    data: Robot {
                                        id,
                                        position: Default::default(),
                                        orientation: 0.0,
                                        has_ball: false,
                                        robot_info: AllyInfo {},
                                    },
                                });
                        } else {
                            self.filter_data
                                .enemies
                                .entry(id)
                                .or_insert_with(|| TrackedRobot {
                                    packets: ConstGenericRingBuffer::new(),
                                    last_update: Instant::now(),
                                    data: Robot {
                                        id,
                                        position: Default::default(),
                                        orientation: 0.0,
                                        has_ball: false,
                                        robot_info: EnemyInfo {},
                                    },
                                });
                        }
                    }
                });
                detection.balls.drain(..).for_each(|b| {
                    self.filter_data.ball.packets.push(CamBall {
                        camera_id,
                        position: Point3::new(b.x, b.y, b.z.unwrap_or(0.0)),
                    })
                });
            }

            if let Some(mut geometry) = packet.geometry {
                dbg!(geometry.field);
            }
        });

        None
    }

    fn close(&mut self) {}
}
