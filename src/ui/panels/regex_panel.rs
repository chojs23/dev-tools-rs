use eframe::egui::{
    Align, Color32, CursorIcon, Layout, Resize, RichText, ScrollArea, TextEdit, Ui,
};

use crate::{
    context::FrameCtx,
    types::error::append_global_error,
    ui::{
        components::{DOUBLE_SPACE, HALF_SPACE, SPACE},
        traits::UiPanel,
    },
};

pub struct RegexPanel;

impl UiPanel for RegexPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.heading("Regular Expression Matcher");
        ui.add_space(DOUBLE_SPACE);

        ui.horizontal(|ui| {
            self.render_main_section(ctx, ui);
        });
    }
}

impl Default for RegexPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RegexPanel {
    pub fn new() -> Self {
        Self
    }

    fn render_main_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.render_pattern_section(ctx, ui);
            ui.add_space(SPACE);
            self.render_options_section(ctx, ui);
            ui.add_space(SPACE);
            self.render_action_buttons(ctx, ui);
            ui.add_space(DOUBLE_SPACE);
            self.render_text_section(ctx, ui);
            ui.add_space(DOUBLE_SPACE);
            self.render_results_section(ctx, ui);
        });
    }

    fn render_pattern_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        Resize::default()
            .id_salt("regex_pattern_container")
            .default_height(300.0)
            .show(ui, |ui| {
                ui.label("Regular Expression Pattern");
                ui.add_space(HALF_SPACE);

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_max_height(ui.available_height() * 1.0);
                        ui.set_max_width(ui.available_width() * 1.0);
                        ui.with_layout(
                            Layout::top_down(Align::Min)
                                .with_main_justify(true)
                                .with_cross_justify(true),
                            |ui| {
                                let input_edit = TextEdit::multiline(&mut ctx.app.regex.pattern)
                                    .desired_width(ui.available_width());

                                ui.add(input_edit);
                            },
                        );
                    });
            });
    }

    fn render_text_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Text to Match");
            let available_height = ui.available_height() * 0.4;
            ScrollArea::vertical()
                .id_salt("regex_text")
                .max_height(available_height)
                .stick_to_bottom(false)
                .show(ui, |ui| {
                    let response = ui.text_edit_multiline(&mut ctx.app.regex.text);

                    // Auto-process when text changes if pattern is not empty
                    if response.changed() && !ctx.app.regex.pattern.is_empty() {
                        if let Err(e) = ctx.app.regex.process() {
                            // Error is already stored in the result, no need to display here
                            let _ = e;
                        }
                    }
                });
        });
    }

    fn render_options_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let case_changed = ui
                .checkbox(&mut ctx.app.regex.case_insensitive, "Case insensitive (i)")
                .changed();

            let multiline_changed = ui
                .checkbox(&mut ctx.app.regex.multiline, "Multiline (m)")
                .changed();

            let dotall_changed = ui
                .checkbox(
                    &mut ctx.app.regex.dot_matches_newline,
                    "Dot matches newline (s)",
                )
                .changed();

            // Auto-process when options change if both pattern and text are not empty
            if (case_changed || multiline_changed || dotall_changed)
                && !ctx.app.regex.pattern.is_empty()
                && !ctx.app.regex.text.is_empty()
            {
                if let Err(e) = ctx.app.regex.process() {
                    // Error is already stored in the result, no need to display here
                    let _ = e;
                }
            }
        });
    }

    fn render_action_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .button("🔍 Match")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                match ctx.app.regex.process() {
                    Ok(_) => {}
                    Err(e) => {
                        append_global_error(e);
                    }
                }
            }

            if ui
                .button("⟲ Clear")
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                ctx.app.regex.clear();
            }
        });
    }

    fn render_results_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.label("Match Results");
        ui.add_space(HALF_SPACE);

        let match_count = ctx.app.regex.result.as_ref().map_or(0, |r| r.matches.len());

        ui.label(format!(
            "Found {} match{}",
            match_count,
            if match_count == 1 { "" } else { "es" }
        ));

        ui.add_space(HALF_SPACE);
        ui.set_min_height(200.0);
        //TODO: resizable

        ScrollArea::vertical()
            .id_salt("regex_results_container")
            .stick_to_bottom(false)
            .show(ui, |ui| {
                ui.set_min_width(400.0);
                if let Some(result) = &ctx.app.regex.result {
                    if !result.is_valid {
                        if let Some(error) = &result.error_message {
                            ui.colored_label(Color32::RED, format!("❌ Error: {}", error));
                            return;
                        }
                    }
                    if !result.matches.is_empty() {
                        for (i, regex_match) in result.matches.iter().enumerate() {
                            ui.group(|ui| {
                                // dynamically set width based on content
                                ui.set_width_range(
                                    ui.available_width() * 0.4..=ui.available_width() * 0.9,
                                );
                                ui.horizontal(|ui| {
                                    ui.label(format!("Match {}:", i + 1));
                                    ui.label(format!(
                                        "Position {}-{}",
                                        regex_match.start, regex_match.end
                                    ));
                                });

                                ui.horizontal(|ui| {
                                    ui.label("Text:");
                                    ui.label(RichText::new(&regex_match.text).code());
                                });

                                if !regex_match.groups.is_empty() {
                                    ui.label("Capture Groups:");
                                    for (group_idx, group) in regex_match.groups.iter().enumerate()
                                    {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("  Group {}:", group_idx + 1));
                                            ui.label(RichText::new(group).code());
                                        });
                                    }
                                }
                            });
                            ui.add_space(HALF_SPACE);
                        }
                    }
                }
            });
    }
}
