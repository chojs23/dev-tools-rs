use super::SPACE;
use eframe::egui::{Align, Layout, Resize, ScrollArea, TextEdit, Ui};

pub struct InputOutputBox {
    input_label: String,
    output_label: String,
    input_hint: Option<String>,
    output_hint: Option<String>,
    input_rows: usize,
    output_rows: usize,
}

impl InputOutputBox {
    pub fn new(input_label: &str, output_label: &str) -> Self {
        Self {
            input_label: input_label.to_string(),
            output_label: output_label.to_string(),
            input_hint: None,
            output_hint: None,
            input_rows: 8,
            output_rows: 8,
        }
    }

    pub fn with_input_hint(mut self, hint: &str) -> Self {
        self.input_hint = Some(hint.to_string());
        self
    }

    pub fn with_output_hint(mut self, hint: &str) -> Self {
        self.output_hint = Some(hint.to_string());
        self
    }

    pub fn with_input_rows(mut self, rows: usize) -> Self {
        self.input_rows = rows;
        self
    }

    pub fn with_output_rows(mut self, rows: usize) -> Self {
        self.output_rows = rows;
        self
    }

    pub fn render(&self, input_text: &mut String, output_text: &mut String, ui: &mut Ui) {
        ui.add_space(SPACE);
        ui.label(&self.input_label);
        Resize::default()
            .id_salt("input_box")
            .default_height(ui.available_height() * 0.5)
            .show(ui, |ui| {
                ui.set_max_height(ui.available_height() * 1.0);
                ScrollArea::vertical()
                    .id_salt("input_scroll")
                    .show(ui, |ui| {
                        ui.set_max_height(ui.available_height() * 1.0);
                        ui.set_max_width(ui.available_width() * 1.0);

                        ui.with_layout(
                            Layout::top_down(Align::Min)
                                .with_main_justify(true)
                                .with_cross_justify(true),
                            |ui| {
                                let mut input_edit = TextEdit::multiline(input_text)
                                    .desired_rows(self.input_rows)
                                    .desired_width(ui.available_width());

                                if let Some(hint) = &self.input_hint {
                                    input_edit = input_edit.hint_text(hint);
                                }

                                ui.add(input_edit)
                            },
                        );
                    });
            });

        ui.add_space(SPACE);
        ui.label(&self.output_label);

        Resize::default()
            .id_salt("output_box")
            .default_height(ui.available_height() * 0.5)
            .show(ui, |ui| {
                ui.set_max_height(ui.available_height() * 1.0);
                ScrollArea::vertical()
                    .id_salt("output_scroll")
                    .stick_to_bottom(false)
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        ui.set_max_height(ui.available_height() * 1.0);
                        ui.set_max_width(ui.available_width() * 1.0);
                        let mut output_edit = TextEdit::multiline(output_text)
                            .desired_rows(self.output_rows)
                            .desired_width(ui.available_width())
                            .code_editor();

                        if let Some(hint) = &self.output_hint {
                            output_edit = output_edit.hint_text(hint);
                        }

                        ui.add(output_edit);
                    });
            });
    }
}
