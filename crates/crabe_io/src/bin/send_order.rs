use crabe_io::network::UDPTransceiver;
use crabe_protocol::protobuf::simulation_packet::robot_move_command::Command::LocalVelocity;
use crabe_protocol::protobuf::simulation_packet::{
    MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse, RobotMoveCommand,
};

fn create_commands() -> RobotControl {
    let robot_command = RobotCommand {
        id: 0,
        move_command: Some(RobotMoveCommand {
            command: Some(LocalVelocity(MoveLocalVelocity {
                forward: 0.0,
                left: 0.0,
                angular: 1.0,
            })),
        }),
        kick_speed: None,
        kick_angle: None,
        dribbler_speed: None,
    };

    RobotControl {
        robot_commands: vec![robot_command],
    }
}

fn main() {
    let mut udp_transceiver = UDPTransceiver::new("127.0.0.1", 10301);

    loop {
        udp_transceiver.send::<RobotControl>(create_commands());
        dbg!(udp_transceiver.receive::<RobotControlResponse>());
    }
}
