//! Synthetic fixture data for the shell. Everything here is presentation
//! material for the simulation — no real people, servers or model output.
use gpui::SharedString;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AuthorKind {
    Human,
    Agent,
}

#[derive(Clone)]
pub struct Message {
    pub author: SharedString,
    pub author_kind: AuthorKind,
    pub time: SharedString,
    pub body: SharedString,
}

#[derive(Clone, Copy)]
pub struct Channel {
    pub name: &'static str,
    pub unread: u32,
    pub topic: &'static str,
}

#[derive(Clone, Copy)]
pub struct InboxItem {
    pub reason: &'static str,
    pub title: &'static str,
    pub source: &'static str,
    pub time: &'static str,
}

pub const CHANNELS: &[Channel] = &[
    Channel {
        name: "checkout",
        unread: 2,
        topic: "Payment-flow wording and recovery",
    },
    Channel {
        name: "design",
        unread: 0,
        topic: "Interface language and review artifacts",
    },
    Channel {
        name: "general",
        unread: 0,
        topic: "Workspace-wide discussion",
    },
];

pub const INBOX_ITEMS: &[InboxItem] = &[
    InboxItem {
        reason: "Review requested",
        title: "Checkout wording — accept or request changes",
        source: "task_checkout · simulated artifact v1",
        time: "2m",
    },
    InboxItem {
        reason: "Blocked work",
        title: "agt_simulator lacks repository access it did not request",
        source: "run seed · awaiting owner decision",
        time: "1h",
    },
    InboxItem {
        reason: "Mention",
        title: "usr_product mentioned you in #checkout",
        source: "Can we get this into the Friday cut?",
        time: "3h",
    },
];

/// The scripted #checkout conversation — mirrors docs/handoff/docs/22.
pub fn seed_messages() -> Vec<Message> {
    vec![
        Message {
            author: "usr_product".into(),
            author_kind: AuthorKind::Human,
            time: "09:41".into(),
            body: "Customer report: people hit a payment error and cannot tell how to recover. Three of them abandoned checkout entirely.".into(),
        },
        Message {
            author: "you".into(),
            author_kind: AuthorKind::Human,
            time: "09:44".into(),
            body: "Let's scope this tightly — clearer wording, a retry path, and a reviewable result. I will turn the report into a task.".into(),
        },
        Message {
            author: "agt_simulator".into(),
            author_kind: AuthorKind::Agent,
            time: "09:45".into(),
            body: "I can prepare a bounded simulation: a wording proposal plus a synthetic patch and test log. No production access, no pull request.".into(),
        },
        Message {
            author: "usr_product".into(),
            author_kind: AuthorKind::Human,
            time: "09:47".into(),
            body: "Good. Keep it to checkout only — anything broader becomes a new proposal.".into(),
        },
    ]
}

/// Synthetic weekly activity shape for stat tiles (visually labeled simulation).
pub const ACTIVITY_SERIES: &[f32] = &[0.2, 0.35, 0.3, 0.55, 0.5, 0.72, 0.95, 0.62, 0.4, 0.18];

pub const PROJECT_NAME: &str = "Checkout experience";
pub const PROJECT_SCOPE: &str = "Synthetic checkout task only — the contract does not authorize deployments, customer messaging, or any external action.";

pub const WORK_TITLE: &str = "Improve the checkout error";
pub const TASK_EVENTS: &[&str] = &[
    "task:draft",
    "task:ready",
    "task:active",
    "task:awaiting_review",
];

pub const REVIEW_SUMMARY: &str = "Synthetic wording proposal: replace the checkout error with \"We could not complete your payment. Please check your details and try again.\" No repository was modified.";

pub const REVIEW_CHECKS: &[&str] = &[
    "Synthetic fixture produced; no real tests executed",
    "Patch is illustrative — no working copy changed",
    "Approval binds artifact v1 only; changes make it stale",
];

pub const REVIEW_LIMITS: &[&str] = &[
    "No test environment exercised",
    "No repository diff produced",
    "Simulated reviewer is not a second authenticated account",
];
