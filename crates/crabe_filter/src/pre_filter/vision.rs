use crate::data::FilterData;
use crate::PreFilter;

use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::world::{TeamColor};

mod detection {
    use chrono::{DateTime, LocalResult, TimeZone, Utc};
    use log::error;
    use crabe_framework::data::world::{TeamColor};
    use crabe_protocol::protobuf::vision_packet::SslDetectionFrame;
    use crate::data::{FilterData, FrameInfo};

    mod robot {
        use nalgebra::Point2;
        use ringbuffer::RingBufferWrite;
        use crabe_framework::data::world::{AllyInfo, EnemyInfo, Robot, TeamColor};
        use crabe_protocol::protobuf::vision_packet::SslDetectionRobot;
        use crate::data::{CamRobot, FrameInfo, TrackedRobot, TrackedRobotMap};

        pub struct RobotDetectionInfo<'a> {
            pub detected_blue: &'a [SslDetectionRobot],
            pub detected_yellow: &'a [SslDetectionRobot],
            pub tracked_allies: &'a mut TrackedRobotMap<AllyInfo>,
            pub tracked_enemies: &'a mut TrackedRobotMap<EnemyInfo>,
        }

        fn track_robots<T: Default>(
            robots: &mut TrackedRobotMap<T>,
            cam_robots: impl Iterator<Item=CamRobot>,
        ) {
            cam_robots.for_each(|r| {
                let robot = robots.entry(r.id as u32).or_insert_with(|| TrackedRobot {
                    data: Robot {
                        id: r.id as u32,
                        ..Default::default()
                    },
                    ..Default::default()
                });

                robot.packets.push(r);
            })
        }


        pub fn detect_robots(detection: &mut RobotDetectionInfo, frame: &FrameInfo, team_color: &TeamColor) {
            let map_packet = |r: &SslDetectionRobot| if let Some(id) = r.robot_id {
                Some(CamRobot {
                    id,
                    frame_info: frame.clone(),
                    position: Point2::new(r.x, r.y),
                    orientation: r.orientation.unwrap_or(0.),
                    confidence: r.confidence,
                })
            } else {
                None
            };

            let yellow = detection.detected_yellow
                .iter()
                .filter_map(map_packet);
            let blue = detection.detected_blue
                .iter()
                .filter_map(map_packet);

            let allies;
            let enemies;

            match team_color {
                TeamColor::Yellow => {
                    allies = yellow;
                    enemies = blue;
                }

                _ => {
                    allies = blue;
                    enemies = yellow;
                }
            }

            track_robots(detection.tracked_allies, allies);
            track_robots(detection.tracked_enemies, enemies);
        }
    }

    mod ball {
        use nalgebra::Point3;
        use crabe_protocol::protobuf::vision_packet::SslDetectionBall;
        use crate::data::{CamBall, FrameInfo, TrackedBall};

        pub struct BallDetectionInfo<'a> {
            pub detected: &'a [SslDetectionBall],
            pub tracked: &'a mut TrackedBall,
        }

        pub fn detect_balls(detection: &mut BallDetectionInfo, frame: &FrameInfo) {
            let ball_packets = detection.detected.iter().map(|b| CamBall {
                frame_info: frame.clone(),
                position: Point3::new(b.x, b.y, b.z.unwrap_or(0.0)),
                confidence: b.confidence,
            });

            detection.tracked.packets.extend(ball_packets);
        }
    }

    fn create_date_time(t_capture: f64) -> DateTime<Utc> {
        match Utc.timestamp_opt((t_capture) as i64, 0) {
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
        }
    }

    pub fn handle_detection(
        detection: &SslDetectionFrame,
        filter_data: &mut FilterData,
        team_color: &TeamColor,
    ) {
        let frame_info = FrameInfo {
            camera_id: detection.camera_id,
            frame_number: detection.frame_number,
            t_capture: create_date_time(detection.t_capture),
        };

        let mut robot_detection_info = robot::RobotDetectionInfo {
            detected_yellow: &detection.robots_yellow,
            detected_blue: &detection.robots_blue,
            tracked_allies: &mut filter_data.allies,
            tracked_enemies: &mut filter_data.enemies,
        };

        robot::detect_robots(&mut robot_detection_info, &frame_info, team_color);

        let mut ball_detection_info = ball::BallDetectionInfo {
            detected: &detection.balls,
            tracked: &mut filter_data.ball,
        };

        ball::detect_balls(&mut ball_detection_info, &frame_info)
    }
}

mod geometry {
    use crabe_protocol::protobuf::vision_packet::SslGeometryData;
    use crate::data::{CamGeometry, FilterData};

    pub fn handle_geometry(geometry: &SslGeometryData, filter_data: &mut FilterData) {
        let cam_geometry = CamGeometry {
            field_length: geometry.field.field_length as f32,
            field_width: geometry.field.field_width as f32,
            goal_width: geometry.field.goal_width as f32,
            goal_depth: geometry.field.goal_depth as f32,
        };

        filter_data.geometry = cam_geometry;
    }
}

pub struct VisionFilter;

impl VisionFilter {
    pub fn new() -> VisionFilter {
        VisionFilter
    }
}

impl PreFilter for VisionFilter {
    fn step(
        &mut self,
        inbound_data: &InboundData,
        team_color: &TeamColor,
        filter_data: &mut FilterData,
    ) {
        inbound_data.vision_packet.iter().for_each(|packet| {
            if let Some(detection) = packet.detection.as_ref() {
                detection::handle_detection(detection, filter_data, team_color);
            }

            if let Some(geometry) = packet.geometry.as_ref() {
                geometry::handle_geometry(geometry, filter_data);
            }
        });
    }
}
