pub mod defense;
pub mod attack;

pub use defense::DefenseStrategy;
pub use attack::AttackStrategy;

use crate::decision::{DecisionTree, Formation};
use crate::utils::KEEPER_ID;



#[derive(Debug, Clone, Copy, PartialEq, Eq,Hash)]
pub enum GlobalStrategyId {
    Defense,
    Attack,
    Balanced,
}

/// Stratégie globale contenant formation mère et arbres de décision
pub struct GlobalStrategy {
    pub id: GlobalStrategyId,
    pub ideal_formation: Formation,
    pub tree_keeper: DecisionTree,
    pub tree_ball_carrier: DecisionTree,
    pub tree_lambda: DecisionTree,
}

impl GlobalStrategy {
    /// Sélectionne l'arbre approprié pour un robot
    pub fn get_tree(&self, robot_id: u8, world: &crabe_framework::data::world::World) -> &DecisionTree {
        // Robot KEEPER_ID = gardien
        if robot_id == KEEPER_ID {
            return &self.tree_keeper;
        }
        
        // Si le robot a la balle
        if let Some(robot) = world.allies_bot.get(&robot_id) {
            if let Some(ball) = &world.ball {
                if robot.distance(&ball.position_2d()) < 0.15 {
                    return &self.tree_ball_carrier;
                }
                
            }
        }
        
        &self.tree_lambda
    }
}