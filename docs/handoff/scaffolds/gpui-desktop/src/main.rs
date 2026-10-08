//! UNCOMPILED integration scaffold. Validate against the pinned published GPUI release.
//! Static shell only: no authentication, persistence, IPC, agent, or privileged tool access.
use gpui::{div, prelude::*, px, rgb, size, Application, Bounds, Context,
           Window, WindowBounds, WindowOptions};

struct WorkspaceShell;

impl Render for WorkspaceShell {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex().flex_col().size_full().p_6().gap_4()
            .bg(rgb(0x11151b)).text_color(rgb(0xe5e9ef))
            .child(div().text_xl().child("Human–Agent Workspace"))
            .child("Native bootstrap — not a completed application")
            .child("Next: composer, IME, accessibility, local cache, and typed IPC.")
            .child("Real execution is disabled. No model, shell, or cloud calls are made.")
    }
}

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(1120.0), px(760.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| WorkspaceShell),
        ).expect("native window creation failed; inspect platform SDK / GPUI setup");
        cx.activate(true);
    });
}
