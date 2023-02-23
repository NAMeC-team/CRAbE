use crate::data_receiver::{ReceiverDataSet, ReceiverTask};
use clap::Args;
use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use log::error;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

#[derive(Args)]
pub struct GameControllerConfig {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    gc_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    gc_port: u32,
}

pub struct GameController {
    rx_gc: Receiver<Referee>,
}

impl GameController {
    pub fn with_config_boxed(cli: GameControllerConfig) -> Box<Self> {
        let (tx_gc, rx_gc) = mpsc::channel::<Referee>();
        let mut gc = MulticastUDPReceiver::new(cli.gc_ip.clone().as_str(), cli.gc_port.clone())
            .expect("Failed to create vision receiver");

        thread::spawn(move || loop {
            if let Some(packet) = gc.receive() {
                if let Err(e) = tx_gc.send(packet) {
                    error!("Error sending GameController packet: {:?}", e);
                }
            }
        });

        Box::new(Self { rx_gc })
    }
}

impl ReceiverTask for GameController {
    fn fetch(&mut self, input: &mut ReceiverDataSet) {
        input.gc_packet.extend(self.rx_gc.try_iter());
    }
}
