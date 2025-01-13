use clap::Args;

// note: why specify tracker_ip and not just ip ?
// because of the #[derive(Args)],
// the name of the struct will be used as a CLI argument name

#[derive(Args)]
pub struct TrackerConfig {
    #[arg(long, default_value = "224.5.23.2")]
    pub tracker_ip: String,
    
    #[arg(long, default_value = "10010")]
    pub tracker_port: u16
}