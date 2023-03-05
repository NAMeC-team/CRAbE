use clap::Args;

#[derive(Args)]
pub struct ToolConfig {
    #[arg(long, default_value_t = 10400)]
    pub tool_port: u16,
}
