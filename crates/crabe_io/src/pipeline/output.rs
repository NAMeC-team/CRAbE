use std::io::Write;
use std::process::Output;
use std::time::Duration;
use log::{debug, error};
use prost::Message;
use serialport::SerialPort;
use uom::si::angular_velocity::radian_per_second;
use uom::si::velocity::meter_per_second;
use crabe_framework::component::{Component, ComponentBoxed, OutputComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::constant::MAX_ID_ROBOTS;
use crabe_framework::data::output::{Command, CommandMap, FeedbackMap, Kick};
use crabe_framework::data::tool::ToolCommands;
use crabe_protocol::protobuf::robot_packet::IaToMainBoard;
use crate::league::real::{Real, RealConfig};
use clap::Args;
use crate::league::simulator::config::SimulatorConfig;
use crate::league::simulator::task::Simulator;

#[derive(Args)]
pub struct OutputConfig {
    #[command(flatten)]
    #[command(next_help_heading = "Real")]
    pub real_cfg: RealConfig,

    #[command(flatten)]
    #[command(next_help_heading = "Simulation")]
    pub simulator_cfg: SimulatorConfig,
}

pub trait CommandSenderTask {
    fn step(&mut self, commands: CommandMap) -> FeedbackMap;
    fn close(&mut self);
}

pub struct OutputPipeline {
    command_task: Box<dyn CommandSenderTask>
}

impl OutputPipeline {
    pub fn with_config(output_cfg: OutputConfig, common_cfg: &CommonConfig) -> OutputPipeline {
        let command_task: Box<dyn CommandSenderTask>;
        if common_cfg.real {
            command_task = Box::new(Real::with_config(output_cfg.real_cfg));
        } else {
            command_task = Box::new(Simulator::with_config(output_cfg.simulator_cfg));
        }

        OutputPipeline {
            command_task
        }
    }
}

impl Component for OutputPipeline {
    fn close(mut self) {
        self.command_task.close()
    }
}

impl OutputComponent for OutputPipeline {
    fn step(&mut self, commands: CommandMap, tool_commands: ToolCommands) -> FeedbackMap {
        self.command_task.step(commands)
    }
}