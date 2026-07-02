use std::time::Instant;
use log::{error, info, warn};
use nalgebra::{Point2, Vector2};
use crabe_framework::data::output::{Command, CommandMap};
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::{TeamColor, World};
use serde::{Deserialize, Serialize};
use crate::pipeline::Guard;
use crate::rules::speed_to_rob_frame;
use serde_json;
use
use crabe_framework::data::world::game_state::{GameState, RunningState};

pub struct PyRoulxsGuard {
    ctx: zmq::Context,
    req_socket: zmq::Socket,
}

impl Default for PyRoulxsGuard {
    fn default() -> PyRoulxsGuard { PyRoulxsGuard::new() }
}

#[derive(Serialize, Default)]
struct SolverRequest {
    obstacles: Vec<Point2<f64>>,
    rob_pos: Point2<f64>,
    alpha: f64,
    v_nom: Point2<f64>,
}

#[derive(Deserialize)]
struct SolverResponse {
    optimal_v: Vector2<f64>
}

impl PyRoulxsGuard {
    pub fn new() -> Self {
        let ctx = zmq::Context::new();
        let req_socket = ctx.socket(zmq::REQ).unwrap();
        req_socket.connect("tcp://localhost:5555").unwrap();
        Self {
            req_socket,
            ctx,
        }
    }
}

fn avoid_allies_and_enemies(world: &World, obstacles: &mut Vec<Point2<f64>>, id: u8) {
    let mut obstacles: Vec<Point2<f64>> = world.allies_bot.iter()
        .filter(|(other_id, _)| id != **other_id)
        .map(|(_, r)| r.pose.position)
        .collect();
    let enemy_poses: Vec<Point2<f64>> = world.enemies_bot.iter().map(|(_, r)| r.pose.position).collect();
    obstacles.extend(enemy_poses);
}

fn avoid_ball(world: &World, obstacles: &mut Vec<Point2<f64>>, cmd: &Command) {
    if let Some(ball) = &world.ball {
        if cmd.avoid_ball {
            obstacles.push(ball.position_2d())
        }
    }
}


fn avoidance_gamestate(world: &World, obstacles: &mut Vec<Point2<f64>>, cmd: &Command) {
    match world.data.ref_orders.state {
        GameState::Halted(_) | GameState::Stopped(_) => {
            if let Some(ball) = &world.ball {
                obstacles.push(ball.position_2d());
            }
        }
        GameState::Running(running_state) => {
            match running_state {
                RunningState::GoalKick(tc) | RunningState::Penalty(tc) | RunningState::FreeKick(tc) |
                RunningState::CornerKick(tc) | RunningState::KickOff(tc) => {
                    if let Some(ball) = &world.ball {
                        obstacles.push(ball.position_2d())
                    }
                }
                _ => {}
            };
        },
    }
}

impl Guard for PyRoulxsGuard {
    fn guard(&mut self, world: &World, commands: &mut CommandMap, _tools_commands: &mut ToolCommands) {
        commands.iter_mut().for_each(|(id, cmd)| {
            // info!("Robot {:?} | Preparing data to send", id);

            let mut obstacles: Vec<Point2<f64>> = vec![];
            avoid_allies_and_enemies(world, &mut obstacles, *id);
            avoid_ball(&world, &mut obstacles, &cmd);
            avoidance_gamestate(&world, &mut obstacles, &cmd);
            
            let rob_info = world.allies_bot.get(&id).unwrap(); // safe unwrap here
            let req = SolverRequest {
                obstacles,
                rob_pos: rob_info.pose.position,
                alpha: 9.,
                v_nom: Point2::new(cmd.forward_velocity as f64, cmd.left_velocity as f64),
            };
            if let Ok(json_string) = serde_json::to_string(&req) {
                let start = Instant::now();
                self.req_socket.send(&json_string, 0).unwrap();
                if let Ok(Ok(resp)) = self.req_socket.recv_string(0) {
                    if let Ok(solver_resp) = serde_json::from_str::<SolverResponse>(resp.as_str()) {
                        // trust me, safe unwrap
                        // you send a cmd so the robot exists lol

                        let speed_robot = speed_to_rob_frame(solver_resp.optimal_v, rob_info);
                        // let speed_robot = speed_to_rob_frame(Vector2::new(cmd.forward_velocity as f64, cmd.left_velocity as f64), rob_info);
                        cmd.forward_velocity = speed_robot.x as f32;
                        cmd.left_velocity = speed_robot.y as f32;

                        // info!("Robot {:?} | Time : {:?} | Parsed solver response and updated speed", id, Instant::now() - start);
                    }
                } else {
                    warn!("Failed to parse response from solver")
                }
            } else {
                error!("Failed serializing")
            }
        })
    }
}