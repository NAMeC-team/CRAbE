use crate::constant::MAX_ROBOTS;

// TODO: Document
#[derive(Debug)]
pub struct FeedbackMap {
    pub feedbacks: [Option<Feedback>; MAX_ROBOTS]
}

#[derive(Debug)]
pub struct Feedback {
    pub has_ball: bool,
    pub voltage: u32,
}

pub struct CommandMap {
    commands: [Option<Command>; MAX_ROBOTS]
}

pub enum Kick {
    StraightKick { power: f32 },
    ChipKick { power: f32 },
}

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