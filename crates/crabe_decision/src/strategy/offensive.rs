/// The `offensive` module contains a strategies used to attack the opponent's goal.
/// for use in a game.
mod attacker;
pub use self::attacker::Attacker;
mod receiver;
pub use self::receiver::Receiver;