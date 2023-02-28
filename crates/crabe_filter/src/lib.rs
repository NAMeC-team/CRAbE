use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, World};
use nalgebra::{Point2, Point3};
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use crabe_protocol::protobuf::vision_packet::SslDetectionRobot;

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
    pub time: Duration,
    pub frame_number: u32,
}

struct CamField {}

struct TrackedRobot<T> {
    pub packets: ConstGenericRingBuffer<CamRobot, 50>,
    // TODO: Make circular vector
    pub last_update: Instant,
    pub data: Robot<T>,
}

struct TrackedBall {
    pub packets: ConstGenericRingBuffer<CamBall, 50>,
    // TODO: Make circular vector
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

fn map_camera_packets(
    robots: impl Iterator<Item=SslDetectionRobot>,
    camera_id: u32,
    frame_number: u32,
    time: f64,
) -> impl Iterator<Item=CamRobot> {
    robots.filter_map(move |r| {
        if let Some(id) = r.robot_id {
            Some(
                CamRobot {
                    id: id as usize,
                    camera_id,
                    position: Point2::new(r.x / 1000.0, r.y / 1000.0),
                    orientation: r.orientation.unwrap_or(0.),
                    time: Duration::from_secs_f64(time), // TODO: Check unit
                    frame_number,
                }
            )
        } else {
            None
        }
    })
}

// TODO: Rename function?
fn handle_camera_packets<T: Default>(robots: &mut TrackedRobotMap<T>, cam_robots: impl Iterator<Item=CamRobot>) {
    cam_robots.for_each(|r| {
        let robot = robots.entry(r.id as u32)
            .or_insert_with(|| TrackedRobot {
                packets: ConstGenericRingBuffer::new(),
                last_update: Instant::now(),
                data: Robot {
                    id: r.id as u32,
                    ..Default::default()
                },
            });

        robot.packets.push(r);
    })
}


impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut data: InboundData, world: &mut World) -> Option<World> {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let time = detection.t_capture;
                let yellow = map_camera_packets(detection.robots_yellow.drain(..), camera_id, frame_number, time);
                let blue = map_camera_packets(detection.robots_blue.drain(..), camera_id, frame_number, time);
                let allies;
                let enemies;
                if self.yellow {
                    allies = yellow;
                    enemies = blue;
                } else {
                    allies = blue;
                    enemies = yellow;
                }

                handle_camera_packets(&mut self.filter_data.allies, allies);
                handle_camera_packets(&mut self.filter_data.enemies, enemies);

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
