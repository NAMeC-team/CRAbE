use chrono::{DateTime, Duration, LocalResult, NaiveDateTime, TimeZone, Utc};
use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, World};
use log::{error, info};
use crabe_protocol::protobuf::vision_packet::SslDetectionRobot;
use nalgebra::{Point2, Point3};
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Args)]
pub struct FilterConfig {}

#[derive(Debug)]
struct CamBall {
    pub position: Point3<f32>,
    pub camera_id: u32,
    pub t_capture: DateTime<Utc>,
    pub frame_number: u32,
    pub confidence: f32,
}

#[derive(Debug)]
struct CamRobot {
    pub id: u32,
    pub camera_id: u32,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub t_capture: DateTime<Utc>,
    pub frame_number: u32,
    pub confidence: f32,
}

pub enum TeamColor {
    Yellow,
    Blue
}

#[derive(Debug, Default)]
struct CamGeometry {
    pub field_length: f32,
    pub field_width: f32,
    pub goal_width: f32,
    pub goal_depth: f32,
    // pub last_update: Instant,
}

struct Tracked<T, U> {
    pub packets: ConstGenericRingBuffer<U, 50>,
    pub data: T,
    pub last_update: Instant,
}

impl<T: Default, U> Default for Tracked<T, U> {
    fn default() -> Self {
        Self {
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

pub type TrackedRobot<T> = Tracked<Robot<T>, CamRobot>;
pub type TrackedBall = Tracked<Ball, CamBall>;

pub type TrackedRobotMap<T> = HashMap<u32, TrackedRobot<T>>;

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
    pub team_color: TeamColor,
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
            team_color: if common_config.yellow { TeamColor::Yellow } else { TeamColor::Blue },
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

        robot.packets.push(r);
    })
}

impl FilterComponent for FilterPipeline {
    fn step(&mut self, mut data: InboundData, world: &mut World) {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let t_capture = match Utc.timestamp_millis_opt((detection.t_capture * 1000.0) as i64) {
                    LocalResult::Single(dt) => dt,
                    LocalResult::None => {
                        let now_utc = Utc::now();
                        error!("Invalid timestamp, using current time: {}", now_utc);
                        now_utc
                    },
                    LocalResult::Ambiguous(dt_min, dt_max) => {
                        let dt_midpoint = dt_min + (dt_max - dt_min) / 2;
                        error!("Ambiguous timestamp resolved to midpoint: {}", dt_midpoint);
                        dt_midpoint
                    }
                };

                let map_robot_packets = |r: SslDetectionRobot| if let Some(id) = r.robot_id {
                    Some(CamRobot {
                        id: id as usize,
                        camera_id,
                        position: Point2::new(r.x / 1000.0, r.y / 1000.0),
                        orientation: r.orientation.unwrap_or(0.),
                        time: t_capture,
                        frame_number,
                        confidence: r.confidence
                    })
                } else {
                    None
                };

                let yellow = detection.robots_yellow.drain(..).filter_map(map_robot_packets);
                let blue = detection.robots_blue.drain(..).filter_map(map_robot_packets);

                let allies;
                let enemies;

                match self.team_color {
                    TeamColor::Yellow => {
                        allies = yellow;
                        enemies = blue;
                    },

                    TeamColor::Blue => {
                        allies = blue;
                        enemies = yellow;
                    }
                }

                handle_camera_robots(&mut self.filter_data.allies, allies);
                handle_camera_robots(&mut self.filter_data.enemies, enemies);

                let ball_packets = detection.balls.drain(..).map(|b| CamBall {
                    camera_id,
                    frame_number,
                    position: Point3::new(b.x, b.y, b.z.unwrap_or(0.0)),
                    t_capture,
                    confidence: b.confidence
                });

                self.filter_data.ball.packets.extend(ball_packets);
            }

            if let Some(mut geometry) = packet.geometry {

                //dbg!(geometry.field);
            }
        });
    }

    fn close(&mut self) {}
}
