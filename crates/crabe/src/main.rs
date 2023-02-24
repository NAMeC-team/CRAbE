mod data_receiver;
mod gc;
mod vision;

use clap::Parser;
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::data_receiver::{DataReceiverConfig, DataReceiverPipeline};
use crabe_framework::config::CommonConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    #[command(next_help_heading = "Common")]
    pub common: CommonConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Data Receiver")]
    pub data_receiver_config: DataReceiverConfig,
}

pub struct System {
    receiver_pipeline: DataReceiverPipeline,
    running: Arc<AtomicBool>,
}

impl System {
    pub fn new(receiver_pipeline: DataReceiverPipeline) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_ctrlc = Arc::clone(&running);

        ctrlc::set_handler(move || {
            running_ctrlc.store(false, Ordering::Relaxed);
        })
        .expect("Failed to set Ctrl-C handler");

        Self {
            receiver_pipeline,
            running,
        }
    }

    pub fn run(&mut self, _refresh_rate: Duration) {
        while self.running.load(Ordering::SeqCst) {
            let receive_data = self.receiver_pipeline.run();
            dbg!(receive_data);
        }
    }

    pub fn close(&mut self) {
        self.receiver_pipeline.close();
    }
}

fn main() {
    let cli = Cli::parse();
    let env = Env::default()
        .filter_or("CRABE_LOG_LEVEL", "debug")
        .write_style_or("CRABE_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let data_receiver = DataReceiverPipeline::with_config(cli.data_receiver_config);

    let mut system = System::new(data_receiver);
    system.run(Duration::from_millis(16));
    system.close();
}
