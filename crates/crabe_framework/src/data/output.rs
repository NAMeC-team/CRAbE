use std::collections::HashMap;

// TODO: Document
pub type FeedbackMap = HashMap<u32, Feedback>;

#[derive(Debug)]
pub struct Feedback {
    pub has_ball: bool,
    pub voltage: f32,
}

pub type CommandMap = HashMap<u32, Command>;

pub enum Kick {
    StraightKick { power: f32 },
    ChipKick { power: f32 },
}

#[derive(Default)]
pub struct Command {
    /// Velocity forward in m.s-1 (towards the dribbler)
    pub forward_velocity: f32,
    /// Velocity to the left in m.s-1
    pub left_velocity: f32,
    /// Angular velocity rad.s-1 in (counter-clockwise)
    pub angular_velocity: f32,
    /// Order to charge the capacitor of the robot
    pub charge: bool,
    /// Order to kick the ball, if None doesn't KICK
    pub kick: Option<Kick>,
    /// Dribbler speed in rounds per minute rpm
    pub dribbler: f32,
}
