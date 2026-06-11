use crate::behaviors::blackboard::{ActionIntent, IntentWriter, OutputPort};
use crate::behaviors::{Behavior, Context, Status};
use crabe_framework::data::output::{CommandPatch, Kick};
use std::time::Duration;

const CHARGE_DURATION: Duration = Duration::from_millis(200); // TODO

#[derive(Debug)]
pub struct KickAction {
    kick: Kick,
    kick_out: IntentWriter<Option<ActionIntent>>,
}

impl KickAction {
    pub fn new(kick_out: IntentWriter<Option<ActionIntent>>, kick: Kick) -> KickAction {
        KickAction { kick_out, kick }
    }
}

impl Behavior for KickAction {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        ctx.commit(&self.kick_out, Some(ActionIntent::Kick(self.kick)));
        Status::Success
    }
}
