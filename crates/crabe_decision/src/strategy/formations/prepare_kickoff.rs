use crate::action::move_to::MoveTo;
use crate::action::{self, ActionWrapper};
use crate::message::MessageData;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors;
use nalgebra::Point2;

/// The PrepareKickOff struct represents a strategy that commands the team to set in the PrepareKickOff formation
#[derive(Default)]
pub struct PrepareKickOff{
    ally: bool,
    messages: Vec<MessageData>,
    ids: Vec<u8>,
}

impl PrepareKickOff{
    pub fn new(ally: bool) -> Self {
        Self {
            ally, 
            messages: vec![],
            ids: vec![],
        }
    }
}

impl Strategy for PrepareKickOff{
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
        "PrepareKickOff"
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
                Point2::new(0.0,0.0)
            }
            Some(ball) => {
                ball.position.xy()
            }
        };

        if self.ally {
            action_wrapper.push(0, MoveTo::new(Point2::new(-1., 1.), vectors::angle_to_point(Point2::new(-1., 1.), ball_pos), 0.0, false, None, false));
            action_wrapper.push(1, MoveTo::new(Point2::new(-0.0, 0.0), vectors::angle_to_point(Point2::new(-0.0, 0.0), ball_pos), 0.0, false, None, false));
            action_wrapper.push(2, MoveTo::new(Point2::new(-1., -1.), vectors::angle_to_point(Point2::new(-1., -1.), ball_pos), 0.0, false, None, false));
            action_wrapper.push(3, MoveTo::new(Point2::new(-0.25, 2.5), vectors::angle_to_point(Point2::new(-0.25, 2.5), ball_pos), 0.0, false, None, false));
            action_wrapper.push(4, MoveTo::new(Point2::new(-0.25, -2.5), vectors::angle_to_point(Point2::new(-0.25, -2.5), ball_pos), 0.0, false, None, false));
        } else {

            let enemy_robots = world.enemies_bot.clone();

            action_wrapper.push(0, MoveTo::new(Point2::new(-0.5, 1.), vectors::angle_to_point(Point2::new(-3., 0.5), ball_pos), 0.0, false, None, false));
            action_wrapper.push(1, MoveTo::new(Point2::new(-0.6, 0.0), vectors::angle_to_point(Point2::new(-0.0, 0.0), ball_pos), 0.0, false, None, false));
            action_wrapper.push(2, MoveTo::new(Point2::new(-0.5, -1.), vectors::angle_to_point(Point2::new(-3., -0.5), ball_pos), 0.0, false, None, false));
            action_wrapper.push(3, MoveTo::new(Point2::new(-0.3, 1.5), vectors::angle_to_point(Point2::new(-0.25, 2.5), ball_pos), 0.0, false, None, false));
            action_wrapper.push(4, MoveTo::new(Point2::new(-0.3, -1.5), vectors::angle_to_point(Point2::new(-0.25, -2.5), ball_pos), 0.0, false, None, false));
        }
        false
    }
}
