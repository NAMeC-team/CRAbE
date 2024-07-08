use crate::{action::move_to::MoveTo, message::MessageData};
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_framework::data::geometry::Penalty;
use crabe_math::shape::Line;
use crabe_math::vectors;
use nalgebra::Point2;
use std::time::{SystemTime, UNIX_EPOCH};

/// The Square struct represents a strategy that commands a robot to move in a square shape
/// in a counter-clockwise. It is used for testing purposes.
#[derive(Default)]
pub struct DefenseWall {
    /// The id of the robot to move.
    ids: Vec<u8>,
    current_pos_along_penaly: f64,
    messages: Vec<MessageData>,
}

impl DefenseWall {
    /// Creates a new DefenseWall instance with the desired robot id.
    pub fn new(ids: Vec<u8>) -> Self {
        Self { ids, current_pos_along_penaly: 0.5,messages: vec![], }
    }

    

    /// Move around the penalty zone
    pub fn oscillate(
        &mut self,
        world: &World,
        action_wrapper: &mut ActionWrapper,
    )-> bool {
        let enlarged_penalty = world.geometry.ally_penalty.enlarged_penalty(0.3);
        for id in self.ids.clone() {
            action_wrapper.clear(id);
        }
        let current_time = SystemTime::now();
        let mut x = 0.;
        if let Ok(duration) = current_time.duration_since(UNIX_EPOCH) {
            let current_time_ms = duration.as_millis() as f64;
            x = current_time_ms ;
        } 
        let oscillating_value = (0.00005 * 2.0 * std::f64::consts::PI * x).sin() * 0.5 + 0.5;
        let pos = enlarged_penalty.on_penalty_line(oscillating_value);
        for id in self.ids.clone() {
            action_wrapper.push(id, MoveTo::new(pos, 0., 0., false, None, false));
        }
        false
    }

    /// Get the average position ratio of the robots along the penalty line
    pub fn get_closest_point_on_penalty_line(
        &mut self,
        world: &World
    ) -> Option<f64> {  
        let enlarged_penalty = world.geometry.ally_penalty.enlarged_penalty(0.3);
        let goal_center = world.geometry.ally_goal.line.center();
        let mut total = 0.;
        let mut total_bot_nb = 0.;//in case some bots don't have intersection line we shouldn't count them in the mean
        for id in &self.ids{
            if !world.allies_bot.contains_key(id) {
                continue;
            }
            let bot_pos = world.allies_bot[id].pose.position;
            let bot_to_goal = Line::new(bot_pos, goal_center);
            if let Some(bot_ratio_pos) = enlarged_penalty.intersection_line_as_ratio(bot_to_goal) {
                total += bot_ratio_pos;
                total_bot_nb += 1.;
            }
        }
        if total_bot_nb <= 0. {
            return None;
        }
        Some(total/total_bot_nb)
    }
}

impl Strategy for DefenseWall {
    fn name(&self) -> &'static str {
        "DefenseWall"
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

    /// Executes the DefenseWall strategy.
    ///
    /// This strategy commands the robot with the specified ID to move around the goal line
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
        for id in &self.ids{
            action_wrapper.clear(*id);
        }
        
        let ball_pos = match world.ball.clone() {
            None => {return false;}
            Some(ball) => {ball.position.xy() }
        };
        
        let enlarged_penalty = world.geometry.ally_penalty.enlarged_penalty(0.3);

        let goal_center = world.geometry.ally_goal.line.center();
        let ball_to_goal = Line::new( ball_pos, goal_center);

        if let Some(mut intersection_shooting_dir_ratio) = enlarged_penalty.intersection_segment_as_ratio(ball_to_goal) {//if ball to goal center intersect the penalty line
            if let Some(current_pos) = self.get_closest_point_on_penalty_line(world){
                self.current_pos_along_penaly = current_pos;
            }
            //clamp new bot position so they have to move along the penalty line instead of just moving through the goal field
            intersection_shooting_dir_ratio = intersection_shooting_dir_ratio.clamp(self.current_pos_along_penaly-0.1, self.current_pos_along_penaly+0.1);

            let tot_penalty_line_length = enlarged_penalty.depth * 2. + enlarged_penalty.width;

            let bot_diameter = world.geometry.robot_radius * 2.;
			let bot_diameter_to_ratio = bot_diameter / tot_penalty_line_length; // bot diameter between 0 and 1 relatively to the penalty line length
            
            // Get the robots (so that we know how many of them can be move)
            let mut robots = vec![];
            for id in self.ids.clone() {
                if let Some(robot) = world.allies_bot.get(&id) {
                    robots.push(robot);
                } 
            }
            
            let robot_nb = robots.len() as f64;
            let wall_starting_pos = intersection_shooting_dir_ratio - (bot_diameter_to_ratio / 2.) * (robot_nb - 1.);

            // Clamp the position of the wall so that he's not going out of the field
            let wall_starting_pos_not_overflowing = wall_starting_pos.clamp(bot_diameter_to_ratio / 2., 1. - bot_diameter_to_ratio / 2. - (robot_nb-1.)*bot_diameter_to_ratio);
            for (i, robot) in robots.iter().enumerate() {
                let relative_ratio = wall_starting_pos_not_overflowing + (i as f64) * bot_diameter_to_ratio;
                let pos_on_penalty_line = enlarged_penalty.on_penalty_line(relative_ratio);
                let orientation = vectors::angle_to_point(robot.pose.position, ball_pos);
                action_wrapper.push(robot.id, MoveTo::new(pos_on_penalty_line, orientation, 0., false, None, false));
            }
        } else {
            println!("No intersection point found");
        }
        false
    }
}