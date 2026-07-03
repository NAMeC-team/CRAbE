use crate::behaviors::{Behavior, Context, Node, Status};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Flow {
    index: usize,
    children: Vec<Node>,
    mode: Mode,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    Sequencer,
    Selector,
}

impl Mode {
    fn short_circuit_state(&self) -> Status {
        match self {
            Mode::Sequencer => Status::Failure,
            Mode::Selector => Status::Success,
        }
    }
}

impl Flow {
    pub fn new(children: Vec<Node>, mode: Mode) -> Flow {
        Self {
            index: 0,
            children,
            mode,
        }
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn sequencer(children: Vec<Node>) -> Self {
        Self::new(children, Mode::Sequencer)
    }

    pub fn selector(children: Vec<Node>) -> Self {
        Self::new(children, Mode::Selector)
    }

    fn reset(&mut self) {
        self.index = 0;
    }
}

impl Behavior for Flow {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        while self.index < self.children.len() {
            let child_state = self.children[self.index].tick(ctx);

            match child_state {
                Status::Running => return Status::Running,
                s if s == self.mode.short_circuit_state() => {
                    self.reset();
                    return s;
                }
                _ => self.index += 1,
            }
        }

        self.reset();

        match self.mode.short_circuit_state() {
            Status::Failure => Status::Success,
            Status::Success => Status::Failure,
            _ => Status::Success,
        }
    }
}
