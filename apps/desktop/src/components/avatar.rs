//! Initials avatar with a deterministic pastel tone per name.
use crate::theme::{Theme, theme};
use gpui::{App, FontWeight, IntoElement, ParentElement, Pixels, Styled, div, px};

#[derive(Clone, Copy)]
pub enum AvatarSize {
    Sm,
    Md,
}

impl AvatarSize {
    fn px(self) -> Pixels {
        px(match self {
            Self::Sm => 22.,
            Self::Md => 30.,
        })
    }
}

/// Pick a soft tone deterministically from the name so humans, the simulated
/// agent and reviewers stay visually distinct without photos.
fn tone_for(name: &str, t: &Theme) -> crate::theme::Tone {
    let tones = [
        t.tone_peach,
        t.tone_mint,
        t.tone_lilac,
        t.tone_blue,
        t.tone_rose,
    ];
    let hash = name
        .bytes()
        .fold(0usize, |acc, b| acc.wrapping_mul(31) + b as usize);
    tones[hash % tones.len()]
}

fn initials(name: &str) -> String {
    name.split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase()
}

pub fn avatar(name: &str, size: AvatarSize, cx: &App) -> impl IntoElement {
    let tone = tone_for(name, theme(cx));
    let dim = size.px();
    let font = px(match size {
        AvatarSize::Sm => 9.,
        AvatarSize::Md => 12.,
    });
    div()
        .size(dim)
        .rounded_full()
        .bg(tone.bg)
        .text_color(tone.fg)
        .text_size(font)
        .font_weight(FontWeight::SEMIBOLD)
        .flex()
        .items_center()
        .justify_center()
        .flex_none()
        .child(initials(name))
}
