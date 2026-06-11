use crate::strategies::Strategy;
use crate::utils::get_enemy_keeper_id;
use bimap::{BiHashMap, BiMap};
use crabe_framework::data::world::World;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::RandomState;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Keeper,
    Defender(usize),
    Midfielder(usize),
    Striker(usize),
}

#[derive(Debug)]
pub struct GreedyAssigner;

impl StrategyAssigner for GreedyAssigner {
    fn assign_strategies<'a>(
        &self,
        strategies: impl IntoIterator<Item = (usize, &'a Box<dyn Strategy>)>,
        world: &World,
    ) -> BiHashMap<u8, usize> {
        let mut assigned: BiMap<u8, usize> = BiMap::new();
        let mut available: HashSet<u8> = world.allies_bot.keys().cloned().collect();
        available.remove(&world.keeper_id);
        for (s_id, strategy) in strategies {
            let robot_costs: Vec<(u8, f64)> = available
                .iter()
                .map(|&robot_id| {
                    let bot = &world.allies_bot[&robot_id];
                    (robot_id, strategy.cost(bot, world))
                })
                .collect();

            if let Some(&(best_robot, _)) = robot_costs
                .iter()
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            {
                assigned.insert(best_robot, s_id);
                available.remove(&best_robot);
            }
        }

        assigned
    }
}

pub trait StrategyAssigner: Debug + Send {
    fn assign_strategies<'a>(
        &self,
        strategies: impl IntoIterator<Item = (usize, &'a Box<dyn Strategy>)>,
        world: &World,
    ) -> BiHashMap<u8, usize>;
}
