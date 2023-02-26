#[derive(Args)]
pub struct GameControllerConfig {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    gc_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    gc_port: u32,
}
