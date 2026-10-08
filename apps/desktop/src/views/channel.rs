//! Channel view: timeline + composer. Sent messages append to the local
//! fixture and stay labeled simulation.
use crate::components::card::card;
use crate::components::controls::{ButtonKind, button, icon_button};
use crate::components::message::{agent_card, message_row, sim_banner};
use crate::fixture::CHANNELS;
use crate::icons::{Icon, icon};
use crate::state::Ui;
use crate::theme::{TEXT_CAPTION, TEXT_TITLE, theme};
use gpui::{App, FontWeight, IntoElement, ParentElement, Styled, div, px};

pub fn view(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let ui = cx.global::<Ui>();
    let channel = CHANNELS
        .iter()
        .find(|c| c.name == ui.channel)
        .unwrap_or(&CHANNELS[0]);
    let messages = ui.messages.get(channel.name).cloned().unwrap_or_default();
    let composer = ui.composer.clone();
    let can_send = !ui.composer.read(cx).is_empty();

    let header = card().gap(4.).pad(18.).child(
        div()
            .flex()
            .items_center()
            .gap_3()
            .child(
                div()
                    .size(px(34.))
                    .rounded(px(crate::theme::R_MD))
                    .bg(t.well)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(t.text_secondary)
                    .child(icon(Icon::Hash).size(px(17.))),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(TEXT_TITLE))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(t.text)
                            .child(format!("#{}", channel.name)),
                    )
                    .child(
                        div()
                            .text_size(px(TEXT_CAPTION))
                            .text_color(t.text_tertiary)
                            .child(channel.topic),
                    ),
            )
            .child(icon_button(Icon::Users, "3 members").on_press(|_, _, _| {}))
            .child(icon_button(Icon::Bell, "Notifications").on_press(|_, _, _| {})),
    );

    let mut timeline = card()
        .gap(0.)
        .pad(18.)
        .child(sim_banner(cx))
        .children(messages.iter().map(|m| message_row(m, cx)));
    if messages.is_empty() {
        timeline = timeline.child(
            div()
                .py_8()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .text_color(t.text_tertiary)
                .child(icon(Icon::Hash).size(px(20.)))
                .child(div().text_size(px(TEXT_CAPTION)).child(
                    "No fixture messages in this channel yet — sent messages stay local to it.",
                )),
        );
    }
    // The scripted agent run is part of the #checkout narrative only.
    if channel.name == "checkout" {
        timeline = timeline.child(agent_card(
            "agt_simulator",
            "run succeeded — review pending",
            "Produced one synthetic artifact: wording proposal v1. Contract forbids external actions; tests are illustrative.",
            cx,
        ));
    }

    let composer_card = card()
        .gap(2.)
        .pad(8.)
        .radius(crate::theme::R_MD)
        .child(
            div()
                .flex()
                .items_end()
                .gap_2()
                .child(div().flex_1().child(composer))
                .child(
                    div().pb_1().pr_1().child(
                        button("Send")
                            .icon(Icon::Send)
                            .kind(ButtonKind::Primary)
                            .disabled(!can_send)
                            .on_press(|_, _, cx| {
                                let composer = cx.global::<Ui>().composer.clone();
                                composer.update(cx, |c, cx| c.submit(cx));
                                cx.refresh_windows();
                            }),
                    ),
                ),
        )
        .child(
            div()
                .px_2()
                .pb_1()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_tertiary)
                .child("Enter to send · Shift+Enter for a newline · IME composition is safe"),
        );

    div()
        .flex()
        .flex_col()
        .gap_4()
        .h_full()
        .child(header)
        .child(timeline)
        .child(composer_card)
}
