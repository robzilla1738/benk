//! Search: live filter over the fixture conversation. Real local filtering —
//! FTS over SQLite lands with the cache work (T005/BENK-003).
use crate::components::card::{card, card_header, divider};
use crate::components::message::message_row;
use crate::state::Ui;
use crate::theme::{TEXT_CAPTION, TEXT_LABEL, theme};
use gpui::{App, IntoElement, ParentElement, Styled, div, px};

pub fn view(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let ui = cx.global::<Ui>();
    let search = ui.search.clone();
    let query = ui.search.read(cx).text().trim().to_lowercase();
    let results: Vec<_> = if query.is_empty() {
        vec![]
    } else {
        ui.messages
            .values()
            .flatten()
            .filter(|m| {
                m.body.to_lowercase().contains(&query) || m.author.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    };

    let mut list = card().gap(0.).pad(18.).child(card_header("Results", cx));
    if query.is_empty() {
        list = list.child(
            div()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text_tertiary)
                .child("Type to filter the fixture conversation in this channel."),
        );
    } else if results.is_empty() {
        list = list.child(
            div()
                .text_size(px(TEXT_LABEL))
                .text_color(t.text_tertiary)
                .child(format!("No matches for “{query}”.")),
        );
    } else {
        for (i, m) in results.iter().enumerate() {
            if i > 0 {
                list = list.child(divider(cx));
            }
            list = list.child(message_row(m, cx));
        }
    }

    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            card()
                .gap(2.)
                .pad(8.)
                .radius(crate::theme::R_MD)
                .child(search)
                .child(
                    div()
                        .px_2()
                        .pb_1()
                        .text_size(px(TEXT_CAPTION))
                        .text_color(t.text_tertiary)
                        .child("Filters fixture messages · full-text search arrives with the local store"),
                ),
        )
        .child(list)
}
