use crate::data::FilterData;
use crate::post_filter::PostFilter;
use crabe_framework::data::world::game_state::{
    GameState, HaltedState, RunningState, StoppedState,
};
use crabe_framework::data::world::{TeamColor, World};

use crate::data::referee::event::{Event, GameEvent};
use crate::data::referee::RefereeCommand;
use std::cell::Ref;
use std::time::Instant;

#[derive(Debug)]
pub struct GameControllerPostFilter {
    previous_game_event: Option<GameEvent>,
    previous_event: Option<Event>,
    previous_command: RefereeCommand,
    last_command: RefereeCommand,
    chrono: Option<Instant>, // TODO: Need to have an option here ?
    kicked_off_once: bool,
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        GameControllerPostFilter {
            previous_game_event: None,
            previous_event: None,
            previous_command: RefereeCommand::Halt,
            last_command: RefereeCommand::Halt,
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
        // consider that timeout is equivalent to normal halt
        world.data.state = GameState::Halted(HaltedState::Halt);
    }

    fn stop_state_branch(
        previous_game_event_opt: &Option<GameEvent>,
        previous_command: &RefereeCommand,
        world: &mut World,
        _kicked_off_once: &mut bool,
        mut _chrono: Option<Instant>,
    ) {
        // TODO: the "world.team_color" isn't right
        if let Some(previous_game_event) = previous_game_event_opt {
            let previous_event = &previous_game_event.event;
            match previous_event {
                // Goal has been scored, prepare for next kickoff phase
                Event::Goal { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(world.team_color));
                }

                Event::BallLeftFieldTouchLine { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement(world.team_color));
                }

                Event::BallLeftFieldGoalLine { .. } => {
                    world.data.state = GameState::Stopped(StoppedState::BallPlacement(world.team_color));
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
            if !*_kicked_off_once {
                //TODO : when we launch the code in the middle of a game this code goes wrong because it's not everytime kick off 
                world.data.state = GameState::Stopped(StoppedState::PrepareKickoff(world.team_color));
                *_kicked_off_once = true;
            } else {
                // this one's totally arbitrary
                // i don't understand how we can fetch a forced free kick from the commands
                // todo: fix what's mentioned above me (fix me !)
                //world.data.state = GameState::Running(RunningState::FreeKick(world.team_color));

                world.data.state = GameState::Stopped(StoppedState::Stop);
            }
        }
        _chrono = Some(Instant::now());
    }

    fn force_start_state_branch(
        previous_event_opt: &Option<Event>,
        previous_command: RefereeCommand,
        world: &mut World,
        mut _chrono: Option<Instant>,
    ) {
        GameControllerPostFilter::normal_start_state_branch(previous_event_opt, previous_command, world, _chrono)
    }

    fn normal_start_state_branch(
        previous_event_opt: &Option<Event>,
        previous_command: RefereeCommand,
        world: &mut World,
        mut _chrono: Option<Instant>,
    ) {
        //TODO team color
        if let Some(chrono) = _chrono {
            if previous_command == RefereeCommand::PrepareKickoff(TeamColor::Blue){
                if chrono.elapsed() >= std::time::Duration::from_secs(10) {
                    world.data.state = GameState::Running(RunningState::Run);
                } else {
                    world.data.state = GameState::Running(RunningState::KickOff(TeamColor::Blue));
                }
                return
            }
            else if previous_command == RefereeCommand::PrepareKickoff(TeamColor::Yellow){
                if chrono.elapsed() >= std::time::Duration::from_secs(10) {
                    world.data.state = GameState::Running(RunningState::Run);
                } else {
                    world.data.state = GameState::Running(RunningState::KickOff(TeamColor::Yellow));
                }
                return
            }
        }
        else if let Some(previous_event) = previous_event_opt {
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
                        return
                    } else {
                        // start chrono
                        _chrono = Some(Instant::now());
                    }
                }

                &_ => {}
            }
        } 
        world.data.state = GameState::Running(RunningState::Run);
    }

    fn timeout_branch(world: &mut World, _team:TeamColor) {
        world.data.state = GameState::Halted(HaltedState::Timeout);
    }

    fn freekick_branch(world: &mut World, mut _chrono_opt: Option<Instant>, team:TeamColor) {
        if let Some(chrono) = _chrono_opt {
            //dbg!(chrono);
            if chrono.elapsed() > std::time::Duration::from_secs(10) {
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
        //TODO : not sure about the indirect free kick and goal refCommand 
        match ref_command {
            RefereeCommand::Halt => GameControllerPostFilter::halt_state_branch(world),
            RefereeCommand::Deprecated |
            RefereeCommand::IndirectFree(_) |
            RefereeCommand::Stop => {
                GameControllerPostFilter::stop_state_branch(&self.previous_game_event, &self.previous_command, world, &mut self.kicked_off_once, self.chrono,)
            },
            RefereeCommand::NormalStart => {
                GameControllerPostFilter::normal_start_state_branch(&self.previous_event, self.previous_command.clone(), world, self.chrono,)
            },
            RefereeCommand::ForceStart => {
                GameControllerPostFilter::force_start_state_branch(&self.previous_event, self.previous_command.clone(), world, self.chrono,)
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
            RefereeCommand::Goal(team) => {//TODO : It's recommended to use the score field from the team infos instead for goal detection and revoked goals.
                GameControllerPostFilter::prepare_kickoff_branch(world, self.chrono, team.opposite())
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
        if ref_command != self.last_command{
            self.previous_command = self.last_command.clone();
            self.chrono = Option::from(Instant::now());
        }
        self.last_command = ref_command.clone();

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
