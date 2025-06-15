use crate::{
    app::{colorbox::ColorBox, ui_trait::UiPanel, ADD_DESCRIPTION, CURRENT_COLOR_BOX_SIZE},
    context::FrameCtx,
    error::append_global_error,
    save_to_clipboard,
    ui::{icon, HALF_SPACE, SPACE},
    zoom_picker::ZoomPicker,
};
use eframe::egui::{CursorIcon, ScrollArea, Ui};

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
}
