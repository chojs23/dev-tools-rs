use eframe::egui::{ScrollArea, TextEdit, Ui};
use crate::context::FrameCtx;
use super::{SPACE, HALF_SPACE};

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

    pub fn render(
        &self,
        input_text: &mut String,
        output_text: &mut String,
        ui: &mut Ui,
    ) {
        ui.vertical(|ui| {
            // Input section
            ui.label(&self.input_label);
            ui.add_space(HALF_SPACE);
            
            let mut input_edit = TextEdit::multiline(input_text)
                .desired_rows(self.input_rows)
                .desired_width(ui.available_width());
            
            if let Some(hint) = &self.input_hint {
                input_edit = input_edit.hint_text(hint);
            }
            
            ui.add(input_edit);
            
            ui.add_space(SPACE);
            
            // Output section
            ui.label(&self.output_label);
            ui.add_space(HALF_SPACE);
            
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(200.0)
                .show(ui, |ui| {
                    let mut output_edit = TextEdit::multiline(output_text)
                        .desired_rows(self.output_rows)
                        .desired_width(ui.available_width())
                        .interactive(false);
                    
                    if let Some(hint) = &self.output_hint {
                        output_edit = output_edit.hint_text(hint);
                    }
                    
                    ui.add(output_edit);
                });
        });
    }
}

