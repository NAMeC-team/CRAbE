use std::f64::consts::PI;

use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::basics::comeback;
use crate::strategy::Strategy;
use crate::utils::closest_bot_to_point;
use crabe_framework::data::{tool::ToolData, world::TeamColor};
use crabe_framework::data::world::World;
use crabe_math::vectors::{angle_to_point, rotate_vector};


/// Strategy prep
pub struct PrepareKickOff {
    ids: Vec<u8>,
    team: TeamColor,
    messages: Vec<MessageData>,
}

impl PrepareKickOff {
    /// Creates a new PrepareKickOff instance
    pub fn new(ids: Vec<u8>, team: TeamColor) -> Self {
        Self {
            ids,
            team,
            messages: vec![],
        }
    }
}

impl Strategy for PrepareKickOff {
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
        self.messages.clear();
        let ball = match &world.ball {
            Some(ball) => ball,
            None => return false,
        };
        let opt_closest_enemy = closest_bot_to_point(world.enemies_bot.values().collect(), ball.position_2d());
        let mut i = 0;
        for id in &self.ids {
            action_wrapper.clear(*id);
            if world.allies_bot.len() >= self.ids.len(){
                if let Some(robot) = &world.allies_bot.get(id) {
                    if robot.pose.position.x > -0.1{
                        action_wrapper.push(*id, comeback(
                            &world.allies_bot[id],
                            world,
                        ));
                    }else{
                        if let Some(closest_enemy) = opt_closest_enemy{
                            let dir = if world.team_color == self.team{
                                (world.geometry.ally_goal.line.center() - ball.position_2d()).normalize()
                            }else{
                                (ball.position_2d() - closest_enemy.pose.position).normalize()
                            };
                            let perp = rotate_vector(dir, PI/2.) * (world.geometry.robot_radius * 2. + 0.02);
                            let target_center = ball.position_2d() + dir;
                            let pos_along_block = perp * i as f64;
                            action_wrapper.push(*id, MoveTo::new(target_center + pos_along_block, angle_to_point(robot.pose.position, ball.position_2d()), 0., false, None, true));
                            i+=1;
                        }else{}
                    }

                }

            }
        }

        false
    }
}
