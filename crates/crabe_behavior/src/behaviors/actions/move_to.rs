use crate::behaviors::blackboard::{InputPort, IntentWriter, OutputPort};
use crate::behaviors::{Behavior, Context, Node, Status, cond, seq};
use crate::utils::pose_to_frame;
use crabe_framework::data::world::{AllyInfo, Pose, Robot, RobotVelocity};
use crabe_math::angles::angle_difference;
use crabe_math::geometry::{frame, frame_inv};
use nalgebra::{Isometry2, Point2, Vector2, Vector3};
use tracing::info;

#[derive(Debug)]
pub struct MoveTo {
    speed: MoveSpeed,
    error_tolerance: f64,
    target: InputPort<Pose>,
    movement: IntentWriter<RobotVelocity>,
}

#[derive(Clone, Copy, Debug)]
pub enum MoveSpeed {
    Fast,
    Normal,
}

impl MoveSpeed {
    pub fn angular(self) -> f64 {
        match self {
            MoveSpeed::Fast => 3.,
            MoveSpeed::Normal => 1.5,
        }
    }

    pub fn linear(self) -> f64 {
        match self {
            MoveSpeed::Fast => 3.,
            MoveSpeed::Normal => 1.5,
        }
    }
}

impl MoveTo {
    pub fn new(target: InputPort<Pose>, movement: IntentWriter<RobotVelocity>) -> Self {
        Self {
            target,
            movement,
            speed: MoveSpeed::Normal,
            error_tolerance: 0.7,
        }
    }

    pub fn with_speed(mut self, speed: MoveSpeed) -> Self {
        self.speed = speed;
        self
    }

    pub fn with_error_tolerance(mut self, error_tolerance: f64) -> Self {
        self.error_tolerance = error_tolerance;
        self
    }
}

impl MoveTo {
    fn compute(&mut self, ctx: &Context) -> (Status, Vector3<f64>) {
        let mut order = Vector3::new(0., 0., 0.);
        let robot = match ctx.robot() {
            Some(robot) => robot,
            None => return (Status::Failure, order),
        };

        let ti = frame_inv(pose_to_frame(robot.pose()));

        // calculate position command
        let target = ctx.get(&self.target);
        //if id != KEEPER_ID{
        //    target = penalty_zone_prevention(&robot.pose.position, &target, world);
        //}
        //if self.avoidance{
        //    target = obstacle_avoidance(&target, robot, world, _tools);
        //}
        //_tools.annotations.add_circle(vec!["target".to_string(), id.to_string()].join("-"),Circle::new(target, 0.1));
        let target_in_robot = ti * Point2::new(target.position.x, target.position.y);
        order.x = target_in_robot[0];
        order.y = target_in_robot[1];

        // calculate orientation command
        let orientation = target.orientation;
        order.z = angle_difference(orientation, robot.orientation());

        // if the robot is close to the target orientation, it will stop this moveto
        let arrived = order.norm() < self.error_tolerance;
        if arrived {
            return (Status::Success, order);
        }

        // mutliply by the speed factors
        order.x = self.speed.linear() * order.x;
        order.y = self.speed.linear() * order.y;
        order.z = self.speed.angular() * order.z;

        (Status::Running, order)
    }
}

impl Behavior for MoveTo {
    // TODO: use crabe_navigation
    fn tick(&mut self, ctx: &mut Context) -> Status {
        let (state, order) = self.compute(ctx);
        let velocity = RobotVelocity {
            linear: Vector2::new(order.x, order.y),
            angular: order.z,
        };
        ctx.commit(&self.movement, velocity);
        state
    }
}
