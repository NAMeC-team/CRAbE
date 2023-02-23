mod data_receiver;
mod gc;
mod vision;

use clap::Parser;
use std::time::Duration;

use crate::data_receiver::{DataReceiverConfig, DataReceiverPipeline, ReceiverTask};
use crate::vision::{Vision, VisionConfig};
use crabe_common::cli::CommonConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    #[command(next_help_heading = "Common")]
    pub common: CommonConfig,

    #[command(flatten)]
    pub data_receiver_config: DataReceiverConfig,
}

pub struct System {
    receiver_pipeline: DataReceiverPipeline,
}

impl System {
    pub fn new(receiver_pipeline: DataReceiverPipeline) -> Self {
        Self { receiver_pipeline }
    }

    pub fn run(&mut self, refresh_rate: Duration) {
        loop {
            let receive_data = self.receiver_pipeline.run();
            dbg!(receive_data);
            // Filters
            // Decision
            // Tools
            // Guard
            // Output
        }
    }
}

fn main() {
    // 1. Logger + CLI
    let cli = Cli::parse();

    let default_pipeline = DataReceiverPipeline::with_config(cli.data_receiver_config);

    // 2. Init
    // Vision_GC_threaded::new()
    let mut system = System::new(default_pipeline);

    system.run(Duration::from_millis(16));

    // 3. loop
    // input_pipeline.run(&input_data, &feedback);
    // filter_pipeline.run(&filter_data, &data_store);
    // let mut commands = decision_pipeline.run(&data_store);
    // send_pipeline.run(&data_store, &commands, &feedback);
}
