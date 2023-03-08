#[derive(Clone)]
pub enum State {
    Running,
    Failed,
    Done,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Running, Self::Running)
                | (Self::Failed, Self::Failed)
                | (Self::Done, Self::Done)
        )
    }
}
