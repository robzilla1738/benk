//! Inbox: decisions queue + compact stats — spec says each row explains why
//! it is present and offers a clear action.
use crate::components::card::{card, card_header, divider};
use crate::components::controls::{ButtonKind, button};
use crate::components::stat::stat_tile;
use crate::fixture::{ACTIVITY_SERIES, INBOX_ITEMS};
use crate::icons::Icon;
use crate::state::{Section, Ui};
use crate::theme::{TEXT_CAPTION, TEXT_LABEL, TEXT_MICRO, theme};
use gpui::{App, FontWeight, IntoElement, ParentElement, Styled, div, px};

fn inbox_item(
    reason: &'static str,
    title: &'static str,
    source: &'static str,
    time: &'static str,
    tone: crate::theme::Tone,
    action: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut App) + 'static,
    cx: &App,
) -> impl IntoElement {
    let t = theme(cx);
    div()
        .flex()
        .items_center()
        .gap_3()
        .py_3()
        .child(
            div().w(px(120.)).flex_none().child(
                div()
                    .px_2()
                    .h(px(22.))
                    .rounded(px(crate::theme::R_SM))
                    .bg(tone.bg)
                    .text_color(tone.fg)
                    .text_size(px(TEXT_MICRO))
                    .font_weight(FontWeight::SEMIBOLD)
                    .flex()
                    .items_center()
                    .child(reason),
            ),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap_px()
                .child(
                    div()
                        .text_size(px(TEXT_LABEL))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(t.text)
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(TEXT_CAPTION))
                        .text_color(t.text_tertiary)
                        .child(source),
                ),
        )
        .child(
            div()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_tertiary)
                .child(time),
        )
        .child(button("Open").kind(ButtonKind::Secondary).on_press(action))
}

pub fn view(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let awaiting = cx.global::<Ui>().decisions_awaiting();

    let stats = div()
        .flex()
        .gap_4()
        .child(
            stat_tile(Icon::Inbox, "Decisions awaiting", awaiting.to_string())
                .series(ACTIVITY_SERIES.to_vec())
                .delta(true, "+2"),
        )
        .child(
            stat_tile(Icon::Bell, "Blocked runs", "1")
                .series(vec![0.8, 0.6, 0.65, 0.4, 0.45, 0.3, 0.35, 0.5]),
        )
        .child(
            stat_tile(Icon::ListChecks, "Open tasks", "1")
                .series(vec![0.1, 0.2, 0.35, 0.3, 0.5, 0.45, 0.6]),
        );

    let rows = div()
        .flex()
        .flex_col()
        .child(inbox_item(
            INBOX_ITEMS[0].reason,
            INBOX_ITEMS[0].title,
            INBOX_ITEMS[0].source,
            INBOX_ITEMS[0].time,
            t.tone_peach,
            |_, _, cx| Ui::show(Section::Work, cx),
            cx,
        ))
        .child(divider(cx))
        .child(inbox_item(
            INBOX_ITEMS[1].reason,
            INBOX_ITEMS[1].title,
            INBOX_ITEMS[1].source,
            INBOX_ITEMS[1].time,
            t.tone_rose,
            |_, _, cx| Ui::show(Section::Work, cx),
            cx,
        ))
        .child(divider(cx))
        .child(inbox_item(
            INBOX_ITEMS[2].reason,
            INBOX_ITEMS[2].title,
            INBOX_ITEMS[2].source,
            INBOX_ITEMS[2].time,
            t.tone_lilac,
            |_, _, cx| Ui::select_channel("checkout", cx),
            cx,
        ));

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(card().child(stats))
        .child(card().child(card_header("Needs you", cx)).child(rows))
}
