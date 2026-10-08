//! Sidebar: brand mark, workspace header, collapsible groups, badges, footer.
use crate::components::avatar::{AvatarSize, avatar};
use crate::components::controls::{count_badge, icon_button, sim_chip};
use crate::fixture::CHANNELS;
use crate::icons::{Icon, icon};
use crate::state::{Section, Ui};
use crate::theme::{TEXT_CAPTION, TEXT_LABEL, theme};
use gpui::prelude::{FluentBuilder, InteractiveElement, StatefulInteractiveElement};
use gpui::{App, FontWeight, IntoElement, ParentElement, SharedString, Styled, div, px};

/// The four-square Benk mark, drawn from primitives (like the reference glyph).
fn brand_mark(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let cell = |shade: gpui::Rgba| {
        div()
            .size(px(9.))
            .rounded(px(3.))
            .bg(shade)
            .border_1()
            .border_color(t.border_strong)
    };
    div()
        .id("brand")
        .flex()
        .flex_col()
        .gap(px(2.))
        .child(
            div()
                .flex()
                .gap(px(2.))
                .child(cell(t.accent_soft))
                .child(cell(t.surface)),
        )
        .child(
            div()
                .flex()
                .gap(px(2.))
                .child(cell(t.surface))
                .child(cell(t.accent_soft)),
        )
}

fn nav_item(
    id: &'static str,
    icon_: Icon,
    label: &'static str,
    target: Section,
    active: bool,
    badge: Option<(u32, crate::theme::Tone)>,
    cx: &App,
) -> impl IntoElement {
    let t = theme(cx);
    let fg = if active { t.text } else { t.text_secondary };
    let mut row = div()
        .id(SharedString::from(id))
        .flex()
        .items_center()
        .gap_3()
        .h(px(34.))
        .px_3()
        .mx_2()
        .rounded(px(crate::theme::R_MD))
        .cursor_pointer()
        .text_color(fg)
        .text_size(px(TEXT_LABEL))
        .font_weight(if active {
            FontWeight::SEMIBOLD
        } else {
            FontWeight::MEDIUM
        })
        .child(icon(icon_).size(px(17.)))
        .child(
            div()
                .flex_1()
                .overflow_hidden()
                .whitespace_nowrap()
                .child(label),
        );
    if active {
        row = row.bg(t.surface).shadow(t.raise_shadow());
    } else {
        row = row.hover(|s| s.bg(t.hover));
    }
    if let Some((count, tone)) = badge {
        row = row.child(count_badge(count, tone));
    }
    row.on_click(move |_, _, cx| Ui::show(target, cx))
}

/// A group header row ("Channels ▾") that toggles expansion.
fn group_header(
    id: &'static str,
    label: &'static str,
    expanded: bool,
    cx: &App,
) -> impl IntoElement {
    let t = theme(cx);
    div()
        .id(SharedString::from(id))
        .flex()
        .items_center()
        .gap_3()
        .h(px(34.))
        .px_3()
        .mx_2()
        .rounded(px(crate::theme::R_MD))
        .cursor_pointer()
        .text_color(t.text)
        .text_size(px(TEXT_LABEL))
        .font_weight(FontWeight::MEDIUM)
        .child(
            icon(Icon::Folder)
                .size(px(17.))
                .text_color(t.text_secondary),
        )
        .child(div().flex_1().child(label))
        .child(
            icon(if expanded {
                Icon::ChevronDown
            } else {
                Icon::ChevronRight
            })
            .size(px(14.))
            .text_color(t.text_tertiary),
        )
        .on_click(move |_, _, cx| {
            cx.global_mut::<Ui>().channels_expanded = !cx.global::<Ui>().channels_expanded;
            cx.refresh_windows();
        })
}

