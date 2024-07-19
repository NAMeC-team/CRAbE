use crabe_framework::data::{tool::ToolData, world::{AllyInfo, Ball, Robot, TeamColor, World}};

use crate::{manager::bigbro::BigBro, strategy::{self, defensive::{DefenseWall, GoalKeeper}, formations::{Halt, MoveAwayFromBall, PrepareKickOff, PrepareStart}}};

use super::{closest_bot_to_point, closest_bots_to_point, filter_robots_not_in_ids, get_enemy_keeper_id, KEEPER_ID};

/// Put all bots to the Halt strategy.
pub fn everyone_halt(bigbro: &mut BigBro, world: &World) {
    let mut ids = vec![];
    for bot in world.allies_bot.values() {
        ids.push(bot.id);
    }
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("Halt") {
        bigbro.move_bots_to_existing_strategy(ids, strategy_index);
    }else{
        let strategy = Box::new(Halt::new(vec![]));
        bigbro.move_bots_to_new_strategy(ids, strategy);
    }
}

/// Put all bots to the Halt strategy.
pub fn everyone_stop(bigbro: &mut BigBro, world: &World) {
    let mut ids = vec![];
    for bot in world.allies_bot.values() {
        ids.push(bot.id);
    }
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("MoveAwayFromBall") {
        bigbro.move_bots_to_existing_strategy(ids, strategy_index);
    }else{
        let strategy = Box::new(MoveAwayFromBall::new(vec![]));
        bigbro.move_bots_to_new_strategy(ids, strategy);
    }
}

/// Prepare start
pub fn prepare_start(bigbro: &mut BigBro, world: &World) {
    let mut ids = vec![];
    for bot in world.allies_bot.values() {
        ids.push(bot.id);
    }
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("PrepareStart") {
        bigbro.move_bots_to_existing_strategy(ids, strategy_index);
    }else{
        let strategy = Box::new(PrepareStart::new(vec![]));
        bigbro.move_bots_to_new_strategy(ids, strategy);
    }
}

/// Put all bots to the Halt strategy.
pub fn everyone_stop_except_keeper(bigbro: &mut BigBro, world: &World) {
    let mut ids = vec![];
    for bot in world.allies_bot.values() {
        if bot.id == KEEPER_ID {
            continue;
        }
        ids.push(bot.id);
    }
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("MoveAwayFromBall") {
        bigbro.move_bots_to_existing_strategy(ids, strategy_index);
    }else{
        let strategy = Box::new(MoveAwayFromBall::new(vec![]));
        bigbro.move_bots_to_new_strategy(ids, strategy);
    }
}

pub fn prepare_kick_off(bigbro: &mut BigBro, world: &World, team: TeamColor) {
    let mut ids = vec![];
    for bot in world.allies_bot.values() {
        if bot.id == KEEPER_ID {
            continue;
        }
        ids.push(bot.id);
    }
    let ally_count = ids.len();
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("PrepareKickOff") {
        bigbro.move_bots_to_existing_strategy(ids, strategy_index);
    }else{
        let strategy = Box::new(PrepareKickOff::new(vec![], team));
        bigbro.move_bots_to_new_strategy(ids, strategy);
    }
    if ally_count > 1{
        put_defense_wall(bigbro, world, &filter_robots_not_in_ids(world.allies_bot.values().collect(), &vec![KEEPER_ID]), ally_count -1);
    }
    put_goal(bigbro);
}

pub fn penalty_state(bigbro: &mut BigBro, world: &World, team: TeamColor){
    for bot in world.allies_bot.values(){
        bigbro.remove_bot_from_strategies(bot.id);
    }
    if team == world.team_color{
        if let Some(ball) = &world.ball{
            put_attacker(bigbro, world, &world.allies_bot.values().collect(), ball);
        }
        
    }else{
        put_goal(bigbro);
    }
    
}

/// Put the goal keeper to the GoalKeeper strategy.
fn put_goal(bigbro: &mut BigBro) {
    if let Some(current_strategy) = bigbro.get_bot_current_strategy(KEEPER_ID) {
        if current_strategy.name() == "GoalKeeper" {
            return;
        }
    }
    let strategy = Box::new(GoalKeeper::new(KEEPER_ID,vec![1,3,4,5,6]));
    bigbro.move_bot_to_new_strategy(KEEPER_ID, strategy);
}

