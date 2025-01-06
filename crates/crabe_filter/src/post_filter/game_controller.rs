use std::time::{Duration, Instant};
use log::warn;
use nalgebra::{distance, Point2};
use crabe_framework::data::referee::{Referee, RefereeCommand};
use crabe_framework::data::referee::event::{BallLeftField, Event};
use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crabe_framework::data::world::game_state::GameState::{Halted, Stopped, Running};
use crabe_framework::data::world::{Ball, EnemyInfo, TeamColor, World};
use crabe_framework::data::world::game_state::RunningState::Run;
use crate::data::FilterData;
use crate::post_filter::PostFilter;

/// TODO: what can be extended/enriched ?
/// ================= IMPLEMENTED
/// Base transitions : Run -> Stop -> FreeKick
/// Wanted transitions :
///     Run -> PrepareCornerKick -> CornerKick
///
/// Ball left field via touch line (ally touched): CornerKick
/// Ball left field via touch line (enemy touched): GoalKick
/// => goal_or_corner_kick()
/// => New states : PrepareCornerKick, PrepareGoalKick
/// => New transitions
///   Run -> Stopped(PrepareCornerKick) -> Running(CornerKick)
///   Run -> Stopped(PrepareCornerKick) -> Running(CornerKick)
/// =================

///
/// Goal scored but invalid : CornerKick
/// => invalidated_goal()
/// => New state : Halted::PossibleGoal
/// PossibleGoal -> (BallPlacement) -> FreeKick
/// PossibleGoal -> Stop -> PrepareKickoff -> Kickoff
///

/// Ball left outer field line :
/// - Check for BallPlacement ?
/// - PrepareFreekick ?

/// Minimum distance to consider whether the ball has moved
/// from a reference point in the rules
const MIN_DIST_BALL_MOVED: f64 = 0.05; // see the rulebook, section 5.4

pub struct GameControllerPostFilter {
    /// Last command sent by the referee
    /// Whenever a new command is issued, we
    /// perform a transition in the state machine
    ref_cmd: RefereeCommand,

    /// The reference position to use when checking
    /// whether the ball is in play.
    ball_ref_pos: Option<Point2<f64>>,
}

impl Default for GameControllerPostFilter {
    fn default() -> Self {
        Self {
            ref_cmd: RefereeCommand::Halt,
            ball_ref_pos: None,
        }
    }
}


/// Returns true if the ball is considered to be "in play",
/// that is, it has moved from its reference position
/// by at least a certain delta (specified by the rulebook)
pub(super) fn is_ball_in_play(ball_ref_pos: &Point2<f64>, opt_ball: &Option<Ball>) -> bool {
    if let Some(ball) = &opt_ball {
        distance(&ball.position_2d(), &ball_ref_pos) > MIN_DIST_BALL_MOVED
    } else {
        warn!("No ball detected, considering ball did not move");
        false
    }
}

impl GameControllerPostFilter {

    /// In a given state, returns the reference position to use.
    /// For example, before a freekick occurs, the ball should be placed at a specific position.
    /// The human referee is allowed to not place the ball exactly at the specific position
    /// designated (in other words, a margin of error of about 0.3 is allowed).
    /// Thus, once we enter a dynamic state (for instance, FreeKick), we should compute
    /// the distance of the ball position before placement, against its current position.
    fn get_reference_position(&self, world: &World, referee: &Referee, state: &GameState) -> Option<Point2<f64>> {
        if matches!(state, Running(RunningState::FreeKick(_))
                        | Running(RunningState::KickOff(_))
        ) {
            match &world.ball {
                Some(ball) => Some(ball.position_2d()),
                None => None
            }
        } else {
            match referee.designated_position {
                Some(pos) => Some(pos),
                None => None,
            }
        }
    }

    fn should_change_state(&self, world: &World, referee: &Referee, ball_ref_pos: &Point2<f64>) -> bool {
        if let Some(time_remaining) = referee.current_action_time_remaining {
            time_remaining.num_seconds() <= 0 // todo: check if valid
        } else {
            is_ball_in_play(ball_ref_pos, &world.ball)
        }
    }

