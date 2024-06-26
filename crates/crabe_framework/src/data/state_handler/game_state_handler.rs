use std::cell::Ref;
use std::time::{Duration, Instant};
use log::{error, warn};
use nalgebra::{distance, Point2};
use crate::data::referee::{Referee, RefereeCommand};
use crate::data::referee::event::{BallLeftField, Event, GameEvent, GameEventType};
use crate::data::state_handler::{GameStateBranch, StateData};
use crate::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crate::data::world::{Ball, Team, TeamColor, World};

/// Checks whether the ball moved from its designated position
/// It is based on the designated position sent by the referee, if there is one
/// This function should only be called for states that require it
/// (such as the FreeKick state, that implies the ball must be placed
/// at a specific location to perform the free kick)
fn ball_moved_from_designated_pos(designated_position: &Point2<f64>, ball_opt: &Option<Ball>) -> bool {
    return if let Some(ball) = &ball_opt {
        distance(&ball.position.xy(), &designated_position.xy()) >= 0.05
    } else {
        false
    }
}

/// Returns where the ball should be placed
/// if team `for_team` is going to do a penalty
fn get_penalty_designated_ball_pos(for_team: TeamColor, referee: &Referee) -> Point2<f64> {
    referee.designated_position.unwrap_or(|| -> Point2<f64> {
        // TODO: penalty position
        Point2::new(0., 0.)
    }())
}

/// Definition of branches to follow for each
/// referee command issued

/// Halt command branch, often triggered manually, but sometimes
/// in case of a foul, or other circumstances
pub struct HaltStateBranch;

impl GameStateBranch for HaltStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        return GameState::Halted(HaltedState::Halt);
    }
}

pub struct TimeoutStateBranch {
    for_team: TeamColor
}

impl TimeoutStateBranch {
    pub fn new(for_team: TeamColor) -> Self { Self { for_team } }
}

impl GameStateBranch for TimeoutStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        // TODO: maybe send information about the timeout
        GameState::Halted(HaltedState::Timeout(self.for_team))
    }
}

/// Stop command branch, used when robots should slow down.
/// It's a transition state used for most of the actions
pub struct StopStateBranch;


impl StopStateBranch {
    /// When the ball leaves the playing field by touching a goal line,
    /// determine whether this will result in a corner kick or a goal kick,
    /// depending on the faulting team
    ///
    /// TODO: this assembly of logic looks fat, maybe it could be reduced
    fn goal_or_corner_kick(&self, world: &World, data: &BallLeftField) -> StoppedState {
        return if let Some(ball_last_location) = &data.location {
            let color_team_positive_side = world.data.positive_half;
            let faulting_team = data.by_team;
            let our_team_color = world.team_color;

            // our team is on x-positive side
            if color_team_positive_side == our_team_color {
                // ball positive side (our side)
                if ball_last_location.x > 0. {
                    if faulting_team == our_team_color {
                        StoppedState::CornerKick(our_team_color.opposite())
                    } else {
                        StoppedState::GoalKick(our_team_color)
                    }
                }
                // ball negative side (enemy side)
                else {
                    if faulting_team == our_team_color {
                        StoppedState::GoalKick(our_team_color.opposite())
                    } else {
                        StoppedState::CornerKick(our_team_color)
                    }
                }
            }
            // our team is on x-negative side
            else {
                // ball positive side (enemy side)
                if ball_last_location.x > 0. {
                    if faulting_team == our_team_color {
                        StoppedState::CornerKick(our_team_color.opposite())
                    } else {
                        StoppedState::GoalKick(our_team_color)
                    }
                }
                // ball negative side (our side)
                else {
                    if faulting_team == our_team_color {
                        StoppedState::CornerKick(our_team_color.opposite())
                    } else {
                        StoppedState::GoalKick(our_team_color)
                    }
                }
            }
        } else {
            // If we don't know where the ball went out, don't try to guess and return a bland state
            StoppedState::Stop
        }
    }

    /// Checks whether a goal has been scored
    fn was_goal_scored(&self, world: &World, referee: &Referee, latest_data: &StateData) -> Option<TeamColor> {
        let our_team_color = world.team_color;
        return if referee.ally.score > latest_data.ally_score {
            Some(our_team_color)
        } else if referee.enemy.score > latest_data.enemy_score {
            Some(our_team_color.opposite())
        } else {
            None
        }
    }
}

