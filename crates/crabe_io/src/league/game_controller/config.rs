use clap::Args;

// TODO: Document
#[derive(Args)]
pub struct GameControllerConfig {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    pub gc_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    pub gc_port: u32,
}
