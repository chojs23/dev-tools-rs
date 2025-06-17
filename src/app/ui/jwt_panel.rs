use crate::{
    app::ui_trait::UiPanel,
    context::FrameCtx,
    error::append_global_error,
    jwt::Algorithm,
    ui::{DOUBLE_SPACE, HALF_SPACE, SPACE},
};
use eframe::egui::{self, Align, CursorIcon, Layout, Resize, ScrollArea, Ui};
use eframe::epaint::Color32;

pub struct JwtPanel;

impl UiPanel for JwtPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.heading("JWT Encoder/Decoder");
        ui.add_space(DOUBLE_SPACE);

        ui.horizontal(|ui| {
            self.render_main_section(ctx, ui);
            self.render_key_section(ctx, ui);
        });
    }
}

//TODO: Maybe remove live conversion and just use buttons for encode/decode.
// Format error messages to be more user-friendly.
impl JwtPanel {
    pub fn new() -> Self {
        Self
    }

    fn render_main_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.render_encoded_section(ctx, ui);
            ui.add_space(HALF_SPACE);
            self.render_algorithm_selection(ctx, ui);
            ui.add_space(SPACE);
            self.render_action_buttons(ctx, ui);
            ui.add_space(HALF_SPACE);
            self.render_decoded_section(ctx, ui);
            ui.add_space(SPACE);
            self.render_header_section(ctx, ui);
        });
    }

    fn render_encoded_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        Resize::default()
            .id_source("jwt_encoded_container")
            .show(ui, |ui| {
                ui.label("Encoded input/output");
                ui.add_space(HALF_SPACE);
                ScrollArea::vertical()
                    .id_source("jwt_encoded")
                    .stick_to_bottom(false)
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        ui.set_max_height(ui.available_height() * 0.9);
                        ui.set_max_width(ui.available_width() * 1.0);
                        ui.with_layout(
                            Layout::top_down(Align::Min)
                                .with_main_justify(true)
                                .with_cross_justify(true),
                            |ui| {
                                let response = ui.text_edit_multiline(&mut ctx.app.jwt.encoded);

                                if response.changed() {
                                    let _ = ctx.app.jwt.verify();

                                    // Trigger live decoding if enabled
                                    if ctx.app.jwt.live_conversion {
                                        if let Err(e) = ctx.app.jwt.decode() {
                                            append_global_error(e);
                                        }
                                    }
                                }
                            },
                        )
                    });

                ui.add_space(HALF_SPACE);
            });
    }

    fn render_algorithm_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Algorithm");
            let mut algorithm_changed = false;

            algorithm_changed |= ui
                .radio_value(&mut ctx.app.jwt.algorithm, Algorithm::HS256, "HS256")
                .changed();
            algorithm_changed |= ui
                .radio_value(&mut ctx.app.jwt.algorithm, Algorithm::HS384, "HS384")
                .changed();
            algorithm_changed |= ui
                .radio_value(&mut ctx.app.jwt.algorithm, Algorithm::HS512, "HS512")
                .changed();
            algorithm_changed |= ui
                .radio_value(&mut ctx.app.jwt.algorithm, Algorithm::RS256, "RS256")
                .changed();
            algorithm_changed |= ui
                .radio_value(&mut ctx.app.jwt.algorithm, Algorithm::RS384, "RS384")
                .changed();
            algorithm_changed |= ui
                .radio_value(&mut ctx.app.jwt.algorithm, Algorithm::RS512, "RS512")
                .changed();

            // Trigger live encoding if enabled and algorithm changed
            if ctx.app.jwt.live_conversion && algorithm_changed {
                if let Err(e) = ctx.app.jwt.encode() {
                    append_global_error(e);
                }
            }
        });
    }

    fn render_action_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut ctx.app.jwt.live_conversion, "Live conversion");

            let live_enabled = ctx.app.jwt.live_conversion;

            ui.add_enabled_ui(!live_enabled, |ui| {
                if ui
                    .button("⬆ Encode")
                    .on_hover_cursor(CursorIcon::PointingHand)
                    .clicked()
                {
                    match ctx.app.jwt.encode() {
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
                    match ctx.app.jwt.decode() {
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
                ctx.app.jwt.clear();
            }

            self.render_verification_status(ctx, ui);
        });
    }

    fn render_verification_status(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.label(
            egui::RichText::new(format!(
                "Verified {}",
                if ctx.app.jwt.verified.is_some() {
                    if ctx.app.jwt.verified.unwrap() {
                        "✔"
                    } else {
                        "✖"
                    }
                } else {
                    ""
                }
            ))
            .color(ctx.app.jwt.verified.map_or(Color32::WHITE, |v| {
                if v {
                    Color32::GREEN
                } else {
                    Color32::RED
                }
            })),
        );
    }

    fn render_decoded_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        Resize::default()
            .id_source("jwt_decoded_container")
            .show(ui, |ui| {
                ui.label("Decoded input/output");
                ui.add_space(HALF_SPACE);
                ScrollArea::vertical()
                    .id_source("jwt_decoded")
                    .stick_to_bottom(false)
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        ui.set_max_height(ui.available_height() * 0.9);
                        ui.set_max_width(ui.available_width() * 1.0);
                        ui.with_layout(
                            Layout::top_down(Align::Min)
                                .with_main_justify(true)
                                .with_cross_justify(true),
                            |ui| {
                                let response = ui.text_edit_multiline(&mut ctx.app.jwt.decoded);

                                if ctx.app.jwt.live_conversion && response.changed() {
                                    if let Err(e) = ctx.app.jwt.encode() {
                                        append_global_error(e);
                                    }
                                }
                            },
                        )
                    });

                ui.add_space(HALF_SPACE);
            });
    }

    fn render_header_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Header");
            ui.add_space(HALF_SPACE);
            let header = ctx.app.jwt.get_header().unwrap_or_default();
            ui.text_edit_multiline(&mut header.as_str());
        });
    }

    fn render_key_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| match ctx.app.jwt.algorithm {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
                self.render_secret_section(ctx, ui);
            }
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
                self.render_rsa_keys_section(ctx, ui);
            }
        });
    }

    fn render_secret_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.label("Secret");
        if ui.text_edit_singleline(&mut ctx.app.jwt.secret).changed() {
            let _ = ctx.app.jwt.verify();

            // Trigger live encoding if enabled and secret changed
            if ctx.app.jwt.live_conversion {
                if let Err(e) = ctx.app.jwt.encode() {
                    append_global_error(e);
                }
            }
        }
    }

    fn render_rsa_keys_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Public Key");
            let scroll_height = ui.available_height() - 30.0;
            ScrollArea::vertical()
                .id_source("public_key")
                .max_height(scroll_height)
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    if ui
                        .text_edit_multiline(&mut ctx.app.jwt.public_key)
                        .changed()
                    {
                        let _ = ctx.app.jwt.verify();

                        // Trigger live encoding if enabled and public key changed
                        if ctx.app.jwt.live_conversion {
                            if let Err(e) = ctx.app.jwt.encode() {
                                append_global_error(e);
                            }
                        }
                    }
                });
        });

        ui.add_space(SPACE * 4.);

        ui.vertical(|ui| {
            ui.label("Private Key");
            let scroll_height = ui.available_height() - 30.0;
            ScrollArea::vertical()
                .id_source("private_key")
                .max_height(scroll_height)
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    if ui
                        .text_edit_multiline(&mut ctx.app.jwt.private_key)
                        .changed()
                    {
                        // Trigger live encoding if enabled and private key changed
                        if ctx.app.jwt.live_conversion {
                            if let Err(e) = ctx.app.jwt.encode() {
                                append_global_error(e);
                            }
                        }
                    }
                });
        });
    }
}
