use std::f64::consts::PI;
use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::{matrix, Isometry2, LpNorm, Matrix2, Norm, Point2, Rotation2, Vector2};
use log::{error};
use quadprog::solve_qp;

pub struct RoulxsGuard {
    alpha: f64
}

impl Default for RoulxsGuard {
    fn default() -> RoulxsGuard {
        Self { alpha: 9. }
    }
}

const FIXED_OBSTACLE: Point2<f64> = Point2::new(0., 0.);
const FIXED_OBS_RADIUS: f64 = 0.5;

const SQ_POSITIVE_CENTER: Point2<f64> = Point2::new(-4., 0.5);
const SQ_NEGATIVE_CENTER: Point2<f64> = Point2::new(-4., -0.5);

impl RoulxsGuard {
    fn new(alpha: f64) -> Self {
        if alpha < 0.0 {
            error!("CBF alpha coefficient must be positive. Optimization will go wrong !");
        }
        Self { alpha }
    }

    /// Generate pair of A and bvec items to avoid a single obstacle
    fn avoid(&self, moving_bot_pos: &Point2<f64>, obs_pos: &Point2<f64>) -> (Vec<f64>, f64) {
        let to_obs = obs_pos - moving_bot_pos;
        (
            vec![obs_pos.x, obs_pos.y],
            (self.alpha / 2.) * (to_obs.norm_squared() - (PI / 6.))
        )
    }
    
    fn zeroing_cbf(&self, robot_location: &Point2<f64>, v_nom: &Vector2<f64>, obstacles: Vec<&Point2<f64>>) -> Vector2<f64>{
        // OSQP solver parameters
        let mut Q= [2., 0., 0., 2.];
        let q_vec = -2. * v_nom;
        let c = [q_vec.x, q_vec.y];
        
        let mut A: Vec<f64> = Vec::new();
        let mut bvec: Vec<f64> = Vec::new();
        
        // generate one for each obstacle
        obstacles.iter().for_each(|obs| {
            let (A_items, b_item) = self.avoid(robot_location, obs);
            A.extend(A_items);
            bvec.push(b_item);
        });
        
        // for a single obstacle
        // let vec_diff = FIXED_OBSTACLE - robot_location;
        // let A = [vec_diff.x, vec_diff.y];
        // let bvec = [(alpha / 2.) * vec_diff.norm_squared() - FIXED_OBS_RADIUS];
        
        if let Ok(solution) = solve_qp(&mut Q, &c, &A, &bvec, 0, false) {
            Vector2::new(solution.sol[0], solution.sol[1])
        } else {
            v_nom.clone()
        }
    }
}

fn speed_to_rob_frame(v: Vector2<f64>, rob_info: &Robot<AllyInfo>) -> Vector2<f64> {
    // let ti = Isometry2::new(Vector2::zeros(), rob_info.pose.<orientation);
    let o = rob_info.pose.orientation;
    let rot = matrix![o.cos(), -o.sin();
                                 o.sin(), o.cos()];
    rot.transpose() * v
}

impl Guard for RoulxsGuard {
    fn guard(
        &mut self,
        world: &World,
        commands: &mut CommandMap,
        _tool_commands: &mut ToolCommands,
    ) {
        commands.iter_mut().for_each(|(key, cmd)| {
            if let Some(rob_info) = world.allies_bot.get(key) {
                let v_nom: Vector2<f64> = Vector2::new(cmd.forward_velocity as f64, cmd.left_velocity as f64);
                let v_optimal= self.zeroing_cbf(&rob_info.pose.position, &v_nom, vec![&SQ_POSITIVE_CENTER, &SQ_NEGATIVE_CENTER]);
                let rob_v_optimal = speed_to_rob_frame(v_optimal, rob_info);
                // dbg!(&v_nom);
                // dbg!(&v_optimal);
                // dbg!(&rob_v_optimal);
                // panic!();
                cmd.forward_velocity = rob_v_optimal.x as f32;
                cmd.left_velocity = rob_v_optimal.y as f32;
            }
        })
    }
}
