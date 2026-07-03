use crate::assignment::StrategyAssigner;
use crate::behaviors::blackboard::{RobotIntentWriters, StateSlots};
use crate::behaviors::executor::{Environment, Executor};
use crate::behaviors::{BehaviorFrame, Context, Node};
use crate::managers::Manager;
use crate::plays::attack::Attack;
use crate::plays::{Play, PlayStrategies};
use bimap::BiHashMap;
use crabe_framework::data::annotation::AnnotationStore;
use crabe_framework::data::output::{CommandMap};
use crabe_framework::data::world::World;
use std::collections::HashMap;
use tracing::warn;

#[derive(Debug)]
pub struct SimpleManager<A> {
    executors: ExecutorStore,
    strategy_assigner: A,
    current_play: Box<dyn Play>,
    assigned: BiHashMap<u8, usize>,
    should_update: bool,
}

#[derive(Debug, Default)]
pub struct ExecutorStore {
    outfield: HashMap<usize, Executor>,
    keeper: Option<Executor>,
}

impl ExecutorStore {
    pub fn update(&mut self, play_strategies: PlayStrategies) {
        self.outfield = HashMap::from_iter(
            play_strategies
                .outfield
                .into_iter()
                .map(|s| Executor::new(s))
                .enumerate(),
        );
        self.keeper = Some(Executor::new(play_strategies.keeper));
    }

    pub fn tick(
        &mut self,
        assigned: &BiHashMap<u8, usize>,
        environment: &mut Environment,
    ) -> HashMap<u8, BehaviorFrame> {
        let keeper_id = environment.world.keeper_id;
        let mut frames: HashMap<u8, BehaviorFrame> = self
            .outfield
            .iter_mut()
            .filter_map(|(strategy_id, executor)| {
                if let Some(robot_id) = assigned.get_by_right(strategy_id) {
                    Some((*robot_id, executor.tick(environment, *robot_id)))
                } else {
                    warn!("Robot {} not assigned", strategy_id);
                    None
                }
            })
            .collect();

        if let Some(keeper) = &mut self.keeper {
            frames.insert(keeper_id, keeper.tick(environment, keeper_id));
        }

        frames
    }
}

impl<A: StrategyAssigner> SimpleManager<A> {
    pub fn new(strategy_assigner: A) -> Self {
        Self {
            executors: ExecutorStore::default(),
            assigned: BiHashMap::new(),
            strategy_assigner,
            current_play: Box::new(Attack),
            should_update: true,
        }
    }

    pub fn update_strategies(&mut self, world: &World) {
        let strategies = self.current_play.generate_strategies(&world);
        self.executors.update(strategies);
    }
}

impl<A: StrategyAssigner> Manager for SimpleManager<A> {
    fn decide(&mut self, world: &World, annotations: &mut AnnotationStore) -> CommandMap {
        if let Some(new_play) = self.current_play.transition(&world) {
            self.current_play = new_play;
            self.should_update = true;
        }
        if self.should_update {
            self.update_strategies(world);
            self.should_update = false;
        }

        self.assigned = self.strategy_assigner.assign_strategies(
            self.executors
                .outfield
                .iter()
                .map(|(id, e)| (*id, e.strategy())),
            world,
        );
        let mut environment = Environment { world, annotations };
        let frames = self.executors.tick(&self.assigned, &mut environment);

        frames.iter().map(|(id, f)| (*id, f.command)).collect()
    }
}
