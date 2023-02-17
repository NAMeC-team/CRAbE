mod data_receiver;
mod gc;
mod vision;

use clap::Parser;
use std::time::Duration;

use crate::data_receiver::DataReceiverPipeline;
use crate::vision::VisionCli;
use crabe_common::cli::CrabeCommonCLI;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    #[command(next_help_heading = "common")]
    pub common: CrabeCommonCLI,

    #[command(flatten)]
    #[command(next_help_heading = "vision")]
    pub vision: VisionCli,
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
            println!("Hello World");
        }
    }
}

fn main() {
    // 1. Logger + CLI
    let cli = Cli::parse();

    // 2. Init
    // Vision_GC_threaded::new()

    // 3. loop
    // input_pipeline.run(&input_data, &feedback);
    // filter_pipeline.run(&filter_data, &data_store);
    // let mut commands = decision_pipeline.run(&data_store);
    // send_pipeline.run(&data_store, &commands, &feedback);
}
