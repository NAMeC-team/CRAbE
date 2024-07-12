use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::{self, game_state::{GameState, RunningState}, Ball, TeamColor, World};
use crabe_decision::utils::closest_bot_to_point;

pub struct BallFilter;

fn calculated_possession(ball: &mut Ball, world: &World) {
    let ball_world = match &world.ball {
        Some(b) => b,
        None => {
            ball.possession = None; return;}
    };
    ball.possession = ball_world.possession;
    let state = world.data.ref_orders.state;
    if ball.acceleration.norm() > 1. {
        let bot_ally = match closest_bot_to_point(world.allies_bot.values().collect(), ball.position.xy()) {
            Some(bot) => bot,
            None => {ball.possession = None; return;}
        };
        let ally_color = world.team_color;
        let bot_enemy = match closest_bot_to_point(world.enemies_bot.values().collect(), ball.position.xy()) {
            Some(bot) => bot,
            None => {ball.possession = None; return;}
        
        };
        let enemy_color = if ally_color == TeamColor::Yellow { TeamColor::Blue } else { TeamColor::Yellow };
        if bot_ally.distance(&ball.position.xy()) < bot_enemy.distance(&ball.position.xy()) {
            ball.possession = Some(ally_color);
        } else {
            ball.possession = Some(enemy_color);
        }
    }
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

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        let mut ball = filter_data.ball.data.clone();
        calculated_possession(&mut ball, &world);
        world.ball = Some(ball);
    }
}