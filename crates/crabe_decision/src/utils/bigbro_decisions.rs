use crabe_framework::data::{tool::ToolData, world::{AllyInfo, Ball, Robot, TeamColor, World}};

use crate::{manager::bigbro::BigBro, strategy::{self, defensive::{DefenseWall, GoalKeeper}, formations::Stop, offensive::Attacker}};

use super::{closest_bot_to_point, closest_bots_to_point, filter_robots_in_ids, filter_robots_not_in_ids, KEEPER_ID};

/// Put all bots to the Stop strategy.
pub fn everyone_stop(bigbro: &mut BigBro) {
    if let Some(strategy_index) = bigbro.get_index_strategy_with_name("Stop") {
        for bot_id in 0..6 {
            bigbro.move_bot_to_existing_strategy(bot_id, strategy_index);
        }
    }else{
        let strategy = Box::new(Stop::new(vec![]));
        bigbro.move_bots_to_new_strategy(vec![0, 1, 2, 3, 4, 5], strategy);
    }
}

fn put_goal(bigbro: &mut BigBro) {
    if let Some(current_strategy) = bigbro.get_bot_current_strategy(KEEPER_ID) {
        if current_strategy.name() == "GoalKeeper" {
            return;
        }
    }
    let strategy = Box::new(GoalKeeper::new(KEEPER_ID));
    bigbro.move_bot_to_new_strategy(KEEPER_ID, strategy);
}

fn put_defense_wall(bigbro: &mut BigBro, world: &World, bots: &Vec<&Robot<AllyInfo>>, num_bots: usize) -> Vec<u8> {
    // let allies_no_keeper = filter_robots_not_in_ids(bots, &vec![KEEPER_ID]);
    let allies_no_keeper = bots.iter().filter(|bot| bot.id != KEEPER_ID).map(|bot| *bot).collect();
    let allies_closest = closest_bots_to_point(allies_no_keeper, world.geometry.ally_goal.line.center());
    let mut ids = vec![];
    for bot in allies_closest.iter().take(num_bots) {
        ids.push(bot.id);
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
    } else{
        let strategy = Box::new(strategy::offensive::Attacker::new(closest_bot.id));
        bigbro.move_bot_to_new_strategy(closest_bot.id, strategy);
    }
    return closest_bot.id;
}   

fn put_marking(bigbro: &mut BigBro, world: &World, bots: &Vec<&Robot<AllyInfo>>, ball: &Ball) {
    let mut marked_enemies = vec![];
    for bot in bots{
        if let Some(current_strategy) = bigbro.get_bot_current_strategy(bot.id) {
            if current_strategy.name() == "BotMarking" || current_strategy.name() == "Receiver" {
                continue;
            }
        }
        let availables_enemies = filter_robots_not_in_ids(world.enemies_bot.values().collect(), &marked_enemies);
        let closest_enemy = match closest_bot_to_point(availables_enemies, ball.position_2d()) {
            Some(bot) => bot,
            None => return,
        };
        marked_enemies.push(closest_enemy.id);
        let strategy = Box::new(strategy::defensive::BotMarking::new(bot.id, closest_enemy.id));
        bigbro.move_bot_to_new_strategy(bot.id, strategy);
    }
}

fn run_state_5line_robots(bigbro: &mut BigBro, allies: Vec<&Robot<AllyInfo>>, ball: &Ball, world: &World, _tools_data: &mut ToolData) {
    let defense_wall_ids = put_defense_wall(bigbro, world, &allies, 2);
    let offensive_line: Vec<&Robot<AllyInfo>> = allies.iter().filter(|bot| !defense_wall_ids.contains(&bot.id)).map(|bot| *bot).collect();

    let attacker_id = put_attacker(bigbro, world, &offensive_line, ball);
    let other_bots: Vec<&Robot<AllyInfo>> = offensive_line.iter().filter(|bot| bot.id != attacker_id).map(|bot| *bot).collect();
    match ball.possession {
        Some(team_possessing) => {
            if team_possessing == world.team_color{
                put_marking(bigbro, world, &other_bots, ball);
            }else{
                put_marking(bigbro, world, &other_bots, ball);
            }
        },
        None => {
            put_marking(bigbro, world, &other_bots, ball);
        }
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
    run_state_5line_robots(bigbro, allies, ball, world, tools_data);
}