use std::cmp::Ordering;

/// Wrapper pour f32 qui implémente Ord
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFloat(pub f32);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Enum contenant tous les types de métriques possibles pour les actions
#[derive(Debug, Clone)]
pub enum ActionMetric {
    /// Simple distance (plus petit = meilleur)
    Distance(OrderedFloat),
    
    /// Distance et angle pondérés
    DistanceAndAngle {
        distance: OrderedFloat,
        angle: OrderedFloat,
    },
    
    /// Score de contrôle de zone
    ZoneControl {
        coverage: OrderedFloat,
        threat_level: OrderedFloat,
    },
    
    /// Score d'attaque
    AttackScore {
        shot_quality: OrderedFloat,
        pass_options: u8,
    },
}

impl ActionMetric {
    /// Compare deux métriques (ordre dépend du type)
    pub fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ActionMetric::Distance(a), ActionMetric::Distance(b)) => a.cmp(b),
            
            (
                ActionMetric::DistanceAndAngle { distance: d1, angle: a1 },
                ActionMetric::DistanceAndAngle { distance: d2, angle: a2 },
            ) => {
                let score1 = d1.0 * 0.7 + a1.0 * 0.3;
                let score2 = d2.0 * 0.7 + a2.0 * 0.3;
                OrderedFloat(score1).cmp(&OrderedFloat(score2))
            }
            
            (
                ActionMetric::ZoneControl { coverage: c1, threat_level: t1 },
                ActionMetric::ZoneControl { coverage: c2, threat_level: t2 },
            ) => {
                let score1 = c1.0 * 0.6 + t1.0 * 0.4;
                let score2 = c2.0 * 0.6 + t2.0 * 0.4;
                OrderedFloat(score2).cmp(&OrderedFloat(score1)) // Inversé (plus = meilleur)
            }
            
            (
                ActionMetric::AttackScore { shot_quality: q1, pass_options: p1 },
                ActionMetric::AttackScore { shot_quality: q2, pass_options: p2 },
            ) => {
                let score1 = q1.0 + (*p1 as f32) * 0.1;
                let score2 = q2.0 + (*p2 as f32) * 0.1;
                OrderedFloat(score2).cmp(&OrderedFloat(score1)) // Inversé
            }
            
            _ => Ordering::Equal, // Types incompatibles
        }
    }
}