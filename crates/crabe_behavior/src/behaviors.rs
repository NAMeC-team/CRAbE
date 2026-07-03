pub mod actions;
pub mod blackboard;
pub mod composites;
pub mod conditions;
pub mod executor;

use crate::behaviors::blackboard::{Blackboard, InputPort, IntentWriter, OutputPort, RobotIntent};
use crate::behaviors::composites::flow::Flow;
use crate::behaviors::composites::parallel::Parallel;
use crate::behaviors::conditions::Condition;
use crabe_framework::data::annotation::{Annotation, AnnotationStore};
use crabe_framework::data::output::{Command};
use crabe_framework::data::world::{AllyInfo, Robot, World};
use nalgebra::Point2;
use std::borrow::Cow;
use std::fmt::Debug;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Status {
    Running,
    Failure,
    Success,
}

pub struct Context<'a> {
    pub robot_id: &'a u8,
    pub world: &'a World,
    pub annotations: &'a mut AnnotationStore,
    blackboard: &'a mut Blackboard,
}

pub struct BehaviorFrame {
    pub command: Command,
    pub state: Status,
}

impl<'a> Context<'a> {
    pub fn world(&self) -> &World {
        self.world
    }

    pub fn robot(&self) -> Option<&Robot<AllyInfo>> {
        self.world.allies_bot.get(&self.robot_id)
    }

    pub fn timestamp(&self) -> Instant {
        self.world.timestamp
    }

    #[inline]
    pub fn get<T>(&self, port: &InputPort<T>) -> &T {
        (port.slot.accessor)(&self.blackboard.state)
    }

    pub fn set<T>(&mut self, port: &OutputPort<T>, value: T) {
        let out = (port.slot.accessor_mut)(&mut self.blackboard.state);
        *out = value;
    }

    pub fn commit<T>(&mut self, writer: &IntentWriter<T>, value: T) {
        (writer.0)(&mut self.blackboard.intents, value);
    }
}

pub struct Output {
    pub command: Command,
    pub annotations: Vec<Annotation>,
}

pub trait Behavior: Send + Debug {
    fn name(&self) -> &'static str {
        "Unnamed"
    }
    fn tick(&mut self, ctx: &mut Context) -> Status;
}

#[derive(Debug)]
pub enum Node {
    Parallel(Parallel),
    Flow(Flow),
    Action(Box<dyn Behavior>),
    Condition(Condition),
}

impl Node {
    pub fn children(&self) -> &[Node] {
        match self {
            Node::Parallel(p) => &p.children(),
            Node::Flow(f) => &f.children(),
            _ => &[],
        }
    }
}

impl Behavior for Node {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        match self {
            Node::Parallel(parallel) => parallel.tick(ctx),
            Node::Flow(sequencer) => sequencer.tick(ctx),
            Node::Action(leaf) => leaf.tick(ctx),
            Node::Condition(condition) => condition.tick(ctx),
        }
    }
}

pub fn action(b: impl Behavior + 'static) -> Node {
    Node::Action(Box::new(b))
}

pub fn cond(name: &'static str, p: impl Fn(&Context) -> bool + Send + 'static) -> Node {
    Node::Condition(Condition::new(name, p))
}

pub fn seq(children: Vec<Node>) -> Node {
    Node::Flow(Flow::sequencer(children))
}

pub fn sel(children: Vec<Node>) -> Node {
    Node::Flow(Flow::selector(children))
}

pub fn par(children: Vec<Node>) -> Node {
    Node::Parallel(Parallel::new(children))
}
