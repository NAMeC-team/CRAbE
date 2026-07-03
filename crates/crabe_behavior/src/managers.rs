pub mod simple;

use crabe_framework::data::annotation::AnnotationStore;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::world::World;
use std::fmt::Debug;

pub trait Manager: Send + Debug {
    //fn select_play(&mut self, world: &World) -> Box<dyn Play>
    // fn step(&mut self, data: &World, tool_data: &mut ToolData) -> CommandMap;
    fn decide(&mut self, world: &World, annotations: &mut AnnotationStore) -> CommandMap;
}
