use crate::action::move_to::MoveTo;
use crate::action::ActionWrapper;
use crate::message::MessageData;
use crate::strategy::Strategy;
use crate::utils::ball_in_trajectory;
use crabe_framework::data::output::Kick;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use crabe_math::{shape::Line, vectors};
use nalgebra::Point2;

/// The Attacker strategy is responsible for moving the robot to the ball and then try scoring a goal
pub struct Attacker {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
    state: ShooterState,
}

impl Attacker {
    /// Creates a new Attacker instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { id, messages: vec![], state:ShooterState::PlaceForShoot}
    }
}
enum ShooterState{
    PlaceForShoot,
    Shoot
}

impl Strategy for Attacker {

    fn name(&self) -> &'static str {
        return "Attacker";
    }
    
    fn get_messages(&self) -> &Vec<MessageData>  {
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
        // Clean the action wrapper otherwise the previous commands will still have to be runned before the one he will calculate now
        action_wrapper.clear(self.id);

        // Get the Attacker robot, otherwise exit the function
        let robot = match world.allies_bot.get(&self.id) {
            Some(robot) => robot,
            None => return false,
        };
        let robot_position = robot.pose.position;
        let robot_direction = vectors::vector_from_angle(robot.pose.orientation);
        
        // Get the ball position, otherwise exit the function
        let ball_position = match world.ball.clone() {
            Some(ball) => ball.position.xy(),
            None => return false,
        };
        let robot_to_ball = ball_position - robot_position;
        let dot_with_ball = robot_direction.normalize().dot(&robot_to_ball.normalize());
        let dist_to_ball = robot_to_ball.norm();
        
        // Set the target shoot position to the center of the goal
        let target_shooting_position: Point2<f64> = world.geometry.enemy_goal.line.center();

        // Placement distance behind the ball
        let distance_to_go_behind_ball = 0.8;

        // Calculate the position behind the ball to prepare the shoot
        let behind_ball_position = ball_position + (ball_position - target_shooting_position).normalize() * distance_to_go_behind_ball; 
                
        // Check if the shooting trajectory will score
        let robot_shooting_trajectory = Line::new(robot_position, robot_position + robot_to_ball * 10.);
        let shooting_trajectory_will_score = match robot_shooting_trajectory.intersection_segments(&world.geometry.enemy_goal.line) {
            Ok(_) => true,
            Err(_) => false,
        };

        // Check if the ball is in the way of the robot placement, in this case we need to dodge the ball (TODO) 
        let ball_in_the_way = ball_in_trajectory(&world, self.id, behind_ball_position);
        match self.state {
            ShooterState::PlaceForShoot => {
                action_wrapper.push(self.id, MoveTo::new(behind_ball_position, vectors::angle_to_point(robot_position, target_shooting_position), 0., false, None));
                if shooting_trajectory_will_score 
                    && dot_with_ball > 0.95 // The robot is correctly facing the ball
                {
                    self.state = ShooterState::Shoot;
                }
            },
            ShooterState::Shoot => {
                // If the robot is close enough to the ball, shoot
                let kick: Option<Kick> = if dist_to_ball < (world.geometry.robot_radius + world.geometry.ball_radius + 0.002) { 
                    Some(Kick::StraightKick {  power: 4. }) 
                }else {None};
                action_wrapper.push(self.id, MoveTo::new(ball_position, vectors::angle_to_point(robot_position,target_shooting_position), 1.,  true, kick));

                // If the ball is in the way or the robot is too far from the ball, go back to the placement state
                if ball_in_the_way || dist_to_ball > distance_to_go_behind_ball+0.1 {
                    self.state = ShooterState::PlaceForShoot;
                }
            }
        };
        false
    }

}