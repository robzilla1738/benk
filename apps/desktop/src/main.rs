//! Benk desktop shell — native GPUI application entry point.
//!
//! Owns the Application, embedded assets, theme, keymap, and the single
//! window. All state lives in the `Ui` global (`state.rs`); views read it.
mod app;
mod assets;
mod components;
mod composer;
mod fixture;
mod icons;
mod nav;
mod state;
mod theme;
mod views;

use app::BenkView;
use composer as composer_actions;
use gpui::{
    App, Application, Bounds, KeyBinding, TitlebarOptions, Window, WindowBounds, WindowOptions,
    actions, point, prelude::*, px, size,
};
use state::{Section, Ui};
use theme::Theme;

actions!(
    benk,
    [
        Quit,
        ToggleTheme,
        Compose,
        SearchNav,
        GoInbox,
        GoChannels,
        GoProjects,
        GoWork
    ]
);

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn toggle_theme(_: &ToggleTheme, cx: &mut App) {
    let next = match theme::theme(cx).mode {
        theme::ThemeMode::Light => theme::ThemeMode::Dark,
        theme::ThemeMode::Dark => theme::ThemeMode::Light,
    };
    cx.set_global(Theme::for_mode(next));
    cx.refresh_windows();
}

fn go(section: Section, cx: &mut App) {
    cx.global_mut::<Ui>().section = section;
    cx.refresh_windows();
}

/// Run `f` against the first open window (the app has exactly one), deferred
/// until the current dispatch finishes — during event dispatch the window is
/// checked out of `cx.windows` for reentrancy.
fn deferred_window_update(cx: &mut App, f: impl FnOnce(&mut Window, &mut App) + 'static) {
    let Some(handle) = cx.windows().first().copied() else {
        return;
    };
    cx.defer(move |cx| {
        if let Err(error) = handle.update(cx, |_, window, cx| f(window, cx)) {
            eprintln!("deferred_window_update: {error}");
        }
    });
}

fn compose(_: &Compose, cx: &mut App) {
    cx.global_mut::<Ui>().section = Section::Channels;
    cx.refresh_windows();
    deferred_window_update(cx, BenkView::focus_composer);
}

fn search_nav(_: &SearchNav, cx: &mut App) {
    deferred_window_update(cx, BenkView::focus_search);
}

/// Keymap and action handlers — shared by `main` and the headless tests.
fn install(cx: &mut App) {
    use composer_actions::*;
    cx.bind_keys([
        // Window-level shortcuts (any context).
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-shift-l", ToggleTheme, None),
        KeyBinding::new("cmd-k", SearchNav, None),
        KeyBinding::new("cmd-shift-c", Compose, None),
        KeyBinding::new("cmd-1", GoInbox, None),
        KeyBinding::new("cmd-2", GoChannels, None),
        KeyBinding::new("cmd-3", GoProjects, None),
        KeyBinding::new("cmd-4", GoWork, None),
    ]);
    // Composer editing keys — active only while a Composer is focused.
    cx.bind_keys([
        KeyBinding::new("enter", Enter, Some("Composer")),
        KeyBinding::new("shift-enter", InsertNewline, Some("Composer")),
        KeyBinding::new("backspace", Backspace, Some("Composer")),
        KeyBinding::new("delete", Delete, Some("Composer")),
        KeyBinding::new("left", MoveLeft, Some("Composer")),
        KeyBinding::new("right", MoveRight, Some("Composer")),
        KeyBinding::new("up", MoveUp, Some("Composer")),
        KeyBinding::new("down", MoveDown, Some("Composer")),
        KeyBinding::new("alt-left", MoveWordLeft, Some("Composer")),
        KeyBinding::new("alt-right", MoveWordRight, Some("Composer")),
        KeyBinding::new("shift-left", SelectLeft, Some("Composer")),
        KeyBinding::new("shift-right", SelectRight, Some("Composer")),
        KeyBinding::new("shift-up", SelectUp, Some("Composer")),
        KeyBinding::new("shift-down", SelectDown, Some("Composer")),
        KeyBinding::new("alt-shift-left", SelectWordLeft, Some("Composer")),
        KeyBinding::new("alt-shift-right", SelectWordRight, Some("Composer")),
        KeyBinding::new("home", Home, Some("Composer")),
        KeyBinding::new("end", End, Some("Composer")),
        KeyBinding::new("cmd-left", Home, Some("Composer")),
        KeyBinding::new("cmd-right", End, Some("Composer")),
        KeyBinding::new("cmd-up", DocStart, Some("Composer")),
        KeyBinding::new("cmd-down", DocEnd, Some("Composer")),
        KeyBinding::new("cmd-shift-left", SelectToLineStart, Some("Composer")),
        KeyBinding::new("cmd-shift-right", SelectToLineEnd, Some("Composer")),
        KeyBinding::new("cmd-shift-up", SelectToDocStart, Some("Composer")),
        KeyBinding::new("cmd-shift-down", SelectToDocEnd, Some("Composer")),
        KeyBinding::new("cmd-a", SelectAll, Some("Composer")),
        KeyBinding::new("cmd-x", Cut, Some("Composer")),
        KeyBinding::new("cmd-c", Copy, Some("Composer")),
        KeyBinding::new("cmd-v", Paste, Some("Composer")),
        KeyBinding::new("cmd-z", Undo, Some("Composer")),
        KeyBinding::new("cmd-shift-z", Redo, Some("Composer")),
    ]);
    cx.on_action(quit);
    cx.on_action(toggle_theme);
    cx.on_action(compose);
    cx.on_action(search_nav);
    cx.on_action(|_: &GoInbox, cx| go(Section::Inbox, cx));
    cx.on_action(|_: &GoChannels, cx| go(Section::Channels, cx));
    cx.on_action(|_: &GoProjects, cx| go(Section::Projects, cx));
    cx.on_action(|_: &GoWork, cx| go(Section::Work, cx));
}

