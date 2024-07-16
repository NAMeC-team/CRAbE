use crabe_math::shape::Line;

// Possible messages types to transmit
#[derive(Clone)]
pub enum Message {
    WantToGoRight,
    WantToGoLeft,
    WantToBeAligned,
    AttackerMessage(AttackerMessage),
}

#[derive(Clone)]
pub enum AttackerMessage{
    WantToPassBallTo(u8, Line),
    NoNeedReceiver,
    BallPassed(u8),
}


// Data type to share information from the bot to the manager
#[derive(Clone)]
pub struct MessageData {
    pub message: Message,
    pub id: u8,
}

impl MessageData {
    pub fn new(message: Message, id: u8) -> Self {
        Self { message, id }
    }
}