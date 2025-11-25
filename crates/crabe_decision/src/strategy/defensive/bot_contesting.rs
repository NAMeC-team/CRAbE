use crate::{
    action::{move_to::MoveTo, ActionWrapper}, strategy::Strategy, utils::closest_bot_to_point
};
use crabe_framework::data::{
    tool::ToolData,
    world::World,
};
use nalgebra::Point2;
use std::time::Instant;
use crabe_math::vectors::angle_to_point;

const DISTANCE_TO_BALL:f64 = 0.06;
const DINANCE_TO_ROBOT:f64 = 0.2;
const ANGULAR_DIFFERENCE:f64 = 2.0;
/// The BotContesting struct represents a strategy that commands a robot to move in a BotContesting shape
/// in a counter-clockwise. It is used for testing purposes.
pub struct BotContesting {
    /// The id of the robot to move.
    id: u8,
    time: Instant,
}


impl BotContesting {
    /// Creates a new BotContesting instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self { 
            id,
            time: Instant::now(),    
        }
    }
}

impl Strategy for BotContesting {
    fn name(&self) -> &'static str {
        "BotContesting"
    }


    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        action_wrapper.clear(self.id);
        let ball = match &world.ball {
            Some(b) => b,
            None => {
                return false;
            }
        };

        let ball_pos = &ball.position_2d();

        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                return false;
            }
        };
        let robot_pos = &robot.pose;

        // We take the closest enemy to the ball and we calculate the direction of the shot by just looking at his orientation
        let enemy =  &match closest_bot_to_point(world.enemies_bot.values().collect(), *ball_pos){
            Some(closest_enemy) => closest_enemy,
            None => {
                return false;
            }
        };
        let enemy_pos = &enemy.pose;

        if (ball_pos - enemy_pos.position).norm() > 0.2 {
            // Ball is too far to enemy
            return false;
        }

        let target ;
        let mut dribble = 0.0;
        let enemy_to_ball = ball_pos - enemy_pos.position;
        let robot_to_ball = ball_pos - robot_pos.position;
        let dot_robot_and_enemy_to_ball = robot_to_ball.normalize().dot(&enemy_to_ball.normalize());


        if dot_robot_and_enemy_to_ball > -0.1 {
            let enemy_to_goal = world.geometry.ally_goal.line.center() - enemy_pos.position;
            let distance_to_robot = 
                (world.geometry.robot_radius + DINANCE_TO_ROBOT) / enemy.distance(&world.geometry.ally_goal.line.center());

            target = 
                enemy_pos.position - Point2::new(enemy_to_goal.x, enemy_to_goal.y)*(-distance_to_robot);
        } else {            
            dribble = 1.0;
            let distance_to_robot = (DISTANCE_TO_BALL+ world.geometry.ball_radius + world.geometry.robot_radius)/enemy.distance(ball_pos);
            target = enemy_pos.position - Point2::new(enemy_to_ball.x, enemy_to_ball.y)*(-distance_to_robot);    
        }
        
        let mut angle;
        if robot.distance(&ball_pos) < 0.3 {
            angle = angle_to_point(robot_pos.position, *ball_pos);
            if self.time.elapsed().as_millis()%2 == 0{
                angle = angle+ANGULAR_DIFFERENCE;
            } else {
                angle = angle-ANGULAR_DIFFERENCE;
            }
        } else {
            angle = angle_to_point(robot_pos.position, enemy_pos.position);
        }

        let fast = robot.distance(&enemy.pose.position) > 1.;

        action_wrapper.push(self.id,  MoveTo::new_all_params(Point2::new(target.x, target.y), angle , dribble , false , None, fast, true ));
        return false;
    }
}
