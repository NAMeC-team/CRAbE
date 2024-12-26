use std::future::{Future, IntoFuture};
use std::os::linux::raw::stat;
use std::time::{Duration, Instant};
use log::warn;
use nalgebra::{distance, min, Point2};
use crabe_framework::data::referee::{Referee, RefereeCommand};
use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crabe_framework::data::world::game_state::GameState::{Halted, Stopped, Running};
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, World};
use crate::data::FilterData;
use crate::post_filter::PostFilter;

const MIN_DIST_BALL_TOUCH: f64 = 0.05;
const MIN_DIST_BALL_MOVED: f64 = MIN_DIST_BALL_TOUCH; // see the rulebook, section 5.4

const ACTION_TIME_LIMIT_SECS: u64 = 10;

pub struct GameControllerPostFilter {
    ref_cmd: RefereeCommand,
    cond_transition: Option<Box<dyn Fn(&StateData) -> bool>>,
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        Self {
            ref_cmd: RefereeCommand::Halt,
            cond_transition: None,
        }
    }
}

struct StateData<'a> {
    ball_pos: Point2<f64>,
    world: &'a World,
}

/// dev note: why not put the return type into a type alias ?
/// this feature is still unstable, so we have to copy it around

fn maker_all(ball_ref_pos: Point2<f64>, secs: u64) -> impl Fn(&StateData) -> bool {
    let ball_moved = maker_ball_moved(ball_ref_pos);
    let timing = maker_timer(secs);
    let robot_touched_ball = maker_robot_touched_ball();
    return move |state_data| {
        timing(state_data) || ball_moved(state_data) || robot_touched_ball(state_data)
    }
}

fn maker_timer(secs: u64) -> impl Fn(&StateData) -> bool {
    let timer = Instant::now();
    return move |_| {
        timer.elapsed() > Duration::from_secs(secs)
    }
}

fn maker_ball_moved(ball_ref_pos: Point2<f64>) -> impl Fn(&StateData) -> bool {
    return move |state_data: &StateData| -> bool {
        distance(&state_data.ball_pos, &ball_ref_pos) > MIN_DIST_BALL_MOVED
    }
}

fn maker_robot_touched_ball() -> impl Fn(&StateData) -> bool {
    return move |state_data: &StateData| {
        if let Some(ball) = &state_data.world.ball {
            // compute the closest enemy and ally to ball
            let opt_c_ally_to_ball = ball.closest_ally_robot(&state_data.world);
            let opt_c_enemy_to_ball = ball.closest_enemy_robot(&state_data.world);

            let min_dist = match (opt_c_ally_to_ball, opt_c_enemy_to_ball) {
                (Some(c_ally), Some(c_enemy)) => {
                    // check, for the closest robot (in terms of distance)
                    // if its distance to the ball meets the condition
                    let d_a = c_ally.2;
                    let d_e= c_enemy.2;
                    d_a.min(d_e)
                }
                (Some(c_ally), None) => c_ally.2,
                (None, Some(c_enemy)) => c_enemy.2,

                // no robot on the field ? that shouldn't happen in a real game
                // the min dist given in this case is meaningless
                (None, None) => 42.
            };
            
            return min_dist < MIN_DIST_BALL_TOUCH
        }
        false
    }
}

