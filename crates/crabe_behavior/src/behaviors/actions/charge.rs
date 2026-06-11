use crate::behaviors::blackboard::{ActionIntent, IntentWriter, OutputPort};
use crate::behaviors::{Behavior, Context, Status};

#[derive(Debug)]
pub struct ChargeAction {
    charge_out: IntentWriter<bool>,
}

impl ChargeAction {
    pub fn new(charge_out: IntentWriter<bool>) -> Self {
        Self { charge_out }
    }
}

impl Behavior for ChargeAction {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        ctx.commit(&self.charge_out, true);
        Status::Success
    }
}
