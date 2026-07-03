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
pub struct PreparePenalty {
    /// The id of the robot to move.
    id: u8,
    shoot_state: u8
}

impl PreparePenalty {
    /// Creates a new StrategyPenalty instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, shoot_state: 0 }
    }
}

impl Strategy for PreparePenalty {
    fn name(&self) -> &'static str {
        "PreparePenalty"
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
        

        if self.id != 0{
                println!("0");
                action_wrapper.push(
                    0,
                    MoveTo::new_all_params(Point2::new(-2.5, 1.0), orientation, 0., false, None, true, true),
                );
            }
            
            if self.id != 1{
                println!("1");
                action_wrapper.push(
                    1,
                    MoveTo::new_all_params(Point2::new(-2.5, 0.0), orientation, 0., false, None, true, true),
                );
            }
    
            if self.id != 2{
                action_wrapper.push(
                    2,
                    MoveTo::new_all_params(Point2::new(-2.5, -1.0), orientation, 0., false, None, true, true),
                );
            }

            if self.id != 3{
                action_wrapper.push(
                    3,
                    MoveTo::new_all_params(Point2::new(-2.5, -2.0), orientation, 0., false, None, true, true),
                );
            }
    
            if self.id != 4{
                action_wrapper.push(
                    4,
                    MoveTo::new_all_params(Point2::new(-2.5, 2.0), orientation, 0., false, None, true, true),
                );
            }

            if self.id != 5{
                action_wrapper.push(
                    5,
                    MoveTo::new_all_params(Point2::new(-2.5, -0.5), orientation, 0., false, None, true, true),
                );
            }

            if self.id != 6{
                action_wrapper.push(
                    6,
                    MoveTo::new_all_params(Point2::new(-2.5, 0.5), orientation, 0., false, None, true, true),
                );
            }

            if self.id != 7{
                action_wrapper.push(
                    7,
                    MoveTo::new_all_params(Point2::new(-2.5, 1.5), orientation, 0., false, None, true, true),
                );
            }

            //ROBOT QUI FASSE LE PENALTY
            
            action_wrapper.push(
                self.id,
                MoveTo::new_all_params(Point2::new(-2.0, 1.5), orientation, 0., false, None, true, true),
            );

        if  self.shoot_state == 9{
            
            //AUTRES ROBOTS Se mettent en arriere
            

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

        false
    }
}

/*

impl Strategy for PreparePenalty {
    fn name(&self) -> &'static str {
        "PreparePenalty"
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
            
            
            //AUTRES ROBOTS Se mettent en arriere
            let mut n = 0;
            while dbg!(n < 16){
                if self.id != n{
                    let y = 0.5*(n as f64) - 3.0 ;
                    action_wrapper.push(
                        n,
                        MoveTo::new_all_params(Point2::new(-2.5, y), orientation, 0., false, None, true, true),
                    );
                }
                else {
                    action_wrapper.push(
                        self.id,
                        MoveTo::new_all_params(Point2::new(-2.0, 1.5), orientation, 0., false, None, true, true),
                    );
                }
                n = n + 1;
            }
    
            // if self.id != 1{
            //     action_wrapper.push(
            //         1,
            //         MoveTo::new_all_params(Point2::new(-2.5, 0.0), orientation, 0., false, None, true, true),
            //     );
            // }
    
            // if self.id != 2{
            //     action_wrapper.push(
            //         2,
            //         MoveTo::new_all_params(Point2::new(-2.5, -1.0), orientation, 0., false, None, true, true),
            //     );
            // }

            // if self.id != 3{
            //     action_wrapper.push(
            //         3,
            //         MoveTo::new_all_params(Point2::new(-2.5, -2.0), orientation, 0., false, None, true, true),
            //     );
            // }
    
            // if self.id != 4{
            //     action_wrapper.push(
            //         4,
            //         MoveTo::new_all_params(Point2::new(-2.5, 2.0), orientation, 0., false, None, true, true),
            //     );
            // }

            // if self.id != 5{
            //     action_wrapper.push(
            //         5,
            //         MoveTo::new_all_params(Point2::new(-2.5, -0.5), orientation, 0., false, None, true, true),
            //     );
            // }

            // if self.id != 6{
            //     action_wrapper.push(
            //         6,
            //         MoveTo::new_all_params(Point2::new(-2.5, 0.5), orientation, 0., false, None, true, true),
            //     );
            // }

            // if self.id != 7{
            //     action_wrapper.push(
            //         7,
            //         MoveTo::new_all_params(Point2::new(-2.5, 1.5), orientation, 0., false, None, true, true),
            //     );
            // }

            //ROBOT QUI FASSE LE PENALTY
            
            

            if (chosed.pose.position.x < -1.9 && chosed.pose.position.x > -2.1 ) && (1.4 < chosed.pose.position.y && chosed.pose.position.y < 1.6 )  {
                self.shoot_state = 1;
                // action_wrapper.clear(self.id);
            }
        }

        // if self.shoot_state == 1 {

        //     action_wrapper.push(
        //         self.id,
        //         MoveTo::new_all_params(Point2::new(-2.5, 0.0), orientation, 0., false, None, true, true),
        //     );

        //     if (chosed.pose.position.x < -2.4 && chosed.pose.position.x > -2.6 ) && (-0.1 < chosed.pose.position.y && chosed.pose.position.y < 0.1 )  {
        //         self.shoot_state = 2;
        //     }
        // }

        false
    }
}


*/