//! Controls: buttons, icon buttons, count badges, delta pills, state chips.
use crate::icons::{Icon, icon};
use crate::theme::{R_MD, R_PILL, R_SM, TEXT_CAPTION, TEXT_LABEL, TEXT_MICRO, Tone, theme};
use benk_domain::TaskState;
use gpui::prelude::{FluentBuilder, InteractiveElement, StatefulInteractiveElement};
use gpui::{
    App, ClickEvent, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};
use std::rc::Rc;

pub type OnPress = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Primary,
    Secondary,
}

#[derive(IntoElement)]
pub struct Button {
    label: SharedString,
    icon: Option<Icon>,
    kind: ButtonKind,
    disabled: bool,
    on_press: Option<OnPress>,
}

pub fn button(label: impl Into<SharedString>) -> Button {
    Button {
        label: label.into(),
        icon: None,
        kind: ButtonKind::Secondary,
        disabled: false,
        on_press: None,
    }
}

impl Button {
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn on_press(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_press = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let (bg, fg, border, hover_bg) = match self.kind {
            ButtonKind::Primary => (t.accent, t.on_accent, t.accent, t.accent_hover),
            ButtonKind::Secondary => (t.surface, t.text, t.border_strong, t.hover),
        };
        let mut el = div()
            .id(SharedString::from(format!("btn-{}", self.label)))
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .h(px(34.))
            .px_4()
            .rounded(px(R_MD))
            .bg(bg)
            .text_color(fg)
            .text_size(px(TEXT_LABEL))
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
            .when(self.kind == ButtonKind::Primary, |s| {
                s.shadow(t.raise_shadow())
            })
            .when(self.kind == ButtonKind::Secondary, |s| {
                s.border_1().border_color(border)
            })
            .when(self.disabled, |s| s.opacity(0.45));
        if let Some(ic) = self.icon {
            el = el.child(icon(ic).size(px(15.)));
        }
        el = el.child(self.label);
        if !self.disabled
            && let Some(on_press) = self.on_press
        {
            el = el.on_click(move |event, window, cx| on_press(event, window, cx));
        }
        el
    }
}

#[derive(IntoElement)]
pub struct IconButton {
    icon: Icon,
    tooltip: &'static str,
    on_press: Option<OnPress>,
}

pub fn icon_button(icon: Icon, tooltip: &'static str) -> IconButton {
    IconButton {
        icon,
        tooltip,
        on_press: None,
    }
}

impl IconButton {
    pub fn on_press(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_press = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let t = theme(cx);
        let mut el = div()
            .id(SharedString::from(self.tooltip))
            .size(px(30.))
            .rounded(px(R_SM))
            .flex()
            .items_center()
            .justify_center()
            .text_color(t.text_secondary)
            .cursor_pointer()
            .hover(move |s| s.bg(t.hover).text_color(t.text))
            .child(icon(self.icon).size(px(16.)));
        if let Some(on_press) = self.on_press {
            el = el.on_click(move |event, window, cx| on_press(event, window, cx));
        }
        el
    }
}

/// Rounded count badge in a semantic tone — the peach "3" / mint "8" pattern.
pub fn count_badge(count: u32, tone: Tone) -> gpui::Div {
    div()
        .min_w(px(20.))
        .h(px(20.))
        .px(px(6.))
        .rounded(px(R_SM))
        .bg(tone.bg)
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(TEXT_MICRO))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(tone.fg)
        .child(count.to_string())
}

/// Delta pill: bordered chip with an arrow and a percentage.
pub fn delta_pill(up: bool, label: impl Into<SharedString>, cx: &App) -> gpui::Div {
    let t = theme(cx);
    let fg = if up { t.success } else { t.danger };
    div()
        .flex()
        .items_center()
        .gap_1()
        .h(px(24.))
        .px(px(8.))
        .rounded(px(R_SM))
        .border_1()
        .border_color(fg)
        .text_color(fg)
        .text_size(px(TEXT_LABEL))
        .font_weight(FontWeight::MEDIUM)
        .child(
            icon(if up {
                Icon::ArrowUpRight
            } else {
                Icon::ArrowDownRight
            })
            .size(px(12.)),
        )
        .child(label.into())
}

/// Task-state chip: icon + text label on a tone (never color alone).
pub fn state_chip(state: TaskState, cx: &App) -> gpui::Div {
    let t = theme(cx);
    let (ic, tone) = match state {
        TaskState::Draft => (Icon::FileText, t.tone_neutral),
        TaskState::Ready => (Icon::CircleDot, t.tone_blue),
        TaskState::Active => (Icon::Sparkles, t.tone_lilac),
        TaskState::Blocked => (Icon::Lock, t.tone_rose),
        TaskState::AwaitingReview => (Icon::Clock, t.tone_amber),
        TaskState::Accepted => (Icon::Check, t.tone_mint),
        TaskState::Cancelled => (Icon::X, t.tone_neutral),
    };
    div()
        .flex()
        .items_center()
        .gap(px(5.))
        .h(px(24.))
        .px_2()
        .rounded(px(R_PILL))
        .bg(tone.bg)
        .text_color(tone.fg)
        .text_size(px(TEXT_CAPTION))
        .font_weight(FontWeight::MEDIUM)
        .child(icon(ic).size(px(12.)))
        .child(state.as_str())
}

/// Persistent honesty marker for simulated surfaces.
pub fn sim_chip(cx: &App) -> gpui::Div {
    let t = theme(cx);
    div()
        .flex()
        .items_center()
        .gap_1()
        .h(px(22.))
        .px_2()
        .rounded(px(R_PILL))
        .bg(t.sim_bg)
        .border_1()
        .border_color(t.sim_border)
        .text_color(t.sim_text)
        .text_size(px(TEXT_MICRO))
        .font_weight(FontWeight::SEMIBOLD)
        .child(icon(Icon::Lock).size(px(10.)))
        .child("SIMULATION")
}
