use std::time::Instant;
use log::{error, info, warn};
use nalgebra::{Point2, Vector2};
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use serde::{Deserialize, Serialize};
use crate::pipeline::Guard;
use crate::rules::speed_to_rob_frame;
use serde_json;

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

impl Guard for PyRoulxsGuard {
    fn guard(&mut self, world: &World, commands: &mut CommandMap, _tools_commands: &mut ToolCommands) {
        commands.iter_mut().for_each(|(id, cmd)| {
            // info!("Robot {:?} | Preparing data to send", id);
            let rob_info = world.allies_bot.get(&id).unwrap(); // safe unwrap here
            let req = SolverRequest {
                obstacles: world.allies_bot.iter()
                    .filter(|(other_id, r)| *id != **other_id)
                    .map(|(_, r)| r.pose.position)
                    .collect(),
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
                        
                        // let speed_robot = speed_to_rob_frame(solver_resp.optimal_v, rob_info);
                        let speed_robot = speed_to_rob_frame(Vector2::new(cmd.forward_velocity as f64, cmd.left_velocity as f64), rob_info);
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