fn channel_row(name: &'static str, unread: u32, active: bool, cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let fg = if active { t.text } else { t.text_secondary };
    div()
        .id(SharedString::from(format!("ch-{name}")))
        .flex()
        .items_center()
        .gap_2()
        .h(px(30.))
        .pl(px(34.))
        .pr_3()
        .mx_2()
        .rounded(px(crate::theme::R_MD))
        .cursor_pointer()
        .text_color(fg)
        .text_size(px(TEXT_LABEL))
        .font_weight(if active {
            FontWeight::SEMIBOLD
        } else {
            FontWeight::MEDIUM
        })
        .child(icon(Icon::Hash).size(px(14.)).text_color(t.text_tertiary))
        .child(div().flex_1().child(name))
        .when(unread > 0, |s| s.child(count_badge(unread, t.tone_mint)))
        .when(active, |s| s.bg(t.surface).shadow(t.raise_shadow()))
        .when(!active, |s| s.hover(|s| s.bg(t.hover)))
        .on_click(move |_, _, cx| {
            Ui::select_channel(name, cx);
        })
}

pub fn sidebar(cx: &App) -> impl IntoElement {
    let t = theme(cx);
    let ui = cx.global::<Ui>();
    let expanded = ui.channels_expanded;
    let section = ui.section;
    let channel = ui.channel;

    let mut groups = div().flex().flex_col().gap_px().mt_2();

    // Inbox — decisions queue.
    let inbox_badge = (crate::fixture::INBOX_ITEMS.len() as u32 > 0)
        .then(|| (crate::fixture::INBOX_ITEMS.len() as u32, t.tone_peach));
    groups = groups.child(nav_item(
        "nav-inbox",
        Icon::Inbox,
        "Inbox",
        Section::Inbox,
        section == Section::Inbox,
        inbox_badge,
        cx,
    ));

    // Channels group with children.
    groups = groups.child(group_header("grp-channels", "Channels", expanded, cx));
    if expanded {
        let mut list = div().flex().flex_col().gap_px();
        for ch in CHANNELS {
            let active = section == Section::Channels && channel == ch.name;
            let unread = ui.unread.get(ch.name).copied().unwrap_or(ch.unread);
            list = list.child(channel_row(ch.name, unread, active, cx));
        }
        groups = groups.child(list);
    }

    groups = groups
        .child(nav_item(
            "nav-projects",
            Icon::Folder,
            "Projects",
            Section::Projects,
            section == Section::Projects,
            None,
            cx,
        ))
        .child(nav_item(
            "nav-work",
            Icon::ListChecks,
            "Work",
            Section::Work,
            section == Section::Work,
            Some((1, t.tone_mint)),
            cx,
        ))
        .child(nav_item(
            "nav-search",
            Icon::Search,
            "Search",
            Section::Search,
            section == Section::Search,
            None,
            cx,
        ));

    div()
        .id("sidebar")
        .w(px(244.))
        .h_full()
        .flex_none()
        .flex()
        .flex_col()
        .bg(t.canvas)
        .border_r_1()
        .border_color(t.border)
        .child(
            // Brand + workspace header.
            div()
                .flex()
                .items_center()
                .gap_3()
                .px_5()
                .pt_5()
                .pb_4()
                .child(brand_mark(cx))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(TEXT_LABEL))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(t.text)
                                .child("Benk"),
                        )
                        .child(
                            div()
                                .text_size(px(TEXT_CAPTION))
                                .text_color(t.text_tertiary)
                                .child("ws_alpha · local"),
                        ),
                ),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .overflow_y_hidden()
                .child(groups),
        )
        .child(
            // Footer: identity, connection, theme toggle.
            div()
                .flex()
                .items_center()
                .gap_2()
                .px_4()
                .py_3()
                .border_t_1()
                .border_color(t.border)
                .child(avatar("You", AvatarSize::Sm, cx))
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(TEXT_CAPTION))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(t.text)
                                .child("you · local"),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(div().size(px(6.)).rounded_full().bg(t.success))
                                .child(
                                    div()
                                        .text_size(px(crate::theme::TEXT_MICRO))
                                        .text_color(t.text_tertiary)
                                        .child("offline fixture"),
                                ),
                        ),
                )
                .child(
                    icon_button(
                        if t.is_dark() { Icon::Sun } else { Icon::Moon },
                        "Toggle theme",
                    )
                    .on_press(|_, _, cx| {
                        let next = if cx.global::<crate::theme::Theme>().is_dark() {
                            crate::theme::Theme::light()
                        } else {
                            crate::theme::Theme::dark()
                        };
                        cx.set_global(next);
                        cx.refresh_windows();
                    }),
                )
                .child(sim_chip(cx)),
        )
}
