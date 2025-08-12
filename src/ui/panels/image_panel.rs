use crate::{
    context::FrameCtx,
    core::image::{
        CompressionOptions, CropRect, FilterType, ImageFormatType, ImageProcessor, ResizeOptions,
    },
    types::error::append_global_error,
    ui::{
        components::{DOUBLE_SPACE, HALF_SPACE, SPACE},
        traits::UiPanel,
    },
};
use eframe::egui::{
    self, Align, Color32, ColorImage, Context, CursorIcon, Layout, Pos2, Rect, Resize, Response,
    ScrollArea, Sense, Stroke, TextureHandle, TextureOptions, Ui, Vec2,
};
use image::DynamicImage;

#[derive(Default)]
pub struct ImagePanel {
    processor: ImageProcessor,
    texture_handle: Option<TextureHandle>,

    // UI State
    selected_format: ImageFormatType,
    resize_width: String,
    resize_height: String,
    maintain_aspect_ratio: bool,
    selected_filter: FilterType,
    compression_quality: u8,

    // Crop state
    is_cropping: bool,
    crop_start: Option<Pos2>,
    crop_end: Option<Pos2>,
    crop_rect: Option<CropRect>,

    // Adjustment controls
    brightness_value: i32,
    contrast_value: f32,
    blur_sigma: f32,

    // Display scaling
    display_scale: f32,

    // Size estimation result
    estimated_size: Option<String>,
}

impl UiPanel for ImagePanel {
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.heading("Image Utilities");
        ui.add_space(DOUBLE_SPACE);

        let available_size = ui.available_size();

        ui.horizontal(|ui| {
            ui.set_height(available_size.y);
            // Left panel
            ui.vertical(|ui| {
                ui.set_width(300.0);

                ScrollArea::vertical()
                    .id_salt("left_panel_scroll")
                    .show(ui, |ui| {
                        self.render_file_section(ctx, ui);
                        ui.add_space(SPACE);
                        self.render_info_section(ui);
                        ui.add_space(SPACE);
                        self.render_resize_section(ui);
                        ui.add_space(SPACE);
                        self.render_crop_section(ui);
                        ui.add_space(SPACE);
                        self.render_transform_section(ui);
                        ui.add_space(SPACE);
                        self.render_adjustment_section(ui);
                        ui.add_space(SPACE);
                        self.render_compression_section(ui);
                        ui.add_space(SPACE);
                        self.render_action_buttons(ui);
                    });
            });

            ui.separator();

            // Right panel - image display
            ui.vertical(|ui| {
                self.render_image_display(ui);
            });
        });
    }
}

impl ImagePanel {
    pub fn new() -> Self {
        Self {
            processor: ImageProcessor::new(),
            texture_handle: None,
            selected_format: ImageFormatType::Png,
            resize_width: String::new(),
            resize_height: String::new(),
            maintain_aspect_ratio: true,
            selected_filter: FilterType::Lanczos3,
            compression_quality: 85,
            is_cropping: false,
            crop_start: None,
            crop_end: None,
            crop_rect: None,
            brightness_value: 0,
            contrast_value: 1.0,
            blur_sigma: 0.0,
            display_scale: 1.0,
            estimated_size: None,
        }
    }

