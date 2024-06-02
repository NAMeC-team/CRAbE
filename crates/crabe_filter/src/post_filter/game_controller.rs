use std::time::Instant;
use crabe_framework::data::referee::referee_orders::RefereeOrders;
use crabe_framework::data::referee::{Referee, RefereeCommand, TeamInfo};
use crabe_framework::data::state_handler::game_state_handler::{ForceStartStateBranch, HaltStateBranch, DeprecatedStateBranch, NormalStartStateBranch, StopStateBranch, PrepareKickoffStateBranch, PreparePenaltyStateBranch, FreekickStateBranch, TimeoutStateBranch, BallPlacementStateBranch};
use crabe_framework::data::state_handler::{GameStateBranch, StateData};
use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState};
use crabe_framework::data::world::{TeamColor, World};
use crate::data::FilterData;
use crate::post_filter::PostFilter;



/// Translates the received events and referee commands
/// into specific game state for the game
pub struct GameControllerPostFilter {
    /// The previous different command from the referee (not the penultimate)
    prev_command: RefereeCommand,
    /// Timer used for events that rely on specific durations.
    /// One example is the duration of the kickoff, during which
    /// if the opponent does not touch the ball after
    timer: Option<Instant>,
    /// Contains multiple information about the current state of the match
    state_data: StateData,
    /// If the current referee command mentions a team
    /// (like for a kickoff), it is stored here
    last_cmd_for_team: Option<TeamColor>,
}



impl GameControllerPostFilter {
    /// Checks whether the first kickoff already occurred,
    /// and saves it in the current state data
    fn save_if_first_kickoff_occurred(&mut self, prev_state: GameState) {
        self.state_data.kicked_off_once = self.state_data.kicked_off_once || prev_state != GameState::Halted(HaltedState::GameNotStarted)
    }

    /// Updates the team scores of both teams
    fn update_team_scores(&mut self, referee: &Referee) {
        self.state_data.ally_score = referee.ally.score;
        self.state_data.enemy_score = referee.enemy.score;
    }

    /// Saves the latest referee command
    fn update_prev_ref_cmd(&mut self, current_ref_cmd: &RefereeCommand) {
        self.prev_command = *current_ref_cmd;
    }
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        Self {
            prev_command: RefereeCommand::Halt,
            timer: None,
            state_data: StateData::default(),
            last_cmd_for_team: None,
        }
    }
}

impl GameControllerPostFilter {
    fn resolve_branch(&mut self, referee_command: &RefereeCommand) -> Box<dyn GameStateBranch> {
        // update which team is the command for
        // TODO: this looks exotic, can someone check it out ?
        match referee_command {
            RefereeCommand::PrepareKickoff(team)
            | RefereeCommand::PreparePenalty(team)
            | RefereeCommand::DirectFree(team)
            | RefereeCommand::Timeout(team)
            | RefereeCommand::BallPlacement(team) => { self.last_cmd_for_team = Some(*team) }

            _ => { self.last_cmd_for_team = None }
        };

        //
        match referee_command {
            RefereeCommand::Halt => Box::new(HaltStateBranch),
            RefereeCommand::Stop => Box::new(StopStateBranch),
            RefereeCommand::NormalStart => Box::new(NormalStartStateBranch),
            RefereeCommand::ForceStart => Box::new(ForceStartStateBranch),
            RefereeCommand::PrepareKickoff(_) => Box::new(PrepareKickoffStateBranch),
            RefereeCommand::PreparePenalty(_) => Box::new(PreparePenaltyStateBranch),
            RefereeCommand::DirectFree(_) => Box::new(FreekickStateBranch),
            RefereeCommand::Timeout(_) => Box::new(TimeoutStateBranch),
            RefereeCommand::BallPlacement(_) => Box::new(BallPlacementStateBranch),

            // Deprecated states (as per the protobuf files)
            RefereeCommand::Goal(_) // Seems weird, but the protobuf file mentioned
                                    // we shouldn't base ourselves off this command
                                    // Tests show this is never sent
            | RefereeCommand::IndirectFree(_)
            | RefereeCommand::Deprecated => Box::new(DeprecatedStateBranch)
        }
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        if let Some(referee) = filter_data.referee.last() {
            let mut new_state = world.data.ref_orders.state;

            // change state only if a new referee command has been issued,
            // or a timer is currently being used
            if self.prev_command != referee.command || self.timer != None {
                dbg!(&referee.command);
                // dbg!(&referee.game_events.last());
                dbg!(&referee.designated_position);
                dbg!(&referee.ally.score);
                dbg!(&referee.enemy.score);

                new_state = self.resolve_branch(&referee.command)
                    .process_state(world,
                                   referee,
                                   &mut self.timer,
                                   self.last_cmd_for_team,
                                   &self.state_data);

                self.save_if_first_kickoff_occurred(world.data.ref_orders.state);
                self.update_team_scores(referee);
                self.update_prev_ref_cmd(&referee.command);
                world.data.ref_orders.update(new_state, referee.game_events.last());
            }

            // update positive half, to see which team resides on the positive
            // side of the field
            if let Some(team_on_positive_half) = referee.positive_half {
                world.data.positive_half = team_on_positive_half
            }

            // self.resolve_branch(&referee.command).process_state(world, referee, self.timer, self.prev_command);
            // read referee command
            // if new command != previous command or timer is used
            //   => run associated branch
            //   => update world.data.ref_orders
        };
    }
}