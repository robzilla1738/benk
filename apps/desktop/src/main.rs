use benk_domain::{TaskState, transition_task};
use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
struct Benk {
    section: usize,
    task: TaskState,
    feedback: SharedString,
}
impl Benk {
    fn advance(&mut self, cx: &mut Context<Self>) {
        let to = match self.task {
            TaskState::Draft => TaskState::Ready,
            TaskState::Ready => TaskState::Active,
            TaskState::Active => TaskState::AwaitingReview,
            _ => return,
        };
        if let Ok(next) = transition_task(self.task, to) {
            self.task = next;
            self.feedback = "Simulation advanced. No external action performed.".into();
        }
        cx.notify();
    }
    fn review(&mut self, cx: &mut Context<Self>) {
        if self.task == TaskState::AwaitingReview {
            if let Ok(next) = transition_task(self.task, TaskState::Accepted) {
                self.task = next;
            }
            self.feedback =
                "Accepted by a simulated second human. Not an authenticated approval.".into();
        } else {
            self.feedback = "A review package is not ready yet.".into();
        }
        cx.notify();
    }
}
impl Render for Benk {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut nav = div()
            .flex()
            .flex_col()
            .gap_2()
            .w(px(220.0))
            .h_full()
            .p_5()
            .bg(rgb(0x121920))
            .child(div().text_xl().child("Benk"))
            .child(div().text_sm().child("Humans + agents, together"));
        for (index, label) in ["Inbox", "Channels", "Projects", "Work"]
            .into_iter()
            .enumerate()
        {
            nav = nav.child(
                div()
                    .id(SharedString::from(format!("nav-{index}")))
                    .p_3()
                    .rounded_md()
                    .bg(rgb(if self.section == index {
                        0x253540
                    } else {
                        0x121920
                    }))
                    .cursor_pointer()
                    .child(label)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.section = index;
                        cx.notify();
                    })),
            );
        }
        let title = ["Inbox", "Channels", "Projects", "Work"][self.section];
        let mut content=div().flex().flex_col().gap_4().flex_1().h_full().p_6()
            .child(div().text_xl().child(title))
            .child(div().p_3().rounded_md().bg(rgb(0x3a3021)).text_color(rgb(0xf5d399))
                .child("FOUNDATION PREVIEW — synthetic data; no models, tools or authenticated users"));
        content=match self.section {
            0=>content.child("Your review queue").child(format!("Checkout wording: {}",self.task.as_str()))
                .child("Live notifications are not implemented."),
            1=>content.child("# checkout")
                .child("Requester: Customers cannot tell how to recover after a payment error.")
                .child("Teammate: Prepare clearer wording and a reviewable result.")
                .child("Agent: I can prepare a bounded simulation. No production access.")
                .child(div().p_4().border_1().border_color(rgb(0x34424f)).rounded_md()
                    .child("Composer not implemented. IME, selection, accessibility and drafts are next.")),
            2=>content.child("Checkout experience")
                .child("Scope: synthetic checkout task only")
                .child("Repositories: none | Model destination: none | Execution: disabled"),
            _=>content.child("Improve the checkout error")
                .child(format!("Task: {}",self.task.as_str()))
                .child("Contract: synthetic wording proposal; no external capabilities.")
                .child(div().flex().gap_3()
                    .child(div().id("advance").p_3().rounded_md().bg(rgb(0x254c50)).cursor_pointer()
                        .child("Advance simulation").on_click(cx.listener(|this,_,_,cx|this.advance(cx))))
                    .child(div().id("review").p_3().rounded_md().bg(rgb(0x33445b)).cursor_pointer()
                        .child("Review as simulated teammate").on_click(cx.listener(|this,_,_,cx|this.review(cx)))))
                .child(if matches!(self.task,TaskState::AwaitingReview|TaskState::Accepted) {
                    "Review package: synthetic wording ready. No repository changed or real tests executed."
                } else { "Advance through ready and active to produce the synthetic review package." })
                .child(self.feedback.clone()),
        };
        div()
            .flex()
            .size_full()
            .bg(rgb(0x19222c))
            .text_color(rgb(0xe9edf0))
            .child(nav)
            .child(content)
    }
}
fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1160.0), px(760.0)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| Benk {
                    section: 3,
                    task: TaskState::Draft,
                    feedback: "Safe simulation ready.".into(),
                })
            },
        );
        if let Err(error) = result {
            eprintln!("Benk could not open its native window: {error}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