    fn render_file_section(&mut self, _ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong("File Operations");
                ui.add_space(HALF_SPACE);

                if ui.button("📁 Open Image").clicked() {
                    match native_dialog::DialogBuilder::file()
                        .set_title("Open Image")
                        .add_filter(
                            "Image Files",
                            &["jpg", "jpeg", "png", "gif", "webp", "tiff", "bmp", "ico"],
                        )
                        .open_single_file()
                        .show()
                    {
                        Ok(Some(path)) => match self.processor.load_from_file(&path) {
                            Ok(()) => {
                                self.update_texture(ui.ctx());
                                self.update_ui_from_image();
                                self.estimated_size = None; // Clear estimate when new image is loaded
                            }
                            Err(e) => append_global_error(&format!("Failed to load image: {}", e)),
                        },
                        Ok(None) => {
                            // User canceled, do nothing
                        }
                        Err(e) => {
                            append_global_error(&format!("Failed to open file dialog: {}", e));
                        }
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("💾 Save As").clicked() {
                        let extension = self.selected_format.extension();
                        let default_filename = format!("image.{}", extension);
                        match native_dialog::DialogBuilder::file()
                            .set_title("Save Image As")
                            .set_filename(&default_filename)
                            .add_filter(&self.selected_format.display_name(), &[extension])
                            .save_single_file()
                            .show()
                        {
                            Ok(Some(mut path)) => {
                                // Ensure the file has the correct extension
                                if path.extension().is_none() {
                                    path.set_extension(extension);
                                }
                                if let Err(e) = self.processor.save_as(&path, self.selected_format)
                                {
                                    append_global_error(&format!("Failed to save image: {}", e));
                                }
                            }
                            Ok(None) => {
                                // User canceled, do nothing
                            }
                            Err(e) => {
                                append_global_error(&format!("Failed to open save dialog: {}", e));
                            }
                        }
                    }

                    if ui.button("🔄 Reset").clicked() {
                        self.processor.reset_to_original();
                        self.update_texture(ui.ctx());
                        self.brightness_value = 0;
                        self.contrast_value = 1.0;
                        self.blur_sigma = 0.0;
                        self.estimated_size = None; // Clear estimate when image is reset
                    }
                });
            });
        });
    }

    fn render_info_section(&self, ui: &mut Ui) {
        if let Some(info) = self.processor.image_info() {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.strong("Image Information");
                    ui.add_space(HALF_SPACE);

                    ui.label(format!("Dimensions: {}x{}", info.width, info.height));
                    if let Some(format) = info.format {
                        ui.label(format!("Format: {}", format.display_name()));
                    }
                    if let Some(size) = info.file_size {
                        ui.label(format!("File Size: {}", format_file_size(size)));
                    }
                    ui.label(format!("Color Type: {}", info.color_type));
                });
            });
        }
    }

    fn render_resize_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong("Resize");
                ui.add_space(HALF_SPACE);

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    ui.text_edit_singleline(&mut self.resize_width);
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    ui.text_edit_singleline(&mut self.resize_height);
                });

                ui.checkbox(&mut self.maintain_aspect_ratio, "Maintain aspect ratio");

                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    egui::ComboBox::from_id_source("resize_filter")
                        .selected_text(self.selected_filter.display_name())
                        .show_ui(ui, |ui| {
                            for filter in FilterType::all() {
                                ui.selectable_value(
                                    &mut self.selected_filter,
                                    filter,
                                    filter.display_name(),
                                );
                            }
                        });
                });

                if ui.button("🔧 Resize").clicked() {
                    let width = if self.resize_width.is_empty() {
                        None
                    } else {
                        self.resize_width.parse().ok()
                    };

                    let height = if self.resize_height.is_empty() {
                        None
                    } else {
                        self.resize_height.parse().ok()
                    };

                    let options = ResizeOptions {
                        width,
                        height,
                        maintain_aspect_ratio: self.maintain_aspect_ratio,
                        filter: self.selected_filter,
                    };

                    if let Err(e) = self.processor.resize(&options) {
                        append_global_error(&format!("Resize failed: {}", e));
                    } else {
                        self.update_texture(ui.ctx());
                    }
                }
            });
        });
    }

    fn render_crop_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong("Crop");
                ui.add_space(HALF_SPACE);

                if ui.button("✂️ Enable Crop Mode").clicked() {
                    self.is_cropping = true;
                    self.crop_start = None;
                    self.crop_end = None;
                }

                if self.is_cropping {
                    ui.label("Click and drag on image to select crop area");

                    if let Some(crop_rect) = self.crop_rect.clone() {
                        ui.label(format!(
                            "Selection: {}x{} at ({}, {})",
                            crop_rect.width, crop_rect.height, crop_rect.x, crop_rect.y
                        ));

                        ui.horizontal(|ui| {
                            if ui.button("✅ Apply Crop").clicked() {
                                if let Err(e) = self.processor.crop(&crop_rect) {
                                    append_global_error(&format!("Crop failed: {}", e));
                                } else {
                                    self.update_texture(ui.ctx());
                                    self.is_cropping = false;
                                    self.crop_rect = None;
                                }
                            }

                            if ui.button("❌ Cancel").clicked() {
                                self.is_cropping = false;
                                self.crop_rect = None;
                            }
                        });
                    }
                }
            });
        });
    }

    fn render_transform_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong("Transform");
                ui.add_space(HALF_SPACE);

                ui.horizontal(|ui| {
                    if ui.button("↻ 90°").clicked() {
                        if let Err(e) = self.processor.rotate_90() {
                            append_global_error(&format!("Rotation failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }

                    if ui.button("↻ 180°").clicked() {
                        if let Err(e) = self.processor.rotate_180() {
                            append_global_error(&format!("Rotation failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }

                    if ui.button("↻ 270°").clicked() {
                        if let Err(e) = self.processor.rotate_270() {
                            append_global_error(&format!("Rotation failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("⟷ Flip H").clicked() {
                        if let Err(e) = self.processor.flip_horizontal() {
                            append_global_error(&format!("Flip failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }

                    if ui.button("↕ Flip V").clicked() {
                        if let Err(e) = self.processor.flip_vertical() {
                            append_global_error(&format!("Flip failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }
                });
            });
        });
    }

    fn render_adjustment_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong("Adjustments");
                ui.add_space(HALF_SPACE);

                ui.horizontal(|ui| {
                    ui.label("Brightness:");
                    if ui
                        .add(egui::Slider::new(&mut self.brightness_value, -100..=100))
                        .changed()
                    {
                        // Apply brightness in real-time
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Contrast:");
                    if ui
                        .add(egui::Slider::new(&mut self.contrast_value, 0.0..=3.0))
                        .changed()
                    {
                        // Apply contrast in real-time
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Blur:");
                    if ui
                        .add(egui::Slider::new(&mut self.blur_sigma, 0.0..=10.0))
                        .changed()
                    {
                        // Apply blur in real-time
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("☀️ Apply Brightness").clicked() {
                        if let Err(e) = self.processor.adjust_brightness(self.brightness_value) {
                            append_global_error(&format!("Brightness adjustment failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }

                    if ui.button("🔆 Apply Contrast").clicked() {
                        if let Err(e) = self.processor.adjust_contrast(self.contrast_value) {
                            append_global_error(&format!("Contrast adjustment failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if ui.button("🌫️ Apply Blur").clicked() {
                        if let Err(e) = self.processor.blur(self.blur_sigma) {
                            append_global_error(&format!("Blur failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }

                    if ui.button("⚫ Grayscale").clicked() {
                        if let Err(e) = self.processor.to_grayscale() {
                            append_global_error(&format!("Grayscale conversion failed: {}", e));
                        } else {
                            self.update_texture(ui.ctx());
                        }
                    }
                });
            });
        });
    }

    fn render_compression_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong("Compression & Export");
                ui.add_space(HALF_SPACE);

                ui.horizontal(|ui| {
                    ui.label("Format:");
                    let response = egui::ComboBox::from_id_source("export_format")
                        .selected_text(self.selected_format.display_name())
                        .show_ui(ui, |ui| {
                            for format in ImageFormatType::all() {
                                ui.selectable_value(
                                    &mut self.selected_format,
                                    format,
                                    format.display_name(),
                                );
                            }
                        });

                    // Clear estimated size when format changes
                    if response.response.changed() {
                        self.estimated_size = None;
                    }
                });

                if self.selected_format == ImageFormatType::Jpeg {
                    ui.horizontal(|ui| {
                        ui.label("Quality:");
                        if ui
                            .add(egui::Slider::new(&mut self.compression_quality, 1..=100))
                            .changed()
                        {
                            // Clear estimated size when quality changes
                            self.estimated_size = None;
                        }
                    });
                }

                if ui.button("📊 Estimate Size").clicked() {
                    let options = CompressionOptions {
                        quality: self.compression_quality,
                        format: self.selected_format,
                    };

                    match self.processor.estimate_compressed_size(&options) {
                        Ok(size) => {
                            self.estimated_size = Some(format_file_size(size as u64));
                        }
                        Err(e) => {
                            append_global_error(&format!("Size estimation failed: {}", e));
                            self.estimated_size = None;
                        }
                    }
                }

                // Display the estimated size if available
                if let Some(size) = &self.estimated_size {
                    ui.label(format!("Estimated size: {}", size));
                }

                if ui.button("💾 Save Compressed").clicked() {
                    let extension = self.selected_format.extension();
                    let default_filename = format!("compressed.{}", extension);
                    match native_dialog::DialogBuilder::file()
                        .set_title("Save Compressed Image")
                        .set_filename(&default_filename)
                        .add_filter(&self.selected_format.display_name(), &[extension])
                        .save_single_file()
                        .show()
                    {
                        Ok(Some(mut path)) => {
                            // Ensure the file has the correct extension
                            if path.extension().is_none() {
                                path.set_extension(extension);
                            }

                            let options = CompressionOptions {
                                quality: self.compression_quality,
                                format: self.selected_format,
                            };

                            if let Err(e) = self.processor.compress_and_save(&path, &options) {
                                append_global_error(&format!("Compression failed: {}", e));
                            }
                        }
                        Ok(None) => {
                            // User canceled, do nothing
                        }
                        Err(e) => {
                            append_global_error(&format!("Failed to open save dialog: {}", e));
                        }
                    }
                }
            });
        });
    }

    fn render_action_buttons(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("🗑️ Clear").clicked() {
                self.processor.clear();
                self.texture_handle = None;
                self.resize_width.clear();
                self.resize_height.clear();
                self.brightness_value = 0;
                self.contrast_value = 1.0;
                self.blur_sigma = 0.0;
                self.estimated_size = None; // Clear estimate when clearing all
            }
        });
    }

    fn render_image_display(&mut self, ui: &mut Ui) {
        if let Some(texture) = self.texture_handle.clone() {
            let available_size = ui.available_size();
            let texture_size = texture.size_vec2();

            // Calculate display size maintaining aspect ratio
            let scale = (available_size.x / texture_size.x)
                .min(available_size.y / texture_size.y)
                .min(1.0)
                * self.display_scale;
            let display_size = texture_size * scale;

            ui.horizontal(|ui| {
                ui.label("Zoom:");
                if ui.button("🔍-").clicked() && self.display_scale > 0.1 {
                    self.display_scale *= 0.8;
                }
                ui.label(format!("{:.0}%", self.display_scale * 100.0));
                if ui.button("🔍+").clicked() && self.display_scale < 5.0 {
                    self.display_scale *= 1.25;
                }
                if ui.button("🔄").clicked() {
                    self.display_scale = 1.0;
                }
            });

            let is_cropping = self.is_cropping;
            let has_image = self.processor.current_image().is_some();
            let crop_start = self.crop_start;
            let crop_end = self.crop_end;

            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let image_response = ui.add(
                        egui::Image::from_texture(&texture)
                            .max_size(display_size)
                            .sense(Sense::click_and_drag()),
                    );

                    // Handle cropping interaction
                    if is_cropping && has_image {
                        self.handle_crop_interaction(&image_response, display_size, texture_size);
                    }

                    // Draw crop selection overlay
                    if let (Some(start), Some(end)) = (crop_start, crop_end) {
                        let painter = ui.painter();
                        let rect = Rect::from_two_pos(start, end);
                        painter.rect_stroke(
                            rect,
                            0.0,
                            Stroke::new(2.0, Color32::YELLOW),
                            egui::StrokeKind::Outside,
                        );
                        painter.rect_filled(
                            rect,
                            0.0,
                            Color32::from_rgba_premultiplied(255, 255, 0, 30),
                        );
                    }
                });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No image loaded. Click 'Open Image' to get started.");
            });
        }
    }

    fn handle_crop_interaction(
        &mut self,
        response: &Response,
        display_size: Vec2,
        texture_size: Vec2,
    ) {
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let rect = response.rect;
                let relative_pos = pos - rect.min;

                // Convert display coordinates to image coordinates
                let scale_x = texture_size.x / display_size.x;
                let scale_y = texture_size.y / display_size.y;

                let image_pos = Pos2::new(
                    (relative_pos.x * scale_x).clamp(0.0, texture_size.x),
                    (relative_pos.y * scale_y).clamp(0.0, texture_size.y),
                );

                if self.crop_start.is_none() {
                    self.crop_start = Some(pos);
                }
            }
        }

        if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.crop_end = Some(pos);
            }
        }

        if response.drag_stopped() {
            if let (Some(start), Some(end)) = (self.crop_start, self.crop_end) {
                let rect = response.rect;

                // Convert display coordinates to image coordinates
                let scale_x = texture_size.x / display_size.x;
                let scale_y = texture_size.y / display_size.y;

                let start_rel = start - rect.min;
                let end_rel = end - rect.min;

                let img_start_x = (start_rel.x * scale_x).clamp(0.0, texture_size.x) as u32;
                let img_start_y = (start_rel.y * scale_y).clamp(0.0, texture_size.y) as u32;
                let img_end_x = (end_rel.x * scale_x).clamp(0.0, texture_size.x) as u32;
                let img_end_y = (end_rel.y * scale_y).clamp(0.0, texture_size.y) as u32;

                let x = img_start_x.min(img_end_x);
                let y = img_start_y.min(img_end_y);
                let width = img_start_x.max(img_end_x) - x;
                let height = img_start_y.max(img_end_y) - y;

                if width > 0 && height > 0 {
                    self.crop_rect = Some(CropRect::new(x, y, width, height));
                }

                self.crop_start = None;
                self.crop_end = None;
            }
        }
    }

    fn update_texture(&mut self, ctx: &Context) {
        if let Some(image) = self.processor.current_image() {
            let color_image = dynamic_image_to_color_image(image);
            self.texture_handle =
                Some(ctx.load_texture("current_image", color_image, TextureOptions::default()));
        }
    }

    fn update_ui_from_image(&mut self) {
        if let Some(info) = self.processor.image_info() {
            self.resize_width = info.width.to_string();
            self.resize_height = info.height.to_string();
        }
    }
}

fn dynamic_image_to_color_image(image: &DynamicImage) -> ColorImage {
    let rgba = image.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    ColorImage::from_rgba_unmultiplied(size, rgba.as_raw())
}

fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}
