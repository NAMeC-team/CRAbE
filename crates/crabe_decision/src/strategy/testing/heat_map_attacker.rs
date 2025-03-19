use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::strategy::Strategy;
use crate::message::MessageData;
use crate::utils::get_open_shoot_window;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::shape::Line;
use nalgebra::Point2;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::prelude::{IntoDrawingArea, Rectangle};
use plotters::style::{Color, HSLColor, WHITE};
use plotters_bitmap::BitMapBackend;
use std::f64::consts::PI;
use std::time::Instant;


/// The HeatMapAttacker struct represents a strategy that generate a heat map of optimal position to shoot from.
#[derive(Default)]
pub struct HeatMapAttacker {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
    heatmap: Option<Vec<Vec<f64>>>,
    render: bool,
}

impl HeatMapAttacker {
    /// Creates a new HeatMapAttacker instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![], heatmap: None, render: false }
    }
    fn generate_heatmap(&mut self, world: &World) -> Option<Vec<Vec<f64>>>{
        self.generate_default_heatmap(world);
        if let Some(heatmap_matrix) = &self.heatmap {
            let mut new_matrix = heatmap_matrix.clone();
            let width: f64 = world.geometry.field.length;    // Field width (can be extracted from World if needed)
            let height: f64 = world.geometry.field.width;  // Field height (can be extracted from World if needed)
            let matrix_width: usize = 100;
            let matrix_height: usize = matrix_width * height as usize / width as usize;
            //start i at heatmap_matrix.len() / 2 to avoid ally side
            // let enemies_reverse_lines: Vec<Vec<Line>> = world.enemies_bot.iter().map(|(_, enemy_robot)| {
            //     let goal_left_to_enemy = (enemy_robot.pose.position - world.geometry.enemy_goal.line.start).normalize();
            //     let goal_right_to_enemy = (enemy_robot.pose.position - world.geometry.enemy_goal.line.end).normalize();
            //     let reverse_line_enemy_to_goal_left = Line::new(enemy_robot.pose.position, enemy_robot.pose.position + goal_left_to_enemy * 10.);
            //     let reverse_line_enemy_to_goal_right = Line::new(enemy_robot.pose.position, enemy_robot.pose.position + goal_right_to_enemy * 10.);
            //     vec![reverse_line_enemy_to_goal_left, reverse_line_enemy_to_goal_right]
            // }).collect();
            for i in (heatmap_matrix.len() / 2)..heatmap_matrix.len() {
                for j in 0..heatmap_matrix[i].len() {       
                    if new_matrix[i][j] <= 0.2 {//TODO threshold should be relative to the max value of the heatmap
                        new_matrix[i][j] = 0.;
                        continue;
                    }
                    let x = width * (i as f64 / (matrix_width  - 1) as f64) - width / 2.0;
                    let y = height * (j as f64 / (matrix_height - 1) as f64) - height / 2.0;
                    let point = Point2::new(x, y);
    

                    // Calculate the reward based on the enemies robots obstruction (not really smart to use this)
                    // let mut enemy_obstruction_reward: f64 = 1.;
                    // for enemy_lines in &enemies_reverse_lines {
                    //     let reverse_line_enemy_to_goal_left = &enemy_lines[0];
                    //     let reverse_line_enemy_to_goal_right = &enemy_lines[1];
                    //     // if !reverse_line_enemy_to_goal_left.point_under(&point) && reverse_line_enemy_to_goal_right.point_under(&point) {
                    //     //     enemy_obstruction_reward = 0.;
                    //     //     break;
                    //     // }
                    //     let dist_left_line = reverse_line_enemy_to_goal_left.distance_to_point(&point);
                    //     let dist_right_line = reverse_line_enemy_to_goal_right.distance_to_point(&point);
                    //     let min_dist = (dist_left_line.min(dist_right_line) - world.geometry.robot_radius).max(0.);
                    //     enemy_obstruction_reward = enemy_obstruction_reward.min(min_dist/world.geometry.robot_radius as f64);
                    // }
                    // enemy_obstruction_reward = enemy_obstruction_reward * 0.5 + 0.5;

                    let shoot_windows = get_open_shoot_window(&point, world);
                    let mut shoot_window_length: f64 = 0.;
                    for window in shoot_windows {
                        shoot_window_length += window.norm();
                    }
                    let goal_length = world.geometry.enemy_goal.line.norm();
                    let shoot_window_reward = shoot_window_length / goal_length;
                    new_matrix[i][j] *= shoot_window_reward;
                }
            }
            if !self.render { 
                self.plot_heatmap(&new_matrix);
                self.render = true;
            }
            return Some(new_matrix);
        }
        None
    }

    fn generate_default_heatmap(&mut self, world: &World) {
        if self.heatmap.is_some() {
            return;
        }
        let width: f64 = world.geometry.field.length;    // Field width (can be extracted from World if needed)
        let height: f64 = world.geometry.field.width;  // Field height (can be extracted from World if needed)
        if (width == 0.0) || (height == 0.0) {
            return;
        }
        let matrix_width: usize = 100;
        let matrix_height: usize = matrix_width * height as usize / width as usize;
    
        let mut heatmap_matrix: Vec<Vec<f64>> = vec![vec![0.0; matrix_height]; matrix_width];
        // Fill the matrix with heatmap values
        for i in 0..matrix_width {
            for j in 0..matrix_height {                      
                let x = width * (i as f64 / (matrix_width - 1) as f64) - width / 2.0;
                let y = height * (j as f64 / (matrix_height - 1) as f64) - height / 2.0;
                let point = Point2::new(x, y);

                // Calculate the reward based on the distance to the enemy goal
                let dist_to_goal = world.geometry.enemy_goal.line.distance_to_point(&point);
                let close_to_goal_reward = (1.3 - dist_to_goal/5.).min(1.).max(0.);


                // Reward if facing the goal (if shooting window is large)
                let goal_to_point_left = (world.geometry.enemy_goal.line.start - point).normalize();
                let goal_to_point_right = (world.geometry.enemy_goal.line.end - point).normalize();
                let line = Line::new(point + goal_to_point_left, point + goal_to_point_right);
                let facing_goal_reward = (line.norm() * 3.).min(1.).max(0.);

                // penalty mask 
                let penalty_mask = if point.x.abs() > world.geometry.enemy_penalty.front_line.start.x.abs() && point.y.abs() < world.geometry.enemy_penalty.front_line.start.y.abs() {
                    0.0
                } else {
                    1.0
                };

                // Calculate the normalized value
                let normalized_value = close_to_goal_reward * facing_goal_reward * penalty_mask;
                heatmap_matrix[i][j] = normalized_value;
            }
        }
        // self.plot_heatmap(&heatmap_matrix);
        self.heatmap = Some(heatmap_matrix);
    }
    

    fn plot_heatmap(&mut self, heatmap_matrix: &Vec<Vec<f64>>) {
        let num_rows = heatmap_matrix.len();
        let num_cols = if num_rows > 0 { heatmap_matrix[0].len() } else { 0 };
        
        // Set the image size relative to the matrix dimensions
        let image_width = num_rows * 6; // Adjust width based on the number of columns
        let image_height = num_cols * 6; // Adjust height based on the number of rows
    
        let root = BitMapBackend::new("heatmap.png", (image_width as u32, image_height as u32)).into_drawing_area();
        root.fill(&WHITE).unwrap(); // Set the background color to white
    
        let width = 10.0;  // Adjust width of the field
        let height = 6.0;  // Adjust height of the field
    
        let mut chart = ChartBuilder::on(&root)
            .caption("Heatmap", ("sans-serif", 20))
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(-width / 2.0..width / 2.0, -height / 2.0..height / 2.0)
            .unwrap();
    
        chart.configure_mesh().draw().unwrap();
    
        // Plot the heatmap matrix
        for i in 0..num_rows {
            for j in 0..num_cols {
                let x = width * (i as f64 / (num_rows - 1) as f64) - width / 2.0;
                let y = height * (j as f64 / (num_cols - 1) as f64) - height / 2.0;
    
                let normalized_value = heatmap_matrix[i][j];
                let color = HSLColor(0.0, 1.0, normalized_value * 0.5 + 0.2); // Normalize color
    
                chart.draw_series(std::iter::once(Rectangle::new(
                    [(x, y), (x + width / num_rows as f64, y + height / num_cols as f64)],
                    color.filled(),
                ))).unwrap();
            }
        }
    
        root.present().unwrap();
        println!("Heatmap saved as 'heatmap.png'");
    }
}

