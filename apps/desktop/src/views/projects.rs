//! Projects: approved context and scope for the fixture workspace.
use crate::components::card::{card, card_header, divider};
use crate::components::controls::count_badge;
use crate::fixture::{PROJECT_NAME, PROJECT_SCOPE};
use crate::theme::{TEXT_CAPTION, TEXT_LABEL, theme};
use gpui::{App, IntoElement, ParentElement, Styled, div, px};

fn field(label: &'static str, value: &'static str, cx: &App) -> impl IntoElement {
    let t = theme(cx);
    div()
        .flex()
        .items_center()
        .gap_3()
        .py_2()
        .child(
            div()
                .w(px(140.))
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_tertiary)
                .child(label),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text)
                .child(value),
        )
}

pub fn view(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let project = card()
        .gap(6.)
        .child(card_header(PROJECT_NAME, cx))
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_secondary)
                .child(PROJECT_SCOPE),
        )
        .child(divider(cx))
        .child(field(
            "Repositories",
            "repo_checkout — synthetic, read-only in fixture",
            cx,
        ))
        .child(field("Environments", "none — execution disabled", cx))
        .child(field(
            "Model destination",
            "none — simulated agent only",
            cx,
        ))
        .child(field(
            "Members",
            "usr_product, usr_reviewer, agt_simulator",
            cx,
        ));

    let decisions = card()
        .gap(8.)
        .child(card_header("Decisions", cx))
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(count_badge(2, t.tone_blue))
                .child(
                    div()
                        .text_size(px(TEXT_LABEL))
                        .text_color(t.text)
                        .child("Scope: checkout wording only; expansion requires a new proposal"),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(count_badge(1, t.tone_neutral))
                .child(
                    div()
                        .text_size(px(TEXT_LABEL))
                        .text_color(t.text)
                        .child("No external actions in this slice — broker remains deny-all"),
                ),
        );

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(project)
        .child(decisions)
}
