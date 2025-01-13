use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use log::info;
use crabe_framework::data::input::InboundData;
use crabe_protocol::protobuf::vision_packet::TrackerWrapperPacket;
use crate::league::tracker::TrackerConfig;
use crate::league::utils::threaded_receiver;
use crate::pipeline::input::ReceiverTask;

pub struct Tracker {
    rx_tracker: Receiver<TrackerWrapperPacket>,
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>
}

impl Tracker {
    pub(crate) fn with_config(tracker_cfg: TrackerConfig) -> Self {
        let (rx_tracker, handle, running) =
        threaded_receiver::<TrackerWrapperPacket>(tracker_cfg.tracker_ip.as_str(), tracker_cfg.tracker_port);
        Self {
            rx_tracker,
            handle: Some(handle),
            running,
        }
    }
}

impl ReceiverTask for Tracker {
    fn fetch(&mut self, input: &mut InboundData) {
        input.tracker_packet.extend(self.rx_tracker.try_iter());
    }

    fn close(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            match handle.join() {
                Ok(_) => info!("Tracker data thread finished successfully"),
                Err(e) => println!("Tracker data thread finished with an error: {:?}", e),
            }
        }
    }
}