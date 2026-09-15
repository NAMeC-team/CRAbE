use super::metrics::ActionMetric;
use crate::tactical_action::ActionId;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Conflict {
    pub action_id: ActionId,
    pub robots: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RobotDecisionState {
    pub robot_id: u8,
    pub banned_actions: std::collections::HashSet<ActionId>,
    pub current_action: Option<(ActionId, ActionMetric)>,
}

impl RobotDecisionState {
    pub fn new(robot_id: u8) -> Self {
        Self {
            robot_id,
            banned_actions: std::collections::HashSet::new(),
            current_action: None,
        }
    }
    
    pub fn reset_for_new_cycle(&mut self) {
        self.banned_actions.clear();
        self.current_action = None;
    }
}

pub struct ConflictResolver;

impl ConflictResolver {
    /// Détecte les conflits (plusieurs robots veulent la même action limitée)
    pub fn detect_conflicts(
        robot_states: &HashMap<u8, RobotDecisionState>,
    ) -> Vec<Conflict> {
        let mut action_groups: HashMap<ActionId, Vec<u8>> = HashMap::new();
        
        for (robot_id, state) in robot_states {
            if let Some((action_id, _)) = &state.current_action {
                action_groups
                    .entry(*action_id)
                    .or_insert_with(Vec::new)
                    .push(*robot_id);
            }
        }
        
        let mut conflicts = Vec::new();
        for (action_id, robots) in action_groups {
            let max_allowed = action_id.max_robots_allowed();
            
            if robots.len() > max_allowed {
                conflicts.push(Conflict { action_id, robots });
            }
        }
        
        conflicts
    }
    
    /// Résout un conflit en désignant les perdants
    pub fn resolve_conflict(conflict: &Conflict, robot_states: &HashMap<u8, RobotDecisionState>) -> Vec<u8> {
        let mut robots_with_metrics: Vec<(u8, ActionMetric)> = conflict
            .robots
            .iter()
            .filter_map(|&robot_id| {
                let state = robot_states.get(&robot_id)?;
                let (_, metric) = state.current_action.as_ref()?;
                Some((robot_id, metric.clone()))
            })
            .collect();
        
        // Trier par métrique (meilleurs en premier)
        robots_with_metrics.sort_by(|a, b| a.1.compare(&b.1));
        
        let max_allowed = conflict.action_id.max_robots_allowed();
        
        // Retourner les IDs des perdants
        robots_with_metrics
            .iter()
            .skip(max_allowed)
            .map(|(id, _)| *id)
            .collect()
    }
}