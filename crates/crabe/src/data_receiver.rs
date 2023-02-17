use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

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
    pub fn new(receivers: Vec<Box<dyn ReceiverTask>>) -> Self {
        DataReceiverPipeline { receivers }
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
