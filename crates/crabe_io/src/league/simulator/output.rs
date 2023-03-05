use crate::communication::MulticastUDPReceiver;
use crate::league::game_controller::GameControllerConfig;
use crate::league::simulator::config::SimulatorConfig;
use crate::module::ReceiverTask;
use crabe_framework::component::{Component, OutputComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::{Command, CommandMap, Feedback, FeedbackMap, Kick};
use crabe_framework::data::receiver::InboundData;
use crabe_framework::data::tool::ToolCommands;
use crabe_protocol::protobuf::game_controller_packet::Referee;
use crabe_protocol::protobuf::simulation_packet::{
    robot_move_command, MoveLocalVelocity, RobotCommand, RobotControl, RobotControlResponse,
    RobotMoveCommand,
};
use log::{debug, error, info};
use prost::Message;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::thread::JoinHandle;
use uom::si::angular_velocity::{radian_per_second, revolution_per_minute};
use uom::si::velocity::meter_per_second;

const SIMULATOR_BUFFER_SIZE: usize = 4096;

pub struct SimulatorOutput {
    socket: UdpSocket,
    buf: [u8; SIMULATOR_BUFFER_SIZE],
}

impl SimulatorOutput {
    pub fn with_config(simulator_config: SimulatorConfig, common_config: &CommonConfig) -> Self {
        let socket = UdpSocket::bind(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0))
            .expect("Failed to bind the UDP Socket");
        socket
            .set_nonblocking(true)
            .expect("Failed to set socket to non-blocking mode");

        let port;
        if common_config.yellow {
            port = simulator_config.yellow_port;
        } else {
            port = simulator_config.blue_port
        }

        let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);

        socket.connect(addr).expect("connect function failed");

        Self {
            socket,
            buf: [0u8; SIMULATOR_BUFFER_SIZE],
        }
    }

    fn prepare_packet(&self, commands: impl Iterator<Item = (u32, Command)>) -> RobotControl {
        let mut packet = RobotControl::default();

        for (id, command) in commands {
            let (kick_speed, kick_angle) = match &command.kick {
                None => (0.0, 0.0),
                Some(Kick::StraightKick { power }) => (*power, 0.0),
                Some(Kick::ChipKick { power }) => (*power, 45.0),
            };

            let robot_command = RobotCommand {
                id,
                move_command: Some(RobotMoveCommand {
                    command: Some(robot_move_command::Command::LocalVelocity {
                        0: MoveLocalVelocity {
                            forward: command.forward_velocity.get::<meter_per_second>(),
                            left: command.left_velocity.get::<meter_per_second>(),
                            angular: command.angular_velocity.get::<radian_per_second>(),
                        },
                    }),
                }),
                kick_speed: Some(kick_speed),
                kick_angle: Some(kick_angle),
                dribbler_speed: Some(command.dribbler.get::<revolution_per_minute>()),
            };
            packet.robot_commands.push(robot_command);
        }

        packet
    }

    fn send(&mut self, packet: RobotControl) {
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());
        packet.encode(&mut buf).unwrap();
        self.socket.send(&buf[0..packet.encoded_len()]).map_or_else(
            |e| error!("couldn't send data"),
            |l| debug!("sent order: {:?}", packet),
        );
    }

    fn receive(&mut self) -> FeedbackMap {
        let mut feedback_map: FeedbackMap = Default::default();
        match self.socket.recv(&mut self.buf) {
            Ok(p_size) => match RobotControlResponse::decode(&self.buf[0..p_size]) {
                Ok(packet) => {
                    for robot_feedback in packet.feedback {
                        debug!(
                            "assigned feedback {:?} to robot #{}",
                            robot_feedback, robot_feedback.id
                        );

                        feedback_map.insert(
                            robot_feedback.id,
                            Feedback {
                                has_ball: robot_feedback.dribbler_ball_contact(),
                                voltage: Default::default(),
                            },
                        );
                    }
                }

                Err(e) => {
                    error!("error decoding packet: {:?}", e);
                }
            },
            Err(e) => {
                error!("couldn't recv from socket, err: {}", e);
            }
        };

        feedback_map
    }
}

impl Component for SimulatorOutput {
    fn close(self) {}
}

impl OutputComponent for SimulatorOutput {
    fn step(&mut self, commands: CommandMap, tool_commands: Option<ToolCommands>) -> FeedbackMap {
        let packet = self.prepare_packet(commands.into_iter());
        self.send(packet);
        return self.receive();
    }
}
