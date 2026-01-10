pub mod defensive;
//pub mod offensive;

use crate::action::ActionWrapper;
use crate::decision::{ActionMetric, RoleId};
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::{AllyInfo, Robot, World};

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

/// Trait pour les actions tactiques (logique de décision)
pub trait TacticalAction: Send + Sync {
    /// Évalue si l'action est possible et retourne la métrique
    fn evaluate(&self, robot: &Robot<AllyInfo>, world: &World) -> Option<ActionMetric>;
    
    /// Exécute l'action en ajoutant des commandes atomiques à l'ActionWrapper
    fn execute(
        &self,
        robot_id: u8,
        robot: &Robot<AllyInfo>,
        world: &World,
        action_wrapper: &mut ActionWrapper,
        tools_data: &mut ToolData,
    );
    
    /// Identifiant unique de l'action
    fn action_id(&self) -> ActionId;
    
    /// Rôle correspondant dans la formation (optionnel)
    fn corresponding_role(&self) -> Option<RoleId> {
        None
    }
}