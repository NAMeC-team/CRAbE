use clap::Args;
#[derive(Args)]
pub struct SimulatorConfig {
    #[arg(long, default_value_t = 10301)]
    pub blue_port: u16,
    #[arg(long, default_value_t = 10302)]
    pub yellow_port: u16,
}
