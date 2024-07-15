use crate::action::ActionWrapper;
use crate::manager::Manager;
use crate::strategy::testing::Prembule;
use crate::strategy::testing::TestVisionMoveTo;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;

const ROBOT_ID: u8 = 0;
/// The `Test_manager` struct represents a decision manager that executes strategies manually
/// added to its list.
/// It's used for testing individual strategies only and not meant to be used during an actual game.
///
/// To add a strategy, simply create a new instance of the desired strategy and add it to the
/// `strategies` field in the `new()` method of the `TestManager` struct.
/// Test_manager add before your strategie the prembule to testing the bot to know if it is working or not
pub struct TestManager {
    strategies: Vec<Box<dyn Strategy>>,
}

impl TestManager {
    /// Creates a new `TestManager` instance with the desired strategies to test.
    pub fn new() -> Self {
        Self {
            strategies: vec![Box::new(Prembule::new(ROBOT_ID)),Box::new(TestVisionMoveTo::new(vec![ROBOT_ID]))],
        }
    }
}

impl Manager for TestManager {
    /// Executes the list of strategies on the given `World` data, `ToolData`, and `ActionWrapper`.
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        //execute only first strategy step then remove it
        if self.strategies.len() > 0 {
            let end = self.strategies[0].step(world, tools_data, action_wrapper);
            if end{
                self.strategies.remove(0);
            }
        }
        
        
        
        
    }
}
