use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use clap::Args;
use clap::Parser;

use crabe_io::communication::MulticastUDPReceiver;
use crabe_common::cli::CrabeCommonCLI;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::vision_packet::SslWrapperPacket;

#[derive(Args)]
pub struct GCCli {
    #[arg(long)]
    gc: bool,

    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    gc_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    gc_port: u32,
}

#[derive(Args, Clone)]
pub struct VisionCli {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    vision_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    vision_port: u32,
}
#[derive(Debug)]
pub struct InputDataSet {
    pub vision_packet: Vec<SslWrapperPacket>,
    pub gc_packet: Vec<Referee>,
}

pub struct DataReceiver {}

struct Vision {
    rx_vision: Receiver<SslWrapperPacket>
}

impl Vision {
    pub fn new(cli: VisionCli) -> Self {
        let (tx_vision, rx_vision) = mpsc::channel::<SslWrapperPacket>();
        let vision = MulticastUDPReceiver::new(cli.vision_ip.clone().as_str(), cli.vision_port.clone()).expect("Failed to create vision receiver");

        std::thread::spawn(move || {
            loop {
                if let Some(packet) = vision.receive() {
                    tx_vision.send(packet).expect("TODO: panic message");
                }
            }
        });


        Self {
            rx_vision
        }
    }

    pub fn process(&mut self, input: InputDataSet) {

    }
}

impl DataReceiver {
    pub fn with_cli(cli: VisionCli, _gc_cli: GCCli, _common: CrabeCommonCLI) -> Self {
        let handles = vec![];
        let tasks = vec![];

    }

    pub fn process(&mut self) -> InputDataSet {
        let mut input = InputDataSet {
            vision_packet: vec![], gc_packet: vec![]
        };
        input.vision_packet.extend(self.rx_vision.try_iter());
        dbg!(input)
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    #[command(next_help_heading = "common")]
    pub common: CrabeCommonCLI,

    #[command(flatten)]
    #[command(next_help_heading = "vision")]
    pub vision: VisionCli,

    #[command(flatten)]
    #[command(next_help_heading = "gc")]
    pub gc: GCCli,
}

fn main() {
    // 1. Logger + CLI
    let cli = Cli::parse();

    let mut data_receiver = DataReceiver::with_cli(cli.vision, cli.gc, cli.common);

    loop {
        let mut data = data_receiver.process();
    }
    // 2. Init
    // Vision_GC_threaded::new()

    // 3. loop
    // input_pipeline.run(&input_data, &feedback);
    // filter_pipeline.run(&filter_data, &data_store);
    // let mut commands = decision_pipeline.run(&data_store);
    // send_pipeline.run(&data_store, &commands, &feedback);
}
