use std::time::{Duration, Instant};
use log::warn;
use nalgebra::{distance, min, Point2};
use crabe_framework::data::referee::{Referee, RefereeCommand};
use crabe_framework::data::world::game_state::{GameState, HaltedState, RunningState, StoppedState};
use crabe_framework::data::world::game_state::GameState::{Halted, Stopped, Running};
use crabe_framework::data::world::{AllyInfo, Ball, EnemyInfo, World};
use crate::data::FilterData;
use crate::post_filter::PostFilter;

/// Minimum distance at which the ball is considered to be touched by a robot
const MIN_DIST_BALL_TOUCH: f64 = 0.05;
/// Minimum distance to consider whether the ball has moved
/// from a reference point in the rules
const MIN_DIST_BALL_MOVED: f64 = MIN_DIST_BALL_TOUCH; // see the rulebook, section 5.4
/// Maximum time of an action (in Div B)
const ACTION_TIME_LIMIT_SECS: u64 = 10;

pub struct GameControllerPostFilter {
    /// Last command sent by the referee
    /// Whenever a new command is issued, we
    /// perform a transition in the state machine
    ref_cmd: RefereeCommand,

    /// A closure to be called for dynamic state updates
    /// Returns true if we must perform the transition,
    /// false if we should stay in the current state
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

/// Returns true if `secs` seconds have passed
/// since the closure was created
fn maker_timer(secs: u64) -> impl Fn(&StateData) -> bool {
    let timer = Instant::now();
    return move |_| {
        timer.elapsed() > Duration::from_secs(secs)
    }
}

/// Returns true if the ball moved from its reference position
/// by at least a certain delta (specified by the rulebook)
fn maker_ball_moved(ball_ref_pos: Point2<f64>) -> impl Fn(&StateData) -> bool {
    return move |state_data: &StateData| -> bool {
        distance(&state_data.ball_pos, &ball_ref_pos) > MIN_DIST_BALL_MOVED
    }
}

/// Returns true if a robot has touched the ball.
/// When this happens, the state changes immediately
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
        // if no ball is detected, consider no robot has touched it
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

