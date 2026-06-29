use std::io::pipe;

use crabe_io::tool;
use gilrs::EventType::ButtonChanged;
use gilrs::ev::Code;
use gilrs::{Axis, Button, Event, Gamepad, GamepadId, Gilrs};
use gilrs::EventType;
use crabe_decision::pipeline::DecisionConfig;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::output::{Command, CommandMap, Kick};
use crabe_framework::data::world::World;
use crabe_framework::component::{Component, DecisionComponent};
use crabe_framework::config::CommonConfig;
use crabe_io::gamepad::{self, GamepadRobotIdConfig};
use log::info;

pub struct GamepadPipeline {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
    is_charging: bool,
    controlled_id: u8,
    dribbler: f32,
}

impl GamepadPipeline {
    pub fn with_config(_decision_cfg: DecisionConfig, _common_cfg: &CommonConfig, gamepad_config: &GamepadRobotIdConfig) -> Self {
        let gilrs = Gilrs::new().unwrap();

        // Iterate over all connected gamepads
        for (_id, gamepad) in gilrs.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }

        Self {
            gilrs,
            active_gamepad: None,
            is_charging: false,
            controlled_id: gamepad_config.robot_id,
            dribbler: 400.
        }
    }
}


fn handle_pressed(gamepad: Gamepad, button: Button, code: Code, command: &mut Command) {
    match button {
        Button::North => command.charge = !command.charge,
        Button::LeftTrigger2 => command.kick = Some(Kick::ChipKick { power: 1.0 }),
        Button::RightTrigger2 => command.kick = Some(Kick::StraightKick { power: 1.0 }),

        _ => (),
    }
}

impl DecisionComponent for GamepadPipeline {
    fn step(&mut self, world: &World) -> (CommandMap, ToolData) {
        // Examine new events
        let mut gamepad_id = self.active_gamepad;
        let tool_data = ToolData::default();
        let mut command = Command::default();

        // why store it then restore it like this ?
        // well, i dont like too many arguments in my functions, this would clobber handle_pressed
        command.charge = self.is_charging;

        // events handle
        while let Some(Event { id, event, time }) = self.gilrs.next_event() {
            if gamepad_id.is_none() {
                gamepad_id = Some(id);
            }

            let self_id = match gamepad_id {
                Some(id) => id,
                None => { self.active_gamepad = Some(id); id },
            };

            if self_id != id {
                continue;
            }

            let gamepad = self.gilrs.gamepad(self_id);


            match event {
                EventType::ButtonPressed(button, code) => handle_pressed(gamepad, button, code, &mut command),
                EventType::Connected =>  if self.active_gamepad == None { self.active_gamepad = Some(id) },
                EventType::Disconnected => self.active_gamepad = None,

                _ => info!("Unandled gamepad event !")
            };
        }

        self.active_gamepad = gamepad_id;

        // state handle
        info!("gamepad {:?}", self.active_gamepad);
        if let Some(gamepad) = self.active_gamepad.map(|id| self.gilrs.gamepad(id)) {
            // Move Local Velocity
            if gamepad.value(Axis::LeftStickY).abs() > 0.2 {
                command.forward_velocity = gamepad.value(Axis::LeftStickY);
            }

            if gamepad.value(Axis::LeftStickX).abs() > 0.2 {
                command.left_velocity = -gamepad.value(Axis::LeftStickX) * 2.0;
            }

            if gamepad.value(Axis::RightStickX).abs() > 0.1 {
                command.angular_velocity = gamepad.value(Axis::RightStickX) * -3.14;
            }

            if gamepad.is_pressed(Button::DPadUp) {
                let nv = self.dribbler + 1.;
                if nv < 400. { self.dribbler = nv; }
            } else if gamepad.is_pressed(Button::DPadDown) {
                let nv = self.dribbler - 1.;
                if nv > -400. { self.dribbler = nv; }
            }

            if gamepad.is_pressed(Button::LeftTrigger) {
                command.dribbler = self.dribbler;
            }
        }

        self.is_charging = command.charge;
        let mut command_map = CommandMap::new();
        command_map.insert(self.controlled_id, command);
        (command_map, tool_data)
    }
}

impl Component for GamepadPipeline {
    fn close(self) {
    }
}
