use clap::Args;

/// Represents the configuration settings for the SSL Game Controller.
#[derive(Args)]
pub struct GameControllerConfig {
    #[arg(long, default_value = "224.5.23.2")]
    pub gc_ip: String,

    #[arg(long, default_value_t = 10020)]
    pub gc_port: u32,
}
