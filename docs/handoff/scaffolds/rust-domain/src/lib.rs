//! UNCOMPILED reducer seed. An allowed edge does not imply command authorization.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TaskState { Draft, Ready, Active, Blocked, AwaitingReview, Accepted, Cancelled }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidTransition { pub from: TaskState, pub to: TaskState }

pub fn transition_task(from: TaskState, to: TaskState) -> Result<TaskState, InvalidTransition> {
    use TaskState::*;
    let allowed = matches!((from, to),
        (Draft, Ready | Cancelled) | (Ready, Active | Cancelled) |
        (Active, Blocked | AwaitingReview | Cancelled) | (Blocked, Active | Cancelled) |
        (AwaitingReview, Active | Accepted | Cancelled) | (Accepted, Ready));
    if allowed { Ok(to) } else { Err(InvalidTransition { from, to }) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn active_can_await_review() {
        assert_eq!(transition_task(TaskState::Active, TaskState::AwaitingReview), Ok(TaskState::AwaitingReview));
    }
    #[test] fn active_cannot_self_accept() {
        assert!(transition_task(TaskState::Active, TaskState::Accepted).is_err());
    }
    #[test] fn cancelled_is_terminal() {
        assert!(transition_task(TaskState::Cancelled, TaskState::Ready).is_err());
    }
}
