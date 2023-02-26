use clap::Args;

// TODO: Document
#[derive(Args)]
pub struct VisionConfig {
    /// ip of the ssl vision server
    #[arg(long, default_value = "224.5.23.2")]
    pub(crate) vision_ip: String,

    /// port of the ssl vision server
    #[arg(long, default_value_t = 10020)]
    pub(crate) vision_port: u32,
}
