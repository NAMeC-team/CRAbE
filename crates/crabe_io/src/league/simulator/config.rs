use clap::Args;
#[derive(Args)]
pub struct SimulatorConfig {
    #[arg(long, default_value_t = 10301)]
    pub simulator_port: u16,
}
