use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::{game_state::{GameState, RunningState}, Ball, BallTouchInfo, TeamColor, World};
use crabe_decision::utils::closest_bot_to_point;

pub struct BallFilter;

const MIN_ACCELERATION_TO_SWITCH_POSSESSION: f64 = 1.;
const MIN_DISTANCE_DIFFERENCE_TO_SWITCH_POSSESSION: f64 = 0.1;
const MAX_DISTANCE_DIFFERENCE_TO_SWITCH_POSSESSION: f64 = 0.3;
const MAX_DIFFERENCE_VELOCITY_TO_SWITCH_POSSESSION: f64 = 0.1;
const DOT_DIFFERENCE_TO_SWITCH_POSSESSION: f64 = 0.75;


fn calculated_possession(ball: &mut Ball, world: &World) {
    let ball_world = match &world.ball {
        Some(b) => b,
        None => {
            ball.possession = None; return;}
    };
    ball.possession = ball_world.possession;
    let state = world.data.ref_orders.state;

    // ALLIES PART
    let bot_ally = match closest_bot_to_point(world.allies_bot.values().collect(), ball.position.xy()) {
        Some(bot) => bot,
        None => {ball.possession = None; return;}
    };
    let ally_color = world.team_color;

    // ENEMIES PART
    let bot_enemy = match closest_bot_to_point(world.enemies_bot.values().collect(), ball.position.xy()) {
        Some(bot) => bot,
        None => {ball.possession = None; return;}
    
    };
    let enemy_color = if ally_color == TeamColor::Yellow { TeamColor::Blue } else { TeamColor::Yellow };

    let ally_distance = bot_ally.distance(&ball.position.xy());
    let enemy_distance = bot_enemy.distance(&ball.position.xy());

    

    ///////////////////////////////////
    // CALCULATE POSSESSION BY BALL VELOCITY
    ///////////////////////////////////
    let ally_possession:bool = ally_distance<MAX_DISTANCE_DIFFERENCE_TO_SWITCH_POSSESSION 
                            && (bot_ally.velocity.linear - ball.velocity.xy()).norm() < MAX_DIFFERENCE_VELOCITY_TO_SWITCH_POSSESSION 
                            && ball.velocity.xy().dot(&bot_ally.velocity.linear) > DOT_DIFFERENCE_TO_SWITCH_POSSESSION;

    let enemy_possession:bool = enemy_distance<MAX_DISTANCE_DIFFERENCE_TO_SWITCH_POSSESSION
                            && (bot_enemy.velocity.linear - ball.velocity.xy()).norm() < MAX_DIFFERENCE_VELOCITY_TO_SWITCH_POSSESSION 
                            && ball.velocity.xy().dot(&bot_enemy.velocity.linear) > DOT_DIFFERENCE_TO_SWITCH_POSSESSION;
    
    if ally_possession && !enemy_possession{
        ball.possession = Some(ally_color);
    }
    else if enemy_possession && !ally_possession{
        ball.possession = Some(enemy_color);
    } else {
        ///////////////////////////////////
        // CALCULATE POSSESSION BY BALL ACCELERATION
        ///////////////////////////////////
        if ball.acceleration.norm() > MIN_ACCELERATION_TO_SWITCH_POSSESSION {

            // DETERMINE THE COLOR OF THE ROBOT THAT IS CLOSER TO THE BALL WHEN ACC IS HIGH
            if ally_distance + MIN_DISTANCE_DIFFERENCE_TO_SWITCH_POSSESSION < enemy_distance {
                ball.possession = Some(ally_color);
            } else {
                ball.possession = Some(enemy_color);
            }
        }
    }
    

    ///////////////
    // RE CALCULATE POSSESSION BY GAME STATE
    ///////////////
    if let GameState::Running(running_state) = state{
        match running_state {
            RunningState::FreeKick(val)
            | RunningState::KickOff(val)
            | RunningState::Penalty(val) => {
                ball.possession = Some(val);
            }
            _ => {}
        }
    }
}

fn calculate_last_touch(filter_data: &FilterData, ball: &mut Ball, world: &World) {
    let ball_world = match &world.ball {
        Some(b) => b,
        None => {
            ball.last_touch = None; return;}
    };
    let ball_dist = world.geometry.robot_radius+world.geometry.ball_radius + 0.01;
    ball.last_touch = ball_world.last_touch.clone();
    let closest_ally = closest_bot_to_point(world.allies_bot.values().collect(), ball.position.xy());
    let closest_enemy = closest_bot_to_point(world.enemies_bot.values().collect(), ball.position.xy());
    match (closest_ally, closest_enemy) {
        (Some(ally), Some(enemy)) => {
            let distance_ally = ally.distance(&ball.position.xy());
            let distance_enemy = enemy.distance(&ball.position.xy());
            if distance_ally < distance_enemy {
                if distance_ally < ball_dist{
                    ball.last_touch = Some(BallTouchInfo {
                        robot_id: ally.id,
                        team_color: world.team_color,
                        timestamp: filter_data.ball.data.timestamp,
                        position: ball.position,
                    });
                }
            } else if distance_enemy < ball_dist{
                ball.last_touch = Some(BallTouchInfo {
                    robot_id: enemy.id,
                    team_color: world.team_color.opposite(),
                    timestamp: filter_data.ball.data.timestamp,
                    position: ball.position,
                });
            }
        }
        (Some(ally), None) => {
            if ally.distance(&ball.position_2d()) < ball_dist{
                ball.last_touch = Some(BallTouchInfo {
                    robot_id: ally.id,
                    team_color: world.team_color,
                    timestamp: filter_data.ball.data.timestamp,
                    position: ball.position,
                });
            }
        }
        (None, Some(enemy)) => {
            if enemy.distance(&ball.position_2d())  < ball_dist{
                ball.last_touch = Some(BallTouchInfo {
                    robot_id: enemy.id,
                    team_color: world.team_color.opposite(),
                    timestamp: filter_data.ball.data.timestamp,
                    position: ball.position,
                });
            }
        }
        (None, None) => {}
    }
}

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let mut ball = filter_data.ball.data.clone();
        calculated_possession(&mut ball, &world);
        calculate_last_touch(&filter_data, &mut ball, &world);
        world.ball = Some(ball);
    }
}
