use crate::{
    context::FrameCtx,
    core::encoding::EncodingType,
    types::error::append_global_error,
    ui::{
        components::{DOUBLE_SPACE, HALF_SPACE, SPACE},
        traits::UiPanel,
    },
};
use eframe::egui::{Align, CursorIcon, Layout, Resize, ScrollArea, Ui};

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

impl Default for EncodingPanel {
    fn default() -> Self {
        Self::new()
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
            self.render_encoded_section(ctx, ui);
        });
    }

    fn render_input_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        Resize::default().id_salt("decoded_section").show(ui, |ui| {
            ui.set_max_height(ui.available_height() * 1.0);
            ui.label("Decoded input/output");
            ui.add_space(HALF_SPACE);
            ScrollArea::vertical()
                .id_salt("decoded_text")
                .stick_to_bottom(false)
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    ui.with_layout(
                        Layout::top_down(Align::Min)
                            .with_main_justify(true)
                            .with_cross_justify(true),
                        |ui| {
                            let response =
                                ui.text_edit_multiline(&mut ctx.app.encoding.decoded_text);

                            // Trigger live encoding if enabled and input changed
                            if ctx.app.encoding.live_conversion && response.changed() {
                                if let Err(e) = ctx.app.encoding.encode() {
                                    append_global_error(e);
                                }
                            }
                        },
                    )
                });
        });
    }

    fn render_encoding_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Encoding Type:");
            let mut encoding_changed = false;

            encoding_changed |= ui
                .radio_value(
                    &mut ctx.app.encoding.encoding_type,
                    EncodingType::Base64,
                    "Base64",
                )
                .changed();
            encoding_changed |= ui
                .radio_value(
                    &mut ctx.app.encoding.encoding_type,
                    EncodingType::Base64Url,
                    "Base64 URL",
                )
                .changed();
            encoding_changed |= ui
                .radio_value(
                    &mut ctx.app.encoding.encoding_type,
                    EncodingType::Base64Mime,
                    "Base64 MIME",
                )
                .changed();
            encoding_changed |= ui
                .radio_value(
                    &mut ctx.app.encoding.encoding_type,
                    EncodingType::Base32,
                    "Base32",
                )
                .changed();
            encoding_changed |= ui
                .radio_value(
                    &mut ctx.app.encoding.encoding_type,
                    EncodingType::UrlEncoding,
                    "URL Encoding",
                )
                .changed();

            // Trigger live encoding if enabled and encoding type changed
            if ctx.app.encoding.live_conversion && encoding_changed {
                if let Err(e) = ctx.app.encoding.encode() {
                    append_global_error(e);
                }
            }
        });
    }

    fn render_options_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let line_breaks_changed = ui
                .checkbox(
                    &mut ctx.app.encoding.handle_line_breaks,
                    "Handle line breaks (\\n, \\r)",
                )
                .changed();

            // Trigger live encoding if enabled and line breaks option changed
            if ctx.app.encoding.live_conversion && line_breaks_changed {
                if let Err(e) = ctx.app.encoding.encode() {
                    append_global_error(e);
                }
            }
        });
    }

    fn render_action_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut ctx.app.encoding.live_conversion, "Live conversion");

            let live_enabled = ctx.app.encoding.live_conversion;

            ui.add_enabled_ui(!live_enabled, |ui| {
                if ui
                    .button("⬇ Encode")
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
                    .button("⬆ Decode")
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
            });

            if ui
                .button("⟲  Clear")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                ctx.app.encoding.clear();
            }

            // Disable swap
            // if ui
            //     .button("⇄ Swap")
            //     .on_hover_cursor(CursorIcon::PointingHand)
            //     .on_hover_text("Swap input and output")
            //     .clicked()
            // {
            //     std::mem::swap(&mut ctx.app.encoding.input, &mut ctx.app.encoding.output);
            // }
        });
    }

    fn render_encoded_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        Resize::default().id_salt("encoded_section").show(ui, |ui| {
            ui.set_max_height(ui.available_height() * 1.0);
            ui.label("Encoded input/output");
            ui.add_space(HALF_SPACE);
            ScrollArea::vertical()
                .id_salt("encoded_text")
                .stick_to_bottom(false)
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    ui.with_layout(
                        Layout::top_down(Align::Min)
                            .with_main_justify(true)
                            .with_cross_justify(true),
                        |ui| {
                            let response =
                                ui.text_edit_multiline(&mut ctx.app.encoding.encoded_text);

                            //TODO: Handle error gracefully
                            if ctx.app.encoding.live_conversion && response.changed() {
                                if let Err(e) = ctx.app.encoding.decode() {
                                    ctx.app.encoding.decoded_text =
                                        "Malformed input : ".to_string() + &e.to_string();
                                }
                            }
                        },
                    )
                });
        });
    }
}
