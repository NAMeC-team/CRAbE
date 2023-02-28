use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, World};
use crabe_protocol::protobuf::vision_packet::SslDetectionRobot;
use nalgebra::{Point2, Point3};
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::collections::HashMap;
use std::time::{Duration, Instant};

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

struct Tracked<T, U> {
    packets: ConstGenericRingBuffer<U, 50>,
    pub data: T,
    pub last_update: Instant,
}

impl<T: Default, U> Default for Tracked<T, U> {
    fn default() -> Self {
        Tracked {
            packets: ConstGenericRingBuffer::new(),
            last_update: Instant::now(),
            data: Default::default(),
        }
    }
}

impl<T: Default, U> Tracked<T, U> {
    fn new(data: T) -> Tracked<T, U> {
        Tracked {
            data,
            ..Default::default()
        }
    }
}

impl<T, U> Tracked<T, U> {
    fn handle_packet(&mut self, packet: U) {
        self.packets.push(packet);
    }

    fn handle_packets(&mut self, packets: impl Iterator<Item = U>) {
        self.packets.extend(packets);
    }
}

type TrackedRobot<T> = Tracked<Robot<T>, CamRobot>;
type TrackedBall = Tracked<Ball, CamBall>;

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

// TODO: Rename function?
fn handle_camera_robots<T: Default>(
    robots: &mut TrackedRobotMap<T>,
    cam_robots: impl Iterator<Item = CamRobot>,
) {
    cam_robots.for_each(|r| {
        let robot = robots.entry(r.id as u32).or_insert_with(|| {
            Tracked::new(Robot {
                id: r.id as u32,
                ..Default::default()
            })
        });

        robot.handle_packet(r);
    })
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut data: InboundData, world: &mut World) {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let time = Duration::try_from_secs_f64(detection.t_capture).unwrap(); // TODO: handle error & check if t_capture is in seconds
                let map_cam_robots = |r| if let Some(id) = r.robot_id {
                    Some(CamRobot {
                        id: id as usize,
                        camera_id,
                        position: Point2::new(r.x / 1000.0, r.y / 1000.0),
                        orientation: r.orientation.unwrap_or(0.),
                        time,
                        frame_number,
                    })
                } else {
                    None
                };

                let yellow = detection.robots_yellow.drain(..).filter_map(map_cam_robots);
                let blue = detection.robots_blue.drain(..).filter_map(map_cam_robots);

                let allies;
                let enemies;

                if self.yellow {
                    allies = yellow;
                    enemies = blue;
                } else {
                    allies = blue;
                    enemies = yellow;
                }

                handle_camera_robots(&mut self.filter_data.allies, allies);
                handle_camera_robots(&mut self.filter_data.enemies, enemies);

                let ball_packets = detection.balls.drain(..).map(|x| CamBall {
                    camera_id,
                    position: Point3::new(b.x, b.y, b.z.unwrap_or(0.0)),
                });

                self.filter_data.ball.handle_packets(ball_packets);
            }

            if let Some(mut geometry) = packet.geometry {
                dbg!(geometry.field);
            }
        });

        None
    }

    fn close(&mut self) {}
}
