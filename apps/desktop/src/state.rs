//! App UI state as a GPUI global. Fixture mutations stay honest: sent
//! messages append locally and are labeled simulation, drafts persist
//! in-memory per channel (durable SQLite drafts are T005).
use crate::composer::{Composer, ComposerEvent};
use crate::fixture::{self, AuthorKind, Message};
use benk_domain::{TaskState, transition_task};
use gpui::{App, AppContext, Entity, Global, SharedString, Subscription};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Section {
    Inbox,
    Channels,
    Projects,
    Work,
    Search,
}

impl Section {
    pub fn title(self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
            Self::Channels => "Channels",
            Self::Projects => "Projects",
            Self::Work => "Work",
            Self::Search => "Search",
        }
    }
}

pub struct Ui {
    pub section: Section,
    pub channel: &'static str,
    pub channels_expanded: bool,
    pub task: TaskState,
    pub task_feedback: SharedString,
    /// Per-channel timelines: the scripted fixture lives in #checkout;
    /// locally sent messages append to whichever channel is open.
    pub messages: HashMap<String, Vec<Message>>,
    /// Per-channel unread counts, cleared when the channel is opened.
    pub unread: HashMap<String, u32>,
    /// In-memory drafts per channel — NOT durable (SQLite drafts are T005).
    pub drafts: HashMap<String, String>,
    pub composer: Entity<Composer>,
    pub search: Entity<Composer>,
    /// Retained so composer event subscriptions stay alive.
    _subs: Vec<Subscription>,
}

impl Ui {
    pub fn init(cx: &mut App) -> Self {
        let composer = cx.new(|cx| Composer::new("Message #checkout", false, cx));
        let search = cx.new(|cx| Composer::new("Search messages", true, cx));

        let send_sub = cx.subscribe(&composer, |_entity, event, cx| {
            if let ComposerEvent::Submitted(text) = event {
                let ui = cx.global_mut::<Ui>();
                let body = text.clone();
                let channel = ui.channel.to_string();
                ui.messages
                    .entry(channel.clone())
                    .or_default()
                    .push(Message {
                        author: "you".into(),
                        author_kind: AuthorKind::Human,
                        time: "now".into(),
                        body: body.into(),
                    });
                ui.drafts.insert(channel, String::new());
                cx.refresh_windows();
            }
        });
        let draft_sub = cx.subscribe(&composer, |_entity, event, cx| {
            if let ComposerEvent::Changed = event {
                let (composer, channel) = {
                    let ui = cx.global_mut::<Ui>();
                    (ui.composer.clone(), ui.channel)
                };
                let text = composer.read(cx).text().to_string();
                cx.global_mut::<Ui>()
                    .drafts
                    .insert(channel.to_string(), text);
                // Re-render so affordances like the Send button track emptiness.
                cx.refresh_windows();
            }
        });
        let search_sub = cx.subscribe(&search, |_entity, event, cx| {
            if let ComposerEvent::Changed = event {
                cx.refresh_windows();
            }
        });

        Self {
            section: Section::Work,
            channel: "checkout",
            channels_expanded: true,
            task: TaskState::Draft,
            task_feedback: "Safe simulation ready.".into(),
            messages: HashMap::from([("checkout".to_string(), fixture::seed_messages())]),
            unread: fixture::CHANNELS
                .iter()
                .map(|c| (c.name.to_string(), c.unread))
                .collect(),
            drafts: HashMap::new(),
            composer,
            search,
            _subs: vec![send_sub, draft_sub, search_sub],
        }
    }

    pub fn select_channel(name: &'static str, cx: &mut App) {
        let (composer, draft) = {
            let ui = cx.global_mut::<Ui>();
            ui.channel = name;
            ui.section = Section::Channels;
            ui.unread.insert(name.to_string(), 0);
            (
                ui.composer.clone(),
                ui.drafts.get(name).cloned().unwrap_or_default(),
            )
        };
        composer.update(cx, |c, cx| {
            c.set_placeholder(format!("Message #{name}"), cx);
            c.set_text(&draft, cx);
        });
        cx.refresh_windows();
    }

    pub fn show(section: Section, cx: &mut App) {
        cx.global_mut::<Ui>().section = section;
        cx.refresh_windows();
    }

    pub fn advance_task(cx: &mut App) {
        let ui = cx.global_mut::<Ui>();
        let to = match ui.task {
            TaskState::Draft => TaskState::Ready,
            TaskState::Ready => TaskState::Active,
            TaskState::Active => TaskState::AwaitingReview,
            _ => return,
        };
        if let Ok(next) = transition_task(ui.task, to) {
            ui.task = next;
            ui.task_feedback = "Simulation advanced. No external action performed.".into();
        }
        cx.refresh_windows();
    }

    pub fn review_task(cx: &mut App) {
        let ui = cx.global_mut::<Ui>();
        if ui.task == TaskState::AwaitingReview {
            let _ = transition_task(ui.task, TaskState::Accepted).map(|n| ui.task = n);
            ui.task_feedback =
                "Accepted by a simulated second human. Not an authenticated approval.".into();
        } else {
            ui.task_feedback = "A review package is not ready yet.".into();
        }
        cx.refresh_windows();
    }

    /// Inbox counts shown on the stat tiles — derived from the fixture.
    pub fn decisions_awaiting(&self) -> usize {
        crate::fixture::INBOX_ITEMS.len()
    }
}

impl Global for Ui {}