    /// Called when the ball goes out of the field.
    /// Returns on which team's side the ball went out of the field.
    /// Note: we consider that at x == 0., it's the team on the negative half.
    fn ball_leave_side(&self, opt_positive_half_team: Option<TeamColor>, event_info: &BallLeftField) -> Option<TeamColor> {
        match (event_info.location, opt_positive_half_team) {
            (Some(leave_loc), Some(positive_half_team)) => {
                if leave_loc.x > 0. {
                    Some(positive_half_team)
                } else {
                    Some(positive_half_team.opposite())
                }
            }
            (..) => None
        }
    }
    
    /// When the ball leaves the field by crossing a goal line,
    /// this function can be called to check if the next state should be a CornerKick
    /// or a GoalKick.
    /// 
    /// If the side on which the ball left cannot be computed, returns a default Stop state
    fn goal_or_corner_kick(&self, opt_positive_half_team: Option<TeamColor>, event_info: &BallLeftField) -> GameState {
        let last_touch = event_info.by_team;
        if let Some(side) = self.ball_leave_side(opt_positive_half_team, event_info) {
            return if side == last_touch {
                // the faulting team pushes the ball through their side of the field,
                // CornerKick for the opposite team
                Stopped(StoppedState::PrepareCornerKick(event_info.by_team.opposite()))
            } else {
                // the faulting team pushed the ball towards their enemies' side of the field,
                // GoalKick for the opposite team
                Stopped(StoppedState::PrepareGoalKick(event_info.by_team.opposite()))
            }
        }
        Stopped(StoppedState::Stop)
    }
    
