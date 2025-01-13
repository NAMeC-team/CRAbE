use crate::league::game_controller::GameControllerConfig;
use crate::pipeline::input::ReceiverTask;
use crabe_framework::data::input::InboundData;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::thread::JoinHandle;
use crate::league::utils::threaded_receiver;

// TODO: Document
pub struct GameController {
    rx_gc: Receiver<Referee>,
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl GameController {
    pub fn with_config(cli: GameControllerConfig) -> Self {
        let (rx_gc, handle, running) = 
            threaded_receiver::<Referee>(cli.gc_ip.as_str(), cli.gc_port);

        Self {
            rx_gc,
            handle: Some(handle),
            running,
        }
    }
}

impl ReceiverTask for GameController {
    fn fetch(&mut self, input: &mut InboundData) {
        input.gc_packet.extend(self.rx_gc.try_iter());
    }

    fn close(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            match handle.join() {
                Ok(_) => info!("GC Thread finished successfully"),
                Err(e) => println!("GC thread finished with an error: {:?}", e),
            }
        }
    }
}
