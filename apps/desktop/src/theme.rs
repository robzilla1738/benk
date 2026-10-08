//! Design tokens: color, type, spacing, elevation. One `Theme` global with
//! light and dark palettes; every component reads `cx.global::<Theme>()`.
use gpui::{BoxShadow, Global, Hsla, Rgba, point, px, rgb, rgba};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    Light,
    Dark,
}

/// A paired fill/foreground for badges, chips and pills. `bg` is a soft tint,
/// `fg` is readable text on it.
#[derive(Clone, Copy)]
pub struct Tone {
    pub bg: Rgba,
    pub fg: Rgba,
}

#[derive(Clone)]
pub struct Theme {
    pub mode: ThemeMode,
    /// App canvas.
    pub canvas: Rgba,
    /// Raised surfaces: cards, popovers, the selected nav pill.
    pub surface: Rgba,
    /// Subtle secondary surface: stat icon wells, code chips.
    pub well: Rgba,
    /// Hovered rows and controls on the canvas.
    pub hover: Rgba,
    /// Inset field background (composer).
    pub inset: Rgba,

    pub text: Rgba,
    pub text_secondary: Rgba,
    pub text_tertiary: Rgba,
    /// Text on `accent` fills.
    pub on_accent: Rgba,

    pub border: Rgba,
    pub border_strong: Rgba,

    /// Primary action fill — near-black in light, near-white in dark.
    pub accent: Rgba,
    /// Primary action hover.
    pub accent_hover: Rgba,
    /// Neutral tinted hover fill.
    pub accent_soft: Rgba,

    pub success: Rgba,
    pub danger: Rgba,

    /// Semantic badge tones.
    pub tone_neutral: Tone,
    pub tone_peach: Tone,
    pub tone_mint: Tone,
    pub tone_lilac: Tone,
    pub tone_blue: Tone,
    pub tone_rose: Tone,
    pub tone_amber: Tone,

    /// Text selection and text caret in editable surfaces.
    pub selection: Rgba,
    pub caret: Rgba,
    /// Keyboard focus ring.
    pub focus_ring: Rgba,
    /// Hairline and soft elevation color.
    pub shadow: Rgba,

