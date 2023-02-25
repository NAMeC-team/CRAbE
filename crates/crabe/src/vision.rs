use crate::data_receiver::ReceiverTask;
use crabe_framework::data::receiver::InboundData;
use clap::Args;
use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

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
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl Vision {
    pub fn with_config_boxed(cli: VisionConfig) -> Box<Self> {
        let (tx_vision, rx_vision) = mpsc::channel::<SslWrapperPacket>();
        let mut vision =
            MulticastUDPReceiver::new(cli.vision_ip.clone().as_str(), cli.vision_port.clone())
                .expect("Failed to create vision receiver");

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                if let Some(packet) = vision.receive() {
                    if let Err(e) = tx_vision.send(packet) {
                        error!("Error sending Vision packet: {:?}", e);
                    }
                }
            }
        });

        Box::new(Self {
            rx_vision,
            handle: Some(handle),
            running,
        })
    }
}

impl ReceiverTask for Vision {
    fn fetch(&mut self, input: &mut InboundData) {
        input.vision_packet.extend(self.rx_vision.try_iter());
    }

    fn close(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            match handle.join() {
                Ok(_) => info!("Vision Thread finished successfully"),
                Err(e) => println!("Vision thread finished with an error: {:?}", e),
            }
        }
    }
}