impl GameStateBranch for StopStateBranch {
    fn process_state(&self,
                     world: &World,
                     referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     latest_data: &mut StateData) -> GameState {

        // handle first kickoff of the match
        if !latest_data.kicked_off_once {
            return match referee.next_command {
                None => GameState::Stopped(StoppedState::PrepareForGameStart),
                Some(next) => {
                    match next {
                        // normally, we should be able to fetch which team will perform the next
                        // kickoff, so we can consider that the Stop state right after a goal
                        // is the same as preparing for a kickoff
                        RefereeCommand::PrepareKickoff(for_team) => {
                            GameState::Stopped(StoppedState::PrepareKickoff(for_team))
                        }
                        _ => GameState::Stopped(StoppedState::PrepareForGameStart)
                    }
                }
            }
        }

        // determine the reason of this Stop command

        // is it because a goal was scored ?
        // Dev note : When a goal is scored, these are the transitions happening
        //  -> Halt | Human referee validates goal
        //  -> Stop | Scores are updated, bots must prepare for kickoff
        // To simplify, we'll just say that we will be put in the PrepareKickoff state
        // after a goal is scored, to allow our robots to prepare themselves in advance
        // It is not against the rules to do so (I believe)
        else if let Some(scoring_team) = self.was_goal_scored(world, referee, latest_data) {
            return GameState::Stopped(StoppedState::PrepareKickoff(scoring_team.opposite()));
        }

        // otherwise, it might be because of an event that occurred
        // (ball out of field, double touch foul etc...)
        else if let Some(game_event) = referee.game_events.last() {
            let stopped_state: StoppedState = match &game_event.event {
                // Common occurrences in a match
                Event::BallLeftFieldTouchLine(data) => StoppedState::BallLeftFieldTouchLine(data.by_team),
                Event::BallLeftFieldGoalLine(data) => self.goal_or_corner_kick(world, data),
                Event::AimlessKick(data) => StoppedState::AimlessKick(data.by_team),

                // Stopping fouls
                Event::AttackerTooCloseToDefenseArea(_) |
                Event::DefenderInDefenseArea(_) |
                Event::BoundaryCrossing(_) |
                Event::KeeperHeldBall(_) |
                Event::BotDribbledBallTooFar(_) |
                Event::BotPushedBot(_) |
                Event::BotHeldBallDeliberately(_) |
                Event::BotTippedOver(_) => StoppedState::FoulStop,


                // The game is not progressing
                Event::NoProgressInGame(_) => StoppedState::NoProgressInGame,

                // Non-Stopping Fouls that can be ignored (or that never happen during a Stop state)
                // TODO: more events might need management
                // Event::AttackerTouchedBallInDefenseArea(_) => {}
                // Event::BotKickedBallTooFast(_) => {}
                // Event::BotCrashUnique(_) => {}
                // Event::BotCrashDrawn(_) => {}
                // Event::DefenderTooCloseToKickPoint(_) => {}
                // Event::BotTooFastInStop(_) => {}
                // Event::BotInterferedPlacement(_) => {}
                // Event::PossibleGoal(_) => {}
                // Event::Goal(_) => {}
                // Event::InvalidGoal(_) => {}
                // Event::AttackerDoubleTouchedBall(_) => {}
                // Event::PlacementSucceeded(_) => {}
                // Event::PenaltyKickFailed(_) => {}
                // Event::PlacementFailed(_) => {}
                // Event::MultipleCards(_) => {}
                // Event::MultipleFouls(_) => {}
                // Event::TooManyRobots(_) => {}
                // Event::BotSubstitution(_) => {}
                // Event::ChallengeFlag(_) => {}
                // Event::EmergencyStop(_) => {}
                // Event::UnsportingBehaviorMinor(_) => {}
                // Event::UnsportingBehaviorMajor(_) => {}
                // Event::DeprecatedEvent => {}
                _ => StoppedState::Stop
            };
            return GameState::Stopped(stopped_state);
        }


        // Default behaviour is to return a bland stop state,
        // in case of no recent event or first kickoff
        return GameState::Stopped(StoppedState::Stop);
    }
}

/// Issued when the operator forces the game to resume
pub struct ForceStartStateBranch;

impl GameStateBranch for ForceStartStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        // reset timer
        *timer_opt = None;
        return GameState::Running(RunningState::Run);
    }
}

/// When the game resumes normally, this command is issued,
/// As far as it has been tested (with the game controller),
/// this command is called after a PrepareKickoff or PreparePenalty
pub struct NormalStartStateBranch;

