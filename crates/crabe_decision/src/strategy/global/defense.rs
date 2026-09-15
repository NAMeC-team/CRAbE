use super::{GlobalStrategy, GlobalStrategyId};
use crate::decision::{DecisionTree, Formation};
use crate::tactical_action::defensive::*;

pub struct DefenseStrategy;

impl DefenseStrategy {
    pub fn build() -> GlobalStrategy {
        let mut tree_lambda = DecisionTree::new();
        tree_lambda.add_action(create_mark_ball_carrier_action());
        tree_lambda.add_action(create_mark_dangerous_opponent_action());
        tree_lambda.add_action(create_defensive_wall_action()); // Action par défaut
        
        GlobalStrategy {
            id: GlobalStrategyId::Defense,
            ideal_formation: Formation::new_defense(),
            tree_keeper: DecisionTree::new(), // TODO: Actions gardien
            tree_ball_carrier: DecisionTree::new(), // TODO
            tree_lambda,
        }
    }
}