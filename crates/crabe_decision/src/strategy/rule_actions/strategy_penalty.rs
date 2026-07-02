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
pub struct StrategyPenalty {
    /// The id of the robot to move.
    id: u8,
    autorised_shoot : bool,
    decide_direction: f64,
    shoot_state: u8
}

impl StrategyPenalty {
    /// Creates a new StrategyPenalty instance with the desired robot id.
    pub fn new(id: u8, autorised_shoot : bool) -> Self {
        Self { id, autorised_shoot, decide_direction: rand::rng().random_range(-0.0008..0.0008), shoot_state: 0 }
        //Self {id}

    }
}

impl Strategy for StrategyPenalty {
    fn name(&self) -> &'static str {
        "StrategyPenalty"
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

        let orientation = vectors::angle_to_point(Point2::new(4.5, 0.0), Point2::new(4.5, 0.0));
        let middle = world.geometry.enemy_goal.line.center() - ball.velocity.xy() / 2.;
        
        if  self.shoot_state == 0{
            
            //ROBOT QUI FASSE LE PENALTY
            
            action_wrapper.push(
                self.id,
                MoveTo::new_all_params(Point2::new(-2.0, 1.5), orientation, 0., false, None, true, true),
            );
            
            //AUTRES ROBOTS Se mettent en arriere
    
            action_wrapper.push(
                0,
                MoveTo::new_all_params(Point2::new(-2.5, 1.0), orientation, 0., false, None, true, true),
            );
    
            action_wrapper.push(
                1,
                MoveTo::new_all_params(Point2::new(-3., 1.0), orientation, 0., false, None, true, true),
            );
    
            action_wrapper.push(
                2,
                MoveTo::new_all_params(Point2::new(-2.5, -1.0), orientation, 0., false, None, true, true),
            );
    
            action_wrapper.push(
                4,
                MoveTo::new_all_params(Point2::new(-3., -1.0), orientation, 0., false, None, true, true),
            );

            if (chosed.pose.position.x < -1.9 && chosed.pose.position.x > -2.1 ) && (1.4 < chosed.pose.position.y && chosed.pose.position.y < 1.6 )  {
                self.shoot_state = 1;
                // action_wrapper.clear(self.id);
            }

        }

        if self.shoot_state == 1 {

            action_wrapper.push(
                self.id,
                MoveTo::new_all_params(Point2::new(-2.5, 0.0), orientation, 0., false, None, true, true),
            );

            if (chosed.pose.position.x < -2.4 && chosed.pose.position.x > -2.6 ) && (-0.1 < chosed.pose.position.y && chosed.pose.position.y < 0.1 )  {
                self.shoot_state = 2;
            }

        }
        
        if self.shoot_state == 2 {
            let stage = world.data.stage_info.stage.clone();
            
            if matches!(stage, Stage::PenaltyShootout) {
               self.autorised_shoot = true;
            }
            
            if self.autorised_shoot {
                self.shoot_state = 3;
            }   
        }

        if self.shoot_state == 3 {
        
            let ball_position = ball.position_2d();
            let orientation = vectors::angle_to_point(chosed.pose.position,Point2::new(ball_position.x, ball_position.y + self.decide_direction));
                       
            let mut delta_x = chosed.pose.position.x - ball.position.x;
            let mut delta_y = chosed.pose.position.y - ball.position.y;
 
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
                        self.shoot_state = 4;                                     
                    }
                }
            }

        }
        
        if self.shoot_state == 4 {

            let ball_position = ball.position_2d();
            let orientation = vectors::angle_to_point(chosed.pose.position,ball_position);
            let to_kicker_pos = vector_from_angle(chosed.pose.orientation) * world.geometry.robot_radius;
            let trajectory = Line::new(ball_position, ball_position + ball.velocity.xy().normalize() * 100.);
            let target = trajectory.closest_point_on_segment(&chosed.pose.position);
            let mut moveto = MoveToBuilder::new();
            moveto.set_target(target - to_kicker_pos).set_orientation(angle_to_point(chosed.pose.position, middle)).charging();
            
            action_wrapper.push(self.id, 
                moveto.set_kick(Some(Kick::StraightKick { power: 5. })).build(),
            );
            
        }

        false
    }
}
