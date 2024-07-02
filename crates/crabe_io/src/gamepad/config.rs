use clap::Args;

#[derive(Args)]
pub struct GamepadRobotIdConfig {
    #[arg(long, default_value_t = 0)]
    pub robot_id: u8
}