    /// Transitions to another state based on the referee's
    /// last command, the current state and the world
    fn transition(&mut self, referee: &Referee, world: &World) -> GameState {
        let cur_state = world.data.ref_orders.state;
        let last_ge = referee.game_events.last();
        let last_ge_event = last_ge.map(|e| &e.event);

        match (cur_state, &referee.command, &self.ball_ref_pos, last_ge_event) {
            // Halt
            (Halted(HaltedState::Halt), RefereeCommand::Stop, ..) => { Stopped(StoppedState::Stop) }

            // Timeout
            (Halted(HaltedState::Timeout(_)), RefereeCommand::Stop, ..) => { Stopped(StoppedState::Stop) }

            // Stop
            (Stopped(StoppedState::Stop), RefereeCommand::BallPlacement(team), ..) => { Stopped(StoppedState::BallPlacement(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::PrepareKickoff(team), ..) => { Stopped(StoppedState::PrepareKickoff(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::PreparePenalty(team), ..) => { Stopped(StoppedState::PreparePenalty(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::ForceStart, ..) => { Running(RunningState::Run) }
            (Stopped(StoppedState::Stop), RefereeCommand::Timeout(team), ..) => { Halted(HaltedState::Timeout(*team)) }
            (Stopped(StoppedState::Stop), RefereeCommand::DirectFree(team), ..) => {
                // todo: what happens if there is no ball ?
                // must add test case
                if let Some(ball) = &world.ball {
                    self.ball_ref_pos = Some(ball.position_2d());
                }
                Running(RunningState::FreeKick(*team))
            }

            // Ball placement
            // is there a command sent when ball placement ends ?
            // yes, on success, returns to Stop and sends FreeKick
            (Stopped(StoppedState::BallPlacement(_)), RefereeCommand::Stop, ..) => { Stopped(StoppedState::Stop) }
            
            // Kickoff
            (Stopped(StoppedState::PrepareKickoff(team)), RefereeCommand::NormalStart, ..) => {
                let ball_pos = match &world.ball {
                    Some(ball) => ball.position_2d(),
                    None => {
                        warn!("No ball in world, guess when will the kickoff finish might be incorrect. Taking default (0., 0.) point as reference");
                        Point2::origin()
                    }
                };
                self.ball_ref_pos = Some(ball_pos);
                Running(RunningState::KickOff(team))
            }
            
            // todo: merge with second branch below ?
            (Running(RunningState::KickOff(team)), RefereeCommand::NormalStart, Some(ball_ref_pos), _) => {
                if self.should_change_state(world, referee, ball_ref_pos) {
                    // kickoff ends, we change to Run
                    self.ball_ref_pos = None;
                    Running(RunningState::Run)
                } else {
                    // KickOff still in progress
                    Running(RunningState::KickOff(team))
                }
            }

            // PreparePenalty
            (Stopped(StoppedState::PreparePenalty(team)), RefereeCommand::NormalStart, ..) => { Running(RunningState::Penalty(team)) }

            // FreeKick (time-dependent ?)
            (Running(RunningState::FreeKick(team)), RefereeCommand::DirectFree(_), Some(ball_ref_pos), _) => {
                if self.should_change_state(world, referee, ball_ref_pos) {
                    self.ball_ref_pos = None;
                    Running(RunningState::Run)
                } else {
                    Running(RunningState::FreeKick(team))
                }
            }
            (Stopped(StoppedState::PrepareCornerKick(_)), RefereeCommand::DirectFree(team), _, _) => {
                if let Some(ball) = &world.ball {
                    self.ball_ref_pos = Some(ball.position_2d());
                    Running(RunningState::CornerKick(*team))
                } else {
                    warn!("Ball position not available when switching to running CornerKick.");
                    Running(RunningState::CornerKick(*team)) // todo: what should we do here ?
                }
            }
            
            (Stopped(StoppedState::PrepareGoalKick(_)), RefereeCommand::DirectFree(team), _, _) => {
                if let Some(ball) = &world.ball {
                    self.ball_ref_pos = Some(ball.position_2d());
                    Running(RunningState::GoalKick(*team))
                } else {
                    warn!("Ball position not available when switching to running GoalKick.");
                    Running(RunningState::GoalKick(*team)) // todo: what should we do here ?
                }
            }

            (Running(RunningState::CornerKick(_)), RefereeCommand::DirectFree(_), Some(ball_ref_pos), _)
            | (Running(RunningState::GoalKick(_)), RefereeCommand::DirectFree(_), Some(ball_ref_pos), _) => {
                match self.should_change_state(&world, &referee, ball_ref_pos) {
                    false => cur_state,
                    true => {
                        self.ball_ref_pos = None;
                        Running(Run)
                    },
                }
            }
            
            // enriched state, we can guess whether
            (_, RefereeCommand::Stop, _, Some(Event::BallLeftFieldGoalLine(data))) => {
                // goal kick or corner kick
                self.goal_or_corner_kick(referee.positive_half, data)
            }

            // any running state can lead to a Stop
            // (this performs the Run -> Stop transition as well)
            (Running(_), RefereeCommand::Stop, ..) => {
                // println!("{:?}", referee.game_events.last());
                Stopped(StoppedState::Stop)
            }

            // The human referee can trigger Stop from any state
            (_, RefereeCommand::Stop, ..) => {
                self.ball_ref_pos = None;
                Stopped(StoppedState::Stop)
            }

            // any state can lead to Halt
            (_, RefereeCommand::Halt, ..) => {
                self.ball_ref_pos = None;
                Halted(HaltedState::Halt)
            }

            // .. means match anything
            (..) => {
                warn!("Unknown transition, staying in Halt just in case");
                println!("({:?}, {:?}, {:?})", &cur_state, &referee.command, &self.ball_ref_pos.is_some());
                self.ball_ref_pos = None;
                Halted(HaltedState::Halt) // TODO: maybe this should be Stop instead
            }
        }
    }
}

impl PostFilter for GameControllerPostFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        if let Some(referee) = filter_data.referee.last() {

            // apply transition to state machine only if
            // - a new ref command is received
            // - the current state must be refreshed with new data
            if self.ref_cmd != referee.command || self.ball_ref_pos.is_some() {
                // some variables are only here for debugging
                let prev_state = world.data.ref_orders.state;
                let new_state = self.transition(referee, world);
                self.ref_cmd = referee.command;

                world.data.ref_orders.update(new_state, referee, self.get_reference_position(world, referee, &new_state));
                // println!("{:?} -> {:?}) (ref_pos: {:?}) (des_pos: {:?}) (next_cmd: {:?})", prev_state, &new_state, world.data.ref_orders.designated_position, &referee.designated_position, &referee.next_command);
                println!("{:?} -> {:?}) (cmd: {:?})", prev_state, &new_state, &referee.command);
                world.data.ref_orders.state = new_state;
            }
        }
    }
}

/// ------------------
///   Testing module
/// ------------------
#[cfg(test)]
mod tests {
    use nalgebra::Point3;
    use crabe_framework::config::CommonConfig;
    use crabe_framework::data::referee::Stage;
    use crabe_framework::data::world::TeamColor;
    use super::RefereeCommand as RC;
    use super::HaltedState::*;
    use super::StoppedState::*;
    use super::RunningState::*;
    use super::*;

    fn all_states() -> Vec<GameState> {
        let team = TeamColor::Blue;
        vec![
            Halted(Halt), Halted(Timeout(team)),
            Stopped(PrepareKickoff(team)), Stopped(PreparePenalty(team)),
            Stopped(BallPlacement(team)),
            Running(KickOff(team)), Running(Penalty(team)), Running(FreeKick(team)),
            Running(Run)
        ]
    }

    /// Provides dummy data configured to the parameters given
    /// I recommend looking at test kickoff_correct_reference_pos() to see how to use it
    fn dummy_data(current_ref_cmd: RefereeCommand, current_state: GameState, next_ref_cmd: RefereeCommand, point: &Point2<f64>) -> (GameControllerPostFilter, FilterData, World) {
        let mut gcpf = GameControllerPostFilter::default();
        gcpf.ref_cmd = current_ref_cmd;

        let mut filter_data = FilterData::default();
        set_refcmd_to(&mut filter_data, next_ref_cmd);

        let mut world = World::with_config(&CommonConfig {gc: false, real: false, yellow: false});
        world.ball = Some(Ball {
            position: Point3::new(point.x, point.y, 0.),
            ..Default::default()
        });
        world.data.ref_orders.state = current_state;

        return (gcpf, filter_data, world)
    }

    fn set_refcmd_to(filter_data: &mut FilterData, command: RefereeCommand) -> &FilterData {
        filter_data.referee.clear();
        filter_data.referee.push(Referee {
            source_identifier: None,
            match_type: None,
            packet_timestamp: Default::default(),
            stage: Stage::NormalFirstHalfPre,
            stage_time_left: None,
            command,
            command_counter: 0,
            command_timestamp: Default::default(),
            ally: Default::default(),
            enemy: Default::default(),
            designated_position: None,
            positive_half: None,
            next_command: None,
            game_events: vec![],
            game_event_proposals: vec![],
            current_action_time_remaining: None,
        });
        filter_data
    }

    /// Moves the ball in the world, always setting its z at 0
    fn move_ball(world: &mut World, new_ball_pos: &Point2<f64>) {
        if let Some(ball) = &mut world.ball {
            ball.position = Point3::new(new_ball_pos.x, new_ball_pos.y, 0.);
        }
    }
    
    #[test]
    fn any_state_to_stop() {
        // setup parameters
        let current_ref_cmd = RC::Halt;
        let current_state = Halted(Halt);
        let next_ref_cmd = RC::Stop;
        let ball_pos = Point2::new(0.05, 0.05);

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &ball_pos);

        all_states().iter().for_each(|state| {
            let prev_state = world.data.ref_orders.state;
            world.data.ref_orders.state = *state;
            gc_postfilter.step(&mut filter_data, &mut world);
            
            assert!(
                matches!(world.data.ref_orders.state, Stopped(_)),
                "Invalid transition {:?} -> {:?}",
                prev_state,
                world.data.ref_orders.state,
            );
            world.data.ref_orders.state = Halted(Halt);
            gc_postfilter.ref_cmd = RC::Halt // actual value doesn't matter, as long as it is different
        });
    }

    #[test]
    fn any_state_to_halt() {
        // setup parameters
        let current_ref_cmd = RC::Stop;
        let current_state = Stopped(Stop);
        let next_ref_cmd = RC::Halt;
        let ball_pos = Point2::new(0.05, 0.05);

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &ball_pos);

        all_states().iter().for_each(|state| {
            let prev_state = world.data.ref_orders.state;
            world.data.ref_orders.state = *state;
            gc_postfilter.step(&mut filter_data, &mut world);

            assert!(
                matches!(world.data.ref_orders.state, Halted(_)),
                "Invalid transition {:?} -> {:?}",
                prev_state,
                world.data.ref_orders.state,
            );
            world.data.ref_orders.state = Stopped(Stop);
            gc_postfilter.ref_cmd = current_ref_cmd
        });
    }

    /// Checks that PrepareKickoff -> Kickoff uses the ball as reference point
    /// and not the referee's designated position
    #[test]
    fn kickoff_correct_reference_pos() {
        // setup parameters
        let ball_pos = Point2::new(0.05, 0.05);
        let (mut gc_postfilter, mut filter_data, mut world) = 
            dummy_data(
                // current state
                RC::PrepareKickoff(TeamColor::Blue), // current ref command saved
                Stopped(PrepareKickoff(TeamColor::Blue)), // current game state we are in
                
                // inbound data
                RC::NormalStart, // Referee command issued
                &ball_pos // where to place the ball in the world
            );

        gc_postfilter.step(&mut filter_data, &mut world);

        let designated_pos= &world.data.ref_orders.designated_position;
        assert!(designated_pos.is_some());
        assert_eq!(ball_pos, designated_pos.unwrap())
    }

    /// Checks that Stop -> FreeKick uses the ball as reference point
    /// and not the referee's designated position
    #[test]
    fn freekick_correct_reference_pos() {
        // setup parameters
        let ball_pos = Point2::new(0.05, 0.05);
        let (mut gc_postfilter, mut filter_data, mut world) =
            dummy_data(RC::Stop, Stopped(Stop), RC::DirectFree(TeamColor::Blue), &ball_pos);

        gc_postfilter.step(&mut filter_data, &mut world);

        let designated_pos= &world.data.ref_orders.designated_position;
        assert!(designated_pos.is_some());
        assert_eq!(ball_pos, designated_pos.unwrap())
    }

    /// Transition condition function must be Some(_) when entering the FreeKick state,
    /// and must be None when leaving FreeKick state.
    /// This test forces to leave the FreeKick state by moving the ball,
    /// if it does not work, this test will fail
    #[test]
    fn freekick_has_ball_reference_position() {
        let ball_t0 = Point2::new(4.0, 3.0);
        let (mut gc_postfilter, mut filter_data, mut world) = 
            dummy_data(RC::Stop, Stopped(Stop), RC::DirectFree(TeamColor::Blue), &ball_t0);

        // Stop -> FreeKick
        // ball should be saved as reference point
        gc_postfilter.step(&mut filter_data, &mut world);
        assert!(
            gc_postfilter.ball_ref_pos.is_some(),
            "No updating function even though FreeKick should be updated dynamically"
        );

        // FreeKick -> Running(Run)
        // on ball move
        let ball_t1 = Point3::new(5.0, 4.0, 0.0); // meets conditions for changing state
        world.ball = Some(Ball {position: ball_t1, ..world.ball.unwrap()});
        gc_postfilter.step(&mut filter_data, &mut world);
        assert!(
            gc_postfilter.ball_ref_pos.is_none(),
            "Transition condition function was not cleared even though we left state FreeKick"
        );
    }

    #[test]
    fn freekick_exit_if_ball_moved() {
        let ball_t0 = Point2::new(4.0, 3.0);
        let next_ref_cmd = RC::DirectFree(TeamColor::Blue);
        let (mut gc_postfilter, mut filter_data, mut world) =
            dummy_data(RC::Stop, Stopped(Stop), next_ref_cmd, &ball_t0);

        // Stop -> FreeKick
        // ball should be saved as reference point
        gc_postfilter.step(&mut filter_data, &mut world);
        let state_t0 = world.data.ref_orders.state;
        assert!(
            matches!(state_t0, Running(FreeKick(_))),
            "Stop -> FreeKick not occurred properly, with command {:?}", next_ref_cmd
        );

        // ball moves such that distance > MIN_DIST_BALL_MOVED
        let ball_t1 = Point2::new(ball_t0.x, ball_t0.y + MIN_DIST_BALL_MOVED * 2.);
        move_ball(&mut world, &ball_t1);

        // gc postfilter should change the state
        gc_postfilter.step(&mut filter_data, &mut world);
        let state_t1 = world.data.ref_orders.state;
        assert_ne!(
            state_t0, state_t1,
            "FreeKick did not change state, even though ball moved by distance {:?} (min dist : {:?})",
            distance(&ball_t0, &ball_t1), MIN_DIST_BALL_MOVED
        );

        assert_eq!(
            state_t1, Running(Run),
            "Expected FreeKick -> Running(Run), got FreeKick -> {:?}", state_t1
        );
    }

    /// Checks that FreeKick -> Running(Run) after 10 seconds as measured by the referee.
    #[test]
    fn freekick_exit_if_ten_seconds_elapsed() {
        // setup
        let (mut gc_postfilter, mut filter_data, mut world) =
            dummy_data(RC::Stop, Stopped(Stop), RC::DirectFree(TeamColor::Blue), &Point2::origin());
        gc_postfilter.step(&mut filter_data, &mut world);

        // wait 10 secs
        let mut referee = filter_data.referee.pop().unwrap();
        referee.current_action_time_remaining = chrono::Duration::new(-1, 0);
        
        filter_data.referee.push(referee);
        
        gc_postfilter.step(&mut filter_data, &mut world);

        // heck we changed states // i never wrote that ??? ~wanchai
        assert_eq!(
            world.data.ref_orders.state, Running(Run),
            "FreeKick -> Running(Run) did not occur after 10 seconds"
        )
    }
    
    /// Checks if this transition (state_t0, ref_cmd) -> expected_state
    /// occurs correctly in the state machine
    fn valid_transition(state_t0: GameState, ref_cmd: RefereeCommand, expected_state: GameState) {
        let (mut gc_postfilter, mut filter_data, mut world) =
            dummy_data(RC::Halt, state_t0, ref_cmd, &Point2::origin());
        
        gc_postfilter.step(&mut filter_data, &mut world);

        assert_eq!(
            world.data.ref_orders.state, expected_state,
            "{:?} -> {:?} transition does not exist in state machine",
            world.data.ref_orders.state, expected_state
        );
    }
    
    #[test]
    fn all_transitions_correct() {
        valid_transition(Halted(Halt), RC::Stop, Stopped(Stop));
        
        // timeout transitions
        valid_transition(Stopped(Stop), RC::Timeout(TeamColor::Blue), Halted(Timeout(TeamColor::Blue)));
        valid_transition(Halted(Timeout(TeamColor::Blue)), RC::Stop, Stopped(Stop));
        
        valid_transition(Stopped(Stop), RC::Timeout(TeamColor::Yellow), Halted(Timeout(TeamColor::Yellow)));
        valid_transition(Halted(Timeout(TeamColor::Yellow)), RC::Stop, Stopped(Stop));
        
        //todo: all of the other arrows
    }
}