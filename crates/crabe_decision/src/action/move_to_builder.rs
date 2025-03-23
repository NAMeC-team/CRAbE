use crabe_framework::data::output::Kick;
use nalgebra::Point2;
use super::move_to::MoveTo;

/// The `MoveToBuilder` struct represents a move to builder so that you can create moveto commands by chaining functions.
#[derive(Clone)]
pub struct MoveToBuilder {
    /// The target position to move to.
    pub target_x: Option<f64>,
    pub target_y: Option<f64>,
    /// The target orientation of the robot.
    pub orientation: Option<f64>,
    pub charge: bool,
    pub dribbler: f32,
    pub kicker: Option<Kick>,
    pub fast: bool,
    pub avoidance: bool,
}

impl From<&mut MoveToBuilder> for MoveToBuilder {
    fn from(other: &mut MoveToBuilder) -> MoveToBuilder {
        MoveToBuilder {
            target_x: other.target_x,
            target_y: other.target_y,
            orientation: other.orientation,
            charge: other.charge,
            dribbler: other.dribbler,
            kicker: other.kicker,
            fast: other.fast,
            avoidance: other.avoidance
        }
    }
}

impl MoveToBuilder {
    /// Creates a new `MoveToBuilder` instance where the default target position and orientation are set to the current position and orientation of the robot.
    /// You can then use the builder pattern to modify parameters.
    pub fn new() -> Self {
        Self {
            target_x: None,
            target_y: None,
            orientation: None,
            charge: false,
            dribbler: 0.,
            kicker: None,
            fast: true,
            avoidance: true
        }
    }

    /// Desactivate obstacle avoidance.
    pub fn no_avoidance(&mut self) -> &mut Self {
        self.avoidance = false;
        self
    }

    /// Makes the robot move slower towards the target position and orientation.
    pub fn slowly(&mut self) -> &mut Self {
        self.fast = false;
        self
    }

    /// Activate the charging mode of the robot.
    pub fn charging(&mut self) -> &mut Self {
        self.charge = true;
        self
    }

    /// Set the kicker to be used by the robot.
    /// 
    /// # Arguments
    /// * `kicker`: The kicker to be used by the robot.
    pub fn set_kick(&mut self, kicker: Kick) -> &mut Self {
        self.kicker = Some(kicker);
        self
    }

    /// Set the dribbler speed of the robot.
    /// 
    /// # Arguments
    /// * `dribbler`: The dribbler speed of the robot.
    pub fn set_dribbler(&mut self, dribbler: f32) -> &mut Self {
        self.dribbler = dribbler;
        self
    }

    /// Set the x target position of the robot.
    /// 
    /// # Arguments
    /// * `x`: The x target position of the robot.
    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.target_x = Some(x);
        self
    }

    /// Set the y target position of the robot. 
    /// 
    /// # Arguments
    /// * `y`: The y target position of the robot.
    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.target_y = Some(y);
        self
    }

    /// Set the target position of the robot.
    /// 
    /// # Arguments
    /// * `target`: A 2d point representing the target position of the robot.
    pub fn set_target(&mut self, target: Point2<f64>) -> &mut Self {
        self.target_x = Some(target.x);
        self.target_y = Some(target.y);
        self
    }

    /// Set the target orientation of the robot.
    /// 
    /// # Arguments
    /// * `orientation`: The target angle of the robot.
    pub fn set_orientation(&mut self, orientation: f64) -> &mut Self {
        self.orientation = Some(orientation);
        self
    }

    /// Build the `MoveTo` command.
    /// 
    /// # Returns
    /// A `MoveTo` command with the specified moveto builder parameters.
    pub fn build(&self) -> MoveTo {
        MoveTo {
            target_x: self.target_x,
            target_y: self.target_y,
            orientation: self.orientation,
            charge: self.charge,
            dribbler: self.dribbler,
            kicker: self.kicker,
            fast: self.fast,
            avoidance: self.avoidance,
            ..Default::default()
        }
    }
}