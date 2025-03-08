use clap::Args;

/// A struct representing some options that are common to multiple CRAbE crates.
#[derive(Args)]
pub struct CommonConfig {
    /// Whether robots are on the yellow team.
    #[arg(short = 'y', long)]
    pub yellow: bool,
    /// Whether robots are operating in the real world or in simulation.
    #[arg(short, long)]
    pub real: bool,
    /// Specifies whether we should receive data
    /// from the external Game Controller of the Robocup SSL.
    #[arg(long)]
    pub gc: bool,
    /// If true, receives filtered vision data from the tracker implementations
    /// that gives proper speed values for ball and robots.
    /// When disabled, velocities and acceleration are computed from vision data.
    /// Note that this computation does not occur when this argument is set
    #[arg(long)]
    pub tracker: bool
}
