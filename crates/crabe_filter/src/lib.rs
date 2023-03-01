use chrono::{DateTime, Duration, LocalResult, NaiveDateTime, TimeZone, Utc};
use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, World};
use log::{error, info};
use nalgebra::{Point2, Point3};
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Args)]
pub struct FilterConfig {}

pub type TrackedRobotMap<T> = HashMap<u32, TrackedRobot<T>>;

#[derive(Debug)]
pub struct CamBall {
    pub position: Point3<f32>,
    pub camera_id: u32,
    pub t_capture: DateTime<Utc>,
    pub frame_number: u32,
    pub confidence: f32,
}

pub struct CamRobot {
    pub id: u32,
    pub camera_id: u32,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub t_capture: DateTime<Utc>,
    pub frame_number: u32,
    pub confidence: f32,
}

#[derive(Debug, Default)]
pub struct CamGeometry {
    pub field_length: f32,
    pub field_width: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
    // pub last_update: Instant,
}

pub struct TrackedRobot<T> {
    pub packets: ConstGenericRingBuffer<CamRobot, 64>,
    pub last_update: Instant,
    pub data: Robot<T>,
}

struct TrackedBall {
    pub packets: ConstGenericRingBuffer<CamBall, 64>,
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
    geometry: CamGeometry,
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
                geometry: Default::default(),
            },
            yellow: common_config.yellow,
        })
    }
}

fn track_robot<T: Default>(tracked_robot_map: &mut TrackedRobotMap<T>, id: u32, robot: CamRobot) {
    let tracked_robot = tracked_robot_map.entry(id).or_insert_with(|| TrackedRobot {
        packets: ConstGenericRingBuffer::new(),
        last_update: Instant::now(),
        data: Robot {
            id,
            position: Default::default(),
            orientation: 0.0,
            has_ball: false,
            robot_info: T::default(),
        },
    });

    tracked_robot.packets.push(robot);
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut data: InboundData, world: &mut World) -> Option<World> {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let t_capture =
                    match Utc.timestamp_millis_opt((detection.t_capture * 1000.0) as i64) {
                        LocalResult::Single(dt) => dt,
                        LocalResult::None => {
                            let now_utc = Utc::now();
                            error!("Invalid timestamp, using current time: {}", now_utc);
                            now_utc
                        }
                        LocalResult::Ambiguous(dt_min, dt_max) => {
                            let dt_midpoint = dt_min + (dt_max - dt_min) / 2;
                            error!("Ambiguous timestamp resolved to midpoint: {}", dt_midpoint);
                            dt_midpoint
                        }
                    };
                info!("t_capture: {}", t_capture);

                detection.robots_blue.drain(..).for_each(|r| {
                    if let Some(id) = r.robot_id {
                        if self.yellow {
                            track_robot::<EnemyInfo>(
                                &mut self.filter_data.enemies,
                                id,
                                CamRobot {
                                    id,
                                    camera_id,
                                    position: Point2::new(r.x, r.y),
                                    orientation: r.orientation.unwrap_or(0.0),
                                    t_capture,
                                    frame_number,
                                    confidence: r.confidence,
                                },
                            );
                        } else {
                            track_robot::<AllyInfo>(
                                &mut self.filter_data.allies,
                                id,
                                CamRobot {
                                    id,
                                    camera_id,
                                    position: Point2::new(r.x, r.y),
                                    orientation: r.orientation.unwrap_or(0.0),
                                    t_capture,
                                    frame_number,
                                    confidence: r.confidence,
                                },
                            );
                        }
                    }
                });
                detection.robots_yellow.drain(..).for_each(|r| {
                    if let Some(id) = r.robot_id {
                        if self.yellow {
                            track_robot::<AllyInfo>(
                                &mut self.filter_data.allies,
                                id,
                                CamRobot {
                                    id,
                                    camera_id,
                                    position: Point2::new(r.x, r.y),
                                    orientation: r.orientation.unwrap_or(0.0),
                                    t_capture,
                                    frame_number,
                                    confidence: r.confidence,
                                },
                            );
                        } else {
                            track_robot::<EnemyInfo>(
                                &mut self.filter_data.enemies,
                                id,
                                CamRobot {
                                    id,
                                    camera_id,
                                    position: Point2::new(r.x, r.y),
                                    orientation: r.orientation.unwrap_or(0.0),
                                    t_capture,
                                    frame_number,
                                    confidence: r.confidence,
                                },
                            );
                        }
                    }
                });
                detection.balls.drain(..).for_each(|b| {
                    self.filter_data.ball.packets.push(CamBall {
                        camera_id,
                        position: Point3::new(b.x, b.y, b.z.unwrap_or(0.0)),
                        confidence: b.confidence,
                        frame_number,
                        t_capture,
                    });
                });
            }

            if let Some(mut geometry) = packet.geometry {

                //dbg!(geometry.field);
            }
        });

        None
    }

    fn close(&mut self) {}
}
