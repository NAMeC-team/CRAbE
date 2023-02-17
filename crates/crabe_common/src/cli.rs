use clap::Args;

/// A struct representing command-line options that are common to multiple CRAbE crates.
///
/// # Fields
///
/// * `yellow`: Whether robots are on the yellow team.
/// * `real`: Whether robots are operating in the real world or in simulation.#[derive(Args)]
#[derive(Args)]
pub struct CrabeCommonCLI {
    #[arg(short = 'y', long)]
    pub yellow: bool,

    #[arg(short, long)]
    pub real: bool,
}