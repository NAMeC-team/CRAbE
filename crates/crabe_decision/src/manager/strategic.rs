use crate::action::ActionWrapper;
use crate::decision::{ConflictResolver, RobotDecisionState};
use crate::manager::Manager;
use crate::strategy::global::{DefenseStrategy, GlobalStrategy, GlobalStrategyId};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use std::collections::HashMap;

/// Manager de stratégie avec système de décision collectif
pub struct StrategicManager {
    strategies: HashMap<GlobalStrategyId, GlobalStrategy>,
    robot_states: HashMap<u8, RobotDecisionState>,
    current_strategy_id: GlobalStrategyId,
}

impl StrategicManager {
    pub fn new() -> Self {
        let mut strategies = HashMap::new();
        strategies.insert(GlobalStrategyId::Defense, DefenseStrategy::build());
        // strategies.insert(GlobalStrategyId::Attack, AttackStrategy::build());
        
        Self {
            strategies,
            robot_states: HashMap::new(),
            current_strategy_id: GlobalStrategyId::Defense,
        }
    }
    
    fn initialize_robots(&mut self, world: &World) {
        for (robot_id, _) in &world.allies_bot {
            if !self.robot_states.contains_key(robot_id) {
                self.robot_states.insert(*robot_id, RobotDecisionState::new(*robot_id));
            }
        }
    }
    
    fn select_global_strategy(&mut self, world: &World) {
        // Simple : défense si adversaire a la balle, attaque sinon
        let enemy_has_ball = world.enemies_bot.iter().any(|(_, enemy)| {
            if let Some(ball) = &world.ball {
                enemy.distance(&ball.position_2d()) < 0.15
            } else {
                false
            }
        });
        
        self.current_strategy_id = if enemy_has_ball {
            GlobalStrategyId::Defense
        } else {
            GlobalStrategyId::Defense // TODO: GlobalStrategyId::Attack
        };
    }
    
    fn assign_initial_actions(
        robot_states: &mut HashMap<u8, RobotDecisionState>,
        strategy: &mut GlobalStrategy,
        world: &World,
    ) {
        for (robot_id, robot) in &world.allies_bot {
            let tree = strategy.get_tree_mut(*robot_id, world);
            let robot_state = robot_states.get_mut(robot_id).unwrap();
            
            for action in &mut tree.actions {
                if robot_state.banned_actions.contains(&action.action_id()) {
                    continue;
                }
                
                if let Some(metric) = action.evaluate(robot, world) {
                    robot_state.current_action = Some((action.action_id(), metric));
                    break;
                }
            }
        }
    }
    
    fn resolve_conflicts(
        robot_states: &mut HashMap<u8, RobotDecisionState>,
        strategy: &mut GlobalStrategy,
        world: &World,
    ) {
        let max_iterations = 20;
        
        for _iteration in 0..max_iterations {
            let conflicts = ConflictResolver::detect_conflicts(&robot_states);
            
            if conflicts.is_empty() {
                break;
            }
            
            for conflict in conflicts {
                let losers = ConflictResolver::resolve_conflict(&conflict, &robot_states);
                
                for loser_id in losers {
                    let robot_state = robot_states.get_mut(&loser_id).unwrap();
                    
                    if let Some((action_id, _)) = robot_state.current_action {
                        robot_state.banned_actions.insert(action_id);
                    }
                    robot_state.current_action = None;
                    
                    // Réassigner
                    if let Some(robot) = world.allies_bot.get(&loser_id) {
                        let tree = strategy.get_tree_mut(loser_id, world);
                        
                        for action in &mut tree.actions {
                            if robot_state.banned_actions.contains(&action.action_id()) {
                                continue;
                            }
                            
                            if let Some(metric) = action.evaluate(robot, world) {
                                robot_state.current_action = Some((action.action_id(), metric));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    fn execute_actions(
        &self,
        strategy: &GlobalStrategy,
        action_wrapper: &mut ActionWrapper,
        world: &World,
    ) {
        for (robot_id, robot_state) in &self.robot_states {
            if let Some((action_id, _)) = &robot_state.current_action {
                let tree = strategy.get_tree(*robot_id, world);
                
                for action in &tree.actions {
                    if action.action_id() == *action_id {
                        action.execute(*robot_id, action_wrapper);
                        break;
                    }
                }
            }
        }
    }
}

impl Manager for StrategicManager {
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) {
        // 0. Initialiser les robots si nécessaire
        self.initialize_robots(world);
        
        // 1. Reset des états
        for state in self.robot_states.values_mut() {
            state.reset_for_new_cycle();
        }
        
        // 2. Sélection stratégie globale
        self.select_global_strategy(world);
        let strategy_id = self.current_strategy_id;
        let strategy = self.strategies.get_mut(&strategy_id).unwrap();
        
        // 3. Attribution initiale des actions
        Self::assign_initial_actions(&mut self.robot_states, strategy, world);
        
        // 4. Résolution des conflits
        Self::resolve_conflicts(&mut self.robot_states, strategy, world);
        
        // 5. Exécution des actions
        action_wrapper.clear_all();
        let strategy = self.strategies.get(&strategy_id).unwrap();
        self.execute_actions(strategy, action_wrapper,world);
    }
}