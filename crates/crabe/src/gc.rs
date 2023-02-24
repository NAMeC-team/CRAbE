use crate::data_receiver::{ReceiverDataSet, ReceiverTask};
use clap::Args;
use crabe_io::communication::MulticastUDPReceiver;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;

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
    handle: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl GameController {
    pub fn with_config_boxed(cli: GameControllerConfig) -> Box<Self> {
        let (tx_gc, rx_gc) = mpsc::channel::<Referee>();
        let mut gc = MulticastUDPReceiver::new(cli.gc_ip.clone().as_str(), cli.gc_port.clone())
            .expect("Failed to create vision receiver");
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let handle = thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                if let Some(packet) = gc.receive() {
                    if let Err(e) = tx_gc.send(packet) {
                        error!("Error sending GameController packet: {:?}", e);
                    }
                }
            }
        });

        Box::new(Self {
            rx_gc,
            handle: Some(handle),
            running,
        })
    }
}

impl ReceiverTask for GameController {
    fn fetch(&mut self, input: &mut ReceiverDataSet) {
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