impl GameStateBranch for NormalStartStateBranch {
    fn process_state(&self,
                     world: &World,
                     referee: &Referee,
                     timer_opt: &mut Option<Instant>,
                     latest_data: &mut StateData) -> GameState {
        // Here is how the game starts (or resumes after a goal)
        // -> Halt
        // -> Stop | Robots place themselves on their side of field
        // -> PrepareKickoff | Kickoff preparation starts (2 seconds)
        // -> NormalStart | Kickoff lasts for 10 seconds

        latest_data.kicked_off_once = true;

        match latest_data.prev_ref_cmd {
            RefereeCommand::PrepareKickoff(of_team) => {
                // A kickoff ends in two ways

                // -> The ball moved from its designated position (ball is now in play)
                if ball_moved_from_designated_pos(&referee.designated_position.unwrap_or(Point2::new(0., 0.)), &world.ball) {
                    *timer_opt = None;
                    GameState::Running(RunningState::Run)
                }
                else if let Some(timer) = timer_opt {
                    // -> 10 seconds have elapsed
                    if timer.elapsed() > Duration::from_secs(10) {
                        *timer_opt = None;
                        GameState::Running(RunningState::Run)
                    } else {
                        // otherwise we're still doing a kickoff
                        GameState::Running(RunningState::KickOff(of_team))
                    }
                } else {
                    // if there's no timer defined,
                    // the kickoff just started, retain a timer to update
                    *timer_opt = Some(Instant::now());
                    GameState::Running(RunningState::KickOff(of_team))
                }
            }

            RefereeCommand::PreparePenalty(of_team) => {
                // yes, it is very similar to how we handle PrepareKickoff above
                // but penalties have their own particularities, which are not
                // completely handled (whether it's failed or successful, for example)
                // the following will be the minimum required, we can change it later
                if ball_moved_from_designated_pos(&get_penalty_designated_ball_pos(of_team, referee), &world.ball) {
                    *timer_opt = None;
                    GameState::Running(RunningState::Run)
                }
                else if let Some(timer) = timer_opt {
                    // -> 10 seconds have elapsed
                    if timer.elapsed() > Duration::from_secs(10) {
                        *timer_opt = None;
                        GameState::Running(RunningState::Run)
                    } else {
                        // otherwise we're still doing a penalty
                        GameState::Running(RunningState::Penalty(of_team))
                    }
                } else {
                    // if there's no timer defined,
                    // the penalty just started, retain a timer to update
                    *timer_opt = Some(Instant::now());
                    GameState::Running(RunningState::Penalty(of_team))
                }
            }
            _ => GameState::Running(RunningState::Run)
        }
    }
}

/// This branch is used as a placeholder if there is a RefereeCommand
/// that was not handled, or a deprecated command has been sent
pub struct DeprecatedStateBranch;

impl GameStateBranch for DeprecatedStateBranch {
    fn process_state(&self,
                     world: &World,
                     _referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        return world.data.ref_orders.state;
    }
}

pub struct FreekickStateBranch {
    for_team: TeamColor
}

impl FreekickStateBranch {
    pub fn new(for_team: TeamColor) -> Self { Self { for_team } }
}

impl GameStateBranch for FreekickStateBranch {
    fn process_state(&self,
                     world: &World,
                     referee: &Referee,
                     timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        // If the ball moved at least 0.05 meters from its designated position,
        // the kicker bot is considered to have touched the ball, and the game can resume normally
        if ball_moved_from_designated_pos(
            &referee.designated_position.unwrap_or(|| -> Point2<f64> {
                warn!("Free kick designated position was not given by referee");
                Ball::default().position_2d()
            }()),
            &world.ball) {
            *timer_opt = None;
            return GameState::Running(RunningState::Run)
        }
        // otherwise, check if we are still in the freekick state
        else if let Some(timer) = &timer_opt {
            // If 10 seconds haven't passed
            if timer.elapsed() < Duration::from_secs(10) {
                // There is still some time for the team to perform the freekick
                GameState::Running(RunningState::FreeKick(self.for_team))
            } else {
                // Free kick time has ended, moving on to the next state
                // it is required to update the state in this case, because the referee
                // will not send a new command telling us we can resume normal play
                *timer_opt = None;
                GameState::Running(RunningState::Run)
            }
        } else {
            // A freekick has just started, save a timer to measure the time
            *timer_opt = Some(Instant::now());
            GameState::Running(RunningState::FreeKick(self.for_team))
        }
    }
}

/// This branch is called when the referee issues the "PrepareKickoff" command
/// We actually set this state way earlier compared to the referee, because
/// the mentioned command is sent to give us a 2 seconds preparation for our robots,
/// even though we're already allowed to prepare ourselves
pub struct PrepareKickoffStateBranch {
    for_team: TeamColor
}

impl PrepareKickoffStateBranch {
    pub fn new(for_team: TeamColor) -> Self { Self { for_team } }
}

impl GameStateBranch for PrepareKickoffStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        GameState::Stopped(StoppedState::PrepareKickoff(self.for_team))
    }
}

pub struct BallPlacementStateBranch {
    by_team: TeamColor
}

impl BallPlacementStateBranch {
    pub fn new(by_team: TeamColor) -> Self { Self { by_team } }
}

impl GameStateBranch for BallPlacementStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {

        GameState::Stopped(StoppedState::BallPlacement(self.by_team))
    }
}

pub struct PreparePenaltyStateBranch {
    for_team: TeamColor
}

impl PreparePenaltyStateBranch {
    pub fn new(for_team: TeamColor) -> Self { Self { for_team } }
}

impl GameStateBranch for PreparePenaltyStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     _timer_opt: &mut Option<Instant>,
                     _latest_data: &mut StateData) -> GameState {
        //TODO: improve this branch, with more StoppedState penalty states
        // to define precisely what we should be doing
        GameState::Stopped(StoppedState::PreparePenalty(self.for_team))
    }
}