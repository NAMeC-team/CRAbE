/// The State enum represents the state of an action. It can have one of the following values:
///
/// * Running: The action is currently being executed.
/// * Failed: The action has failed to execute.
/// * Done: The action has been successfully executed.
#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Running,
    Failed,
    Done,
}
