//! Stat tile: circle icon, label with help dot, big number, delta pill,
//! sparkline — the overview pattern from the design reference.
use crate::components::card::help_dot;
use crate::components::controls::delta_pill;
use crate::components::sparkline::sparkline;
use crate::icons::{Icon, icon};
use crate::theme::{TEXT_CAPTION, TEXT_DISPLAY, theme};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, FontWeight, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div, px,
};

#[derive(IntoElement)]
pub struct StatTile {
    icon: Icon,
    label: SharedString,
    value: SharedString,
    delta: Option<(bool, SharedString)>,
    series: Vec<f32>,
}

pub fn stat_tile(
    icon: Icon,
    label: impl Into<SharedString>,
    value: impl Into<SharedString>,
) -> StatTile {
    StatTile {
        icon,
        label: label.into(),
        value: value.into(),
        delta: None,
        series: vec![],
    }
}

impl StatTile {
    pub fn delta(mut self, up: bool, label: impl Into<SharedString>) -> Self {
        self.delta = Some((up, label.into()));
        self
    }
    pub fn series(mut self, points: Vec<f32>) -> Self {
        self.series = points;
        self
    }
}

impl RenderOnce for StatTile {
    fn render(self, _window: &mut gpui::Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let line_color: Hsla = t.success.into();
        let mut el = div().flex().flex_col().gap_3().min_w(px(180.));
        el = el.child(
            div()
                .size(px(40.))
                .rounded_full()
                .bg(t.well)
                .flex()
                .items_center()
                .justify_center()
                .text_color(t.text)
                .child(icon(self.icon).size(px(18.))),
        );
        el = el.child(
            div()
                .flex()
                .items_center()
                .gap_1()
                .text_size(px(TEXT_CAPTION))
                .text_color(t.text_secondary)
                .child(self.label)
                .child(help_dot(cx)),
        );
        el = el.child(
            div()
                .flex()
                .items_end()
                .gap_4()
                .child(
                    div()
                        .text_size(px(TEXT_DISPLAY))
                        .font_weight(FontWeight::BOLD)
                        .text_color(t.text)
                        .line_height(px(TEXT_DISPLAY))
                        .child(self.value),
                )
                .when(!self.series.is_empty(), |s| {
                    s.child(sparkline(self.series.clone(), line_color).size(px(120.), px(44.)))
                }),
        );
        if let Some((up, label)) = self.delta {
            el = el.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(delta_pill(up, label, cx))
                    .child(
                        div()
                            .text_size(px(TEXT_CAPTION))
                            .text_color(t.text_tertiary)
                            .child("vs last week"),
                    ),
            );
        }
        el
    }
}
