//! Surfaces: cards, dividers, section headers.
use crate::theme::{R_LG, TEXT_HEADING, theme};
use gpui::{App, Div, IntoElement, ParentElement, RenderOnce, Styled, div, px};

#[derive(IntoElement)]
pub struct Card {
    children: Vec<gpui::AnyElement>,
    pad: f32,
    radius: f32,
    gap: f32,
}

/// The signature raised squircle surface.
pub fn card() -> Card {
    Card {
        children: vec![],
        pad: 22.0,
        radius: R_LG,
        gap: 10.0,
    }
}

impl Card {
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }

    pub fn pad(mut self, pad: f32) -> Self {
        self.pad = pad;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut gpui::Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        div()
            .flex()
            .flex_col()
            .gap(px(self.gap))
            .p(px(self.pad))
            .bg(t.surface)
            .rounded(px(self.radius))
            .border_1()
            .border_color(t.border)
            .shadow(t.card_shadow())
            .children(self.children)
    }
}

/// Card title row: heading on the left, optional quiet trailing controls.
pub fn card_header(title: &'static str, cx: &App) -> Div {
    let t = theme(cx);
    div().flex().items_center().justify_between().child(
        div()
            .text_size(px(TEXT_HEADING))
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .text_color(t.text)
            .child(title),
    )
}

pub fn divider(cx: &App) -> Div {
    div().h(px(1.)).w_full().bg(theme(cx).border)
}

/// Small muted helper row, e.g. a trailing "?".
pub fn help_dot(cx: &App) -> Div {
    let t = theme(cx);
    div()
        .size(px(16.))
        .rounded_full()
        .border_1()
        .border_color(t.border_strong)
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(10.))
        .text_color(t.text_tertiary)
        .child("?")
}
