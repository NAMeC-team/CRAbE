use std::cell::Ref;
use std::time::{Duration, Instant};
use log::{error, warn};
use nalgebra::{distance, Point2};
use crate::data::referee::{Referee, RefereeCommand};
use crate::data::referee::event::{BallLeftField, Event, GameEvent, GameEventType};
use crate::data::state_handler::{GameStateBranch, GameStateData};
use crate::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crate::data::world::{Ball, Team, TeamColor, World};

/// Checks whether the ball moved from its designated position
/// It is based on the designated position sent by the referee, if there is one
/// This function should only be called for states that require it
/// (such as the FreeKick state, that implies the ball must be placed
/// at a specific location to perform the free kick)
fn ball_moved_from_designated_pos(designated_position: &Point2<f64>, ball_opt: &Option<Ball>) -> bool {
    return if let Some(ball) = &ball_opt {
        let dist = distance(&ball.position.xy(), &designated_position.xy());
        dist >= 0.05
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
                     time_based_refresh: &mut bool,
                     _latest_data: &mut GameStateData) -> GameState {

        *time_based_refresh = false;
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
                     _time_based_refresh: &mut bool,
                     _latest_data: &mut GameStateData) -> GameState {
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
    fn was_goal_scored(&self, world: &World, referee: &Referee, latest_data: &GameStateData) -> Option<TeamColor> {
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
                     time_based_refresh: &mut bool,
                     latest_data: &mut GameStateData) -> GameState {

        *time_based_refresh = false;

        // determine the reason of this Stop command

        // probably because of a kickoff incoming ? or the game hasn't started yet
        match referee.next_command {
            Some(next) => {
                match next {
                    // normally, we should be able to fetch which team will perform the next
                    // kickoff, so we can consider that the Stop state right after a goal
                    // is the same as preparing for a kickoff
                    RefereeCommand::PrepareKickoff(for_team) => {
                        return GameState::Stopped(StoppedState::PrepareKickoff(for_team))
                    }
                    _ => return GameState::Stopped(StoppedState::PrepareForGameStart)
                }
            }
            None => {  }
        }

        // otherwise, it might be because of an event that occurred
        // (ball out of field, double touch foul etc...)
        return if let Some(game_event) = referee.game_events.last() {
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

                Event::PossibleGoal(_) => {
                    // is it because a goal was scored ?

                    // Dev note : When a goal is scored, these are the transitions happening
                    //  -> Halt | Human referee validates goal
                    //  -> Stop | One of the following happens
                    //      - Goal is validated, scores are updated, bots must prepare for kickoff
                    //      - Goal is not validated, we move forward onto a free kick
                    //
                    // To simplify, we'll just say that we will be put in the PrepareKickoff state
                    // after a goal has been scored, to allow our robots to prepare themselves in advance
                    // It is not against the rules to do so (I believe)

                    if let Some(scoring_team) = self.was_goal_scored(world, referee, latest_data) {
                        // goal is accepted
                        StoppedState::PrepareKickoff(scoring_team.opposite())
                    } else {
                        // goal is refused, we move on to a free kick
                        // generally, the ref will provide the next command
                        match referee.next_command {
                            Some(cmd) => {
                                if let RefereeCommand::DirectFree(for_team) = cmd {
                                    StoppedState::PrepareFreekick(for_team)
                                } else {
                                    error!("Next command after a PossibleGoal is not a direct free ??");
                                    StoppedState::PrepareFreekick(world.team_color.opposite())
                                }
                            }
                            None => {
                                warn!("Referee did not provide next command for the following free kick");
                                StoppedState::PrepareFreekick(world.team_color.opposite())
                            }
                        }
                    }
                }

                Event::AttackerDoubleTouchedBall(foul) => {
                    StoppedState::PrepareFreekick(foul.by_team.opposite())
                }

                // Non-Stopping Fouls that can be ignored (or that never happen during a Stop state)
                // TODO: more events might need management
                // Event::AttackerTouchedBallInDefenseArea(_) => {}
                // Event::BotKickedBallTooFast(_) => {}
                // Event::BotCrashUnique(_) => {}
                // Event::BotCrashDrawn(_) => {}
                // Event::DefenderTooCloseToKickPoint(_) => {}
                // Event::BotTooFastInStop(_) => {}
                // Event::BotInterferedPlacement(_) => {}
                // Event::Goal(_) => {}  // do not use this, not recommended
                // Event::InvalidGoal(_) => {}
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
        } else {
            GameState::Stopped(StoppedState::Stop)
        }
    }
}

/// Issued when the operator forces the game to resume
pub struct ForceStartStateBranch;

impl GameStateBranch for ForceStartStateBranch {
    fn process_state(&self,
                     _world: &World,
                     _referee: &Referee,
                     time_based_refresh: &mut bool,
                     latest_data: &mut GameStateData) -> GameState {
        *time_based_refresh = false;
        // implies that the first kick-off was issued already
        latest_data.kicked_off_once = true;
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
                     time_based_refresh: &mut bool,
                     latest_data: &mut GameStateData) -> GameState {
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
                    *time_based_refresh = false;
                    GameState::Running(RunningState::Run)
                }
                else if let Some(time_remaining) = referee.current_action_time_remaining { // todo: we could use referee.current_action_time_remaining instead
                    // -> 10 seconds have elapsed
                    if time_remaining.num_seconds() <= -1 { // TODO (bug): <= -1 is valid only when .num_seconds() attains -2
                        *time_based_refresh = false;
                        GameState::Running(RunningState::Run)
                    } else {
                        // otherwise we're still doing a kickoff
                        *time_based_refresh = true;
                        GameState::Running(RunningState::KickOff(of_team))
                    }
                } else {
                    warn!("Referee did not provide time remaining for kickoff");
                    *time_based_refresh = true;
                    GameState::Running(RunningState::KickOff(of_team))
                }
            }

            RefereeCommand::PreparePenalty(of_team) => {
                // yes, it is very similar to how we handle PrepareKickoff above
                // but penalties have their own particularities, which are not
                // completely handled (whether it's failed or successful, for example)
                // the following will be the minimum required, we can change it later
                if ball_moved_from_designated_pos(&get_penalty_designated_ball_pos(of_team, referee), &world.ball) {
                    *time_based_refresh = false;
                    GameState::Running(RunningState::Run)
                }
                else if let Some(time_remaining) = referee.current_action_time_remaining {
                    // -> 10 seconds have elapsed
                    if time_remaining.num_seconds() <= -1 {
                        *time_based_refresh = false;
                        GameState::Running(RunningState::Run)
                    } else {
                        // otherwise we're still doing a penalty
                        *time_based_refresh = true;
                        GameState::Running(RunningState::Penalty(of_team))
                    }
                } else {
                    warn!("Referee did not provide time remaining for penalty");
                    *time_based_refresh = true;
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
                     _time_based_refresh: &mut bool,
                     _latest_data: &mut GameStateData) -> GameState {
        warn!("Deprecated state has been used");
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
                     time_based_refresh: &mut bool,
                     latest_data: &mut GameStateData) -> GameState {
        // If the ball moved at least 0.05 meters from its designated position,
        // the kicker bot is considered to have touched the ball, and the game can resume normally
        // precondition: the last designated pos has been provided by the referee
        if let Some(designated_pos) = &latest_data.last_designated_pos {
            if ball_moved_from_designated_pos(designated_pos, &world.ball) {
                *time_based_refresh = false;
                return GameState::Running(RunningState::Run)
            }
            return GameState::Running(RunningState::FreeKick(self.for_team))
        }
        // otherwise, check if we are still in the freekick state
        else if let Some(time_remaining) = referee.current_action_time_remaining {
            // If 10 seconds haven't passed
            if !time_remaining.num_seconds() <= -1 {
                // There is still some time for the team to perform the freekick
                *time_based_refresh = true;
                GameState::Running(RunningState::FreeKick(self.for_team))
            } else {
                // Free kick time has ended, moving on to the next state
                // it is required to update the state in this case, because the referee
                // will not send a new command telling us we can resume normal play
                *time_based_refresh = false;
                GameState::Running(RunningState::Run)
            }
        } else {
            warn!("Referee did not provide time remaining for free kick");
            *time_based_refresh = true;
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
                     _time_based_refresh: &mut bool,
                     _latest_data: &mut GameStateData) -> GameState {
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
                     _time_based_refresh: &mut bool,
                     _latest_data: &mut GameStateData) -> GameState {

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
                     _time_based_refresh: &mut bool,
                     _latest_data: &mut GameStateData) -> GameState {
        //TODO: improve this branch, with more StoppedState penalty states
        // to define precisely what we should be doing
        GameState::Stopped(StoppedState::PreparePenalty(self.for_team))
    }
}