use crate::action::move_to_builder::MoveToBuilder;
use crate::action::{self, ActionWrapper};
use crate::message::MessageData;
use crate::strategy::Strategy;
use crabe_framework::data::geometry::Field;
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
    itteration: f64,
}

impl Choregraphy {
    /// Creates a new Choregraphy instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self {
            ids,
            messages: vec![],
            circle: Circle::new(Point2::new(0., 0.), 0., ),
            itteration: 0.,
        }
    }

    fn lemniscate_pattern(&mut self, bots: &Vec<&Robot<AllyInfo>>, field: &Field, action_wrapper: &mut ActionWrapper) {
        let (x, y) = lemniscate_position(self.itteration);
        self.circle.center.x = x*2.;
        self.circle.center.y = y*2.;
        self.circle.radius = 0.5 + self.itteration.cos().abs() * 0.2;
        bots.iter().enumerate().for_each(|(i, robot)| {
            action_wrapper.clear(robot.id);
            let a = ((i as f64) / (bots.len() as f64)) * (PI * 2.);
            let dir = vector_from_angle(a);
            let mut target = self.circle.center + dir * self.circle.radius;
            target = full_field_to_half_left_field(target, &field);
            let mut moveto = MoveToBuilder::new();
            moveto.set_target(target).set_orientation(robot.angle_to(self.circle.center));
            action_wrapper.push(
                robot.id,
                moveto.build(),
            );
        });
    }

    fn snake_pattern(&mut self, bots: &Vec<&Robot<AllyInfo>>, field: &Field, action_wrapper: &mut ActionWrapper) {
        bots.iter().enumerate().for_each(|(i, robot)| {
            action_wrapper.clear(robot.id);
            let (x, y) = lemniscate_position(self.itteration + i as f64 * 0.06);
            //if first robot, go to x y , else go behind the previous robot
            let mut moveto = MoveToBuilder::new();
            let target = full_field_to_half_left_field(Point2::new(x, y), &field);
            moveto.set_target(target);
            moveto.set_orientation(robot.angle_to(Point2::new(x, y)));
            action_wrapper.push(
                robot.id,
                moveto.build(),
            );
        });
    }

    fn graph_pattern(&mut self, bots: &Vec<&Robot<AllyInfo>>, field: &Field, action_wrapper: &mut ActionWrapper) {
        let clamped_itteration = (self.itteration * 4.) % 1.;
        bots.iter().enumerate().for_each(|(i, robot)| {
            action_wrapper.clear(robot.id);
            let x = (i as f64) * 0.5; // x position based on index
            let mut y = 0.;
            if clamped_itteration<0.2{
                y = 0.;
            } else if clamped_itteration<0.4 {
                y = i as f64 * 0.4; // even index robots go up
            } else if clamped_itteration<0.6 {
                y = (bots.len() - 1 - i) as f64 * 0.4; // even index robots go up
            } else if clamped_itteration<0.8 {
                y = 0.;
            } else {
                y = 1.;
            }
            let mut moveto = MoveToBuilder::new();
            let target = full_field_to_half_left_field(Point2::new(x, y), &field);
            moveto.set_target(target);
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
        if self.itteration % 3. < 1. {
            self.graph_pattern(&bots, &world.geometry.field, action_wrapper);
        }else if self.itteration % 3. < 2. {
            self.snake_pattern(&bots, &world.geometry.field, action_wrapper);
        }else{
            self.lemniscate_pattern(&bots, &world.geometry.field, action_wrapper);
        }
        self.itteration += 0.0005;
        false
    }
}

fn lemniscate_position(t: f64) -> (f64, f64) {
    // return the position along a lemniscate curve (integers values of t return the center of the lemniscate)
    let scaled_t = t * 2.0 * PI;
    let x = scaled_t.sin();
    let y = scaled_t.sin() * scaled_t.cos();
    let scale_factor = 2.0; // Adjust this value to change the size of the lemniscate
    let x = x * scale_factor;
    let y = y * scale_factor;
    (x, y)
}

fn full_field_to_half_left_field(point: Point2<f64>, field: &Field) -> Point2<f64> {
    if (field.width == 0.) || (field.length == 0.) {
        return point;
    }
    let x = ((point.y / field.width) - 0.5) * (field.length/2.);
    let y = (point.x / field.length) * field.width;
    Point2::new(x, y)
}

fn full_field_to_half_right_field(point: Point2<f64>, field: &Field) -> Point2<f64> {
    if (field.width == 0.) || (field.length == 0.) {
        return point;
    }
    let x = ((point.y / field.width) + 0.5) * (field.length/2.);
    let y = (point.x / field.length) * field.width;
    Point2::new(x, y)
}