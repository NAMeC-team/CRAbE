use crate::{
    action::{ self, move_to::MoveTo, order_raw::RawOrder, ActionWrapper}, message::MessageData, strategy::Strategy
};

use crabe_framework::data::{
    output::{Kick::StraightKick},
    tool::ToolData,
    world::World,
};
use crabe_framework::data::output::Command;
use std::time::Instant;



/// The Prembule struct represents a strategy that commands a robot to middle of the ground
pub struct Prembule {
    /// The id of the robot to move.
    id: u8,
    messages: Vec<MessageData>,
    state: States,
    start_time: Instant,
    
}


#[derive(PartialEq, Eq)]
pub enum States {
    FIRST,
    SECOND,
    THIRD,
    STOP,
}

impl Prembule {
    /// Creates a new Prembule instance with the desired robot id.
    pub fn new(id: u8) -> Self {
        Self {
            id,
            messages: vec![],
            state: States::FIRST,
            start_time: Instant::now(),
            
        }
    }
    
}

impl Strategy for Prembule {
    fn name(&self) -> &'static str {
        "Prembule"
    }

    fn get_messages(&self) -> &Vec<MessageData>  {
        &self.messages
    }
    fn get_ids(&self) -> Vec<u8> {
        vec![self.id]
    }
    fn put_ids(&mut self, ids: Vec<u8>) {
        if ids.len() == 1{
            self.id = ids[0];
        }
    }

    #[allow(unused_variables)]
    fn step(
        &mut self,
        world: &World,
        tools_data: &mut ToolData,
        action_wrapper: &mut ActionWrapper,
    ) -> bool {
        
        let robot = &match world.allies_bot.get(&self.id) {
            Some(r) => r,
            None => {
                eprintln!("Cannot get robot");
                return false;
            }
        }.pose;
        let orient = robot.orientation;
        let speed:f32 = 3.14;    
        match self.state {
            States::FIRST => {
                if self.start_time.elapsed().as_secs() > 1{
                    self.start_time = Instant::now();
                    self.state = States::SECOND;
                } else {
                    action_wrapper.push(
                        self.id,
                        RawOrder::new(Command {
                            dribbler: 1.0,
                            angular_velocity: speed,
                            ..Default::default()
                        }),
                    );
                }
            }
            States::SECOND => {
                
                if self.start_time.elapsed().as_secs() > 1{
                    self.start_time = Instant::now();
                    self.state = States::THIRD;
                } else {
                    action_wrapper.push(
                        self.id,
                        RawOrder::new(Command {
                            dribbler: 1.0,
                            angular_velocity: -speed,
                            ..Default::default()
                        }),
                    );
                };
            }
            States::THIRD => {
                
                action_wrapper.push(
                    self.id,
                    MoveTo::new(robot.position, orient+2. , 0.0 , true , None,false )
                );
                action_wrapper.push(
                    self.id,
                    MoveTo::new(robot.position, orient , 0.0 , false , Some(StraightKick { power: 1.0 }),false )
                );
                self.state = States::STOP;
                
                
            }
            States::STOP => {

                if let Some(action) = action_wrapper.actions.get(&self.id) {
                    return action.actions.len() == 0;
                }

            }

        
        }
    return false;
    }
}