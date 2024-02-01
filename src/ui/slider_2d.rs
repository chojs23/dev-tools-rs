use super::slider_1d::NUM_OF_VERTICES;

use crate::color::Color;
use eframe::egui::{epaint::Mesh, lerp, remap_clamp, Sense, Shape, Stroke, Vec2};
use eframe::egui::{pos2, Color32, CursorIcon, Response, Ui};
use eframe::epaint::CircleShape;
use std::ops::RangeInclusive;

pub fn color(
    ui: &mut Ui,
    x_value: &mut f32,
    y_value: &mut f32,
    x_range: RangeInclusive<f32>,
    y_range: RangeInclusive<f32>,
    color_at: impl Fn(f32, f32) -> Color32,
) -> Response {
    let width = ui.spacing().slider_width * 2.;
    let desired_size = Vec2::new(width, width * 2. / 3.);
    let (rect, mut response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

    if let Some(mpos) = response.interact_pointer_pos() {
        *x_value = remap_clamp(mpos.x, rect.left()..=rect.right(), x_range.clone());
        *y_value = remap_clamp(mpos.y, rect.bottom()..=rect.top(), y_range.clone());
    }

    let visuals = ui.style().interact(&response);
    let mut mesh = Mesh::default();

    for xi in 0..=NUM_OF_VERTICES {
        for yi in 0..=NUM_OF_VERTICES {
            let xt = xi as f32 / (NUM_OF_VERTICES as f32);
            let yt = yi as f32 / (NUM_OF_VERTICES as f32);
            let color_x = lerp(x_range.clone(), xt);
            let color_y = lerp(y_range.clone(), yt);
            let color = color_at(color_x, color_y);
            let x = lerp(rect.left()..=rect.right(), xt);
            let y = lerp(rect.bottom()..=rect.top(), yt);
            mesh.colored_vertex(pos2(x, y), color);

            if xi < NUM_OF_VERTICES && yi < NUM_OF_VERTICES {
                let x_offset = 1;
                let y_offset = NUM_OF_VERTICES + 1;
                let tl = yi * y_offset + xi;
                mesh.add_triangle(tl, tl + x_offset, tl + y_offset);
                mesh.add_triangle(tl + x_offset, tl + y_offset, tl + y_offset + x_offset);
            }
        }
    }
    ui.painter().add(Shape::mesh(mesh)); // fill

    ui.painter().rect_stroke(rect, 0.0, visuals.bg_stroke); // outline

    let x = remap_clamp(*x_value, x_range, rect.left()..=rect.right());
    let y = remap_clamp(*y_value, y_range, rect.bottom()..=rect.top());

    let picked_color = Color::Color32(color_at(*x_value, *y_value));

    // Show where the slider is at:
    ui.painter().add(Shape::Circle(CircleShape {
        center: pos2(x, y),
        radius: 7.,
        fill: picked_color.color32(),
        stroke: Stroke::new(visuals.fg_stroke.width, picked_color.contrast().color32()),
    }));

    response = response.on_hover_cursor(CursorIcon::Move);

    response
}
