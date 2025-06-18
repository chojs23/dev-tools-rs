use crate::core::color::Color;
use eframe::egui::{epaint::Mesh, lerp, remap_clamp, Shape, Stroke};
use eframe::egui::{pos2, vec2, Color32, CursorIcon, Response, Sense, StrokeKind, Ui};
use std::ops::{Neg, RangeInclusive};

/// Number of vertices per dimension in the color sliders.
/// We need at least 6 for hues, and more for smooth 2D areas.
/// Should always be a multiple of 6 to hit the peak hues in HSV/HSL (every 60°).
pub const NUM_OF_VERTICES: u32 = 6 * 6;

pub fn color(
    ui: &mut Ui,
    value: &mut f32,
    range: RangeInclusive<f32>,
    color_at: impl Fn(f32) -> Color32,
) -> Response {
    let width = ui.spacing().slider_width * 2.;

    let range_start = *range.start();
    let _range_end = *range.end();

    let range_end = if range_start.is_sign_negative() {
        _range_end + range_start.neg()
    } else {
        _range_end
    };

    let desired_size = vec2(width, ui.spacing().interact_size.y * 2.);
    let (rect, mut response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

    if let Some(mpos) = response.interact_pointer_pos() {
        *value = remap_clamp(mpos.x, rect.left()..=rect.right(), range);
    }

    let visuals = ui.style().interact(&response);

    {
        // fill color:
        let mut mesh = Mesh::default();
        for i in 0..=NUM_OF_VERTICES {
            let pos = i as f32 / (NUM_OF_VERTICES as f32);
            let color_pos = lerp(range_start..=_range_end, pos);
            let color = color_at(color_pos);
            let mesh_pos = lerp(rect.left()..=rect.right(), pos);
            mesh.colored_vertex(pos2(mesh_pos, rect.top()), color);
            mesh.colored_vertex(pos2(mesh_pos, rect.bottom()), color);
            if i < NUM_OF_VERTICES {
                mesh.add_triangle(2 * i, 2 * i + 1, 2 * i + 2);
                mesh.add_triangle(2 * i + 1, 2 * i + 2, 2 * i + 3);
            }
        }
        ui.painter().add(Shape::mesh(mesh));
    }

    ui.painter()
        .rect_stroke(rect, 0.0, visuals.bg_stroke, StrokeKind::Outside);

    {
        let x = *value;
        let picked_color = Color::Color32(color_at(x));
        let x = if range_start.is_sign_negative() {
            x + range_start.neg()
        } else {
            x
        };
        let x = rect.left() + (x / range_end) * width;
        let r = rect.height() / 4.0;

        ui.painter().add(Shape::convex_polygon(
            vec![
                pos2(x - r, rect.bottom()),
                pos2(x + r, rect.bottom()),
                pos2(x, rect.center().y),
            ],
            picked_color,
            Stroke::new(visuals.fg_stroke.width, picked_color.contrast()),
        ));
    }

    response = response.on_hover_cursor(CursorIcon::ResizeHorizontal);

    response
}
