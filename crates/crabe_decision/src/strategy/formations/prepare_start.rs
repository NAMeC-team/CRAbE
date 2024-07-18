use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::basics::comeback;
use crate::strategy::Strategy;
use crate::utils::KEEPER_ID;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors::angle_to_point;


/// Strategy prep
#[derive(Default)]
pub struct PrepareStart {
    ids: Vec<u8>,
    messages: Vec<MessageData>,
}

impl PrepareStart {
    /// Creates a new PrepareStart instance
    pub fn new(ids: Vec<u8>) -> Self {
        Self {
            ids,
            messages: vec![],
        }
    }
}

impl Strategy for PrepareStart {
    fn name(&self) -> &'static str {
        "PrepareStart"
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
        self.messages.clear();
        let mut i = 0;
        for id in &self.ids {
            action_wrapper.clear(*id);
            if world.allies_bot.len() >= self.ids.len(){
                let robot = &world.allies_bot[id];
                let orientation  = angle_to_point(robot.pose.position, nalgebra::Point2::new(0.0, 0.0));
                if *id == KEEPER_ID{
                    action_wrapper.push(*id, MoveTo::new(world.geometry.ally_goal.line.center(), orientation, 0.0, false, None, true));
                }else{
                    let target = nalgebra::Point2::new(world.geometry.ally_penalty.front_line.center().x + 0.2,i as f64 * (world.geometry.robot_radius *2. + 0.02) - (self.ids.len() as f64 / 2.)* (world.geometry.robot_radius *2. + 0.02));
                    action_wrapper.push(*id, MoveTo::new(target, orientation, 0.0, false, None, true));
                    i += 1;
                }
            }
        }

        false
    }
}
