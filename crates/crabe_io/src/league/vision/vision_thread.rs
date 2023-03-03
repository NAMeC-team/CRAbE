use std::net::Ipv4Addr;
use std::str::FromStr;
use crate::communication::MulticastUDPReceiver;
use crate::league::vision::VisionConfig;
use crate::module::ReceiverTask;
use crabe_framework::data::receiver::InboundData;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

// TODO: Document
pub struct Vision {
    rx_vision: Receiver<SslWrapperPacket>,
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl Vision {
    pub fn with_config(cli: VisionConfig) -> Self {
        let (tx_vision, rx_vision) = mpsc::channel::<SslWrapperPacket>();
        let ipv4 = Ipv4Addr::from_str(cli.vision_ip.as_str()).expect("Failed to create an ipv4 address with the ip");
        let mut vision = MulticastUDPReceiver::new(ipv4, cli.vision_port)
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

        Self {
            rx_vision,
            handle: Some(handle),
            running,
        }
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
