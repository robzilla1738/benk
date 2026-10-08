//! Typed icon set over embedded Lucide SVGs. GPUI paints SVGs as alpha masks
//! tinted by `text_color`, so every icon follows the active theme.
use gpui::{Hsla, IntoElement, Pixels, RenderOnce, Styled, svg};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    ArrowDownRight,
    ArrowUpRight,
    Bell,
    Check,
    ChevronDown,
    ChevronRight,
    CircleDot,
    CircleHelp,
    Clock,
    FileText,
    Folder,
    Hash,
    Inbox,
    ListChecks,
    Lock,
    Moon,
    Search,
    Send,
    ShieldCheck,
    Sparkles,
    Sun,
    Users,
    X,
}

impl Icon {
    pub fn path(self) -> &'static str {
        match self {
            Self::ArrowDownRight => "icons/arrow-down-right.svg",
            Self::ArrowUpRight => "icons/arrow-up-right.svg",
            Self::Bell => "icons/bell.svg",
            Self::Check => "icons/check.svg",
            Self::ChevronDown => "icons/chevron-down.svg",
            Self::ChevronRight => "icons/chevron-right.svg",
            Self::CircleDot => "icons/circle-dot.svg",
            Self::CircleHelp => "icons/circle-help.svg",
            Self::Clock => "icons/clock.svg",
            Self::FileText => "icons/file-text.svg",
            Self::Folder => "icons/folder.svg",
            Self::Hash => "icons/hash.svg",
            Self::Inbox => "icons/inbox.svg",
            Self::ListChecks => "icons/list-checks.svg",
            Self::Lock => "icons/lock.svg",
            Self::Moon => "icons/moon.svg",
            Self::Search => "icons/search.svg",
            Self::Send => "icons/send-horizontal.svg",
            Self::ShieldCheck => "icons/shield-check.svg",
            Self::Sparkles => "icons/sparkles.svg",
            Self::Sun => "icons/sun.svg",
            Self::Users => "icons/users.svg",
            Self::X => "icons/x.svg",
        }
    }
}

#[derive(IntoElement)]
pub struct IconElement {
    icon: Icon,
    size: Pixels,
    color: Option<Hsla>,
}

impl IconElement {
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            size: gpui::px(18.),
            color: None,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// Explicit tint. When unset, the SVG inherits the ambient text color,
    /// which is what makes theme switching work through the cascade.
    pub fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl RenderOnce for IconElement {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        let el = svg().path(self.icon.path()).size(self.size).flex_none();
        match self.color {
            Some(color) => el.text_color(color),
            None => el,
        }
    }
}

pub fn icon(icon: Icon) -> IconElement {
    IconElement::new(icon)
}
