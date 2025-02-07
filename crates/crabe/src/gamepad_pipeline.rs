use gilrs::{Axis, Button, Event, GamepadId, Gilrs};
use crabe_decision::pipeline::DecisionConfig;
use crabe_framework::data::tool::ToolData;
use crabe_framework::data::output::{Command, CommandMap, Kick};
use crabe_framework::data::world::World;
use crabe_framework::component::{Component, DecisionComponent};
use crabe_framework::config::CommonConfig;
use crabe_io::gamepad::GamepadRobotIdConfig;

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

impl DecisionComponent for GamepadPipeline {
    fn step(&mut self, world: &World) -> (CommandMap, ToolData) {
        // Examine new events
        while let Some(Event { id, event, time }) = self.gilrs.next_event() {
            // println!("{:?} New event from {}: {:?}", time, id, event);
            self.active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = self.active_gamepad.map(|id| self.gilrs.gamepad(id)) {
            let mut command = Command::default();

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

            if gamepad.is_pressed(Button::North) {
                self.is_charging = !self.is_charging;
            }

            command.charge = self.is_charging;

            // TODO : Handle power !
            if gamepad.is_pressed(Button::RightTrigger2) {
                command.kick = Some(Kick::StraightKick { power: 1.0 });
            } else if gamepad.is_pressed(Button::LeftTrigger2) {
                command.kick = Some(Kick::ChipKick { power: 1.0 });
            }

            let mut command_map = CommandMap::new();
            command_map.insert(self.controlled_id, command);


            (command_map, ToolData::default())
        } else {
            (CommandMap::default(), ToolData::default())
        }
    }

}

impl Component for GamepadPipeline {
    fn close(self) {
    }
}
