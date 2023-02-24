use crate::gc::{GameController, GameControllerConfig};
use crate::vision::{Vision, VisionConfig};
use clap::Args;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

#[derive(Args)]
pub struct DataReceiverConfig {
    #[arg(long)]
    gc: bool,

    #[command(flatten)]
    #[command(next_help_heading = "Vision")]
    pub vision_cfg: VisionConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Game Controller")]
    pub gc_cfg: GameControllerConfig,
}

#[derive(Debug)]
pub struct ReceiverDataSet {
    pub vision_packet: Vec<SslWrapperPacket>,
    pub gc_packet: Vec<Referee>,
}

impl Default for ReceiverDataSet {
    fn default() -> Self {
        Self {
            vision_packet: vec![],
            gc_packet: vec![],
        }
    }
}

pub trait ReceiverTask {
    fn fetch(&mut self, input: &mut ReceiverDataSet);
    fn close(&mut self);
}

pub struct DataReceiverPipeline {
    receivers: Vec<Box<dyn ReceiverTask>>,
}

impl DataReceiverPipeline {
    pub fn with_config(config: DataReceiverConfig) -> Self {
        let mut tasks: Vec<Box<dyn ReceiverTask>> =
            vec![Vision::with_config_boxed(config.vision_cfg)];

        if config.gc {
            tasks.push(GameController::with_config_boxed(config.gc_cfg));
        }

        Self {
            receivers: tasks, // How to box ?
        }
    }

    pub fn run(&mut self) -> ReceiverDataSet {
        let mut data = ReceiverDataSet::default();
        self.receivers.iter_mut().for_each(|x| x.fetch(&mut data));
        data
    }

    pub fn close(&mut self) {
        self.receivers.iter_mut().for_each(|x| x.close());
    }
}