    /// Honest "simulated" markers — visible but quiet.
    pub sim_bg: Rgba,
    pub sim_border: Rgba,
    pub sim_text: Rgba,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            canvas: rgb(0xf5f4f1),
            surface: rgb(0xffffff),
            well: rgb(0xf0efeb),
            hover: rgb(0xeceae5),
            inset: rgb(0xf3f2ef),
            text: rgb(0x1a1b1e),
            text_secondary: rgb(0x5f646b),
            text_tertiary: rgb(0x9a9ea3),
            on_accent: rgb(0xffffff),
            border: rgb(0xe9e7e3),
            border_strong: rgb(0xd8d5cf),
            accent: rgb(0x1a1b1e),
            accent_hover: rgb(0x33353b),
            accent_soft: rgb(0xe7e5e0),
            success: rgb(0x1f9d66),
            danger: rgb(0xd44c3c),
            tone_neutral: Tone {
                bg: rgb(0xefeeea),
                fg: rgb(0x5f646b),
            },
            tone_peach: Tone {
                bg: rgb(0xffddc6),
                fg: rgb(0x8f4c18),
            },
            tone_mint: Tone {
                bg: rgb(0xd2f0de),
                fg: rgb(0x1c7a45),
            },
            tone_lilac: Tone {
                bg: rgb(0xe7dff9),
                fg: rgb(0x6447ae),
            },
            tone_blue: Tone {
                bg: rgb(0xdceafb),
                fg: rgb(0x2b5ea7),
            },
            tone_rose: Tone {
                bg: rgb(0xfbdde3),
                fg: rgb(0xb03a5b),
            },
            tone_amber: Tone {
                bg: rgb(0xfceccb),
                fg: rgb(0x86621a),
            },
            selection: rgba(0x1a1b1e22),
            caret: rgb(0x1a1b1e),
            focus_ring: rgba(0x4c8dff66),
            shadow: rgba(0x1a1b1e14),
            sim_bg: rgb(0xfbf1da),
            sim_border: rgb(0xeaddb6),
            sim_text: rgb(0x7a5b14),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            canvas: rgb(0x15171a),
            surface: rgb(0x1e2125),
            well: rgb(0x262a2f),
            hover: rgb(0x282c32),
            inset: rgb(0x191c20),
            text: rgb(0xedeef0),
            text_secondary: rgb(0xa3a9b0),
            text_tertiary: rgb(0x6e757d),
            on_accent: rgb(0x15171a),
            border: rgb(0x2b2f35),
            border_strong: rgb(0x3a3f47),
            accent: rgb(0xedeef0),
            accent_hover: rgb(0xffffff),
            accent_soft: rgb(0x2a2e35),
            success: rgb(0x4cc38a),
            danger: rgb(0xf06a6a),
            tone_neutral: Tone {
                bg: rgba(0xa3a9b024),
                fg: rgb(0xa3a9b0),
            },
            tone_peach: Tone {
                bg: rgba(0xf5a8772e),
                fg: rgb(0xf5a877),
            },
            tone_mint: Tone {
                bg: rgba(0x4cc38a28),
                fg: rgb(0x63d197),
            },
            tone_lilac: Tone {
                bg: rgba(0xb79ff52b),
                fg: rgb(0xb79ff5),
            },
            tone_blue: Tone {
                bg: rgba(0x5c9bfa2e),
                fg: rgb(0x7fb0fb),
            },
            tone_rose: Tone {
                bg: rgba(0xf188a02e),
                fg: rgb(0xf188a0),
            },
            tone_amber: Tone {
                bg: rgba(0xe3b9532b),
                fg: rgb(0xe3b953),
            },
            selection: rgba(0xedeef02e),
            caret: rgb(0xedeef0),
            focus_ring: rgba(0x7fb0fb80),
            shadow: rgba(0x00000066),
            sim_bg: rgb(0x2e281a),
            sim_border: rgb(0x54471f),
            sim_text: rgb(0xe0b95c),
        }
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }

    /// The soft card elevation of the visual language: gentle lift in light,
    /// deeper ambient shadow in dark.
    pub fn card_shadow(&self) -> Vec<BoxShadow> {
        let color: Hsla = self.shadow.into();
        vec![BoxShadow {
            color,
            offset: point(px(0.), px(1.)),
            blur_radius: px(if self.is_dark() { 8. } else { 14. }),
            spread_radius: px(0.),
        }]
    }

    /// Barely-there lift used by pills and raised controls.
    pub fn raise_shadow(&self) -> Vec<BoxShadow> {
        let color: Hsla = self.shadow.into();
        vec![BoxShadow {
            color,
            offset: point(px(0.), px(1.)),
            blur_radius: px(5.),
            spread_radius: px(0.),
        }]
    }
}

impl Global for Theme {}

// Type scale (px) — system font only; no bundled commercial fonts.
pub const TEXT_DISPLAY: f32 = 28.0;
pub const TEXT_TITLE: f32 = 20.0;
pub const TEXT_HEADING: f32 = 15.0;
pub const TEXT_BODY: f32 = 14.5;
pub const TEXT_LABEL: f32 = 13.0;
pub const TEXT_CAPTION: f32 = 12.0;
pub const TEXT_MICRO: f32 = 11.0;

// Radii (px) — the squircle-adjacent language of the reference design.
pub const R_SM: f32 = 8.0;
pub const R_MD: f32 = 12.0;
pub const R_LG: f32 = 16.0;
pub const R_PILL: f32 = 999.0;

/// Current theme, shorthand for `cx.global::<Theme>()`.
pub fn theme(cx: &gpui::App) -> &Theme {
    cx.global::<Theme>()
}
