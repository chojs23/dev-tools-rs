use eframe::egui::{Color32, FontId, Pos2, ScrollArea,  TextEdit, Ui};
use std::time::{Duration, Instant};

use crate::{
    context::FrameCtx, core::generators::GeneratorType, types::error::append_global_error, ui::{
        components::{DOUBLE_SPACE,  SPACE},
        traits::UiPanel,
    }
};

static COPY_ANIMATION_DURATION: Duration = Duration::from_millis(500);
static ERROR_ANIMATION_DURATION: Duration = Duration::from_millis(1000);
static MAXIMUM_BULK_GENERATION_COUNT: usize = 1000;

pub struct GeneratorsPanel {
    copy_animation: Option<CopyAnimation>,
    error_animation: Option<ErrorAnimation>,
}

struct CopyAnimation {
    start_time: Instant,
    start_pos: Pos2,
}

struct ErrorAnimation {
    start_time: Instant,
    start_pos: Pos2,
    message: String,
}

impl CopyAnimation {
    fn new(pos: Pos2) -> Self {
        Self {
            start_time: Instant::now(),
            start_pos: pos,
        }
    }

    fn is_finished(&self) -> bool {
        self.start_time.elapsed() > COPY_ANIMATION_DURATION
    }

    fn get_current_pos_and_alpha(&self) -> (Pos2, f32) {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        let duration = COPY_ANIMATION_DURATION.as_millis() as f32;
        let progress = (elapsed / duration).min(1.0);

        // Move up and fade out
        let y_offset = progress * 25.0;
        let current_pos = Pos2::new(self.start_pos.x, self.start_pos.y - y_offset);
        let alpha = (1.0 - progress).max(0.0);

        (current_pos, alpha)
    }
}

impl ErrorAnimation {
    fn new(pos: Pos2, message: String) -> Self {
        Self {
            start_time: Instant::now(),
            start_pos: pos,
            message,
        }
    }

    fn is_finished(&self) -> bool {
        self.start_time.elapsed() > ERROR_ANIMATION_DURATION
    }

    fn get_current_pos_and_alpha(&self) -> (Pos2, f32) {
        let elapsed = self.start_time.elapsed().as_millis() as f32;
        let duration = ERROR_ANIMATION_DURATION.as_millis() as f32;
        let progress = (elapsed / duration).min(1.0);

        // Move up and fade out
        let y_offset = progress * 30.0; // Move 30 pixels up
        let current_pos = Pos2::new(self.start_pos.x, self.start_pos.y - y_offset);
        let alpha = (1.0 - progress).max(0.0);

        (current_pos, alpha)
    }
}

impl GeneratorsPanel {
    pub fn new() -> Self {
        Self {
            copy_animation: None,
            error_animation: None,
        }
    }

    fn copy_individual_to_clipboard(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            use arboard::Clipboard;
            let mut clipboard = Clipboard::new()?;
            clipboard.set_text(text)?;
        }
        Ok(())
    }
}

impl Default for GeneratorsPanel {
    fn default() -> Self {
        Self::new()
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
                        if count > 0 && count <= MAXIMUM_BULK_GENERATION_COUNT {
                            ctx.app.generator.generated_count = count;
                        } else if count > MAXIMUM_BULK_GENERATION_COUNT {
                            let error_message = format!(
                                "Cannot exceed maximum count of {} (entered: {})",
                                MAXIMUM_BULK_GENERATION_COUNT, count
                            );
                            self.error_animation =
                                Some(ErrorAnimation::new(response.rect.center(), error_message));
                        }
                    }
                }
            });

            ui.add_space(SPACE);

            // Password options (only show when Password generator is selected)
            if ctx.app.generator.generator_type == GeneratorType::Password {
                ui.group(|ui| {
                    ui.label("Password Options:");
                    ui.add_space(SPACE / 2.0);
                    
                    // Password length
                    ui.horizontal(|ui| {
                        ui.label("Length:");
                        let mut length_text = ctx.app.generator.password_length.to_string();
                        let response = ui.add(
                            TextEdit::singleline(&mut length_text)
                                .desired_width(60.0)
                                .hint_text("12")
                        );
                        
                        if response.changed() {
                            if let Ok(length) = length_text.parse::<usize>() {
                                if length > 0 && length <= 128 { // Reasonable limit for passwords
                                    ctx.app.generator.password_length = length;
                                } else if length > 128 {
                                    let error_message = format!("Password length cannot exceed 128 characters! (entered: {})", length);
                                    self.error_animation = Some(ErrorAnimation::new(
                                        response.rect.center(),
                                        error_message,
                                    ));
                                }
                            }
                        }
                    });
                    
                    ui.add_space(SPACE / 2.0);
                    
                    // Character type options
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut ctx.app.generator.include_uppercase, "Uppercase (A-Z)");
                        ui.checkbox(&mut ctx.app.generator.include_lowercase, "Lowercase (a-z)");
                        ui.checkbox(&mut ctx.app.generator.include_numbers, "Numbers (0-9)");
                        ui.checkbox(&mut ctx.app.generator.include_symbols, "Symbols (!@#$...)");
                        ui.checkbox(&mut ctx.app.generator.exclude_ambiguous, "Exclude ambiguous (0,O,1,l,I)");
                    });
                });
                
                ui.add_space(SPACE);
            }

            ui.horizontal(|ui| {
                if ui.button("Generate").clicked() {
                    if let Err(e) = ctx.app.generator.generate() {
                        append_global_error(e.to_string());
                    }
                }

                let copy_button = ui.button("📋 Copy ALL");
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
                if ctx.app.generator.output.is_empty() {
                    ui.label("No output generated yet.");
                } else {
                    let outputs: Vec<&str> = ctx.app.generator.output.lines().collect();

                    ui.vertical(|ui| {
                        for (index, output) in outputs.iter().enumerate() {
                            if !output.trim().is_empty() {
                                let button_response = ui.add(
                                    eframe::egui::Button::new(*output)
                                        .min_size([output.len() as f32 * 2.0, 30.0].into())
                                        .wrap(),
                                );

                                if button_response.clicked() {
                                    if let Err(e) = self.copy_individual_to_clipboard(output) {
                                        append_global_error(e.to_string());
                                    } else {
                                        self.copy_animation =
                                            Some(CopyAnimation::new(button_response.rect.center()));
                                    }
                                }

                                if index < outputs.len() - 1 {
                                    ui.add_space(2.0);
                                }
                            }
                        }
                    });
                }
            });
        });

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

                ctx.egui.request_repaint();
            }
        }

        if let Some(animation) = &self.error_animation {
            if animation.is_finished() {
                self.error_animation = None;
            } else {
                let (pos, alpha) = animation.get_current_pos_and_alpha();
                let color = Color32::from_rgba_premultiplied(255, 100, 100, (255.0 * alpha) as u8);

                ui.painter().text(
                    pos,
                    eframe::egui::Align2::CENTER_CENTER,
                    &animation.message,
                    FontId::proportional(14.0),
                    color,
                );

                ctx.egui.request_repaint();
            }
        }
    }
}
