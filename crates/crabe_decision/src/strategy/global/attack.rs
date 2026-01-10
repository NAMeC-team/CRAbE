use super::{GlobalStrategy, GlobalStrategyId};
use crate::decision::{DecisionTree, Formation};

pub struct AttackStrategy;

impl AttackStrategy {
    pub fn build() -> GlobalStrategy {
        GlobalStrategy {
            id: GlobalStrategyId::Attack,
            ideal_formation: Formation::new_attack(),
            tree_keeper: DecisionTree::new(),
            tree_ball_carrier: DecisionTree::new(),
            tree_lambda: DecisionTree::new(),
        }
    }
}