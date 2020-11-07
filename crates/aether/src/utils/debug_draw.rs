#![allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]

use crate::{darksiders1::gfc, utils::liang_barsky::liang_barsky};
use na::{Point2, Vector4};

#[allow(dead_code)]
pub fn clunky_draw_line(renderer: &gfc::UIRenderer, p: Point2<f32>, q: Point2<f32>) {
    let viewport = gfc::KGGraphics::get_instance().get_viewport();
    let viewport = viewport.convert(|x| x as f32);
    let (p, q, steps) = match plan_line(&viewport, p, q) {
        Some(plan) => plan,
        None => return,
    };
    for i in 0..steps {
        let step = i as f32;
        let (x, y, w, h) = positive_rect(
            p.x + (q.x - p.x) * (step / steps as f32),
            p.y + (q.y - p.y) * (step / steps as f32),
            (q.x - p.x) / steps as f32,
            (q.y - p.y) / steps as f32,
        );
        renderer.fill_rect(
            x,
            y,
            w.max(1.0),
            h.max(1.0),
            &Vector4::new(0.0, 0.0, 1.0, 1.0),
            &Vector4::new(0.0, 0.0, 1.0, 1.0),
        );
    }
}

fn plan_line(
    viewport: &gfc::TRect<f32>,
    p: Point2<f32>,
    q: Point2<f32>,
) -> Option<(Point2<f32>, Point2<f32>, i32)> {
    let [p, q] = liang_barsky(viewport, p, q)?;

    let mut steps = f32::min((q.x - p.x).abs(), (q.y - p.y).abs()) as i32;
    steps = (steps / 2).max(1);
    if steps > 100 {
        steps = 100;
    }

    Some((p, q, steps))
}

fn positive_rect(x: f32, y: f32, w: f32, h: f32) -> (f32, f32, f32, f32) {
    let (x, w) = if w >= 0.0 { (x, w) } else { (x + w, -w) };
    let (y, h) = if h >= 0.0 { (y, h) } else { (y + h, -h) };
    (x, y, w, h)
}
