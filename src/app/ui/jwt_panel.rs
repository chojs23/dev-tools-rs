use crate::{
    app::ui_trait::UiPanel,
    context::FrameCtx,
    error::append_global_error,
    jwt::Algorithm,
    ui::{DOUBLE_SPACE, HALF_SPACE, SPACE},
};
use eframe::egui::{self, CursorIcon, ScrollArea, Ui};
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
        ui.vertical(|ui| {
            ui.label("Encoded");
            if ui.text_edit_multiline(&mut ctx.app.jwt.encoded).changed() {
                let _ = ctx.app.jwt.verify();
            }
        });
    }

    fn render_algorithm_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Algorithm");
            ui.radio_value(&mut ctx.app.jwt.algorithm, Algorithm::HS256, "HS256");
            ui.radio_value(&mut ctx.app.jwt.algorithm, Algorithm::HS384, "HS384");
            ui.radio_value(&mut ctx.app.jwt.algorithm, Algorithm::HS512, "HS512");
            ui.radio_value(&mut ctx.app.jwt.algorithm, Algorithm::RS256, "RS256");
            ui.radio_value(&mut ctx.app.jwt.algorithm, Algorithm::RS384, "RS384");
            ui.radio_value(&mut ctx.app.jwt.algorithm, Algorithm::RS512, "RS512");
        });
    }

    fn render_action_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
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
        ui.vertical(|ui| {
            ui.label("Decoded");
            ui.text_edit_multiline(&mut ctx.app.jwt.decoded);
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
                    ui.text_edit_multiline(&mut ctx.app.jwt.private_key);
                });
        });
    }
}
