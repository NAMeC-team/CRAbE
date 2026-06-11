use crate::behaviors::{Behavior, Context, Status};
use std::fmt::{Debug, Formatter};

pub struct Condition {
    name: &'static str,
    predicate: Box<dyn Fn(&Context) -> bool + Send>,
}

impl Debug for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name) // TODO
    }
}

impl Condition {
    pub fn new(name: &'static str, predicate: impl Fn(&Context) -> bool + Send + 'static) -> Self {
        Self {
            name,
            predicate: Box::new(predicate),
        }
    }
}

impl Behavior for Condition {
    fn name(&self) -> &'static str {
        self.name
    }
    fn tick(&mut self, ctx: &mut Context) -> Status {
        if (self.predicate)(ctx) {
            Status::Success
        } else {
            Status::Failure
        }
    }
}
