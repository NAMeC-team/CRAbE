use clap::Args;

#[derive(Args)]
pub struct GamepadRobotIdConfig {
    #[arg(long, visible_alias = "id", default_value_t = 0)]
    pub robot_id: u8
}