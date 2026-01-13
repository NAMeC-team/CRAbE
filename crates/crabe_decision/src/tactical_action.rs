pub mod defensive;


use crate::action::ActionWrapper;
use crate::decision::{ActionMetric, RoleId};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{Robot, AllyInfo, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionId {
    // Défensives
    MarkBallCarrier,
    MarkDangerousOpponent,
    FormDefensiveWall,
    StayInGoal,
    
    // Offensives
    ShootOnGoal,
    PassToBestPlayer,
    DribbleForward,
    PositionForPass,
    
    // Par défaut
    DefaultPosition,
}

impl ActionId {
    /// Nombre maximum de robots pouvant exécuter cette action simultanément
    pub fn max_robots_allowed(&self) -> usize {
        match self {
            ActionId::MarkBallCarrier => 1,
            ActionId::MarkDangerousOpponent => 2,
            ActionId::ShootOnGoal => 1,
            ActionId::StayInGoal => 1,
            _ => usize::MAX, // Pas de limite
        }
    }
}

use crate::action::move_to::MoveTo;

/// Action tactique = logique de décision + stockage du résultat
pub struct TacticalAction {
    id: ActionId,
    role: Option<RoleId>,
    evaluate_fn: Box<dyn FnMut(&Robot<AllyInfo>, &World) -> Option<(ActionMetric, MoveTo)> + Send + Sync>,
    cached_move: Option<MoveTo>,
}

impl TacticalAction {
    pub fn new<F>(id: ActionId, role: Option<RoleId>, evaluate_fn: F) -> Self
    where
        F: FnMut(&Robot<AllyInfo>, &World) -> Option<(ActionMetric, MoveTo)> + Send + Sync + 'static,
    {
        Self {
            id,
            role,
            evaluate_fn: Box::new(evaluate_fn),
            cached_move: None,
        }
    }
    
    /// Évalue l'action et stocke le MoveTo calculé
    pub fn evaluate(&mut self, robot: &Robot<AllyInfo>, world: &World) -> Option<ActionMetric> {
        if let Some((metric, move_to)) = (self.evaluate_fn)(robot, world) {
            self.cached_move = Some(move_to);
            Some(metric)
        } else {
            self.cached_move = None;
            None
        }
    }
    
    /// Exécute l'action en utilisant le MoveTo précédemment calculé
    pub fn execute(&self, robot_id: u8, action_wrapper: &mut ActionWrapper) {
        if let Some(move_to) = &self.cached_move {
            action_wrapper.push(robot_id, move_to.clone());
        }
    }
    
    pub fn action_id(&self) -> ActionId {
        self.id
    }
    
    pub fn corresponding_role(&self) -> Option<RoleId> {
        self.role
    }
}
