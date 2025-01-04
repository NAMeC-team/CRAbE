use crate::data::annotation::AnnotationStore;
use serde::Serialize;

/// The `ToolData` struct stores additional data that can be sent to
/// external tools, such as a viewer or joystick handler.
/// When adding new content to send to these tools, this struct should be modified, not its wrapper ToolMessage.
/// 
#[derive(Clone, Default, Serialize)]
pub struct ToolData {
    #[serde(flatten)]
    pub annotations: AnnotationStore,
}

/// Represents data sent by (our) external tools to the software.
pub struct ToolCommands;
