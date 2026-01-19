use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use nalgebra::{Point2, Vector2};
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
const FIXED_OBS_RADIUS: f64 = 0.3;

impl RoulxsGuard {
    fn new(alpha: f64) -> Self {
        if alpha < 0.0 {
            error!("CBF alpha coefficient must be positive. Optimization will go wrong !");
        }
        Self { alpha }
    }
    
    fn zeroing_cbf(&self, robot_location: &Point2<f64>, v_nom: &Vector2<f64>) -> Vector2<f64>{
        let alpha = self.alpha;

        // OSQP solver parameters
        let mut Q= [2., 0., 0., 2.];
        let q_vec = -2. * v_nom;
        let c = [q_vec.x, q_vec.y];

        let vec_diff = FIXED_OBSTACLE - robot_location;
        let A = [vec_diff.x, vec_diff.y];
        let bvec = [(alpha / 2.) * vec_diff.norm_squared() - FIXED_OBS_RADIUS];
        
        if let Ok(solution) = solve_qp(&mut Q, &c, &A, &bvec, 0, false) {
            Vector2::new(solution.sol[0], solution.sol[1])
        } else {
            v_nom.clone()
        }
    }
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
                let v_optimal = self.zeroing_cbf(&rob_info.pose.position, &v_nom);
                cmd.forward_velocity = v_optimal.x as f32;
                cmd.left_velocity = v_optimal.y as f32;
            }
        })
    }
}
