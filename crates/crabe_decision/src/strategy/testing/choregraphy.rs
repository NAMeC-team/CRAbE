use crate::action::move_to_builder::MoveToBuilder;
use crate::action::{self, ActionWrapper};
use crate::message::MessageData;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{World, Robot, AllyInfo};
use crabe_math::shape::Circle;
use crabe_math::vectors::vector_from_angle;
use nalgebra::Point2;
use std::f64::consts::PI;

pub struct Choregraphy {
    ids: Vec<u8>,
    messages: Vec<MessageData>,
    circle: Circle,
    itteration: usize,
}

impl Choregraphy {
    /// Creates a new Choregraphy instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self {
            ids,
            messages: vec![],
            circle: Circle::new(Point2::new(0., 0.), 0., ),
            itteration: 0,
        }
    }

    fn lemniscate_pattern(&mut self, bots: &Vec<&Robot<AllyInfo>>, action_wrapper: &mut ActionWrapper) {
        let (x, y) = lemniscate_position(self.itteration as f64 * 0.005);
        self.circle.center.x = x*2.;
        self.circle.center.y = y*2.;
        self.circle.radius = 0.5 + (self.itteration as f64 * 0.005).cos().abs() * 0.2;
        bots.iter().enumerate().for_each(|(i, robot)| {
            action_wrapper.clear(robot.id);
            let a = ((i as f64) / (bots.len() as f64)) * (PI * 2.) + self.itteration as f64 * 0.005;
            let dir = vector_from_angle(a);
            let target = self.circle.center + dir * self.circle.radius;

            let mut moveto = MoveToBuilder::new();
            moveto.set_target(target).set_orientation(robot.angle_to(self.circle.center));
            action_wrapper.push(
                robot.id,
                moveto.build(),
            );
        });
    }

    fn snake_pattern(&mut self, bots: &Vec<&Robot<AllyInfo>>, action_wrapper: &mut ActionWrapper) {
        let (x, y) = lemniscate_position(self.itteration as f64 * 0.005);
        let mut previous_x = x;
        let mut previous_y = y;
        bots.iter().enumerate().for_each(|(i, robot)| {
            //if first robot, go to x y , else go behind the previous robot
            action_wrapper.clear(robot.id);
            let mut moveto = MoveToBuilder::new();
            if i == 0 {
                moveto.set_target(Point2::new(x, y));
            } else {
                let angle = robot.angle_to(Point2::new(previous_x, previous_y));
                let dir = vector_from_angle(angle);
                let target = Point2::new(previous_x, previous_y) + dir * 0.5;
                moveto.set_target(target);
                previous_x = target.x;
                previous_y = target.y;
            }

            moveto.set_orientation(robot.angle_to(Point2::new(x, y)));
            action_wrapper.push(
                robot.id,
                moveto.build(),
            );
        });
    }
}

impl Strategy for Choregraphy {
    fn name(&self) -> &'static str {
        "Choregraphy"
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
        self.lemniscate_pattern(&bots, action_wrapper);
        self.itteration += 1;
        false
    }
}

fn lemniscate_position(t: f64) -> (f64, f64) {
    let x = t.sin();
    let y = t.sin() * t.cos();
    (x, y)
}