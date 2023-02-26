extern crate core;

use clap::Parser;
use crabe_framework::component::Receiver;
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::Feedback;
use crabe_io::module::{DataReceiverConfig, DataReceiverPipeline};
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

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
        let mut feedback = Feedback;
        while self.running.load(Ordering::SeqCst) {
            let receive_data = self.receiver_pipeline.step(&mut feedback);
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
    // FilterPipeline
    // DecisionPipeline
    // ToolsPipeline
    // GuardPipeline
    // OutputPipeline

    let mut system = System::new(data_receiver);
    system.run(Duration::from_millis(16));
    system.close();
}
