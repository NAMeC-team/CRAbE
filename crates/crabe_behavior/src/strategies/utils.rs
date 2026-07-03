use crate::behaviors::Node::Parallel;
use crate::behaviors::actions::move_to::MoveTo;
use crate::behaviors::actions::target::ball::ComputeBallTarget;
use crate::behaviors::blackboard::InputPort;
use crate::behaviors::{Node, action, par};
use nalgebra::Point2;
