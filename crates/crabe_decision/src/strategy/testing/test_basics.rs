use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crate::strategy::basics::pass;
use nalgebra::Point2;
use std::f64::consts::PI;

/// The TestBasics struct represents a strategy that commands a robot to move in a TestBasics shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct TestBasics {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
}

impl TestBasics {
    /// Creates a new TestBasics instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![]}
    }
}

impl Strategy for TestBasics {
    fn name(&self) -> &'static str {
        "TestBasics"
    }

    fn get_messages(&self) -> &Vec<MessageData> {
        &self.messages
    }
    fn get_ids(&self) -> Vec<u8> {
        vec![self.id]
    }
    fn put_ids(&mut self, ids: Vec<u8>) {
        if ids.len() == 1{
            self.id = ids[0];
        }
    }
    /// Executes the TestBasics strategy.
    ///
    /// This strategy commands the robot with the specified ID to move in a TestBasics shape in a
    /// counter-clockwise direction.
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
    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear(self.id);
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                return false;
            }
        };

        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                return false;
            }
        };

        let robot2 = &match world.allies_bot.get(&1) {
            Some(r) => r,
            None => {
                return false;
            }
        };

        let target_shooting_position = world.geometry.enemy_goal.line.center();
        action_wrapper.push(self.id,pass(robot, robot2,&ball,&world));

        false
    }
}
