use crate::context::FrameCtx;
use eframe::egui::{CursorIcon, Ui};

/// Trait for UI components that can be rendered
pub trait UiComponent {
    /// Render the UI component
    fn render(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui);
}

/// Trait for UI panels and sections
pub trait UiPanel {
    /// Display the panel/section
    fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui);
}

/// Trait for UI windows
pub trait UiWindow {
    /// Show/hide the window
    fn toggle(&mut self);

    /// Check if window is open
    fn is_open(&self) -> bool;

    /// Display the window
    fn display(&mut self, ctx: &mut FrameCtx<'_>);
}
