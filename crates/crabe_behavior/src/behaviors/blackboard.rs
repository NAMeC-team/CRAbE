use crabe_behavior_derive::{Slots, Writers};
use nalgebra::{Point2, Vector3};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
//use crabe_behavior_derive::{IntentPorts, Slots};
use crabe_framework::data::output::{Command, Kick};
use crabe_framework::data::world::{Pose, RobotVelocity};

pub struct Slot<P, T> {
    pub(super) accessor: fn(&P) -> &T,
    pub(super) accessor_mut: fn(&mut P) -> &mut T,
}

impl<P, T> Copy for Slot<P, T> {}
impl<P, T> Debug for Slot<P, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slot").finish() // TODO
    }
}

impl<P, T> Clone for Slot<P, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Slot<State, T> {
    pub fn output(self) -> OutputPort<T> {
        OutputPort::new(self)
    }
    pub fn input(self) -> InputPort<T> {
        InputPort::new(self)
    }
}

impl<P, T> Slot<P, T> {
    pub fn new(accessor: fn(&P) -> &T, accessor_mut: fn(&mut P) -> &mut T) -> Self {
        Self {
            accessor,
            accessor_mut,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActionIntent {
    Kick(Kick),
    Dribble(f32),
}
#[derive(Clone, Default, Writers, Debug)]
pub struct RobotIntent {
    pub movement: RobotVelocity,
    pub action: Option<ActionIntent>,
    pub charge: bool,
}

impl RobotIntent {
    pub fn reset(&mut self) {
        self.movement = Default::default();
        self.action = None;
    }
}

impl Into<Command> for &RobotIntent {
    fn into(self) -> Command {
        let (kick, dribbler) = match self.action {
            Some(ActionIntent::Kick(kick_action)) => (Some(kick_action), 0.0),
            Some(ActionIntent::Dribble(dribble_action)) => (None, dribble_action),
            None => (None, 0.0),
        };

        Command {
            forward_velocity: self.movement.linear.x as f32,
            left_velocity: self.movement.linear.y as f32,
            angular_velocity: self.movement.angular as f32,
            charge: self.charge,
            kick,
            dribbler,
            avoid_ball: false,
        }
    }
}

#[derive(Default, Clone, Slots, Debug)]
pub struct State {
    // TODO: rename to avoid confusion with node state
    pub target: Pose,
}

#[derive(Default, Debug)]
pub struct Blackboard {
    pub state: State,
    pub intents: RobotIntent,
}

#[derive(Copy, Clone, Debug)]
pub struct InputPort<T> {
    pub(super) slot: Slot<State, T>,
}

impl<T> InputPort<T> {
    pub fn new(slot: Slot<State, T>) -> Self {
        Self { slot }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OutputPort<T> {
    pub(super) slot: Slot<State, T>,
}

impl<T> OutputPort<T> {
    pub fn new(slot: Slot<State, T>) -> Self {
        Self { slot }
    }
}

pub struct Writer<P, T>(pub(super) fn(&mut P, T));
impl<P, T> Debug for Writer<P, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Writer").finish()
    }
}

pub type IntentWriter<T> = Writer<RobotIntent, T>;

impl<T, P> Copy for Writer<T, P> {}

impl<T, P> Clone for Writer<T, P> {
    fn clone(&self) -> Self {
        *self
    }
}
