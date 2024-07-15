/// The `square` module contains a strategy that commands a robot to move in a square shape
/// in a counter-clockwise direction. It is used for testing purposes only and is not intended
/// for use in a game.
mod square;
pub use self::square::Square;

mod go_left;
pub use self::go_left::GoLeft;
mod go_right;
pub use self::go_right::GoRight;
mod aligned;
pub use self::aligned::Aligned;
mod prembule;
pub use self::prembule::Prembule;