/// Put the num_robots closest bots to the goal to the DefenseWall strategy.
fn put_defense_wall(bigbro: &mut BigBro, world: &World, robots: &Vec<&Robot<AllyInfo>>, num_robots: usize) -> Vec<u8> {
    if num_robots == 0 {
        return vec![];
    }
    // let allies_no_keeper = filter_robots_not_in_ids(bots, &vec![KEEPER_ID]);
    let allies_no_keeper = robots.iter().filter(|bot| bot.id != KEEPER_ID).map(|bot| *bot).collect();
    let allies_closest = closest_bots_to_point(allies_no_keeper, world.geometry.ally_goal.line.center());
    let mut ids = vec![];
    for robot in allies_closest.iter().take(num_robots) {
        ids.push(robot.id);
    }
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("DefenseWall") {
        bigbro.strategies[strategy_index].put_ids(vec![]);
        bigbro.move_bots_to_existing_strategy(ids.clone(), strategy_index);
    } else{
        let strategy = Box::new(DefenseWall::new(vec![]));
        bigbro.move_bots_to_new_strategy(ids.clone(), strategy); // clone because move bots to new strategy don't take refs but values
    }
    ids
}

/// Put the closest bot to the ball to the Attacker strategy. (if there is already an attacker and he's close to the ball, don't change)
fn put_attacker(bigbro: &mut BigBro, world: &World, bots: &Vec<&Robot<AllyInfo>>, ball: &Ball) -> u8 {
    let closest_bot = match closest_bot_to_point(bots.iter().map(|bot| *bot).collect(), ball.position_2d()) {
        Some(bot) => bot,
        None => return 7,
    };
    if let Some(attacker_strategy_index) = bigbro.get_index_strategy_with_name("Attacker") {
        let current_attackers = bigbro.strategies[attacker_strategy_index].get_ids();
        if let Some(current_attacker_id) = current_attackers.last() {
            if *current_attacker_id == closest_bot.id  { // already the closest bot who's attacker
                return closest_bot.id;
            }
            if let Some(current_attacker) = world.allies_bot.get(current_attacker_id) {
                let current_attacker_dist_to_ball = current_attacker.distance(&ball.position_2d());
                if current_attacker_dist_to_ball < 0.5 {
                    return *current_attacker_id;
                }
            }
        }
        bigbro.strategies[attacker_strategy_index].put_ids(vec![]);
        bigbro.move_bot_to_existing_strategy(closest_bot.id, attacker_strategy_index);
        return closest_bot.id;
    } 
    let strategy = Box::new(strategy::offensive::Attacker::new(closest_bot.id));
    bigbro.move_bot_to_new_strategy(closest_bot.id, strategy);
    return closest_bot.id;
}   

/// Run the strategy for the running state with 5 line robots.
fn run_state_line_robots(bigbro: &mut BigBro, allies: Vec<&Robot<AllyInfo>>, ball: &Ball, world: &World, _tools_data: &mut ToolData) {
    if allies.len() == 0{return;}
    if world.geometry.ally_penalty.is_inside(&ball.position_2d()){
        let defense_wall_ids = put_defense_wall(bigbro, world, &allies, allies.len());
    }else{
        let defense_wall_ids = put_defense_wall(bigbro, world, &allies, allies.len() -1);
        let offensive_line: Vec<&Robot<AllyInfo>> = allies.iter().filter(|bot| !defense_wall_ids.contains(&bot.id)).map(|bot| *bot).collect();
        put_attacker(bigbro, world, &offensive_line, ball);
    }
}

/// Run the strategy for the running state.
pub fn run_state(bigbro: &mut BigBro, world: &World, tools_data: &mut ToolData) {
    put_goal(bigbro);
    let ball = match &world.ball {
        Some(ball) => ball,
        None => return,
    };
    let allies = filter_robots_not_in_ids(world.allies_bot.values().collect(), &vec![KEEPER_ID]);
    run_state_line_robots(bigbro, allies, ball, world, tools_data);
}