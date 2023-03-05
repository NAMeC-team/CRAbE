use clap::Args;
#[derive(Args)]
pub struct SimulatorConfig {
    pub simulator_port: Option<u16>,
}
