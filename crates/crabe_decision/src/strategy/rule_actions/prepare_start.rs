use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;

use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::vectors::angle_to_point;


/// Strategy prep
#[derive(Default)]
pub struct PrepareStart {
    ids: Vec<u8>,
}

impl PrepareStart {
    /// Creates a new PrepareStart instance
    pub fn new(ids: Vec<u8>) -> Self {
        Self {
            ids,
        }
    }
}

impl Strategy for PrepareStart {
    fn name(&self) -> &'static str {
        "PrepareStart"
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        //self.messages.clear();
        let mut i = 0;
        for id in &self.ids {
            action_wrapper.clear(*id);
            if let Some(robot) = &world.allies_bot.get(id) {
                let orientation = angle_to_point(robot.pose.position, nalgebra::Point2::new(0.0, 0.0));

                let is_goal = match world.get_goalkeeper(world.team_color){
                    Some(x) => *id == x as u8,
                    None => false,
                };

                if is_goal {
                    action_wrapper.push(*id, MoveTo::new_all_params(world.geometry.ally_goal.line.center(), orientation, 0.0, false, None, true, true));
                } else {
                    let target = nalgebra::Point2::new(world.geometry.ally_penalty.front_line.center().x + 0.2, i as f64 * (world.geometry.robot_radius * 2. + 0.02) - (((self.ids.len() as f64 -2.) / 2.) * (world.geometry.robot_radius * 2. + 0.02)));
                    action_wrapper.push(*id, MoveTo::new_all_params(target, orientation, 0.0, false, None, true, true));
                    i += 1;
                }
            }
        }

        false
    }
}
