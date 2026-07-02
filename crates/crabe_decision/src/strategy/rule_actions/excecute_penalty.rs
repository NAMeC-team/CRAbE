use crate::action::move_to::MoveTo;
use crate::action::move_to_builder::MoveToBuilder;
// use crate::action::state::State;
use crate::action::{self, ActionWrapper, state};
use crate::strategy::Strategy;
use crabe_framework::data::referee::Stage;
// use crabe_framework::data::referee::Stage::{self, PenaltyShootout};
use crabe_framework::data::{output::Kick, tool::ToolData};
use crabe_framework::data::world::{self, World};
use crabe_math::shape::Line;
use crabe_math::vectors::{self, angle_to_point, vector_from_angle};
use nalgebra::Point2;
use rand::RngExt;

#[derive(Default, Debug)]
pub struct ExcecutePenalty {
    /// The id of the robot to move.
    id: u8,
    decide_direction: f64,
    shoot_state: u8
}

impl ExcecutePenalty {
    /// Creates a new StrategyPenalty instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, decide_direction: rand::rng().random_range(-0.3..0.3), shoot_state: 0 }

    }
}

impl Strategy for ExcecutePenalty {
    fn name(&self) -> &'static str {
        "ExcecutePenalty"
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear(self.id);
        action_wrapper.clear_all();

        
        // if self.shoot_state == 0 {
        //     let stage = world.data.stage_info.stage.clone();
            
        //     if matches!(stage, Stage::PenaltyShootout) {
        //        self.autorised_shoot = true;
        //     }
            
        //     if self.autorised_shoot {
        //         self.shoot_state = 3;
        //     }   
        // }
    
        let ball = if let Some(ball) = &world.ball {
            ball
        } else {
            return false;
        };
        
        let chosed = if let Some(chosed) = world.allies_bot.get(&self.id) {
            chosed
        } else {
            return false;
        };

        let ball_position = ball.position_2d();
        let orientation = vectors::angle_to_point(chosed.pose.position,Point2::new(ball_position.x, ball_position.y + self.decide_direction));
                    
        let mut delta_x = chosed.pose.position.x - ball.position.x;
        let mut delta_y = chosed.pose.position.y - ball.position.y;

        if self.shoot_state == 0 {
 
            action_wrapper.push(
                self.id,
                MoveTo::new_all_params(ball.position_2d(), orientation, 0., false, None, true, true),
            );

            if delta_x < 0.0 {
                delta_x = - delta_x;
            }

            if delta_y < 0.0 {
                delta_y = -delta_y;
            }

            if delta_x < 0.11 && delta_y < 0.11{
                if (chosed.pose.position - ball_position).norm() < 0.01 + world.geometry.robot_radius + world.geometry.ball_radius {
                    
                    if chosed.pose.position.x > 0.0 {
                        self.shoot_state = 1;                                     
                    }
                }
            }

        }
        
        if self.shoot_state == 1 {

            let ball_position = ball.position_2d();
            let orientation = vectors::angle_to_point(chosed.pose.position,ball_position);
            let to_kicker_pos = vector_from_angle(chosed.pose.orientation) * world.geometry.robot_radius;
            let trajectory = Line::new(ball_position, ball_position + ball.velocity.xy().normalize() * 100.);
            let target = trajectory.closest_point_on_segment(&chosed.pose.position);
            let mut moveto = MoveToBuilder::new();
            moveto.set_target(target - to_kicker_pos).set_orientation(angle_to_point(chosed.pose.position, Point2::new(4.5, self.decide_direction))).charging();
            
            action_wrapper.push(self.id, 
                moveto.set_kick(Kick::StraightKick { power: 5. }).build(),
            );
            
        }

        false
    }
}
