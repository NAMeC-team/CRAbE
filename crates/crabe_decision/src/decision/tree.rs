use crate::tactical_action::TacticalAction;

/// Arbre de décision = liste ordonnée d'actions tactiques
pub struct DecisionTree {
    pub actions: Vec<Box<dyn TacticalAction>>,
}

impl DecisionTree {
    /// Crée un nouvel arbre vide
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
    
    /// Ajoute une action à l'arbre (ordre = priorité)
    pub fn add_action(&mut self, action: Box<dyn TacticalAction>) {
        self.actions.push(action);
    }
}
