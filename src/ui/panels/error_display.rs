use eframe::egui::{self, Id, Label, RichText, Ui};
use eframe::epaint::Color32;

use crate::context::FrameCtx;
use crate::types::error::DisplayError;
use crate::ui::traits::UiComponent;

pub struct ErrorDisplay {
    pub errors: Vec<DisplayError>,
}

impl UiComponent for ErrorDisplay {
    fn render(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        let mut top_padding = 0.;
        let mut err_idx = 0;

        self.errors.retain(|e| {
            let elapsed = crate::elapsed(e.timestamp());
            if elapsed >= crate::app::ERROR_DISPLAY_DURATION {
                false
            } else {
                if let Some(rsp) = egui::Window::new("Error!")
                    .collapsible(true)
                    .id(Id::new(format!("err_ntf_{err_idx}")))
                    .anchor(
                        egui::Align2::RIGHT_BOTTOM,
                        (-ctx.app.sidepanel.box_width - 25., -top_padding),
                    )
                    .hscroll(true)
                    .fixed_size((ctx.app.sidepanel.box_width + 200., 50.))
                    .show(ui.ctx(), |ui| {
                        let label = Label::new(RichText::new(e.message()).color(Color32::RED));
                        ui.add(label);
                    })
                {
                    top_padding += rsp.response.rect.height() + 8.;
                    err_idx += 1;
                };
                true
            }
        });
    }
}

impl ErrorDisplay {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add_error(&mut self, error: DisplayError) {
        self.errors.push(error);
    }
}
