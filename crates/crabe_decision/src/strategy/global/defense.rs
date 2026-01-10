use super::{GlobalStrategy, GlobalStrategyId};
use crate::decision::{DecisionTree, Formation};
use crate::tactical_action::defensive::*;

pub struct DefenseStrategy;

impl DefenseStrategy {
    pub fn build() -> GlobalStrategy {
        let mut tree_lambda = DecisionTree::new();
        tree_lambda.add_action(Box::new(MarkBallCarrierAction));
        tree_lambda.add_action(Box::new(MarkDangerousOpponentAction));
        tree_lambda.add_action(Box::new(DefensiveWallAction)); // Action par défaut
        
        GlobalStrategy {
            id: GlobalStrategyId::Defense,
            ideal_formation: Formation::new_defense(),
            tree_keeper: DecisionTree::new(), // TODO: Actions gardien
            tree_ball_carrier: DecisionTree::new(), // TODO
            tree_lambda,
        }
    }
}