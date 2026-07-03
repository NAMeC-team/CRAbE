use crate::behaviors::blackboard::OutputPort;
use crate::behaviors::{Behavior, Context, Status};
use std::fmt::Debug;

#[derive(Debug)]
pub struct WriteConst<T: Debug> {
    value: T,
    output_port: OutputPort<T>,
}

impl<T: Clone + Debug + Send> WriteConst<T> {
    pub fn new(value: T, output_port: OutputPort<T>) -> Self {
        Self { value, output_port }
    }
}

impl<T: Clone + Debug + Send> Behavior for WriteConst<T> {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        ctx.set(&self.output_port, self.value.clone());
        Status::Success
    }
}
