use nalgebra::{matrix, Vector2};
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolCommands;
use crabe_framework::data::world::{AllyInfo, Robot, World};
use crate::pipeline::Guard;

#[derive(Default)]
pub struct SpeedToRobFrame;

pub fn speed_to_rob_frame(v: Vector2<f32>, rob_info: &Robot<AllyInfo>) -> Vector2<f32> {
    // let ti = Isometry2::new(Vector2::zeros(), rob_info.pose.<orientation);
    let o = rob_info.pose.orientation as f32;
    let rot = matrix![o.cos(), -o.sin();
                                 o.sin(), o.cos()];
    rot.transpose() * v
}

impl Guard for SpeedToRobFrame {
    fn guard(&mut self, world: &World, commands: &mut CommandMap, tools_commands: &mut ToolCommands) {
        commands.iter_mut().for_each(|(id, cmd)| {
            if let Some(rob_info) = world.allies_bot.get(id) {
                let speed_robot = speed_to_rob_frame(Vector2::new(cmd.forward_velocity, cmd.left_velocity), rob_info);
                // let speed_robot = speed_to_rob_frame(Vector2::new(cmd.forward_velocity as f64, cmd.left_velocity as f64), rob_info);
                cmd.forward_velocity = speed_robot.x;
                cmd.left_velocity = speed_robot.y;
            }
        });
    }
}