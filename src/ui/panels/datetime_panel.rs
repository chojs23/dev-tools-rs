use chrono::Datelike;
use eframe::egui::{CursorIcon, Resize, ScrollArea, Ui};
use eframe::epaint::Color32;

use crate::{
    context::FrameCtx,
    core::datetime::DateTimeFormat,
    types::error::append_global_error,
    ui::{
        components::{DOUBLE_SPACE, HALF_SPACE, SPACE},
        traits::UiPanel,
    },
};

pub struct DateTimePanel;

impl UiPanel for DateTimePanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        // Update current timestamp from cache for real-time updates
        ctx.app.datetime.update_current_timestamp();
        ctx.egui.request_repaint();

        ui.heading("Date and Time Tools");
        ui.add_space(DOUBLE_SPACE);

        // Format selection
        self.render_format_selection(ctx, ui);
        ui.add_space(SPACE);

        // Action buttons
        self.render_action_buttons(ctx, ui);
        ui.add_space(SPACE);

        // Main input/output sections horizontally
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                self.render_timestamp_section(ctx, ui);
            });
            ui.add_space(SPACE);
            ui.vertical(|ui| {
                self.render_formatted_section(ctx, ui);
            });
        });

        ui.add_space(SPACE);

        // Utilities section below
        self.render_utilities_section(ctx, ui);
    }
}

impl DateTimePanel {
    pub fn new() -> Self {
        Self
    }

