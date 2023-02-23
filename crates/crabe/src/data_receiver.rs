use crate::vision::{Vision, VisionConfig};
use clap::Args;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

#[derive(Args)]
pub struct DataReceiverConfig {
    #[command(flatten)]
    #[command(next_help_heading = "Vision")]
    pub vision: VisionConfig,

    #[arg(long)]
    gc: bool,
}

#[derive(Debug)]
pub struct ReceiverDataSet {
    pub vision_packet: Vec<SslWrapperPacket>,
    pub gc_packet: Vec<Referee>,
}

pub trait ReceiverTask {
    fn fetch(&mut self, input: &mut ReceiverDataSet);
}

pub struct DataReceiverPipeline {
    receivers: Vec<Box<dyn ReceiverTask>>,
}

impl DataReceiverPipeline {
    pub fn with_config(config: DataReceiverConfig) -> Self {
        Self {
            receivers: vec![Vision::with_config_boxed(config.vision)], // How to box ?
        }
    }

    pub fn run(&mut self) -> ReceiverDataSet {
        let mut data = ReceiverDataSet {
            vision_packet: vec![],
            gc_packet: vec![],
        };
        self.receivers.iter_mut().for_each(|x| x.fetch(&mut data));
        data
    }
}
