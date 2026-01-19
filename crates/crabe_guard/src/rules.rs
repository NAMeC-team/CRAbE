use crate::pipeline::Guard;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::World;
use nalgebra::{Point2, Vector2};
use log::{error, info, warn};
use osqp::{CscMatrix, Status};

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
        let P= &[[2., 0.], [0., 2.]];
        let q_vec = -2. * v_nom;
        let q = &[q_vec.x, q_vec.y];

        let vec_diff = FIXED_OBSTACLE - robot_location;
        let A = &[
            [vec_diff.x, vec_diff.y],
        ];
        let u = &[(alpha / 2.) * vec_diff.norm_squared() - FIXED_OBS_RADIUS];
        let l = &[0.];

        let P = CscMatrix::from(P).into_upper_tri();

        let settings = osqp::Settings::default().verbose(false);

        let mut prob = osqp::Problem::new(P, q, A, l, u, &settings).expect("Failed to setup problem");
        let result = prob.solve();

        let mut v_optimal = v_nom.clone(); // optimal speed computed
        match result {
            Status::Solved(sol) => {
                let s = sol.x();
                let cmd_diff = (v_nom.x - s[0]) + (v_nom.y - s[1]);
                if s.len() == 2 {
                    if cmd_diff > 1e-3 {
                        info!("Optimal value computed");
                        v_optimal = Vector2::new(s[0], s[1]);
                        dbg!(&v_optimal);
                    }
                } else {
                    warn!("Not enough values for solution?")
                }
            }
            Status::SolvedInaccurate(_) |
            Status::MaxIterationsReached(_) |
            Status::TimeLimitReached(_) |
            Status::PrimalInfeasible(_) |
            Status::PrimalInfeasibleInaccurate(_) |
            Status::DualInfeasible(_) |
            Status::DualInfeasibleInaccurate(_) |
            Status::__Nonexhaustive => {
                warn!("Could not resolve problem :(");
            }
            Status::NonConvex(_) => {
                error!("Problem is non convex");
            }
        }

        v_optimal
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
