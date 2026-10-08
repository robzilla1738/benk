//! BenkView: app chrome — sidebar + content column with a top strip.
//! Same screen surface as before, restyled to the design system.
use crate::nav;
use crate::state::{Section, Ui};
use crate::theme::{TEXT_CAPTION, TEXT_TITLE, theme};
use crate::views;
use benk_domain::TaskState;
use gpui::prelude::{InteractiveElement, StatefulInteractiveElement};
use gpui::{App, Context, FontWeight, IntoElement, ParentElement, Render, Styled, Window, div, px};

pub struct BenkView;

impl BenkView {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }

    fn subtitle_for(section: Section, task: TaskState) -> &'static str {
        match (section, task) {
            (Section::Work, TaskState::Draft) => "task_checkout — draft",
            (Section::Work, TaskState::Ready) => "task_checkout — ready",
            (Section::Work, TaskState::Active | TaskState::Blocked) => "task_checkout — active",
            (Section::Work, TaskState::AwaitingReview) => "task_checkout — awaiting review",
            (Section::Work, TaskState::Accepted) => "task_checkout — accepted",
            (Section::Work, TaskState::Cancelled) => "task_checkout — cancelled",
            (Section::Inbox, _) => "What needs your attention",
            (Section::Channels, _) => "Fixture conversation · simulated",
            (Section::Projects, _) => "Approved context and scope",
            (Section::Search, _) => "Filter the fixture conversation",
        }
    }

    pub fn focus_composer(window: &mut Window, cx: &mut App) {
        let composer = cx.global::<Ui>().composer.clone();
        let handle = composer.read(cx).focus_handle();
        handle.focus(window);
    }

    pub fn focus_search(window: &mut Window, cx: &mut App) {
        let search = {
            let ui = cx.global_mut::<Ui>();
            ui.section = Section::Search;
            ui.search.clone()
        };
        let handle = search.read(cx).focus_handle();
        handle.focus(window);
        cx.refresh_windows();
    }
}

impl Render for BenkView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = theme(cx);
        let ui = cx.global::<Ui>();
        let section = ui.section;
        let task = ui.task;

        let topbar = div()
            .h(px(52.))
            .px_5()
            .flex()
            .items_center()
            .justify_between()
            .bg(t.surface)
            .border_b_1()
            .border_color(t.border)
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap_3()
                    .child(
                        div()
                            .text_size(px(TEXT_TITLE))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(t.text)
                            .child(section.title()),
                    )
                    .child(
                        div()
                            .text_size(px(TEXT_CAPTION))
                            .text_color(t.text_tertiary)
                            .child(Self::subtitle_for(section, task)),
                    ),
            )
            .child(
                div()
                    .text_size(px(TEXT_CAPTION))
                    .text_color(t.text_tertiary)
                    .child("simulation only"),
            );

        let content = div()
            .id("content-scroll")
            .flex_1()
            .overflow_y_scroll()
            .p_5()
            .child(match section {
                Section::Inbox => views::inbox::view(cx).into_any_element(),
                Section::Channels => views::channel::view(cx).into_any_element(),
                Section::Projects => views::projects::view(cx).into_any_element(),
                Section::Work => views::work::view(cx).into_any_element(),
                Section::Search => views::search::view(cx).into_any_element(),
            });

        div()
            .size_full()
            .flex()
            .bg(t.canvas)
            .text_color(t.text)
            .font_family(".SystemUIFont")
            .child(nav::sidebar(cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .child(topbar)
                    .child(content),
            )
    }
}
