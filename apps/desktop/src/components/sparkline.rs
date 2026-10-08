//! Canvas-painted sparkline: smoothed stroke + translucent area fill.
use gpui::{
    App, Bounds, Hsla, IntoElement, PathBuilder, Pixels, Point, RenderOnce, Styled, Window, canvas,
    point, px,
};

#[derive(IntoElement)]
pub struct Sparkline {
    /// Normalized samples; will be scaled into the element's bounds.
    points: Vec<f32>,
    color: Hsla,
    width: Pixels,
    height: Pixels,
}

pub fn sparkline(points: Vec<f32>, color: Hsla) -> Sparkline {
    Sparkline {
        points,
        color,
        width: px(120.),
        height: px(36.),
    }
}

impl Sparkline {
    pub fn size(mut self, width: Pixels, height: Pixels) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

/// Catmull-Rom to cubic segments for a smooth line.
fn smooth_path(points: &[Point<Pixels>], builder: &mut PathBuilder) {
    if points.len() < 2 {
        return;
    }
    builder.move_to(points[0]);
    for i in 0..points.len() - 1 {
        let p0 = points[i.saturating_sub(1)];
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = points[(i + 2).min(points.len() - 1)];
        let c1 = point(p1.x + (p2.x - p0.x) / 6., p1.y + (p2.y - p0.y) / 6.);
        let c2 = point(p2.x - (p3.x - p1.x) / 6., p2.y - (p3.y - p1.y) / 6.);
        builder.cubic_bezier_to(p2, c1, c2);
    }
}

impl RenderOnce for Sparkline {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color;
        let points = self.points;
        canvas(
            move |_bounds, _window, _cx| points.clone(),
            move |bounds: Bounds<Pixels>, points, window, _cx| {
                if points.len() < 2 || points.iter().all(|p| *p == points[0]) {
                    return;
                }
                let (min, max) = points
                    .iter()
                    .fold((f32::MAX, f32::MIN), |(lo, hi), p| (lo.min(*p), hi.max(*p)));
                let span = (max - min).max(f32::EPSILON);
                let n = points.len() as f32;
                let pad = px(2.);
                let usable_w = bounds.size.width - pad * 2.;
                let usable_h = bounds.size.height - pad * 2.;
                let pts: Vec<Point<Pixels>> = points
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        point(
                            bounds.origin.x + pad + usable_w * (i as f32 / (n - 1.)),
                            bounds.origin.y + pad + usable_h * (1. - (*v - min) / span),
                        )
                    })
                    .collect();

                // Area fill.
                let mut fill = PathBuilder::fill();
                smooth_path(&pts, &mut fill);
                fill.line_to(point(
                    pts[pts.len() - 1].x,
                    bounds.origin.y + bounds.size.height,
                ));
                fill.line_to(point(pts[0].x, bounds.origin.y + bounds.size.height));
                fill.close();
                let mut fill_color = color;
                fill_color.a = 0.12;
                if let Ok(path) = fill.build() {
                    window.paint_path(path, fill_color);
                }

                // Stroke.
                let mut stroke = PathBuilder::stroke(px(1.8));
                smooth_path(&pts, &mut stroke);
                if let Ok(path) = stroke.build() {
                    window.paint_path(path, color);
                }

                // End dot.
                if let Some(last) = pts.last() {
                    let dot = gpui::fill(
                        Bounds::new(
                            point(last.x - px(3.), last.y - px(3.)),
                            gpui::size(px(6.), px(6.)),
                        ),
                        color,
                    )
                    .corner_radii(gpui::Corners::all(px(3.)));
                    window.paint_quad(dot);
                }
            },
        )
        .w(self.width)
        .h(self.height)
    }
}
