use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use crabe_framework::component::ToolComponent;
use crabe_framework::data::tool::{ToolCommands, ToolData};
use crabe_framework::data::world::World;
use crabe_io::communication::WebSocketTransceiver;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
enum CrabeToolMessage {
    World(World)
}

#[derive(Deserialize)]
enum CrabeToolRequest {

}

pub struct CrabeTool {
    websocket: WebSocketTransceiver<CrabeToolRequest, CrabeToolMessage>
}

impl CrabeTool {
    pub fn with_config() -> Self {
        Self {
            // TODO: Tool config
            websocket: WebSocketTransceiver::spawn(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 10400).into())
        }
    }
}

impl ToolComponent for CrabeTool {
    fn step(&mut self, world_data: &World, tool_data: &mut ToolData) -> ToolCommands {
        self.websocket.send(CrabeToolMessage::World(world_data.clone()));
        ToolCommands{}
    }

    fn close(&mut self) {
        todo!()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