fn main() {
    Application::new()
        .with_assets(assets::EmbeddedAssets)
        .run(|cx: &mut App| {
            cx.set_global(Theme::light());
            let ui = Ui::init(cx);
            cx.set_global(ui);

            install(cx);

            let bounds = Bounds::centered(None, size(px(1240.0), px(800.0)), cx);
            let result = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Benk".into()),
                        appears_transparent: true,
                        traffic_light_position: Some(point(px(14.), px(14.))),
                    }),
                    ..Default::default()
                },
                |_, cx| cx.new(BenkView::new),
            );
            if let Err(error) = result {
                eprintln!("Benk could not open its native window: {error}");
                cx.quit();
                return;
            }
            cx.activate(true);
        });
}

#[cfg(test)]
mod tests {
    //! Headless integration tests. `simulate_input`/`simulate_keystrokes` drive
    //! the same dispatch path as the OS: keymap → actions → the platform input
    //! handler registered by the focused composer element.
    use super::*;
    use gpui::{EntityInputHandler, TestAppContext};

    fn app() -> TestAppContext {
        let cx = TestAppContext::single();
        cx.update(|cx| {
            cx.set_global(Theme::light());
            let ui = Ui::init(cx);
            cx.set_global(ui);
            install(cx);
        });
        cx
    }

    #[test]
    fn nav_shortcuts_and_theme_toggle() {
        let mut app = app();
        let (_view, cx) = app.add_window_view(|_, cx| BenkView::new(cx));

        cx.simulate_keystrokes("cmd-1");
        cx.update(|_, cx| assert_eq!(cx.global::<Ui>().section, Section::Inbox));
        cx.simulate_keystrokes("cmd-3");
        cx.update(|_, cx| assert_eq!(cx.global::<Ui>().section, Section::Projects));
        cx.simulate_keystrokes("cmd-2");
        cx.update(|_, cx| assert_eq!(cx.global::<Ui>().section, Section::Channels));

        cx.simulate_keystrokes("cmd-shift-l");
        cx.update(|_, cx| assert!(theme::theme(cx).is_dark()));
        cx.simulate_keystrokes("cmd-shift-l");
        cx.update(|_, cx| assert!(!theme::theme(cx).is_dark()));
    }

    #[test]
    fn composer_typing_and_submit() {
        let mut app = app();
        let (_view, cx) = app.add_window_view(|_, cx| BenkView::new(cx));

        cx.simulate_keystrokes("cmd-shift-c");
        cx.update(|window, cx| {
            let handle = cx.global::<Ui>().composer.read(cx).focus_handle();
            assert!(handle.is_focused(window));
        });

        cx.simulate_input("hello benk");
        cx.update(|_, cx| {
            assert_eq!(cx.global::<Ui>().composer.read(cx).text(), "hello benk");
            assert_eq!(cx.global::<Ui>().drafts["checkout"], "hello benk");
        });

        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            let ui = cx.global::<Ui>();
            assert_eq!(ui.composer.read(cx).text(), "");
            let messages = &ui.messages["checkout"];
            assert_eq!(messages.len(), 5);
            let sent = messages
                .last()
                .map(|m| (m.author.as_ref(), m.body.as_ref()));
            assert_eq!(sent, Some(("you", "hello benk")));
            assert_eq!(ui.drafts["checkout"], "");
        });
    }

    #[test]
    fn channel_switch_preserves_drafts() {
        let mut app = app();
        let (_view, cx) = app.add_window_view(|_, cx| BenkView::new(cx));

        cx.simulate_keystrokes("cmd-shift-c");
        cx.simulate_input("checkout draft");
        cx.update(|_, cx| Ui::select_channel("general", cx));
        cx.update(|_, cx| {
            assert_eq!(cx.global::<Ui>().composer.read(cx).text(), "");
            assert_eq!(cx.global::<Ui>().unread["general"], 0);
        });

        cx.simulate_input("general note");
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            let ui = cx.global::<Ui>();
            let sent = ui.messages["general"].last().map(|m| m.body.as_ref());
            assert_eq!(sent, Some("general note"));
            assert_eq!(ui.messages["checkout"].len(), 4);
        });

        cx.update(|_, cx| Ui::select_channel("checkout", cx));
        cx.update(|_, cx| {
            assert_eq!(cx.global::<Ui>().composer.read(cx).text(), "checkout draft");
        });
    }

    #[test]
    fn ime_enter_does_not_submit() {
        let mut app = app();
        let (_view, cx) = app.add_window_view(|_, cx| BenkView::new(cx));

        cx.simulate_keystrokes("cmd-shift-c");
        cx.update(|window, cx| {
            let composer = cx.global::<Ui>().composer.clone();
            composer.update(cx, |c, ctx| {
                c.replace_and_mark_text_in_range(None, "ko", None, window, ctx)
            });
        });
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            let ui = cx.global::<Ui>();
            assert_eq!(ui.composer.read(cx).text(), "ko");
            assert_eq!(ui.messages["checkout"].len(), 4);
        });

        cx.update(|window, cx| {
            let composer = cx.global::<Ui>().composer.clone();
            composer.update(cx, |c, ctx| c.unmark_text(window, ctx));
        });
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            assert_eq!(cx.global::<Ui>().messages["checkout"].len(), 5);
        });
    }
}
