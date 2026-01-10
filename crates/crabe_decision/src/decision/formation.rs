use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoleId {
    Keeper,
    DefensiveWall,
    BallCarrierMarker,
    OpponentMarker,
    Supporter,
    Attacker,
}

impl RoleId {
    /// Priorité du rôle (plus haut = plus important à respecter)
    pub fn priority(&self) -> u8 {
        match self {
            RoleId::Keeper => 100,
            RoleId::BallCarrierMarker => 90,
            RoleId::DefensiveWall => 80,
            RoleId::OpponentMarker => 70,
            RoleId::Supporter => 50,
            RoleId::Attacker => 60,
        }
    }
}

/// Formation mère définissant la composition idéale de l'équipe
#[derive(Debug, Clone)]
pub struct Formation {
    /// Rôle -> nombre de robots souhaités
    pub roles: HashMap<RoleId, usize>,
}

impl Formation {
    pub fn new_defense() -> Self {
        let mut roles = HashMap::new();
        roles.insert(RoleId::Keeper, 1);
        roles.insert(RoleId::DefensiveWall, 2);
        roles.insert(RoleId::BallCarrierMarker, 1);
        roles.insert(RoleId::OpponentMarker, 2);
        Self { roles }
    }
    
    pub fn new_attack() -> Self {
        let mut roles = HashMap::new();
        roles.insert(RoleId::Keeper, 1);
        roles.insert(RoleId::DefensiveWall, 1);
        roles.insert(RoleId::Attacker, 2);
        roles.insert(RoleId::Supporter, 2);
        Self { roles }
    }
}