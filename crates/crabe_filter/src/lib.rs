use clap::Args;
use crabe_framework::component::FilterComponent;
use crabe_framework::config::CommonConfig;
use crabe_framework::constant::MAX_ROBOTS;
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, TeamColor, World};
use nalgebra::{Point2, Point3};
use std::time::Instant;

#[derive(Args)]
pub struct FilterConfig {}

struct CamBall {
    pub camera_id: usize,
    pub position: Point3<f32>,
}

struct CamRobot {
    pub id: usize,
    pub camera_id: usize,
    pub position: Point2<f32>,
    pub orientation: f32,
    pub time: Instant,
    pub frame_number: u32,
}

struct CamField {}

struct TrackedRobot<T> {
    pub history: Vec<CamRobot>, // TODO: Make circular vector
    pub last_update: Instant,
    pub data: Robot<T>,
}

struct TrackedBall {
    pub history: Vec<CamBall>, // TODO: Make circular vector
    pub last_update: Instant,
    pub data: Ball,
}

pub struct FilterData {
    allies: [Option<TrackedRobot<AllyInfo>>; MAX_ROBOTS], // TODO: Use HASHMAP ?
    enemies: [Option<TrackedRobot<EnemyInfo>>; MAX_ROBOTS], // TODO: Use HASHMAP ?
    ball: Option<TrackedBall>,                            // TODO: Remove option ?
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
    fn step(&mut self, mut data: InboundData) -> Option<World> {
        data.vision_packet.drain(..).for_each(|packet| {
            if let Some(mut detection) = packet.detection {
                let camera_id = detection.camera_id;
                let frame_number = detection.frame_number;
                let time = detection.t_capture;

                detection.robots_blue.drain(..).for_each(|r| {});
                detection.robots_yellow.drain(..).for_each(|r| {});
                detection.balls.drain(..).for_each(|b| {});
            }

            if let Some(mut geometry) = packet.geometry {
                dbg!(geometry.field);
            }
        });

        None
    }

    fn close(&mut self) {}
}