impl GameControllerPostFilter {
    fn transition(&mut self, referee: &Referee, world: &World) -> GameState {
        let cur_state = world.data.ref_orders.state;
        
        // don't apply

        return match (cur_state, &referee.command, &self.cond_transition) {
            // Halt
            (Halted(HaltedState::Halt), RefereeCommand::Stop, _) => { Stopped(StoppedState::Stop) }

            // Timeout
            (Halted(HaltedState::Timeout(_)), RefereeCommand::Stop, _) => { Stopped(StoppedState::Stop) }

            // Stop
            (Stopped(StoppedState::Stop), RefereeCommand::BallPlacement(team), _) => { Stopped(StoppedState::BallPlacement(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::PrepareKickoff(team), _) => { Stopped(StoppedState::PrepareKickoff(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::PreparePenalty(team), _) => { Stopped(StoppedState::PreparePenalty(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::ForceStart, _) => { Running(RunningState::Run) }
            (Stopped(StoppedState::Stop), RefereeCommand::Timeout(team), _) => { Halted(HaltedState::Timeout(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::DirectFree(team), _) => {
                if let Some(ball) = &world.ball {
                    self.cond_transition = Some(Box::new(maker_all(ball.position_2d(), ACTION_TIME_LIMIT_SECS)));
                }
                Running(RunningState::FreeKick(*team))
            }

            // Ball placement
            (Stopped(StoppedState::BallPlacement(_)), RefereeCommand::Stop, _) => { Stopped(StoppedState::Stop) }
            // todo : command sent when ball placement ends ?
            // no, on success, returns to Stop and sends FreeKick
            
            // Kickoff
            (Stopped(StoppedState::PrepareKickoff(team)), RefereeCommand::NormalStart, _) => {
                let ball_pos = match &world.ball {
                    Some(ball) => ball.position_2d(),
                    None => {
                        warn!("No ball in world, cannot guess when will the kickoff finish. Taking default (0., 0.) point as reference");
                        Point2::origin()
                    }
                };
                self.cond_transition = Some(Box::new(maker_all(ball_pos, ACTION_TIME_LIMIT_SECS)));
                Running(RunningState::KickOff(team))
            }
            
            (Running(RunningState::KickOff(team)), RefereeCommand::NormalStart, Some(updater)) => {
                return if let Some(ball) = &world.ball {
                    let switch_state = updater(&StateData { ball_pos: ball.position_2d(), world});
                    if switch_state {
                        self.cond_transition = None;
                        Running(RunningState::Run)
                    } else {
                        Running(RunningState::KickOff(team))
                    }
                } else {
                    warn!("No ball detected, staying in current state");
                    cur_state 
                }
            }

            // PreparePenalty
            (Stopped(StoppedState::PreparePenalty(team)), RefereeCommand::NormalStart, _) => { Running(RunningState::Penalty(team)) }
            
            // Penalty (todo, this one is more complicated)
            // (Running(RunningState::Penalty(team)), RefereeCommand::NormalStart, Some(updater)) => {
            //     let switch_state = updater();
            //     return if switch_state {
            //         
            //     }
            // }

            // FreeKick (time-dependent ?)
            (Running(RunningState::FreeKick(team)), RefereeCommand::DirectFree(_), Some(updater)) => {
                if let Some(ball) = &world.ball {
                    let switch_state = updater(&StateData { ball_pos: ball.position_2d(), world });
                    return if switch_state {
                        self.cond_transition = None;
                        Running(RunningState::Run)
                    } else {
                        Running(RunningState::FreeKick(team))
                    }
                } else {
                    warn!("No ball detected, staying in current state");
                    cur_state
                }
            }

            // any running state can lead to a Stop
            // (this performs the Run -> Stop transition as well)
            (Running(_), RefereeCommand::Stop, _) => {
                println!("{:?}", referee.game_events.last());
                Stopped(StoppedState::Stop)
            }

            // The human referee can trigger Stop from any state
            (_, RefereeCommand::Stop, _) => {
                self.cond_transition = None;
                Stopped(StoppedState::Stop)
            }

            // any state can lead to Halt
            (_, RefereeCommand::Halt, _) => {
                self.cond_transition = None;
                Halted(HaltedState::Halt)
            }

            (_, _, _) => {
                // todo: handle rollback ?
                warn!("Unknown transition (should implement rollback ?)");
                println!("({:?}, {:?}, {:?})", &cur_state, &referee.command, &self.cond_transition.is_some());
                 self.cond_transition = None; Halted(HaltedState::Halt) } }
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        if let Some(referee) = filter_data.referee.last() {
            // dbg!(&referee.command_timestamp);
            
            // apply transition to state machine only if
            // - a new ref command is received
            // - the current state must be refreshed with new data
            if self.ref_cmd != referee.command || self.cond_transition.is_some() {
                let prev_state = world.data.ref_orders.state;
                world.data.ref_orders.state = self.transition(referee, world);
                println!("{:?} -> {:?} (enemies : {:?})", prev_state, &world.data.ref_orders.state, world.enemies_bot.len());
                self.ref_cmd = referee.command;
            }
        }
    }
}