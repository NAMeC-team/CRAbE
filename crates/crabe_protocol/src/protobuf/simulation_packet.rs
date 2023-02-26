/// RobotId is the combination of a team and a robot id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotId {
    /// the robot number
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    /// the team that the robot belongs to
    #[prost(enumeration = "Team", optional, tag = "2")]
    pub team: ::core::option::Option<i32>,
}
/// Team is either blue or yellow
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Team {
    /// team not set
    Unknown = 0,
    /// yellow team
    Yellow = 1,
    /// blue team
    Blue = 2,
}
impl Team {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Team::Unknown => "UNKNOWN",
            Team::Yellow => "YELLOW",
            Team::Blue => "BLUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "YELLOW" => Some(Self::Yellow),
            "BLUE" => Some(Self::Blue),
            _ => None,
        }
    }
}
/// Division denotes the current division, which influences some rules
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Division {
    DivUnknown = 0,
    DivA = 1,
    DivB = 2,
}
impl Division {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Division::DivUnknown => "DIV_UNKNOWN",
            Division::DivA => "DIV_A",
            Division::DivB => "DIV_B",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIV_UNKNOWN" => Some(Self::DivUnknown),
            "DIV_A" => Some(Self::DivA),
            "DIV_B" => Some(Self::DivB),
            _ => None,
        }
    }
}
/// A 2D float vector.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector2f {
    #[prost(float, required, tag = "1")]
    pub x: f32,
    #[prost(float, required, tag = "2")]
    pub y: f32,
}
/// Represents a field marking as a line segment represented by a start point p1,
/// and end point p2, and a line thickness. The start and end points are along
/// the center of the line, so the thickness of the line extends by thickness / 2
/// on either side of the line.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslFieldLineSegment {
    /// Name of this field marking.
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Start point of the line segment.
    #[prost(message, required, tag = "2")]
    pub p1: Vector2f,
    /// End point of the line segment.
    #[prost(message, required, tag = "3")]
    pub p2: Vector2f,
    /// Thickness of the line segment.
    #[prost(float, required, tag = "4")]
    pub thickness: f32,
    /// The type of this shape
    #[prost(enumeration = "SslFieldShapeType", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
}
/// Represents a field marking as a circular arc segment represented by center point, a
/// start angle, an end angle, and an arc thickness.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslFieldCircularArc {
    /// Name of this field marking.
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Center point of the circular arc.
    #[prost(message, required, tag = "2")]
    pub center: Vector2f,
    /// Radius of the arc.
    #[prost(float, required, tag = "3")]
    pub radius: f32,
    /// Start angle in counter-clockwise order.
    #[prost(float, required, tag = "4")]
    pub a1: f32,
    /// End angle in counter-clockwise order.
    #[prost(float, required, tag = "5")]
    pub a2: f32,
    /// Thickness of the arc.
    #[prost(float, required, tag = "6")]
    pub thickness: f32,
    /// The type of this shape
    #[prost(enumeration = "SslFieldShapeType", optional, tag = "7")]
    pub r#type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryFieldSize {
    #[prost(int32, required, tag = "1")]
    pub field_length: i32,
    #[prost(int32, required, tag = "2")]
    pub field_width: i32,
    #[prost(int32, required, tag = "3")]
    pub goal_width: i32,
    #[prost(int32, required, tag = "4")]
    pub goal_depth: i32,
    #[prost(int32, required, tag = "5")]
    pub boundary_width: i32,
    #[prost(message, repeated, tag = "6")]
    pub field_lines: ::prost::alloc::vec::Vec<SslFieldLineSegment>,
    #[prost(message, repeated, tag = "7")]
    pub field_arcs: ::prost::alloc::vec::Vec<SslFieldCircularArc>,
    #[prost(int32, optional, tag = "8")]
    pub penalty_area_depth: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "9")]
    pub penalty_area_width: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryCameraCalibration {
    #[prost(uint32, required, tag = "1")]
    pub camera_id: u32,
    #[prost(float, required, tag = "2")]
    pub focal_length: f32,
    #[prost(float, required, tag = "3")]
    pub principal_point_x: f32,
    #[prost(float, required, tag = "4")]
    pub principal_point_y: f32,
    #[prost(float, required, tag = "5")]
    pub distortion: f32,
    #[prost(float, required, tag = "6")]
    pub q0: f32,
    #[prost(float, required, tag = "7")]
    pub q1: f32,
    #[prost(float, required, tag = "8")]
    pub q2: f32,
    #[prost(float, required, tag = "9")]
    pub q3: f32,
    #[prost(float, required, tag = "10")]
    pub tx: f32,
    #[prost(float, required, tag = "11")]
    pub ty: f32,
    #[prost(float, required, tag = "12")]
    pub tz: f32,
    #[prost(float, optional, tag = "13")]
    pub derived_camera_world_tx: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "14")]
    pub derived_camera_world_ty: ::core::option::Option<f32>,
    #[prost(float, optional, tag = "15")]
    pub derived_camera_world_tz: ::core::option::Option<f32>,
    #[prost(uint32, optional, tag = "16")]
    pub pixel_image_width: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "17")]
    pub pixel_image_height: ::core::option::Option<u32>,
}
/// Two-Phase model for straight-kicked balls.
/// There are two phases with different accelerations during the ball kicks:
/// 1. Sliding
/// 2. Rolling
/// The full model is described in the TDP of ER-Force from 2016, which can be found here:
/// <https://ssl.robocup.org/wp-content/uploads/2019/01/2016_ETDP_ER-Force.pdf>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslBallModelStraightTwoPhase {
    /// Ball sliding acceleration \[m/s^2\] (should be negative)
    #[prost(double, required, tag = "1")]
    pub acc_slide: f64,
    /// Ball rolling acceleration \[m/s^2\] (should be negative)
    #[prost(double, required, tag = "2")]
    pub acc_roll: f64,
    /// Fraction of the initial velocity where the ball starts to roll
    #[prost(double, required, tag = "3")]
    pub k_switch: f64,
}
/// Fixed-Loss model for chipped balls.
/// Uses fixed damping factors for xy and z direction per hop.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslBallModelChipFixedLoss {
    /// Chip kick velocity damping factor in XY direction for the first hop
    #[prost(double, required, tag = "1")]
    pub damping_xy_first_hop: f64,
    /// Chip kick velocity damping factor in XY direction for all following hops
    #[prost(double, required, tag = "2")]
    pub damping_xy_other_hops: f64,
    /// Chip kick velocity damping factor in Z direction for all hops
    #[prost(double, required, tag = "3")]
    pub damping_z: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryModels {
    #[prost(message, optional, tag = "1")]
    pub straight_two_phase: ::core::option::Option<SslBallModelStraightTwoPhase>,
    #[prost(message, optional, tag = "2")]
    pub chip_fixed_loss: ::core::option::Option<SslBallModelChipFixedLoss>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslGeometryData {
    #[prost(message, required, tag = "1")]
    pub field: SslGeometryFieldSize,
    #[prost(message, repeated, tag = "2")]
    pub calib: ::prost::alloc::vec::Vec<SslGeometryCameraCalibration>,
    #[prost(message, optional, tag = "3")]
    pub models: ::core::option::Option<SslGeometryModels>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SslFieldShapeType {
    Undefined = 0,
    CenterCircle = 1,
    TopTouchLine = 2,
    BottomTouchLine = 3,
    LeftGoalLine = 4,
    RightGoalLine = 5,
    HalfwayLine = 6,
    CenterLine = 7,
    LeftPenaltyStretch = 8,
    RightPenaltyStretch = 9,
    LeftFieldLeftPenaltyStretch = 10,
    LeftFieldRightPenaltyStretch = 11,
    RightFieldLeftPenaltyStretch = 12,
    RightFieldRightPenaltyStretch = 13,
}
impl SslFieldShapeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SslFieldShapeType::Undefined => "Undefined",
            SslFieldShapeType::CenterCircle => "CenterCircle",
            SslFieldShapeType::TopTouchLine => "TopTouchLine",
            SslFieldShapeType::BottomTouchLine => "BottomTouchLine",
            SslFieldShapeType::LeftGoalLine => "LeftGoalLine",
            SslFieldShapeType::RightGoalLine => "RightGoalLine",
            SslFieldShapeType::HalfwayLine => "HalfwayLine",
            SslFieldShapeType::CenterLine => "CenterLine",
            SslFieldShapeType::LeftPenaltyStretch => "LeftPenaltyStretch",
            SslFieldShapeType::RightPenaltyStretch => "RightPenaltyStretch",
            SslFieldShapeType::LeftFieldLeftPenaltyStretch => {
                "LeftFieldLeftPenaltyStretch"
            }
            SslFieldShapeType::LeftFieldRightPenaltyStretch => {
                "LeftFieldRightPenaltyStretch"
            }
            SslFieldShapeType::RightFieldLeftPenaltyStretch => {
                "RightFieldLeftPenaltyStretch"
            }
            SslFieldShapeType::RightFieldRightPenaltyStretch => {
                "RightFieldRightPenaltyStretch"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Undefined" => Some(Self::Undefined),
            "CenterCircle" => Some(Self::CenterCircle),
            "TopTouchLine" => Some(Self::TopTouchLine),
            "BottomTouchLine" => Some(Self::BottomTouchLine),
            "LeftGoalLine" => Some(Self::LeftGoalLine),
            "RightGoalLine" => Some(Self::RightGoalLine),
            "HalfwayLine" => Some(Self::HalfwayLine),
            "CenterLine" => Some(Self::CenterLine),
            "LeftPenaltyStretch" => Some(Self::LeftPenaltyStretch),
            "RightPenaltyStretch" => Some(Self::RightPenaltyStretch),
            "LeftFieldLeftPenaltyStretch" => Some(Self::LeftFieldLeftPenaltyStretch),
            "LeftFieldRightPenaltyStretch" => Some(Self::LeftFieldRightPenaltyStretch),
            "RightFieldLeftPenaltyStretch" => Some(Self::RightFieldLeftPenaltyStretch),
            "RightFieldRightPenaltyStretch" => Some(Self::RightFieldRightPenaltyStretch),
            _ => None,
        }
    }
}
/// Movement limits for a robot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotLimits {
    /// Max absolute speed-up acceleration \[m/s^2\]
    #[prost(float, optional, tag = "1")]
    pub acc_speedup_absolute_max: ::core::option::Option<f32>,
    /// Max angular speed-up acceleration \[rad/s^2\]
    #[prost(float, optional, tag = "2")]
    pub acc_speedup_angular_max: ::core::option::Option<f32>,
    /// Max absolute brake acceleration \[m/s^2\]
    #[prost(float, optional, tag = "3")]
    pub acc_brake_absolute_max: ::core::option::Option<f32>,
    /// Max angular brake acceleration \[rad/s^2\]
    #[prost(float, optional, tag = "4")]
    pub acc_brake_angular_max: ::core::option::Option<f32>,
    /// Max absolute velocity \[m/s\]
    #[prost(float, optional, tag = "5")]
    pub vel_absolute_max: ::core::option::Option<f32>,
    /// Max angular velocity \[rad/s\]
    #[prost(float, optional, tag = "6")]
    pub vel_angular_max: ::core::option::Option<f32>,
}
/// Robot wheel angle configuration
/// all angles are relative to looking forward,
/// all wheels / angles are clockwise
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotWheelAngles {
    /// Angle front right \[rad\]
    #[prost(float, required, tag = "1")]
    pub front_right: f32,
    /// Angle back right \[rad\]
    #[prost(float, required, tag = "2")]
    pub back_right: f32,
    /// Angle back left \[rad\]
    #[prost(float, required, tag = "3")]
    pub back_left: f32,
    /// Angle front left \[rad\]
    #[prost(float, required, tag = "4")]
    pub front_left: f32,
}
/// Specs of a robot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotSpecs {
    /// Id of the robot
    #[prost(message, required, tag = "1")]
    pub id: RobotId,
    /// Robot radius \[m\]
    #[prost(float, optional, tag = "2", default = "0.09")]
    pub radius: ::core::option::Option<f32>,
    /// Robot height \[m\]
    #[prost(float, optional, tag = "3", default = "0.15")]
    pub height: ::core::option::Option<f32>,
    /// Robot mass \[kg\]
    #[prost(float, optional, tag = "4")]
    pub mass: ::core::option::Option<f32>,
    /// Max linear kick speed \[m/s\] (unset = unlimited)
    #[prost(float, optional, tag = "7")]
    pub max_linear_kick_speed: ::core::option::Option<f32>,
    /// Max chip kick speed \[m/s\] (unset = unlimited)
    #[prost(float, optional, tag = "8")]
    pub max_chip_kick_speed: ::core::option::Option<f32>,
    /// Distance from robot center to dribbler \[m\] (implicitly defines the opening angle and dribbler width)
    #[prost(float, optional, tag = "9")]
    pub center_to_dribbler: ::core::option::Option<f32>,
    /// Movement limits
    #[prost(message, optional, tag = "10")]
    pub limits: ::core::option::Option<RobotLimits>,
    /// Wheel angle configuration
    #[prost(message, optional, tag = "13")]
    pub wheel_angles: ::core::option::Option<RobotWheelAngles>,
    /// Custom robot spec for specific simulators (the protobuf files are managed by the simulators)
    #[prost(message, optional, tag = "14")]
    pub custom: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealismConfig {
    /// Custom config for specific simulators (the protobuf files are managed by the simulators)
    #[prost(message, optional, tag = "1")]
    pub custom: ::core::option::Option<::prost_types::Any>,
}
/// Change the simulator configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulatorConfig {
    /// Update the geometry
    #[prost(message, optional, tag = "1")]
    pub geometry: ::core::option::Option<SslGeometryData>,
    /// Update the robot specs
    #[prost(message, repeated, tag = "2")]
    pub robot_specs: ::prost::alloc::vec::Vec<RobotSpecs>,
    /// Update realism configuration
    #[prost(message, optional, tag = "3")]
    pub realism_config: ::core::option::Option<RealismConfig>,
    /// Change the vision publish port
    #[prost(uint32, optional, tag = "4")]
    pub vision_port: ::core::option::Option<u32>,
}
/// Errors in the simulator
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulatorError {
    /// Unique code of the error for automatic handling on client side
    #[prost(string, optional, tag = "1")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// Human readable description of the error
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// Teleport the ball to a new location and optionally set it to some velocity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeleportBall {
    /// x-coordinate \[m\]
    #[prost(float, optional, tag = "1")]
    pub x: ::core::option::Option<f32>,
    /// y-coordinate \[m\]
    #[prost(float, optional, tag = "2")]
    pub y: ::core::option::Option<f32>,
    /// z-coordinate (height) \[m\]
    #[prost(float, optional, tag = "3")]
    pub z: ::core::option::Option<f32>,
    /// Velocity in x-direction \[m/s\]
    #[prost(float, optional, tag = "4")]
    pub vx: ::core::option::Option<f32>,
    /// Velocity in y-direction \[m/s\]
    #[prost(float, optional, tag = "5")]
    pub vy: ::core::option::Option<f32>,
    /// Velocity in z-direction \[m/s\]
    #[prost(float, optional, tag = "6")]
    pub vz: ::core::option::Option<f32>,
    /// Teleport the ball safely to the target, for example by
    /// moving robots out of the way in case of collision and set speed of robots close-by to zero
    #[prost(bool, optional, tag = "7", default = "false")]
    pub teleport_safely: ::core::option::Option<bool>,
    /// Adapt the angular ball velocity such that the ball is rolling
    #[prost(bool, optional, tag = "8", default = "false")]
    pub roll: ::core::option::Option<bool>,
}
/// Teleport a robot to some location and give it a velocity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeleportRobot {
    /// Robot id to teleport
    #[prost(message, required, tag = "1")]
    pub id: RobotId,
    /// x-coordinate \[m\]
    #[prost(float, optional, tag = "2")]
    pub x: ::core::option::Option<f32>,
    /// y-coordinate \[m\]
    #[prost(float, optional, tag = "3")]
    pub y: ::core::option::Option<f32>,
    /// Orientation \[rad\], measured from the x-axis counter-clockwise
    #[prost(float, optional, tag = "4")]
    pub orientation: ::core::option::Option<f32>,
    /// Global velocity \[m/s\] towards x-axis
    #[prost(float, optional, tag = "5", default = "0")]
    pub v_x: ::core::option::Option<f32>,
    /// Global velocity \[m/s\] towards y-axis
    #[prost(float, optional, tag = "6", default = "0")]
    pub v_y: ::core::option::Option<f32>,
    /// Angular velocity \[rad/s\]
    #[prost(float, optional, tag = "7", default = "0")]
    pub v_angular: ::core::option::Option<f32>,
    /// Robot should be present on the field?
    /// true -> robot will be added, if it does not exist yet
    /// false -> robot will be removed, if it is present
    #[prost(bool, optional, tag = "8")]
    pub present: ::core::option::Option<bool>,
}
/// Control the simulation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulatorControl {
    /// Teleport the ball
    #[prost(message, optional, tag = "1")]
    pub teleport_ball: ::core::option::Option<TeleportBall>,
    /// Teleport robots
    #[prost(message, repeated, tag = "2")]
    pub teleport_robot: ::prost::alloc::vec::Vec<TeleportRobot>,
    /// Change the simulation speed
    #[prost(float, optional, tag = "3")]
    pub simulation_speed: ::core::option::Option<f32>,
}
/// Command from the connected client to the simulator
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulatorCommand {
    /// Control the simulation
    #[prost(message, optional, tag = "1")]
    pub control: ::core::option::Option<SimulatorControl>,
    /// Configure the simulation
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<SimulatorConfig>,
}
/// Response of the simulator to the connected client
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulatorResponse {
    /// List of errors, like using unsupported features
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<SimulatorError>,
}
/// Full command for a single robot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotCommand {
    /// Id of the robot
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    /// Movement command
    #[prost(message, optional, tag = "2")]
    pub move_command: ::core::option::Option<RobotMoveCommand>,
    /// Absolute (3 dimensional) kick speed \[m/s\]
    #[prost(float, optional, tag = "3")]
    pub kick_speed: ::core::option::Option<f32>,
    /// Kick angle \[degree\] (defaults to 0 degrees for a straight kick)
    #[prost(float, optional, tag = "4", default = "0")]
    pub kick_angle: ::core::option::Option<f32>,
    /// Dribbler speed in rounds per minute \[rpm\]
    #[prost(float, optional, tag = "5")]
    pub dribbler_speed: ::core::option::Option<f32>,
}
/// Wrapper for different kinds of movement commands
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotMoveCommand {
    #[prost(oneof = "robot_move_command::Command", tags = "1, 2, 3")]
    pub command: ::core::option::Option<robot_move_command::Command>,
}
/// Nested message and enum types in `RobotMoveCommand`.
pub mod robot_move_command {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        /// Move with wheel velocities
        #[prost(message, tag = "1")]
        WheelVelocity(super::MoveWheelVelocity),
        /// Move with local velocity
        #[prost(message, tag = "2")]
        LocalVelocity(super::MoveLocalVelocity),
        /// Move with global velocity
        #[prost(message, tag = "3")]
        GlobalVelocity(super::MoveGlobalVelocity),
    }
}
/// Move robot with wheel velocities
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveWheelVelocity {
    /// Velocity \[m/s\] of front right wheel
    #[prost(float, required, tag = "1")]
    pub front_right: f32,
    /// Velocity \[m/s\] of back right wheel
    #[prost(float, required, tag = "2")]
    pub back_right: f32,
    /// Velocity \[m/s\] of back left wheel
    #[prost(float, required, tag = "3")]
    pub back_left: f32,
    /// Velocity \[m/s\] of front left wheel
    #[prost(float, required, tag = "4")]
    pub front_left: f32,
}
/// Move robot with local velocity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveLocalVelocity {
    /// Velocity forward \[m/s\] (towards the dribbler)
    #[prost(float, required, tag = "1")]
    pub forward: f32,
    /// Velocity to the left \[m/s\]
    #[prost(float, required, tag = "2")]
    pub left: f32,
    /// Angular velocity counter-clockwise \[rad/s\]
    #[prost(float, required, tag = "3")]
    pub angular: f32,
}
/// Move robot with global velocity
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveGlobalVelocity {
    /// Velocity on x-axis of the field \[m/s\]
    #[prost(float, required, tag = "1")]
    pub x: f32,
    /// Velocity on y-axis of the field \[m/s\]
    #[prost(float, required, tag = "2")]
    pub y: f32,
    /// Angular velocity counter-clockwise \[rad/s\]
    #[prost(float, required, tag = "3")]
    pub angular: f32,
}
/// Command from the connected client to the simulator
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotControl {
    /// Control the robots
    #[prost(message, repeated, tag = "1")]
    pub robot_commands: ::prost::alloc::vec::Vec<RobotCommand>,
}
/// Feedback from a robot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotFeedback {
    /// Id of the robot
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    /// Has the dribbler contact to the ball right now
    #[prost(bool, optional, tag = "2")]
    pub dribbler_ball_contact: ::core::option::Option<bool>,
    /// Custom robot feedback for specific simulators (the protobuf files are managed by the simulators)
    #[prost(message, optional, tag = "3")]
    pub custom: ::core::option::Option<::prost_types::Any>,
}
/// Response to RobotControl from the simulator to the connected client
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotControlResponse {
    /// List of errors, like using unsupported features
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<SimulatorError>,
    /// Feedback of the robots
    #[prost(message, repeated, tag = "2")]
    pub feedback: ::prost::alloc::vec::Vec<RobotFeedback>,
}
