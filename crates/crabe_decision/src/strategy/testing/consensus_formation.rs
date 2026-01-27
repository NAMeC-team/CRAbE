use std::cmp::Ordering;
use log::error;
use nalgebra::{Isometry2, Point2, Vector2};
use crabe_framework::data::output::Command;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crate::action::ActionWrapper;
use crate::action::order_raw::RawOrder;
use crate::strategy::Strategy;

pub struct ConsensusFormation {
    epsilon: f32,
    adj_graph: Vec<Vec<i32>>,
    // Diagonal matrix with in-degree of the graph
    out_degree_graph: Vec<Vec<i32>>,
    max_in_degree: usize,
    offsets_x: Vec<f64>,
    offsets_y: Vec<f64>
}

impl ConsensusFormation {
    pub fn new() -> Self {
        Self {
            epsilon: 0.9,
            adj_graph: Vec::from([
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 0, 0],
            ]),
            out_degree_graph: Vec::from([
                vec![1, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1],
            ]),
            max_in_degree: 1,
            offsets_x: vec![1., 1., -1., -1.],
            offsets_y: vec![1., -1., -1., 1.],
        }
    }
    
    // fn laplacian(&self) -> Vec<Vec<i32>> {
    //     let mut res: Vec<Vec<i32>> = Vec::from([Vec::new(), Vec::new(), Vec::new()]);
    //     for i in 0..self.adj_graph.len() {
    //         for j in 0..self.out_degree_graph.len() {
    //             res[i][j] = self.out_degree_graph[i][j] - self.adj_graph[i][j];
    //         }
    //     }
    //     
    //     res
    // }
    
    fn is_neighbour(&self, node_i: usize, neighbour_j: usize) -> bool {
        self.adj_graph[node_i][neighbour_j] == 1
    }
    
    fn discrete_consensus_cfunc(&self, X0: Vec<f64>, offsets: &Vec<f64>) -> Vec<f64> {
        if !(self.epsilon * (self.max_in_degree as f32) < 1.) {
            panic!("epsilon * delta value superior to 1, change epsilon")
        }
        
        let mut res: Vec<f64> = Vec::new();
        for node_i in 0..self.adj_graph.len() {
            let mut s = 0.;
            for neighbour_j in 0..self.adj_graph.len() {
                if self.is_neighbour(node_i, neighbour_j) {
                    s += X0[neighbour_j] - X0[node_i];
                    s -= offsets[node_i]
                }
            }
            res.push((self.epsilon as f64) * s);
        }
        
        res
    }
}

fn frame(x: f64, y: f64, orientation: f64) -> Isometry2<f64> {
    Isometry2::new(Vector2::new(x, y), orientation)
}

fn frame_inv(frame: Isometry2<f64>) -> Isometry2<f64> {
    frame.inverse()
}

fn robot_frame(robot: &Robot<AllyInfo>) -> Isometry2<f64> {
    frame(
        robot.pose.position.x,
        robot.pose.position.y,
        robot.pose.orientation,
    )
}

impl Strategy for ConsensusFormation {
    fn name(&self) -> &'static str { "ConsensusFormation" }

    fn step(&mut self, world: &World, _tools_data: &mut ToolData, action_wrapper: &mut ActionWrapper) -> bool {
        let ids = vec![0, 1, 2, 3];
        let mut robots: Vec<&Robot<AllyInfo>> = world.allies_bot
            .iter()
            .filter_map(|(id, ally_info)| {
                if ids.contains(id) {
                    Some(ally_info)
                } else {
                    None
                }
            }).collect();
        
        robots.sort();
        if robots.len() == ids.len() {
            action_wrapper.clear_all();
            // yes, vec iterator is ordered
            let positions: Vec<&Point2<f64>> = robots
                .iter()
                .map(|ally_info| &ally_info.pose.position)
                .collect();
            
            let x_pos: Vec<f64> = positions.iter().map(|&p| p.x).collect();
            let y_pos: Vec<f64> = positions.iter().map(|&p| p.y).collect();
            
            let speeds_x = self.discrete_consensus_cfunc(x_pos, &self.offsets_x);
            let speeds_y = self.discrete_consensus_cfunc(y_pos, &self.offsets_y);
            
            let speeds: Vec<Point2<f64>> = speeds_x
                .iter().zip(speeds_y.iter())
                .map(|(x, y)| Point2::new(*x, *y))
                .collect();
            
            ids.iter().zip(speeds.iter())
                .for_each(|(id, speed)| {
                    if let Some(rob) = world.allies_bot.get(id) {
                        let ti = Isometry2::new(Vector2::zeros(), rob.pose.orientation).inverse();
                        let speed_ti = ti * speed;
                        action_wrapper.push(*id, RawOrder::new(Command {
                            forward_velocity: speed_ti.x as f32,
                            left_velocity: speed_ti.y as f32,
                            ..Command::default()
                    }))                                            
                }
            });
        }
        
        false
    }
}