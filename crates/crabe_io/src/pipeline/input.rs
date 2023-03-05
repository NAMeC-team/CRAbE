use crate::league::game_controller::{GameController, GameControllerConfig};
use crate::league::vision::{Vision, VisionConfig};
use clap::Args;
use crabe_framework::component::{Component, InputComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::FeedbackMap;
use crabe_framework::data::receiver::InboundData;

#[derive(Args)]
pub struct InputConfig {
    #[arg(long)]
    gc: bool,

    #[command(flatten)]
    #[command(next_help_heading = "Vision")]
    pub vision_cfg: VisionConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Game Controller")]
    pub gc_cfg: GameControllerConfig,
}

pub trait ReceiverTask {
    fn fetch(&mut self, input: &mut InboundData);
    fn close(&mut self);
}

pub struct InputPipeline {
    receivers: Vec<Box<dyn ReceiverTask>>,
}

impl InputPipeline {
    pub fn with_config(config: InputConfig, _: &CommonConfig) -> Self {
        let mut tasks: Vec<Box<dyn ReceiverTask>> =
            vec![Box::new(Vision::with_config(config.vision_cfg))];

        if config.gc {
            tasks.push(Box::new(GameController::with_config(config.gc_cfg)));
        }

        Self { receivers: tasks }
    }
}

impl Component for InputPipeline {
    fn close(mut self) {
        self.receivers.drain(..).for_each(|mut x| x.close());
    }
}

impl InputComponent for InputPipeline {
    fn step(&mut self, _feedback: &mut FeedbackMap) -> InboundData {
        let mut data = InboundData::default();
        self.receivers.iter_mut().for_each(|x| x.fetch(&mut data));
        data
    }
}

#[derive(Args)]
pub struct OutputPipeline {

}