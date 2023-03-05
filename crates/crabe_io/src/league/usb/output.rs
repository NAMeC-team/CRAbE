use std::io::Write;
use std::time::Duration;
use log::{debug, error};
use prost::Message;
use serialport::SerialPort;
use uom::si::angular_velocity::radian_per_second;
use uom::si::velocity::meter_per_second;
use crabe_framework::component::{Component, OutputComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::constant::MAX_ID_ROBOTS;
use crabe_framework::data::output::{Command, CommandMap, FeedbackMap, Kick};
use crabe_framework::data::tool::ToolCommands;
use crabe_protocol::protobuf::robot_packet::IaToMainBoard;
use crate::league::usb::UsbConfig;

pub struct UsbOutput {
    port: Box<dyn SerialPort>,
}

impl UsbOutput {
    pub fn with_config(usb_config: UsbConfig, _common_config: &CommonConfig) -> Self {
        Self {
            port: serialport::new(usb_config.usb_port, usb_config.usb_baud)
                .timeout(Duration::from_millis(1))
                .open()
                .expect("Failed to open port"),
        }
    }

    fn prepare_packet(&mut self, id: u32, command: Command) -> IaToMainBoard {
        let (kicker_cmd, kick_power) = match command.kick {
            None => {
                (0, 0.0 as f32) // TODO : Remove this 0 and use the kicker enum
            }
            Some(Kick::StraightKick { power }) => (1, power),
            Some(Kick::ChipKick { power }) => (2, power),
        };

        IaToMainBoard {
            robot_id: id,
            normal_speed: command.forward_velocity.get::<meter_per_second>(),
            tangential_speed: command.left_velocity.get::<meter_per_second>(),
            angular_speed: command.angular_velocity.get::<radian_per_second>(),
            motor_break: false,
            kicker_cmd,
            kick_power,
            charge: command.charge,
            dribbler: command.dribbler.is_sign_positive(),
        }
    }

    fn send(&mut self, packet: IaToMainBoard) {
        // TODO : Buffer on struct?
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len() + 1);
        buf.push(packet.encoded_len() as u8);
        packet.encode(&mut buf).unwrap();

        match self.port.write(&buf[0..packet.encoded_len() + 1]) {
            Ok(_v) => {
                debug!("sent order: {:?}", packet);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

impl Component for UsbOutput {
    fn close(mut self) {
        let mut commands: CommandMap = Default::default();
        for id in 0..MAX_ID_ROBOTS {
            commands.insert(id as u32, Default::default());
        }

        self.step(commands, None);
    }
}

impl OutputComponent for UsbOutput {
    fn step(&mut self, commands: CommandMap, tool_commands: Option<ToolCommands>) -> FeedbackMap {
        for (id, command) in commands.into_iter() {
            let packet = self.prepare_packet(id, command);
            self.send(packet);


        }
        Default::default()
    }
}