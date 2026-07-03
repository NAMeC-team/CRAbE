use crate::behaviors::{Behavior, Context, Node, Status};

#[derive(Debug)]
pub struct Parallel {
    children: Vec<Node>,
    policy: FinishPolicy,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FinishPolicy {
    AnySuccess,
    AnyFailure,
    AllFinished,
}

impl Parallel {
    pub fn new(children: Vec<Node>) -> Self {
        Self {
            children,
            policy: FinishPolicy::AllFinished,
        }
    }

    pub fn with_policy(mut self, policy: FinishPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

impl Behavior for Parallel {
    fn tick(&mut self, ctx: &mut Context) -> Status {
        let mut has_failed = false;
        let mut is_running = false;
        for child in &mut self.children {
            match child.tick(ctx) {
                Status::Running => is_running = true,
                Status::Success => {
                    if self.policy == FinishPolicy::AnySuccess {
                        return Status::Success;
                    }
                }
                Status::Failure => {
                    if self.policy == FinishPolicy::AnyFailure {
                        return Status::Failure;
                    }

                    has_failed = true;
                }
            }
        }

        if is_running {
            Status::Running
        } else if has_failed {
            Status::Failure
        } else {
            Status::Success
        }
    }
}
