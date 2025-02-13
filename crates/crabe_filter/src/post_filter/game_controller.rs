use crabe_framework::data::referee::{Referee, RefereeCommand};
use crabe_framework::data::state_handler::game_state_handler::{ForceStartStateBranch, HaltStateBranch, DeprecatedStateBranch, NormalStartStateBranch, StopStateBranch, PrepareKickoffStateBranch, PreparePenaltyStateBranch, FreekickStateBranch, TimeoutStateBranch, BallPlacementStateBranch};
use crabe_framework::data::state_handler::{GameStateBranch, GameStateData};
use crabe_framework::data::world::World;
use crate::data::FilterData;
use crate::post_filter::PostFilter;



/// Translates the received events and referee commands
/// into specific game state for the game
pub struct GameControllerPostFilter {
    /// If true, means we must re-enter the current state branch
    /// because it depends on a timer provided by the referee
    time_based_refresh: bool,
    /// Contains multiple information about the current state of the match
    state_data: GameStateData,
}



impl GameControllerPostFilter {

    /// Updates the team scores of both teams in the latest
    /// saved state (does not update world, only internal)
    fn update_team_scores(&mut self, referee: &Referee) {
        self.state_data.ally_score = referee.ally.score;
        self.state_data.enemy_score = referee.enemy.score;
    }

    /// Updates most fields concerning the latest valid state data
    /// we've saved. The only field not modified by this function
    /// is `kicked_off_once`. State branches are responsible for
    /// updating this field instead
    fn update_latest_state_data(&mut self, referee: &Referee) {
        if referee.command != self.state_data.last_ref_cmd {
            self.state_data.prev_ref_cmd = self.state_data.last_ref_cmd;
        }
        self.state_data.last_ref_cmd = referee.command;

        match self.state_data.prev_ref_cmd {
            RefereeCommand::PrepareKickoff(_)
            | RefereeCommand::PreparePenalty(_)
            | RefereeCommand::DirectFree(_) => {
                self.state_data.last_designated_pos = None
            }
            _ => {
                if let Some(designated_pos) = referee.designated_position {
                    self.state_data.last_designated_pos = Some(designated_pos / 1000.);
                }
            }
       }
    }
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        Self {
            time_based_refresh: false,
            state_data: GameStateData::default(),
        }
    }
}

impl GameControllerPostFilter {
    fn resolve_branch(&mut self, referee_command: &RefereeCommand) -> Box<dyn GameStateBranch> {
        match referee_command {
            RefereeCommand::Halt => Box::new(HaltStateBranch),
            RefereeCommand::Stop => Box::new(StopStateBranch),
            RefereeCommand::NormalStart => Box::new(NormalStartStateBranch),
            RefereeCommand::ForceStart => Box::new(ForceStartStateBranch),
            RefereeCommand::PrepareKickoff(for_team) => Box::new(PrepareKickoffStateBranch::new(*for_team)),
            RefereeCommand::PreparePenalty(for_team) => Box::new(PreparePenaltyStateBranch::new(*for_team)),
            RefereeCommand::DirectFree(for_team) => Box::new(FreekickStateBranch::new(*for_team)),
            RefereeCommand::Timeout(for_team) => Box::new(TimeoutStateBranch::new(*for_team)),
            RefereeCommand::BallPlacement(by_team) => Box::new(BallPlacementStateBranch::new(*by_team)),

            // Deprecated states (as per the protobuf files)
            RefereeCommand::Goal(_) // Seems weird, but the protobuf file mentioned
                                    // we shouldn't base ourselves off this command
                                    // Tests show this is never sent
            | RefereeCommand::IndirectFree(_)
            | RefereeCommand::Deprecated => Box::new(DeprecatedStateBranch)
        }
    }

    fn update_state(&mut self, world: &mut World, referee: &Referee) {
        let mut new_state = world.data.ref_orders.state;

        // change state only if a new referee command has been issued,
        // or if the state is time-dependent
        if self.state_data.last_ref_cmd != referee.command || self.time_based_refresh {
            // dbg!(&referee.command);
            // dbg!(referee.next_command);
            // dbg!(&referee.designated_position);
            // dbg!(&referee.current_action_time_remaining);

            self.update_latest_state_data(referee);

            new_state = self.resolve_branch(&referee.command)
                .process_state(world,
                               referee,
                               &mut self.time_based_refresh,
                               &mut self.state_data);

            dbg!(&new_state);

            self.update_team_scores(referee);
            world.data.ref_orders.update(new_state, referee);
        }
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        if let Some(referee) = filter_data.referee.last() {

            self.update_state(world, referee);

            // update positive half, to see which team resides on the positive
            // side of the field
            if let Some(team_on_positive_half) = referee.positive_half {
                world.data.positive_half = team_on_positive_half
            }

            // update stage information
            world.data.stage_info.stage = referee.stage;
            world.data.stage_info.time_left = referee.stage_time_left;

            // update team infos
            world.data.ally.update_info(&referee.ally);
            world.data.enemy.update_info(&referee.enemy);
        };
    }
}