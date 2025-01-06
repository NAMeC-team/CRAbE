use crate::communication::WebSocketTransceiver;
use crate::tool::config::ToolConfig;
use crabe_framework::component::{Component, ToolComponent};
use crabe_framework::config::CommonConfig;
use crabe_framework::data::output::CommandMap;
use crabe_framework::data::tool::{ToolCommands, ToolData};
use crabe_framework::data::world::World;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::net::{Ipv4Addr, SocketAddrV4};

const TOOL_MESSAGE_PROTO_VER: u8 = 1;

/// Actual message sent to external tools as JSON.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ToolMessage {
    // current message version.
    // should be updated whenever one of the struct members
    // change in content
    version: u8,
    world: World,
    data: ToolData,
}

/// Represents requests emitted by external tools.
///
/// The attribute `#[serde_as]` serves as another way to specify a different
/// serializer/deserialize function to transform the data.
/// See [serde_as docs](https://docs.rs/serde_with/latest/serde_with/guide/serde_as/index.html#switching-from-serdes-with-to-serde_as) for more details.
///
/// The keyword `tag` in the serde attribute changes the format
/// of the packet.
/// 
/// See [serde's docs](https://serde.rs/enum-representations.html) for more details
#[serde_as]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "requestType", content = "payload")]
enum ToolRequest {
    Commands(#[serde_as(as = "Vec<(_, _)>")] CommandMap),
}

/// Handles connection with external tools.
///
/// At the moment, only a TCP websocket is available and is meant
/// to connect to our viewer implementation.
pub struct ToolServer {
    websocket: WebSocketTransceiver<ToolRequest, ToolMessage>,
}

impl ToolServer {
    /// Creates a new instance, spawning a separate websocket thread
    /// once created.
    pub fn with_config(tool_config: ToolConfig, _common_config: &CommonConfig) -> Self {
        Self {
            websocket: WebSocketTransceiver::spawn(
                // SocketAddrV4 can be converted .into() a SocketAddr struct
                SocketAddrV4::new(Ipv4Addr::LOCALHOST, tool_config.tool_port).into(),
            ),
        }
    }
}

impl Component for ToolServer {
    fn close(self) {
        self.websocket.close();
    }
}

impl ToolComponent for ToolServer {
    fn step(
        &mut self,
        world_data: &World,
        tool_data: &mut ToolData,
        commands: &mut CommandMap,
    ) -> ToolCommands {
        let msg = ToolMessage {
            version: TOOL_MESSAGE_PROTO_VER,
            data: tool_data.clone(),
            world: world_data.clone(),
        };
        self.websocket.send(msg);
        if let Some(request) = self.websocket.receive() {
            println!("request");
            match request {
                ToolRequest::Commands(tool_commands) => {
                    commands.extend(tool_commands);
                }
            }
        }
        ToolCommands {}
    }
}
