use crate::constant::{VISION_PORT_REAL, VISION_PORT_SIM};
use crate::league::vision::VisionConfig;
use crate::pipeline::input::ReceiverTask;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::input::InboundData;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use crate::league::utils::threaded_receiver;

/// Thread-based vision data receiver.
/// Data can be fetched from the rx_vision Receiver,
/// while the thread fetches data from the external vision system.
pub struct Vision {
    /// Used for retrieval of packets
    rx_vision: Receiver<SslWrapperPacket>,
    /// Handle on the vision thread launched
    handle: Option<JoinHandle<()>>,
    /// Atomic reference to halt work of the vision thread
    running: Arc<AtomicBool>,
}

impl Vision {
    pub fn with_config(vision_cfg: VisionConfig, common_cfg: &CommonConfig) -> Self {
        let port = if let Some(port) = vision_cfg.vision_port {
            port
        } else if common_cfg.real {
            VISION_PORT_REAL
        } else {
            VISION_PORT_SIM
        };

        let (rx_vision, handle, running) = 
            threaded_receiver::<SslWrapperPacket>(vision_cfg.vision_ip.as_str(), port);

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
