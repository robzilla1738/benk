//! Compile-time embedded assets. Icons are Lucide (ISC) — see assets/THIRD_PARTY.md.
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

pub struct EmbeddedAssets;

macro_rules! icon {
    ($name:literal) => {
        (
            concat!("icons/", $name, ".svg"),
            include_bytes!(concat!("../assets/icons/", $name, ".svg")) as &'static [u8],
        )
    };
}

static ICONS: &[(&str, &[u8])] = &[
    icon!("inbox"),
    icon!("hash"),
    icon!("folder"),
    icon!("list-checks"),
    icon!("search"),
    icon!("chevron-down"),
    icon!("chevron-right"),
    icon!("send-horizontal"),
    icon!("sun"),
    icon!("moon"),
    icon!("circle-help"),
    icon!("arrow-up-right"),
    icon!("arrow-down-right"),
    icon!("check"),
    icon!("sparkles"),
    icon!("x"),
    icon!("users"),
    icon!("bell"),
    icon!("file-text"),
    icon!("clock"),
    icon!("shield-check"),
    icon!("lock"),
    icon!("circle-dot"),
];

impl AssetSource for EmbeddedAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(ICONS
            .iter()
            .find(|(p, _)| *p == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(p, _)| p.starts_with(path))
            .map(|(p, _)| SharedString::from(*p))
            .collect())
    }
}
