//! Pure task reducer. Allowed edges are NOT command authorization.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TaskState {
    Draft,
    Ready,
    Active,
    Blocked,
    AwaitingReview,
    Accepted,
    Cancelled,
}
impl TaskState {
    pub const ALL: [Self; 7] = [
        Self::Draft,
        Self::Ready,
        Self::Active,
        Self::Blocked,
        Self::AwaitingReview,
        Self::Accepted,
        Self::Cancelled,
    ];
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Ready => "ready",
            Self::Active => "active",
            Self::Blocked => "blocked",
            Self::AwaitingReview => "awaiting_review",
            Self::Accepted => "accepted",
            Self::Cancelled => "cancelled",
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidTransition {
    pub from: TaskState,
    pub to: TaskState,
}
pub fn transition_task(from: TaskState, to: TaskState) -> Result<TaskState, InvalidTransition> {
    use TaskState::*;
    let allowed = matches!(
        (from, to),
        (Draft, Ready | Cancelled)
            | (Ready, Active | Cancelled)
            | (Active, Blocked | AwaitingReview | Cancelled)
            | (Blocked, Active | Cancelled)
            | (AwaitingReview, Active | Accepted | Cancelled)
            | (Accepted, Ready)
    );
    if allowed {
        Ok(to)
    } else {
        Err(InvalidTransition { from, to })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhaustive_edges() {
        let expected = [
            vec![1, 6],
            vec![2, 6],
            vec![3, 4, 6],
            vec![2, 6],
            vec![2, 5, 6],
            vec![1],
            vec![],
        ];
        for (i, from) in TaskState::ALL.into_iter().enumerate() {
            for (j, to) in TaskState::ALL.into_iter().enumerate() {
                assert_eq!(
                    transition_task(from, to).is_ok(),
                    expected[i].contains(&j),
                    "{} -> {}",
                    from.as_str(),
                    to.as_str()
                );
            }
        }
    }
    #[test]
    fn run_success_does_not_accept_task() {
        assert!(transition_task(TaskState::Active, TaskState::Accepted).is_err());
    }
    #[test]
    fn cancelled_is_terminal() {
        for to in TaskState::ALL {
            assert!(transition_task(TaskState::Cancelled, to).is_err());
        }
    }
}
