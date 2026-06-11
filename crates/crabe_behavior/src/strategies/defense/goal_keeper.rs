use crate::behaviors::actions::charge::ChargeAction;
use crate::behaviors::actions::kick::KickAction;
use crate::behaviors::actions::move_to::MoveTo;
use crate::behaviors::actions::target::ball::ComputeBallTarget;
use crate::behaviors::blackboard::RobotIntentWriters;
use crate::behaviors::blackboard::{InputPort, OutputPort, StateSlots};
use crate::behaviors::{Node, action, cond, par, sel, seq};
use crate::strategies::Strategy;
use crate::strategies::defense::goal_keeper::clamp_target::ClampGoalKeepingTarget;
use crate::strategies::defense::goal_keeper::compute_target::ComputeGoalKeepingTarget;
use crabe_framework::data::output::Kick;
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, Robot, World};
use crabe_math::shape::Line;
use crabe_math::vectors::vector_from_angle;
use nalgebra::Point2;
mod clamp_target;
mod compute_target;

const SLOW_BALL_VELOCITY: f64 = 1.0;

#[derive(Debug)]
pub struct GoalKeeper;

impl GoalKeeper {
    pub fn new() -> GoalKeeper {
        GoalKeeper
    }
}

impl Strategy for GoalKeeper {
    fn cost(&self, robot: &Robot<AllyInfo>, world: &World) -> f64 {
        0.0
    }

    fn build_tree(&self, state: &StateSlots, intents: &RobotIntentWriters) -> Node {
        let charge_and_move_to_ball = par(vec![
            action(ChargeAction::new(intents.charge)), // charges kicker simultaneously
            action(MoveTo::new(state.target.input(), intents.movement)),
        ]);

        let clear_ball = seq(vec![
            cond("BallInPenalty", |ctx| {
                ctx.world().ball.as_ref().map_or(false, |b| {
                    ctx.world()
                        .geometry
                        .ally_penalty
                        .is_inside(&b.position_2d())
                })
            }),
            cond("BallIsSlow", |ctx| {
                ctx.world()
                    .ball
                    .as_ref()
                    .map_or(false, |b| b.velocity.norm() < SLOW_BALL_VELOCITY)
            }),
            action(ComputeBallTarget::new(state.target.output())),
            charge_and_move_to_ball,
            action(KickAction::new(
                intents.action,
                Kick::ChipKick { power: 4.0 },
            )),
        ]);

        let normal_goalkeeping = seq(vec![
            action(ComputeGoalKeepingTarget::new(state.target.output())),
            action(MoveTo::new(state.target.input(), intents.movement)),
        ]);

        sel(vec![clear_ball, normal_goalkeeping])
    }
}

fn follow_velocity_trajectory(ball: &Ball, world: &World) -> Option<Point2<f64>> {
    let ball_pos = ball.position_2d();
    let ball_velocity_trajectory =
        Line::new(ball_pos, ball_pos + ball.velocity.xy().normalize() * 100.);
    if ball.velocity.norm() > 0.01 {
        if let Ok(intersection) = world
            .geometry
            .ally_goal
            .line
            .intersection_segments(&ball_velocity_trajectory)
        {
            return Some(intersection);
        }
    }
    None
}

fn follow_enemy_to_ball_trajectory(
    ball: &Ball,
    world: &World,
    enemy: &Robot<EnemyInfo>,
) -> Option<Point2<f64>> {
    let ball_pos = ball.position_2d();
    let enemy_to_ball = ball_pos - enemy.position();
    let trajectory = Line::new(ball_pos, ball_pos + enemy_to_ball.normalize() * 100.);
    if let Ok(intersection) = world
        .geometry
        .ally_goal
        .line
        .intersection_segments(&trajectory)
    {
        return Some(intersection);
    }
    None
}

fn follow_enemy_direction(world: &World, enemy: &Robot<EnemyInfo>) -> Option<Point2<f64>> {
    let dir = vector_from_angle(enemy.orientation()) * 100.;
    let trajectory = Line::new(enemy.position(), enemy.position() + dir);
    if let Ok(intersection) = world
        .geometry
        .ally_goal
        .line
        .intersection_segments(&trajectory)
    {
        return Some(intersection);
    }
    None
}