    /// Transitions to another state based on the referee's
    /// last command, the current state and the world
    fn transition(&mut self, referee: &Referee, world: &World) -> GameState {
        let cur_state = world.data.ref_orders.state;
        
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
                        // kickoff ends, we change to Run
                        self.cond_transition = None;
                        Running(RunningState::Run)
                    } else {
                        // KickOff still in progress
                        Running(RunningState::KickOff(team))
                    }
                } else {
                    warn!("No ball detected, staying in current state");
                    cur_state
                }
            }

            // PreparePenalty
            (Stopped(StoppedState::PreparePenalty(team)), RefereeCommand::NormalStart, _) => { Running(RunningState::Penalty(team)) }

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
                // println!("{:?}", referee.game_events.last());
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
                self.cond_transition = None;
                Halted(HaltedState::Halt)
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
            if self.ref_cmd != referee.command || self.cond_transition.is_some() {
                // some variables are only here for debugging
                let prev_state = world.data.ref_orders.state;
                let new_state = self.transition(referee, world);
                self.ref_cmd = referee.command;

                world.data.ref_orders.update(new_state, referee, self.get_reference_position(world, referee, &new_state));
                println!("{:?} -> {:?}) (ref_pos: {:?}) (des_pos: {:?}) (next_cmd: {:?})", prev_state, &new_state, world.data.ref_orders.designated_position, &referee.designated_position, &referee.next_command);
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
    use std::thread::sleep;
    use nalgebra::Point3;
    use crabe_framework::config::CommonConfig;
    use crabe_framework::data::referee::Stage;
    use crabe_framework::data::world::{Robot, TeamColor};
    use crate::data::TrackedRobot;
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
        let current_ref_cmd = RC::PrepareKickoff(TeamColor::Blue);
        let current_state = Stopped(PrepareKickoff(TeamColor::Blue));
        let next_ref_cmd = RC::NormalStart;
        let ball_pos = Point2::new(0.05, 0.05);

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &ball_pos);
        
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
        let current_ref_cmd = RC::Stop;
        let current_state = Stopped(Stop);
        let next_ref_cmd = RC::DirectFree(TeamColor::Blue);
        let ball_pos = Point2::new(0.05, 0.05);
        
        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &ball_pos);
        
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
    fn freekick_has_transition_condition_func() {
        let current_ref_cmd = RC::Stop;
        let current_state = Stopped(Stop);
        let next_ref_cmd = RC::DirectFree(TeamColor::Blue);
        let ball_t0 = Point2::new(4.0, 3.0);

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &ball_t0);
        
        // Stop -> FreeKick
        // ball should be saved as reference point
        gc_postfilter.step(&mut filter_data, &mut world);
        assert!(
            gc_postfilter.cond_transition.is_some(),
            "No updating function even though FreeKick should be updated dynamically"
        );

        // FreeKick -> Running(Run)
        // on ball move
        let ball_t1 = Point3::new(5.0, 4.0, 0.0); // meets conditions for changing state
        world.ball = Some(Ball {position: ball_t1, ..world.ball.unwrap()});
        gc_postfilter.step(&mut filter_data, &mut world);
        assert!(
            gc_postfilter.cond_transition.is_none(),
            "Transition condition function was not cleared even though we left state FreeKick"
        );
    }
    
    #[test]
    fn freekick_exit_if_ball_moved() {
        let current_ref_cmd = RC::Stop;
        let current_state = Stopped(Stop);
        let next_ref_cmd = RC::DirectFree(TeamColor::Blue);
        let ball_t0 = Point2::new(4.0, 3.0);

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &ball_t0);
        
        // Stop -> FreeKick
        // ball should be saved as reference point
        gc_postfilter.step(&mut filter_data, &mut world);
        let state_t0 = world.data.ref_orders.state;
        assert!(
            matches!(state_t0, Running(FreeKick(_))),
            "Stop -> FreeKick not occurred properly, with command {:?}", next_ref_cmd
        );
        
        // ball moves such that distance > MIN_DIST_BALL_MOVED
        let ball_t1 = Point3::new(ball_t0.x, ball_t0.y + MIN_DIST_BALL_MOVED * 2., 0.);
        world.ball = Some(Ball {
            position: ball_t1,
            ..world.ball.unwrap()
        });
        
        // gc postfilter should change the state
        gc_postfilter.step(&mut filter_data, &mut world);
        let state_t1 = world.data.ref_orders.state;
        assert_ne!(
            state_t0, state_t1, 
            "FreeKick did not change state, even though ball moved by distance {:?} (min dist : {:?})", 
            distance(&ball_t0, &ball_t1.xy()), MIN_DIST_BALL_MOVED
        );
        
        assert_eq!(
            state_t1, Running(Run),
            "Expected FreeKick -> Running(Run), got FreeKick -> {:?}", state_t1
        );
    }
    
    /// Checks that FreeKick -> Running(Run) after 10 seconds.
    /// This test is ignored by default because it needs to wait ten seconds on the main thread.
    /// It's pretty annoying. Use `cargo test -- --include-ignored` to run absolutely all tests.
    #[test]
    #[ignore]
    fn freekick_exit_if_ten_seconds_elapsed() {
        let current_ref_cmd = RC::Stop;
        let current_state = Stopped(Stop);
        let next_ref_cmd = RC::DirectFree(TeamColor::Blue);

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &Point2::origin());
        gc_postfilter.step(&mut filter_data, &mut world);
        
        sleep(Duration::from_secs(10));
        gc_postfilter.step(&mut filter_data, &mut world);

        assert_eq!(
            world.data.ref_orders.state, Running(Run),
            "FreeKick -> Running(Run) did not occur after 10 seconds"
        )
    }
    
    #[test]
    fn freekick_exit_if_robot_touches_ball() {
        let current_ref_cmd = RC::Stop;
        let current_state = Stopped(Stop);
        let next_ref_cmd = RC::DirectFree(TeamColor::Blue);
        let ball_pos = Point2::origin();

        let (mut gc_postfilter, mut filter_data, mut world) = dummy_data(current_ref_cmd, current_state, next_ref_cmd, &Point2::origin());
        gc_postfilter.step(&mut filter_data, &mut world);

        // place robot right next to ball (such that dist(robot, ball) < MIN_DIST_BALL_TOUCH)
        let mut enemy = Robot::<EnemyInfo>::default();
        enemy.pose.position = Point2::new(0.02, 0.02);
        world.enemies_bot.insert(0, enemy);
        
        // step through state machine again
        gc_postfilter.step(&mut filter_data, &mut world);
        
        let enemy_pose = world.enemies_bot.get(&0).unwrap().pose.position;
        assert_eq!(
            world.data.ref_orders.state, Running(Run),
            "FreeKick -> Running(Run) did not occur, even though dist(enemy_0, ball) = {:?} < {:?}",
            distance(&enemy_pose, &ball_pos), MIN_DIST_BALL_TOUCH
        );
    }
}