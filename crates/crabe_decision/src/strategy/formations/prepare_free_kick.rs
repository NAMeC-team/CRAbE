use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::Strategy;
use crate::utils::closest_bots_to_point;

use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;

/// The PrepareFreeKick struct represents a strategy that commands the team to set in the PrepareFreeKick formation
#[derive(Default)]
pub struct PrepareFreeKick{
    ally: bool,
    messages: Vec<MessageData>,
    ids: Vec<u8>,
}

impl PrepareFreeKick{
    pub fn new(ally: bool) -> Self {
        Self {
            ally, 
            messages: vec![],
            ids: vec![],
        }
    }
}

impl Strategy for PrepareFreeKick{
    /// Executes the PrepareKickOffstrategy.
    ///
    /// This strategy commands all the robots to move in position for
    ///
    /// # Arguments
    ///
    /// * world: The current state of the game world.
    /// * tools_data: A collection of external tools used by the strategy, such as a viewer.    
    /// * action_wrapper: An `ActionWrapper` instance used to issue actions to the robot.
    ///
    /// # Returns
    ///
    /// A boolean value indicating whether the strategy is finished or not.
    
    fn name(&self) -> &'static str {
        "PrepareFreeKick"
    }

    fn get_messages(&self) -> &Vec<MessageData> {
        &self.messages
    }
    fn get_ids(&self) -> Vec<u8> {
        self.ids.clone()
    }
    fn put_ids(&mut self, ids: Vec<u8>) {
        self.ids = ids;
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {

        for robot in self.ids.iter() {
            action_wrapper.clear(*robot);
        }

        let ball_pos = match world.ball.clone() {
            None => {
                Point2::new(-3.0,0.0)
            }
            Some(ball) => {
                ball.position.xy()
            }
        };

        if self.ally {

            let attackers = closest_bots_to_point(world.allies_bot.values().collect(), ball_pos);

            if attackers.len() > 0 {
                let freekick_taker = attackers[0];

                let pass_receiver = attackers[1];
                            // put ally behind the ball and try to shoot
                action_wrapper.push(freekick_taker.id, MoveTo::new(Point2::new(ball_pos.x - 1., ball_pos.y), vectors::angle_to_point(Point2::new(ball_pos.x - 1., ball_pos.y), ball_pos), 0.0, false, None, false));

                // and another ally paralel to him to try a pass depending on wich side of the field the free kick is
                if ball_pos.y < 0. {
                    action_wrapper.push(pass_receiver.id, MoveTo::new(Point2::new(ball_pos.x - 1., ball_pos.y + 2.), vectors::angle_to_point(Point2::new(ball_pos.x - 1., ball_pos.y + 2.), ball_pos), 0.0, false, None, false));
                } else {
                    action_wrapper.push(pass_receiver.id, MoveTo::new(Point2::new(ball_pos.x - 1., ball_pos.y - 2.), vectors::angle_to_point(Point2::new(ball_pos.x - 1., ball_pos.y - 2.), ball_pos), 0.0, false, None, false));
                }
                return false;
            }

        } else {
            let closest_bots = closest_bots_to_point(world.enemies_bot.values().collect(), ball_pos);

            if closest_bots.len() > 0 {
                // put a wall in front of the ball with 3 last robots from the list
                for i in (0..=2).rev() {
                    if !(ball_pos.y - 1.0 + (i as f64 - 0.0) > 3.0 || ball_pos.y - 1.0 + (i as f64 - 0.0) < - 3.0) {
                        let y_cord = ball_pos.y - 1.0 + (i as f64 - 0.0);
                        action_wrapper.push(closest_bots[i].id, MoveTo::new(Point2::new(ball_pos.x - 1., y_cord), vectors::angle_to_point(Point2::new(ball_pos.x - 1., y_cord), ball_pos), 0.0, false, None, false));
                    }
                }
            }
        }
        false
    }
}