    fn render_timestamp_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Unix Timestamp:");
                if ui.button("📅 Now").clicked() {
                    ctx.app.datetime.get_current_timestamp();
                }
            });

            let available_height = ui.available_height() * 0.1;
            Resize::default()
                .id_salt("timestamp_input_resize")
                .default_width(100.0)
                .max_height(SPACE)
                .max_width(200.0)
                .show(ui, |ui| {
                    ScrollArea::horizontal()
                        .id_salt("timestamp_input")
                        .max_height(available_height)
                        .show(ui, |ui| {
                            let response =
                                ui.text_edit_singleline(&mut ctx.app.datetime.timestamp_input);
                            if response.changed() {
                                ctx.app.datetime.timestamp_to_formatted();
                            }
                        });
                });

            // Show timestamp interpretation
            if !ctx.app.datetime.timestamp_input.is_empty() {
                if let Ok(timestamp) = ctx.app.datetime.timestamp_input.parse::<i64>() {
                    let relative_time = ctx.app.datetime.get_relative_time(timestamp);
                    ui.label(format!("📍 {}", relative_time));
                }
            }

            // Show formatted result under timestamp input
            ui.add_space(HALF_SPACE);
            ui.label("Formatted Result:");
            if !ctx.app.datetime.formatted_result.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("📅");
                    let response = ui.selectable_label(false, &ctx.app.datetime.formatted_result);
                    if response.clicked() {
                        self.copy_to_clipboard(&ctx.app.datetime.formatted_result);
                    }
                    response
                        .on_hover_text("Click to copy")
                        .on_hover_cursor(CursorIcon::PointingHand);
                });
            }
        });
    }

    fn render_formatted_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Formatted Date/Time:");

            let available_height = ui.available_height() * 0.1;
            Resize::default()
                .id_salt("formatted_input_resize")
                .max_height(SPACE)
                .max_width(250.0)
                .show(ui, |ui| {
                    ScrollArea::horizontal()
                        .id_salt("formatted_input")
                        .max_height(available_height)
                        .show(ui, |ui| {
                            let response =
                                ui.text_edit_singleline(&mut ctx.app.datetime.formatted_input);
                            if response.changed() {
                                ctx.app.datetime.formatted_to_timestamp();
                            }
                        });
                });

            // Show timestamp result under formatted input
            ui.add_space(HALF_SPACE);
            ui.label("Timestamp Result:");
            if !ctx.app.datetime.timestamp_result.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("🕐");
                    let response = ui.selectable_label(false, &ctx.app.datetime.timestamp_result);
                    if response.clicked() {
                        self.copy_to_clipboard(&ctx.app.datetime.timestamp_result);
                    }
                    response
                        .on_hover_text("Click to copy")
                        .on_hover_cursor(CursorIcon::PointingHand);
                });
            }

            // Error display
            if !ctx.app.datetime.error_message.is_empty() {
                ui.colored_label(Color32::RED, &ctx.app.datetime.error_message);
            }
        });
    }

    fn render_format_selection(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label("Date Format:");

            ui.horizontal(|ui| {
                let formats = DateTimeFormat::all_formats();
                for format in formats {
                    let selected = std::mem::discriminant(&ctx.app.datetime.selected_format)
                        == std::mem::discriminant(&format);

                    if ui.selectable_label(selected, format.to_string()).clicked() {
                        ctx.app.datetime.update_selected_format(format);
                    }
                }
            });

            // Custom format input
            if matches!(ctx.app.datetime.selected_format, DateTimeFormat::Custom(_)) {
                ui.horizontal(|ui| {
                    ui.label("Custom Format:");
                    let response = ui.text_edit_singleline(&mut ctx.app.datetime.custom_format);
                    if response.changed() {
                        ctx.app
                            .datetime
                            .update_custom_format(ctx.app.datetime.custom_format.clone());
                    }

                    if ui.button("ℹ").on_hover_text("Format Help").clicked() {
                        // Could show a help popup here
                    }
                });

                // Show format examples
                ui.label("Examples: %Y-%m-%d %H:%M:%S, %d/%m/%Y, %B %d, %Y");
            }
        });
    }

    fn render_action_buttons(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("🔄 Convert Timestamp").clicked() {
                ctx.app.datetime.timestamp_to_formatted();
            }

            if ui.button("🔄 Parse Date").clicked() {
                ctx.app.datetime.formatted_to_timestamp();
            }

            if ui.button("🧹 Clear").clicked() {
                ctx.app.datetime.timestamp_input.clear();
                ctx.app.datetime.formatted_input.clear();
                ctx.app.datetime.timestamp_result.clear();
                ctx.app.datetime.formatted_result.clear();
                ctx.app.datetime.error_message.clear();
            }
        });
    }

    fn render_utilities_section(&self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Utilities");
            ui.add_space(SPACE);

            // Horizontal layout for all three utility groups
            ui.horizontal_wrapped(|ui| {
                // Current time in different formats
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("Current Time:");
                        ui.add_space(HALF_SPACE);

                        let formats = [
                            (
                                "Unix Timestamp",
                                ctx.app.datetime.current_timestamp.to_string(),
                            ),
                            (
                                &format!("Selected Format ({})", ctx.app.datetime.selected_format,),
                                ctx.app.datetime.format_current_time(),
                            ),
                            (
                                "Local Time",
                                chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            ),
                            (
                                "UTC",
                                chrono::Utc::now()
                                    .format("%Y-%m-%d %H:%M:%S UTC")
                                    .to_string(),
                            ),
                        ];

                        for (label, value) in formats.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}:", label));
                                let response = ui.selectable_label(false, value);
                                if response.clicked() {
                                    self.copy_to_clipboard(value);
                                }
                                response.on_hover_text("Click to copy");
                            });
                        }
                    })
                });

                ui.add_space(SPACE);

                // Quick timestamp generators
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("Quick Actions:");
                        ui.add_space(HALF_SPACE);

                        if ui.button("📅 Start of Today").clicked() {
                            let start_of_day = chrono::Local::now()
                                .date_naive()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_local_timezone(chrono::Local)
                                .unwrap()
                                .with_timezone(&chrono::Utc);
                            ctx.app.datetime.timestamp_input = start_of_day.timestamp().to_string();
                            ctx.app.datetime.timestamp_to_formatted();
                        }

                        if ui.button("📅 End of Today").clicked() {
                            let end_of_day = chrono::Local::now()
                                .date_naive()
                                .and_hms_opt(23, 59, 59)
                                .unwrap()
                                .and_local_timezone(chrono::Local)
                                .unwrap()
                                .with_timezone(&chrono::Utc);
                            ctx.app.datetime.timestamp_input = end_of_day.timestamp().to_string();
                            ctx.app.datetime.timestamp_to_formatted();
                        }

                        if ui.button("📅 Start of Week").clicked() {
                            let now = chrono::Local::now();
                            let days_since_monday = now.weekday().num_days_from_monday();
                            let start_of_week = now
                                .date_naive()
                                .checked_sub_days(chrono::Days::new(days_since_monday as u64))
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_local_timezone(chrono::Local)
                                .unwrap()
                                .with_timezone(&chrono::Utc);
                            ctx.app.datetime.timestamp_input =
                                start_of_week.timestamp().to_string();
                            ctx.app.datetime.timestamp_to_formatted();
                        }

                        if ui.button("📅 Start of Month").clicked() {
                            let now = chrono::Local::now();
                            let start_of_month = now
                                .date_naive()
                                .with_day(1)
                                .unwrap()
                                .and_hms_opt(0, 0, 0)
                                .unwrap()
                                .and_local_timezone(chrono::Local)
                                .unwrap()
                                .with_timezone(&chrono::Utc);
                            ctx.app.datetime.timestamp_input =
                                start_of_month.timestamp().to_string();
                            ctx.app.datetime.timestamp_to_formatted();
                        }
                    });
                });

                ui.add_space(SPACE);

                // Common timestamp values
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("Common Timestamps:");
                        ui.add_space(HALF_SPACE);

                        let common_timestamps = [
                            ("Unix Epoch", "0"),
                            ("Y2K", "946684800"),
                            ("2038 Problem", "2147483647"),
                        ];

                        for (label, timestamp) in common_timestamps.iter() {
                            if ui.button(format!("{} ({})", label, timestamp)).clicked() {
                                ctx.app.datetime.timestamp_input = timestamp.to_string();
                                ctx.app.datetime.timestamp_to_formatted();
                            }
                        }
                    });
                });
            });
        });
    }

    fn copy_to_clipboard(&self, text: &str) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Err(e) =
                arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text))
            {
                append_global_error(format!("Failed to copy to clipboard: {}", e));
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, we'd need to use web APIs
            append_global_error("Clipboard not supported in web version".to_string());
        }
    }
}
