//! Conversation pieces: message rows, agent status card, simulation banner.
use crate::components::avatar::{AvatarSize, avatar};
use crate::components::controls::sim_chip;
use crate::fixture::{AuthorKind, Message};
use crate::theme::{R_MD, TEXT_BODY, TEXT_CAPTION, theme};
use gpui::{App, FontWeight, IntoElement, ParentElement, Styled, div, px};

/// Quiet one-line honesty strip shown above the timeline.
pub fn sim_banner(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    div()
        .flex()
        .items_center()
        .gap_2()
        .px_3()
        .py_2()
        .rounded(px(R_MD))
        .bg(t.sim_bg)
        .border_1()
        .border_color(t.sim_border)
        .text_color(t.sim_text)
        .text_size(px(TEXT_CAPTION))
        .child(sim_chip(cx))
        .child("Synthetic fixture — no models, tools, authenticated users or external actions.")
}

pub fn message_row(message: &Message, cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let mut meta = div()
        .flex()
        .items_center()
        .gap_2()
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(t.text)
                .child(message.author.clone()),
        )
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_tertiary)
                .child(message.time.clone()),
        );
    if message.author_kind == AuthorKind::Agent {
        meta = meta.child(sim_chip(cx));
    }
    div()
        .flex()
        .gap_3()
        .py_2()
        .child(avatar(&message.author, AvatarSize::Md, cx))
        .child(
            div().flex().flex_col().gap_1().child(meta).child(
                div()
                    .text_size(px(TEXT_BODY))
                    .text_color(t.text)
                    .line_height(px(20.))
                    .child(message.body.clone()),
            ),
        )
}

/// Compact agent run card — meaningful state only; token streams live in the
/// (future) run view per the experience spec.
pub fn agent_card(title: &str, status: &str, detail: &str, cx: &App) -> impl IntoElement {
    let t = theme(cx);
    div()
        .ml(px(42.))
        .flex()
        .flex_col()
        .gap_1()
        .p_3()
        .rounded(px(R_MD))
        .bg(t.well)
        .border_1()
        .border_color(t.border)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .size(px(18.))
                        .rounded(px(5.))
                        .bg(t.tone_lilac.bg)
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(t.tone_lilac.fg)
                        .child(crate::icons::icon(crate::icons::Icon::Sparkles).size(px(11.))),
                )
                .child(
                    div()
                        .text_size(px(TEXT_CAPTION))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(t.text)
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_size(px(TEXT_CAPTION))
                        .text_color(t.text_tertiary)
                        .child(status.to_string()),
                ),
        )
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_secondary)
                .child(detail.to_string()),
        )
}
