use crate::{
    app::ui_trait::UiPanel,
    context::FrameCtx,
    encoding::{EncodingProcessor, EncodingType},
    error::append_global_error,
    ui::{DOUBLE_SPACE, HALF_SPACE, SPACE},
};
use eframe::egui::{self, CursorIcon, ScrollArea, Ui};

pub struct EncodingPanel;

impl UiPanel for EncodingPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.heading("Text Encoder/Decoder");
        ui.add_space(DOUBLE_SPACE);

        ui.horizontal(|ui| {
            self.render_main_section(ctx, ui);
        });
    }
}

impl EncodingPanel {
    pub fn new() -> Self {
        Self
    }

    fn render_main_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.render_input_section(ctx, ui);
            ui.add_space(HALF_SPACE);
            self.render_encoding_selection(ctx, ui);
            ui.add_space(SPACE);
            self.render_options_section(ctx, ui);
            ui.add_space(SPACE);
            self.render_action_buttons(ctx, ui);
            ui.add_space(HALF_SPACE);
            self.render_output_section(ctx, ui);
        });
    }

    fn render_input_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Input");
            let available_height = ui.available_height() * 0.35;
            ScrollArea::vertical()
                .id_source("input_text")
                .max_height(available_height)
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut ctx.app.encoding.input);
                });
        });
    }

    fn render_encoding_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Encoding Type:");
            ui.radio_value(
                &mut ctx.app.encoding.encoding_type,
                EncodingType::Base64,
                "Base64",
            );
            ui.radio_value(
                &mut ctx.app.encoding.encoding_type,
                EncodingType::Base64Url,
                "Base64 URL",
            );
            ui.radio_value(
                &mut ctx.app.encoding.encoding_type,
                EncodingType::Base64Mime,
                "Base64 MIME",
            );
            ui.radio_value(
                &mut ctx.app.encoding.encoding_type,
                EncodingType::Base32,
                "Base32",
            );
            ui.radio_value(
                &mut ctx.app.encoding.encoding_type,
                EncodingType::UrlEncoding,
                "URL Encoding",
            );
        });
    }

    fn render_options_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(
                &mut ctx.app.encoding.handle_line_breaks,
                "Handle line breaks (\\n, \\r)",
            );
        });
    }

    fn render_action_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .button("⬆ Encode")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                match ctx.app.encoding.encode() {
                    Ok(_) => {}
                    Err(e) => {
                        append_global_error(e);
                    }
                }
            }

            if ui
                .button("⬇ Decode")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                match ctx.app.encoding.decode() {
                    Ok(_) => {}
                    Err(e) => {
                        append_global_error(e);
                    }
                }
            }

            if ui
                .button("⟲  Clear")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                ctx.app.encoding.clear();
            }

            if ui
                .button("⇄ Swap")
                .on_hover_cursor(CursorIcon::PointingHand)
                .on_hover_text("Swap input and output")
                .clicked()
            {
                std::mem::swap(&mut ctx.app.encoding.input, &mut ctx.app.encoding.output);
            }
        });
    }

    fn render_output_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Output");
            let available_height = ui.available_height() * 0.8;
            ScrollArea::vertical()
                .id_source("output_text")
                .max_height(available_height)
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    ui.text_edit_multiline(&mut ctx.app.encoding.output);
                });
        });
    }
}
