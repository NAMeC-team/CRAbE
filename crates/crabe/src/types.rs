use crabe_framework::data::IncomingDataset;
use crate::world::World;

pub struct Feedback;

pub struct Commands;

trait ReceiverPipeline {
    fn step(&self, feedback: Feedback) -> IncomingDataset;
    fn close();
}

trait FilterPipeline {
    fn step(&self, data: IncomingDataset) -> World;
    fn close();
}

trait DecisionPipeline {
    fn step(&self, data: &mut World) -> Commands;
}

trait ToolsPipeline {
    fn step(&self, data: &mut World, command: &mut Commands) -> Commands;
    fn close();
}

trait GuardPipeline {
    fn step(&self, data: &mut World, command: &mut Commands);
    fn close();
}

trait OutputPipeline {
    fn step(&self, command: &mut Commands) -> Feedback;
    fn close();
}