use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::{TeamColor, World};

use crate::data::referee::event::{Event, GameEvent};
use crate::data::referee::RefereeCommand;
use std::time::Instant;

#[derive(Debug)]
pub struct GameControllerPostFilter {
    previous_game_event: Option<GameEvent>,
    previous_event: Option<Event>,
    chrono: Option<Instant>, // TODO: Need to have an option here ?
    kicked_off_once: bool,
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        GameControllerPostFilter {
            previous_game_event: None,
            previous_event: None,
            chrono: Option::from(Instant::now()),
            kicked_off_once: false,
        }
    }
}

/// Contains all the possible actions that we may be able
/// to execute from a referee command
/// The job of these functions is just to change
/// the current world state according to the command
/// and the last event that occurred
impl GameControllerPostFilter {
    fn fix_yourself(&mut self) {
        if self.chrono == None {
            self.chrono = Option::from(Instant::now());
        }
    }

    fn halt_state_branch(world: &mut World) {
        // Halt event, all robots should stop
        // Done: 2/2
        // consider that timeout is equivalent to normal halt
        world.data.state = GameState::Halted(HaltedState::Halt);
        println!("Stop all robots");
    }

    fn stop_state_branch(
        previous_event_opt: &Option<Event>,
        world: &mut World,
        mut _kicked_off_once: bool,
    ) {
        //dbg!("w",&world);
        // TODO: the "world.team_color" isn't right I think
        if let Some(previous_event) = previous_event_opt {
            match previous_event {
                // Goal has been scored, prepare for next kickoff phase
                Event::Goal { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(world.team_color));
                    println!("Prepare for kickoff");
                }

                Event::BallLeftFieldTouchLine { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement(world.team_color));
                    println!("Ball got out of the field by the touch lines !");
                }

                Event::BallLeftFieldGoalLine { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement(world.team_color));
                    println!("Ball got out of the field by the goal lines !");
                }
                //TODO : check if all these events have to be stopped
                Event::AimlessKick(_) |
                Event::AttackerTooCloseToDefenseArea(_) |
                Event::DefenderInDefenseArea(_) |
                Event::BoundaryCrossing(_) |
                Event::KeeperHeldBall(_) |
                Event::BotDribbledBallTooFar(_) |
                Event::BotPushedBot(_) |
                Event::BotHeldBallDeliberately(_) |
                Event::BotTippedOver(_) |
                Event::AttackerTouchedBallInDefenseArea(_) |
                Event::BotKickedBallTooFast(_) |
                Event::BotCrashUnique(_) |
                Event::BotCrashDrawn(_) |
                Event::DefenderTooCloseToKickPoint(_) |
                Event::BotTooFastInStop(_) |
                Event::BotInterferedPlacement(_) |
                Event::PossibleGoal(_) |
                Event::InvalidGoal(_) |
                Event::AttackerDoubleTouchedBall(_) |
                Event::PlacementSucceeded(_) |
                Event::PenaltyKickFailed(_) |
                Event::NoProgressInGame(_) |
                Event::PlacementFailed(_) |
                Event::MultipleCards(_) |
                Event::MultipleFouls(_) |
                Event::TooManyRobots(_) |
                Event::BotSubstitution(_) |
                Event::ChallengeFlag(_) |
                Event::EmergencyStop(_) |
                Event::UnsportingBehaviorMinor(_) |
                Event::UnsportingBehaviorMajor(_) |
                Event::DeprecatedEvent => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement(world.team_color));
                }
            }
        } else {
            // Particularly, it should be None when we just started the match
            // Thus, it's a kickoff
            if !_kicked_off_once {
                world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(world.team_color));
                _kicked_off_once = true;
            } else {
                // this one's totally arbitrary
                // i don't understand how we can fetch a forced free kick from the commands
                // todo: fix what's mentioned above me (fix me !)
                world.data.state = GameState::Running(RunningState::FreeKick(world.team_color));
            }
        }
    }

    fn force_start_state_branch(
        previous_event_opt: &Option<Event>,
        world: &mut World,
        mut _chrono: Option<Instant>,
    ) {
        GameControllerPostFilter::normal_start_state_branch(previous_event_opt, world, _chrono)
    }
    fn normal_start_state_branch(
        previous_event_opt: &Option<Event>,
        world: &mut World,
        mut _chrono: Option<Instant>,
    ) {
        //TODO team color
        if let Some(previous_event) = previous_event_opt {
            match previous_event {
                Event::Goal(g) => {
                    // Kickoff is in progress by a team, place accordingly on your side
                    // 10s until we go into normal state
                    if let Some(chrono) = _chrono {
                        println!("Kickoff in progress ! It lasts for 10s at most");
                        if chrono.elapsed() > std::time::Duration::from_secs(10) {
                            //let kickoff_team = g.by_team as TeamColor;
                            //world.data.state = GameState::Running(RunningState::KickOff(kickoff_team));
                            world.data.state =
                                GameState::Running(RunningState::KickOff(g.by_team.opposite()));
                        }
                    } else {
                        println!("Running normally after kickoff");
                        // start chrono
                        _chrono = Some(Instant::now());
                        world.data.state = GameState::Running(RunningState::Run);
                    }
                }

                &_ => {
                    // Just play the game when no particular state is found (like penalty kick failed)
                    // - what's your problem green ?
                    // - me said alone ramp, me said alone ramp
                    // - (proceeds to destroy his table)
                    world.data.state = GameState::Running(RunningState::Run);
                }
            }
        } else {
            //println!("Play by default");
            world.data.state = GameState::Running(RunningState::Run);
        }
    }

    //TODO : color different state
    fn timeout_branch(world: &mut World, _team:TeamColor) {
        world.data.state = GameState::Halted(HaltedState::Timeout);
    }

    fn freekick_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team:TeamColor) {
        if let Some(chrono) = _chrono_opt {
            if chrono.elapsed() > std::time::Duration::from_secs(10) {
                dbg!("ya");
                // if 10s have passed, game runs normally
                world.data.state = GameState::Running(RunningState::Run);
                _chrono_opt = Some(Instant::now());
            } else {
                // otherwise, we are still in the FreeKick state
                world.data.state = GameState::Running(RunningState::FreeKick(team));
            }
        }
    }

    fn prepare_penalty_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team:TeamColor) {
        //TODO : the penalty comportement is complex, maybe we're missing a penalty RunningState
        world.data.state = GameState::Stopped(StoppedState::PreparePenalty(team));
    }

    fn prepare_kickoff_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team: TeamColor) {
        world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(team));
    }

    fn ball_placement_branch(world: &mut World, chrono_opt: Option<Instant>, team:TeamColor) {
        if let Some(chrono) = chrono_opt {
            // [ALLEMAGNE] chrono check peut être enlevé si pas de ball placement auto
            if chrono.elapsed() >= std::time::Duration::from_secs(30) {
                world.data.state = GameState::Running(RunningState::Run);
            } else {
                world.data.state = GameState::Stopped(StoppedState::BallPlacement(team));
            }
        }
        // The chrono should always be available to us
        unreachable!(); // todo remove
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        self.fix_yourself();
        // grab data
        let last_referee_packet = match filter_data.referee.last() {
            None => {
                return;
            }
            Some(r) => r,
        };

        let ref_command = last_referee_packet.command.clone();
        //dbg!(&ref_command);
        //TODO : i'm not sure about the indirect free kick and goal refCommand 
        match ref_command {
            RefereeCommand::Halt => GameControllerPostFilter::halt_state_branch(world),
            RefereeCommand::Deprecated |
            RefereeCommand::IndirectFree(_) |
            RefereeCommand::Goal(_) |
            RefereeCommand::Stop => {
                GameControllerPostFilter::stop_state_branch(&self.previous_event, world, self.kicked_off_once,)
            },
            RefereeCommand::NormalStart => {
                GameControllerPostFilter::normal_start_state_branch(&self.previous_event, world, self.chrono,)
            },
            RefereeCommand::ForceStart => {
                GameControllerPostFilter::force_start_state_branch(&self.previous_event, world, self.chrono,)
            },
            RefereeCommand::Timeout(team) => {
                GameControllerPostFilter::timeout_branch(world, team)
            }
            RefereeCommand::DirectFree(team) => {
                GameControllerPostFilter::freekick_branch(world, self.chrono, team)
            }
            RefereeCommand::BallPlacement(team) => {
                GameControllerPostFilter::ball_placement_branch(world, self.chrono, team)
            }
            RefereeCommand::PreparePenalty(team) => {
                GameControllerPostFilter::prepare_penalty_branch(world, self.chrono, team)
            }
            RefereeCommand::PrepareKickoff(team) => {
                GameControllerPostFilter::prepare_kickoff_branch(world, self.chrono, team)
            }
        }

        // Update previous gamestate & event
        if let Some(previous_game_event) = last_referee_packet.game_events.last() {
            self.previous_game_event = Option::from(previous_game_event.clone());
            self.previous_event = Option::from(previous_game_event.event.clone());
            //todo: don't clone this, specify lifetime
        }

        // todo: Don't forget to update positive half
        // if let Some(blue_team_on_positive_half) = last_referee_packet.blue_team_on_positive_half {
        //     if blue_team_on_positive_half {
        //        world.data.positive_half = TeamColor::Blue
        //     } else {
        //         world.data.positive_half = TeamColor::Yellow
        //     }
        // };
    }
}
