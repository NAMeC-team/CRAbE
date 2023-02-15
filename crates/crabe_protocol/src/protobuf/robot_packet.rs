#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainBoardToBrushless {
    /// Brushless command
    #[prost(enumeration = "Commands", tag = "1")]
    pub command: i32,
    /// m.s-1
    #[prost(float, tag = "2")]
    pub speed: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrushlessToMainBoard {
    /// Number of SPI transmission errors
    #[prost(uint32, tag = "1")]
    pub error_count: u32,
    /// m.s-1
    #[prost(float, tag = "2")]
    pub measured_speed: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IaToMainBoard {
    /// Robot identifier
    #[prost(uint32, tag = "1")]
    pub robot_id: u32,
    /// m.s-1
    #[prost(float, tag = "2")]
    pub normal_speed: f32,
    /// m.s-1
    #[prost(float, tag = "3")]
    pub tangential_speed: f32,
    /// rad.s-1
    #[prost(float, tag = "4")]
    pub angular_speed: f32,
    /// Break
    #[prost(bool, tag = "5")]
    pub motor_break: bool,
    /// dont kick, kick with Kicker 1 or Kicker 2
    #[prost(enumeration = "Kicker", tag = "6")]
    pub kicker_cmd: i32,
    /// kick power (uS)
    #[prost(float, tag = "7")]
    pub kick_power: f32,
    /// enable / disable charge kicker
    #[prost(bool, tag = "8")]
    pub charge: bool,
    /// Enable / disable dribbler
    #[prost(bool, tag = "9")]
    pub dribbler: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MainboardToIa {
    /// Robot identifier
    #[prost(uint32, tag = "1")]
    pub robot_id: u32,
    /// m.s-1
    #[prost(float, tag = "2")]
    pub measured_normal_speed: f32,
    /// m.s-1
    #[prost(float, tag = "3")]
    pub measured_tangential_speed: f32,
    /// rad.s-1
    #[prost(float, tag = "4")]
    pub measured_angular_speed: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Commands {
    Stop = 0,
    Run = 1,
    Break = 2,
}
impl Commands {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Commands::Stop => "STOP",
            Commands::Run => "RUN",
            Commands::Break => "BREAK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP" => Some(Self::Stop),
            "RUN" => Some(Self::Run),
            "BREAK" => Some(Self::Break),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Kicker {
    NoKick = 0,
    Kick1 = 1,
    Kick2 = 2,
}
impl Kicker {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Kicker::NoKick => "NO_KICK",
            Kicker::Kick1 => "KICK1",
            Kicker::Kick2 => "KICK2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_KICK" => Some(Self::NoKick),
            "KICK1" => Some(Self::Kick1),
            "KICK2" => Some(Self::Kick2),
            _ => None,
        }
    }
}
