use crate::{
    app::ui_trait::UiPanel,
    context::FrameCtx,
    error::append_global_error,
    generators::GeneratorType,
    ui::{DOUBLE_SPACE, SPACE},
};
use eframe::egui::{Color32, FontId, Pos2, ScrollArea, TextBuffer, TextEdit, Ui};
use std::time::{Duration, Instant};

pub struct GeneratorsPanel {
    copy_animation: Option<CopyAnimation>,
}

struct CopyAnimation {
    start_time: Instant,
    start_pos: Pos2,
}

impl CopyAnimation {
    fn new(pos: Pos2) -> Self {
        Self {
            start_time: Instant::now(),
            start_pos: pos,
        }
    }

    fn is_finished(&self) -> bool {
        self.start_time.elapsed() > Duration::from_millis(1000)
    }

    fn get_current_pos_and_alpha(&self) -> (Pos2, f32) {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        let duration = 1000.0;
        let progress = (elapsed / duration).min(1.0);

        // Move up and fade out
        let y_offset = progress * 50.0; // Move 50 pixels up
        let current_pos = Pos2::new(self.start_pos.x, self.start_pos.y - y_offset);
        let alpha = (1.0 - progress).max(0.0);

        (current_pos, alpha)
    }
}

impl GeneratorsPanel {
    pub fn new() -> Self {
        Self {
            copy_animation: None,
        }
    }
}

impl UiPanel for GeneratorsPanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.heading("Generators");
        ui.add_space(DOUBLE_SPACE);

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut ctx.app.generator.generator_type,
                    GeneratorType::Uuid,
                    "UUID",
                );
                ui.radio_value(
                    &mut ctx.app.generator.generator_type,
                    GeneratorType::Ulid,
                    "ULID",
                );
                ui.radio_value(
                    &mut ctx.app.generator.generator_type,
                    GeneratorType::NanoId,
                    "Nano ID",
                );
                ui.radio_value(
                    &mut ctx.app.generator.generator_type,
                    GeneratorType::Password,
                    "Password",
                );
            });

            ui.add_space(SPACE);

            // Bulk count input
            ui.horizontal(|ui| {
                ui.label("Bulk generation count:");
                let mut count_text = ctx.app.generator.generated_count.to_string();
                let response = ui.add(
                    TextEdit::singleline(&mut count_text)
                        .desired_width(60.0)
                        .hint_text("1"),
                );

                if response.changed() {
                    if let Ok(count) = count_text.parse::<usize>() {
                        if count > 0 && count <= 1000 {
                            // Reasonable limit
                            ctx.app.generator.generated_count = count;
                        }
                    }
                }
            });

            ui.add_space(SPACE);

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    if let Err(e) = ctx.app.generator.generate() {
                        append_global_error(e.to_string());
                    }
                }

                let copy_button = ui.button("📋 Copy");
                if copy_button.clicked() {
                    match ctx.app.generator.copy_to_clipboard() {
                        Ok(()) => {
                            // Start the animation at the button position
                            self.copy_animation =
                                Some(CopyAnimation::new(copy_button.rect.center()));
                        }
                        Err(e) => {
                            append_global_error(e.to_string());
                        }
                    }
                }

                if ui.button("Clear").clicked() {
                    ctx.app.generator.clear();
                }
            });

            ui.add_space(SPACE);

            ui.label("Generated Output:");
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut ctx.app.generator.output.as_str())
                        .desired_width(f32::INFINITY),
                );
            });
        });

        // Render copy animation
        if let Some(animation) = &self.copy_animation {
            if animation.is_finished() {
                self.copy_animation = None;
            } else {
                let (pos, alpha) = animation.get_current_pos_and_alpha();
                let color = Color32::from_rgba_premultiplied(0, 200, 0, (255.0 * alpha) as u8);

                ui.painter().text(
                    pos,
                    eframe::egui::Align2::CENTER_CENTER,
                    "Copied! 🎉",
                    FontId::proportional(16.0),
                    color,
                );

                // Request repaint for smooth animation
                ctx.egui.request_repaint();
            }
        }
    }
}
