use crabe_framework::data::input::InboundData;
use crabe_framework::data::world::TeamColor;
use crabe_protocol::protobuf::vision_packet;
use crabe_protocol::protobuf::vision_packet::{TrackedFrame};
use crate::data::{FilterData};
use crate::pre_filter::PreFilter;

pub struct TrackerFilter;

/// Takes a ball packet and updates its data
/// in the `filter_data` variable.
/// Might cause problems if there is more than one ball.
fn handle_balls(t_frame: &TrackedFrame, filter_data: &mut FilterData) {
    // watch out, there's the TrackedBall type from the proto (1)
    // and our own TrackedBall type (2)
    t_frame.balls.iter().last().map(|tball| {  // (1)
        if let Some(tracked_ball) = &mut filter_data.ball {  // (2)
            if let Some(vel) = &tball.vel {
                tracked_ball.data.velocity = nalgebra::Vector3::new(vel.x as f64, vel.y as f64, vel.z as f64);
            }

            tracked_ball.data.position = nalgebra::Point3::new(tball.pos.x as f64, tball.pos.y as f64, tball.pos.z as f64);
        }
    });
}

fn to_own_team_color(proto_tc: vision_packet::TeamColor) -> Option<TeamColor> {
    match proto_tc {
        vision_packet::TeamColor::Unknown => None,
        vision_packet::TeamColor::Yellow => Some(TeamColor::Yellow),
        vision_packet::TeamColor::Blue => Some(TeamColor::Blue)
    }
}

fn handle_vel(r_team_color: TeamColor, ally_team_color: &TeamColor, tracked_robot: &vision_packet::TrackedRobot, filter_data: &mut FilterData) {
    let rid = &tracked_robot.robot_id.id;
    let opt_vel = if r_team_color == *ally_team_color {
            filter_data.allies.get_mut(&(*rid as u8)).map(|ally_info| &mut ally_info.data.velocity)
        } else {
            filter_data.enemies.get_mut(&(*rid as u8)).map(|ally_info| &mut ally_info.data.velocity)
    };

    if let Some(vel) = opt_vel {
        if let Some(tracked_vel) = &tracked_robot.vel {
            vel.linear = nalgebra::Vector2::new(tracked_vel.x as f64, tracked_vel.y as f64);
        }
        if let Some(tracked_angular_vel) = &tracked_robot.vel_angular {
            vel.angular = *tracked_angular_vel as f64
        }
    }
}

fn handle_position(r_team_color: TeamColor, ally_team_color: &TeamColor, tracked_robot: &vision_packet::TrackedRobot, filter_data: &mut FilterData) {
    let rid = &tracked_robot.robot_id.id;
    let opt_pos = if r_team_color == *ally_team_color {
        filter_data.allies.get_mut(&(*rid as u8)).map(|r| &mut r.data.pose)
    } else {
        filter_data.enemies.get_mut(&(*rid as u8)).map(|r| &mut r.data.pose)
    };
    
    if let Some(pos) = opt_pos {
        pos.position = nalgebra::Point2::new(tracked_robot.pos.x as f64, tracked_robot.pos.y as f64);
        pos.orientation = tracked_robot.orientation as f64
    }
}


fn handle_robots(t_frame: &TrackedFrame, filter_data: &mut FilterData, our_team_color: &TeamColor) {
    t_frame.robots.iter().for_each(|robot| {
        if let Ok(proto_r_team_color) = vision_packet::TeamColor::try_from(robot.robot_id.team_color) {
            // convert packet TeamColor into our own TeamColor
            // todo: maybe this should be elsewhere (unless not performed in an other place)
            if let Some(r_team_color) = to_own_team_color(proto_r_team_color) {
                handle_vel(r_team_color, our_team_color, robot, filter_data);
                handle_position(r_team_color, our_team_color, robot, filter_data);
            }
        }
    });
}

impl PreFilter for TrackerFilter {
    fn step(&mut self, inbound_data: &mut InboundData, team_color: &TeamColor, filter_data: &mut FilterData) {
        inbound_data.tracker_packet.iter().for_each(|packet| {
            if let Some(tracked_frame) = &packet.tracked_frame {
                handle_balls(tracked_frame, filter_data);
                handle_robots(tracked_frame, filter_data, team_color);
            }
        })
    }
}