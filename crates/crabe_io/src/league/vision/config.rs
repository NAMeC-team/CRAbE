use clap::Args;

/// Represents the configuration settings for SSL-Vision or the Simulator vision
/// module.
#[derive(Args)]
pub struct VisionConfig {
    #[arg(long, default_value = "255.255.255.255")]
    pub vision_ip: String,
    #[arg(long)]
    pub vision_port: Option<u16>,
}
