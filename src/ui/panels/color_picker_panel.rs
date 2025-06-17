use eframe::egui::{show_tooltip_text, Color32, CursorIcon, Id, ScrollArea, Sense, Stroke, Ui};

use crate::{
    app::{ADD_DESCRIPTION, CURRENT_COLOR_BOX_SIZE},
    context::FrameCtx,
    save_to_clipboard,
    types::error::append_global_error,
    ui::{
        components::{colorbox::ColorBox, icon, HALF_SPACE, SPACE},
        traits::UiPanel,
    },
    utils::zoom_picker::ZoomPicker,
};

pub struct ColorPickerPanel {
    pub zoom_picker: ZoomPicker,
}

impl UiPanel for ColorPickerPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(HALF_SPACE);
            ui.vertical(|ui| {
                self.render_current_color_section(ctx, ui);
                self.zoom_picker.display(ctx, ui);
            });
        });

        ui.add_space(SPACE);

        ScrollArea::vertical()
            .id_source("picker scroll")
            .show(ui, |ui| {
                // Display color picking history
                self.render_color_picking_history(ctx, ui);

                let mut available_space = ui.available_size_before_wrap();
                if ctx.app.sidepanel.show {
                    available_space.x -= ctx.app.sidepanel.response_size.x;
                }
                ui.allocate_space(available_space);
            });
    }
}

impl ColorPickerPanel {
    pub fn new() -> Self {
        Self {
            zoom_picker: ZoomPicker::default(),
        }
    }

    fn render_current_color_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Current color: ");
            if ui
                .button(icon::COPY)
                .on_hover_text("Copy color to clipboard")
                .on_hover_cursor(CursorIcon::Alias)
                .clicked()
            {
                if let Err(e) =
                    save_to_clipboard(ctx.app.clipboard_color(&ctx.app.picker.current_color))
                {
                    append_global_error(format!("Failed to save color to clipboard - {}", e));
                }
            }
            if ui
                .button(icon::ADD)
                .on_hover_text(ADD_DESCRIPTION)
                .on_hover_cursor(CursorIcon::Copy)
                .clicked()
            {
                ctx.app.add_cur_color();
            }

            let pick_text = if ctx.app.color_picking_enabled {
                "Picking color..."
            } else {
                "Pick Color"
            };

            if ui
                .button(pick_text)
                .on_hover_text("Click to toggle color picking from screen. When picking: press ` (backtick) to select color, or Esc to cancel")
                .on_hover_cursor(CursorIcon::Crosshair)
                .clicked()
            {
                ctx.app.color_picking_enabled = !ctx.app.color_picking_enabled;
            }
        });

        let cb = ColorBox::builder()
            .size((CURRENT_COLOR_BOX_SIZE, CURRENT_COLOR_BOX_SIZE))
            .color(ctx.app.picker.current_color)
            .label(true)
            .border(true)
            .build();

        ui.horizontal(|ui| {
            cb.display(ctx, ui);
        });
    }

    fn render_color_picking_history(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        // Check if we need to clear history first
        let mut clear_history = false;

        let history_len = ctx.app.color_picking_history.len();
        if history_len > 0 {
            ui.separator();
            ui.label("Color Picking History:");
            ui.add_space(HALF_SPACE);

            // Create a list of colors with their display strings
            let mut history_items = Vec::new();
            for (index, &color) in ctx.app.color_picking_history.iter().enumerate().rev() {
                let color_display = ctx.app.display_color(&color);
                history_items.push((color, color_display, index));
            }

            // Display colors in a grid layout
            ui.horizontal_wrapped(|ui| {
                for (color, color_display, index) in &history_items {
                    let _history_box = ColorBox::builder()
                        .size((30.0, 30.0))
                        .color(*color)
                        .label(false)
                        .border(true)
                        .hover_help(format!("#{}: {}", index + 1, color_display))
                        .build();

                    ui.horizontal(|ui| {
                        // Create a simple color box without using the display method to avoid borrowing issues
                        let color_rect = ui.allocate_response([30.0, 30.0].into(), Sense::click());
                        let painter = ui.painter();
                        painter.rect_filled(color_rect.rect, 2.0, color.color32());
                        painter.rect_stroke(color_rect.rect, 2.0, Stroke::new(1.0, Color32::BLACK));

                        if color_rect.clicked() {
                            ctx.app.picker.current_color = *color;
                        }

                        if color_rect.hovered() {
                            show_tooltip_text(
                                ui.ctx(),
                                Id::new(format!("history_{}", index + 1)),
                                format!("#{}: {}", index + 1, color_display),
                            );
                        }
                    });

                    // Break line after every 10 colors for better layout
                    if (index + 1) % 10 == 0 {
                        ui.end_row();
                    }
                }
            });

            ui.add_space(SPACE);

            // Add clear history button
            ui.horizontal(|ui| {
                if ui.button("Clear History").clicked() {
                    clear_history = true;
                }
                ui.label(format!("({} colors)", history_len));
            });
        }

        // Clear history outside of the borrow scope
        if clear_history {
            ctx.app.color_picking_history.clear();
        }
    }
}
