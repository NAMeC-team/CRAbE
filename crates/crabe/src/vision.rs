use crate::data_receiver::{ReceiverDataSet, ReceiverTask};
use clap::Args;
use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use log::error;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

#[derive(Args, Clone)]
pub struct VisionConfig {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    vision_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    vision_port: u32,
}

pub struct Vision {
    rx_vision: Receiver<SslWrapperPacket>,
}

impl Vision {
    pub fn with_config_boxed(cli: VisionConfig) -> Box<Self> {
        let (tx_vision, rx_vision) = mpsc::channel::<SslWrapperPacket>();
        let mut vision =
            MulticastUDPReceiver::new(cli.vision_ip.clone().as_str(), cli.vision_port.clone())
                .expect("Failed to create vision receiver");

        thread::spawn(move || loop {
            if let Some(packet) = vision.receive() {
                if let Err(e) = tx_vision.send(packet) {
                    error!("Error sending Vision packet: {:?}", e);
                }
            }
        });

        Box::new(Self { rx_vision })
    }
}

impl ReceiverTask for Vision {
    fn fetch(&mut self, input: &mut ReceiverDataSet) {
        input.vision_packet.extend(self.rx_vision.try_iter());
    }
}
