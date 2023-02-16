/// Size of the buffer for packet (game_controller, vision, etc...).
/// This buffer size was chosen to accommodate the largest possible packet size for the protocols that use it, including overhead and padding.
pub const BUFFER_SIZE: usize = 4096;
