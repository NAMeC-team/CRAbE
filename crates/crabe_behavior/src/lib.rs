pub mod assignment;
pub mod behaviors;
pub mod engine;
pub mod formations;
pub mod managers;
pub mod plays;
pub mod strategies;
mod utils;

pub enum Role {
    Attacker,
    Support,
    Defender,
    Keeper,
}

pub enum Goal {}