impl Strategy for HeatMapAttacker {
    fn name(&self) -> &'static str {
        "HeatMapAttacker"
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
    /// Executes the HeatMapAttacker strategy.
    ///
    /// This strategy generate a heat map representing the optimal positions to shoot from.
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
        //calculate time execution
        let start = Instant::now();
        let new_heatmap = self.generate_heatmap(world);
        let elapsed = start.elapsed();
        if let Some(map) = new_heatmap {
            //get max x and y 
            let mut max = 0.0;
            let mut max_x = 0;
            let mut max_y = 0;
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] > max {
                        max = map[i][j];
                        max_x = i;
                        max_y = j;
                    }
                }
            }
            let width: f64 = world.geometry.field.length;    // Field width (can be extracted from World if needed)
            let height: f64 = world.geometry.field.width;  // Field height (can be extracted from World if needed)
            let matrix_width: usize = 100;
            let matrix_height: usize = matrix_width * height as usize / width as usize;
            let x = width * (max_x as f64 / (matrix_width - 1) as f64) - width / 2.0;
            let y = height * (max_y as f64 / (matrix_height - 1) as f64) - height / 2.0;
            action_wrapper.clear(self.id);
            action_wrapper.push(
                self.id,
                MoveTo::new().set_x(x).set_y(y),
            );
        }
        println!("Time elapsed in heatmap generation: {:?}", elapsed);
        false
    }
}

