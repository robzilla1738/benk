//! Work view: task detail + contract + review package. The simulated
//! advance/review logic is unchanged; only presentation is new.
use crate::components::avatar::{AvatarSize, avatar};
use crate::components::card::{card, card_header, divider};
use crate::components::controls::{ButtonKind, button, sim_chip, state_chip};
use crate::components::message::sim_banner;
use crate::fixture::{REVIEW_CHECKS, REVIEW_LIMITS, REVIEW_SUMMARY, TASK_EVENTS, WORK_TITLE};
use crate::icons::{Icon, icon};
use crate::state::Ui;
use crate::theme::{TEXT_CAPTION, TEXT_LABEL, TEXT_TITLE, theme};
use benk_domain::TaskState;
use gpui::{App, FontWeight, IntoElement, ParentElement, Styled, div, px};

fn check_row(text: &'static str, cx: &App) -> impl IntoElement {
    let t = theme(cx);
    div()
        .flex()
        .items_center()
        .gap_2()
        .py(px(3.))
        .child(icon(Icon::Check).size(px(13.)).text_color(t.success))
        .child(
            div()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text_secondary)
                .child(text),
        )
}

fn limit_row(text: &'static str, cx: &App) -> impl IntoElement {
    let t = theme(cx);
    div()
        .flex()
        .items_center()
        .gap_2()
        .py(px(3.))
        .child(
            icon(Icon::CircleHelp)
                .size(px(13.))
                .text_color(t.text_tertiary),
        )
        .child(
            div()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text_secondary)
                .child(text),
        )
}

pub fn view(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let ui = cx.global::<Ui>();
    let task = ui.task;
    let feedback = ui.task_feedback.clone();

    let header = card()
        .gap(6.)
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_size(px(TEXT_TITLE))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(t.text)
                                .child(WORK_TITLE),
                        )
                        .child(
                            div()
                                .text_size(px(TEXT_CAPTION))
                                .text_color(t.text_tertiary)
                                .child("task_checkout · contract revision 1 · ws_alpha"),
                        ),
                )
                .child(state_chip(task, cx))
                .child(sim_chip(cx)),
        )
        .child(divider(cx))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(avatar("usr_product", AvatarSize::Sm, cx))
                .child(
                    div()
                        .text_size(px(TEXT_CAPTION))
                        .text_color(t.text_secondary)
                        .child(
                            "Owner: usr_product · Reviewer: usr_reviewer · Agent: agt_simulator",
                        ),
                ),
        );

    let contract = card()
        .gap(6.)
        .child(card_header("Work contract", cx))
        .child(
            div()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text_secondary)
                .child("Goal: clearer checkout error wording with a reviewable synthetic result. Allowed actions: none (fixture). Budget: not configured. Approval: human review before anything external."),
        );

    let mut review = card().gap(6.).child(card_header("Review package", cx));

    if matches!(task, TaskState::AwaitingReview | TaskState::Accepted) {
        review = review
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        icon(Icon::FileText)
                            .size(px(15.))
                            .text_color(t.tone_blue.fg),
                    )
                    .child(
                        div()
                            .text_size(px(TEXT_LABEL))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(t.text)
                            .child("artifact_demo_checkout · v1"),
                    )
                    .child(
                        div()
                            .text_size(px(TEXT_CAPTION))
                            .text_color(t.text_tertiary)
                            .child("sha256:4f39d52e…42f4bcb"),
                    ),
            )
            .child(
                div()
                    .text_size(px(TEXT_LABEL))
                    .text_color(t.text)
                    .child(REVIEW_SUMMARY),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(card_header("Checks", cx).text_size(px(TEXT_CAPTION)))
                    .children(REVIEW_CHECKS.iter().map(|c| check_row(c, cx))),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(card_header("Limitations", cx).text_size(px(TEXT_CAPTION)))
                    .children(REVIEW_LIMITS.iter().map(|c| limit_row(c, cx))),
            );
    } else {
        review = review.child(
            div()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text_tertiary)
                .child("Advance the simulation through ready and active to produce the synthetic review package."),
        );
    }

    let actions = card()
        .gap(8.)
        .child(card_header("Decide", cx))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    button("Advance simulation")
                        .kind(ButtonKind::Secondary)
                        .icon(Icon::ArrowUpRight)
                        .disabled(matches!(
                            task,
                            TaskState::AwaitingReview | TaskState::Accepted | TaskState::Cancelled
                        ))
                        .on_press(|_, _, cx| Ui::advance_task(cx)),
                )
                .child(
                    button("Review as simulated teammate")
                        .kind(ButtonKind::Primary)
                        .icon(Icon::ShieldCheck)
                        .disabled(task != TaskState::AwaitingReview)
                        .on_press(|_, _, cx| Ui::review_task(cx)),
                ),
        )
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_tertiary)
                .child(feedback),
        );

    let mut activity = card().gap(2.).child(card_header("Activity", cx));
    let visible: &[&'static str] = match task {
        TaskState::Draft => &TASK_EVENTS[..1],
        TaskState::Ready => &TASK_EVENTS[..2],
        TaskState::Active | TaskState::Blocked => &TASK_EVENTS[..3],
        TaskState::AwaitingReview => &TASK_EVENTS[..4],
        TaskState::Accepted => &[
            "task:draft",
            "task:ready",
            "task:active",
            "task:awaiting_review",
            "task:accepted",
        ],
        TaskState::Cancelled => &TASK_EVENTS[..1],
    };
    for event in visible {
        activity = activity.child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .py(px(3.))
                .child(div().size(px(6.)).rounded_full().bg(t.tone_blue.fg))
                .child(
                    div()
                        .text_size(px(TEXT_CAPTION))
                        .text_color(t.text_secondary)
                        .child(*event),
                ),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(header)
        .child(sim_banner(cx))
        .child(contract)
        .child(review)
        .child(actions)
        .child(activity)
}
