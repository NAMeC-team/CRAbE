use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::shape::Circle;
use crabe_math::vectors::angle_to_point;
use crabe_math::vectors::vector_from_angle;
use std::f64::consts::PI;

pub struct PassCircle {
    ids: Vec<u8>,
    messages: Vec<MessageData>,
    circle: Circle
}

impl PassCircle {
    /// Creates a new PassCircle instance with the desired robot id.
    pub fn new(ids: Vec<u8>, circle: Circle) -> Self {
        Self {
            ids,
            messages: vec![],
            circle,
        }
    }
}

impl Strategy for PassCircle {
    fn name(&self) -> &'static str {
        "PassCircle"
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
        let mut bots = vec![];
        for id in &self.ids {
            match world.allies_bot.get(id) {
                Some(bot) => {
                    bots.push(bot);
                }
                None => {}
            }
        }
        let ball = if let Some(ball) = &world.ball {
            ball
        } else {
            return false;
        };
        bots.iter().enumerate().for_each(|(i, robot)| {
            action_wrapper.clear(robot.id);
            let a = ((i as f64) / (bots.len() as f64)) * (PI * 2.);
            let dir = vector_from_angle(a);
            let mut target = self.circle.center + dir;


            let robot_to_ball = ball.position_2d() - robot.pose.position;
            let dist_to_ball = robot_to_ball.norm();
            let mut orientation_target = ball.position_2d();

            let next_bot_index = (i + 1) % bots.len();
            let passer = bots[next_bot_index];
            
            let robot_to_ally = passer.pose.position - robot.pose.position;
            let robot_dir = vector_from_angle(robot.pose.orientation);
            let dot_to_ball = robot_to_ball.normalize().dot(&robot_dir.normalize());
            let dot_to_ally = robot_dir.normalize().dot(&robot_to_ally.normalize());
            if dist_to_ball < 0.6{
                if dot_to_ball < 0.99 {
                    orientation_target = ball.position_2d();
                } else{
                    if dist_to_ball < (world.geometry.robot_radius + world.geometry.ball_radius + 0.001) {
                        orientation_target = passer.pose.position;
                        if dot_to_ally > 0.99 && robot.velocity.angular.abs() < 0.01 {
                        }
                    }else{
                        target = ball.position_2d();
                    }
                }
            }
            
            action_wrapper.push(
                robot.id,
                MoveTo::new().set_target(target).set_orientation(angle_to_point(robot.pose.position, orientation_target)),
            );
        });
        false
    }
}