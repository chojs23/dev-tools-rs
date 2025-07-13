#![allow(unused_imports)]
use crate::{
    app::CURRENT_COLOR_BOX_SIZE,
    context::FrameCtx,
    platform::display_picker::{self, DisplayPickerExt},
    types::error::append_global_error,
    ui::components::{
        colorbox::{ColorBox, COLORBOX_PICK_TOOLTIP},
        icon,
    },
};

use eframe::egui::{self, Button, CursorIcon, Ui};
use std::rc::Rc;

#[cfg(target_os = "linux")]
use x11rb::protocol::xproto;

#[cfg(windows)]
use crate::platform::display_picker::windows::{HWND, SW_SHOWDEFAULT, WS_BORDER, WS_POPUP};

#[cfg(any(target_os = "linux"))]
const ZOOM_IMAGE_WIDTH: u16 = ZOOM_WIN_WIDTH / ZOOM_SCALE as u16;
#[cfg(any(target_os = "linux"))]
const ZOOM_IMAGE_HEIGHT: u16 = ZOOM_WIN_HEIGHT / ZOOM_SCALE as u16;
#[cfg(any(target_os = "linux"))]
const ZOOM_WIN_BORDER_WIDTH: u32 = 2;
#[cfg(any(target_os = "linux", windows))]
static CURSOR_PICKER_WINDOW_NAME: &str = "cursor picker";
#[cfg(any(target_os = "linux", windows))]
const ZOOM_SCALE: f32 = 10.;
#[cfg(any(target_os = "linux", windows))]
const ZOOM_WIN_WIDTH: u16 = 160;
#[cfg(any(target_os = "linux", windows))]
const ZOOM_WIN_HEIGHT: u16 = 160;
#[cfg(any(target_os = "linux", windows))]
const ZOOM_WIN_OFFSET: i32 = 50;
#[cfg(any(target_os = "linux", windows))]
const ZOOM_WIN_POINTER_DIAMETER: u16 = 10;
#[cfg(windows)]
const ZOOM_WIN_POINTER_RADIUS: u16 = ZOOM_WIN_POINTER_DIAMETER / 2;
#[cfg(any(target_os = "linux", windows))]
const ZOOM_IMAGE_X_OFFSET: i32 = ((ZOOM_WIN_WIDTH / 2) as f32 / ZOOM_SCALE) as i32;
#[cfg(any(target_os = "linux", windows))]
const ZOOM_IMAGE_Y_OFFSET: i32 = ((ZOOM_WIN_HEIGHT / 2) as f32 / ZOOM_SCALE) as i32;

pub struct ZoomPicker {
    pub display_picker: Option<Rc<dyn DisplayPickerExt>>,
    #[cfg(target_os = "linux")]
    picker_window: Option<(xproto::Window, xproto::Gcontext)>,
    #[cfg(windows)]
    picker_window: Option<HWND>,
}

impl Default for ZoomPicker {
    fn default() -> Self {
        Self {
            display_picker: crate::platform::display_picker::init_display_picker(),
            #[cfg(target_os = "linux")]
            picker_window: None,
            #[cfg(windows)]
            picker_window: None,
        }
    }
}
impl ZoomPicker {
    pub fn display(&mut self, ctx: &mut FrameCtx<'_>, ui: &mut Ui) {
        if ctx.app.color_picking_enabled {
            if let Some(picker) = self.display_picker.clone() {
                if let Ok(color) = picker.get_color_under_cursor() {
                    ctx.app.cursor_pick_color = color;
                    ui.horizontal(|ui| {
                        ui.label("Color at cursor: ");
                        self.zoom_picker_impl(ctx, ui, picker);
                    });
                    let cb = ColorBox::builder()
                        .size((CURRENT_COLOR_BOX_SIZE, CURRENT_COLOR_BOX_SIZE))
                        .color(color)
                        .label(true)
                        .hover_help(COLORBOX_PICK_TOOLTIP)
                        .border(true)
                        .build();
                    ui.horizontal(|ui| {
                        cb.display(ctx, ui);
                    });
                }
            };
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn display_zoom_window(&mut self, ctx: &mut FrameCtx<'_>, picker: &Rc<dyn DisplayPickerExt>) {
        if self.picker_window.is_none() {
            ctx.app.toggle_mouse(CursorIcon::Crosshair);
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();

            #[cfg(target_os = "linux")]
            if let Ok(window) = picker.spawn_window(
                CURSOR_PICKER_WINDOW_NAME,
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i16,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i16,
                ZOOM_WIN_WIDTH + (ZOOM_WIN_BORDER_WIDTH * 2) as u16,
                ZOOM_WIN_HEIGHT + (ZOOM_WIN_BORDER_WIDTH * 2) as u16,
                picker.screen_num(),
                display_picker::x11::WindowType::Notification,
            ) {
                self.picker_window = Some(window);
            }

            #[cfg(windows)]
            if let Ok(window) = picker.spawn_window(
                "DEV_TOOLS_DIALOG",
                CURSOR_PICKER_WINDOW_NAME,
                cursor_pos.0 - ZOOM_IMAGE_X_OFFSET,
                cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET,
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
                WS_POPUP | WS_BORDER,
            ) {
                self.picker_window = Some(window);
                if let Err(e) = picker.show_window(window, SW_SHOWDEFAULT) {
                    append_global_error(e);
                }
            }
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn hide_zoom_window(&mut self, picker: &Rc<dyn DisplayPickerExt>) {
        if let Some(picker_window) = self.picker_window {
            #[cfg(target_os = "linux")]
            let _ = picker.destroy_window(picker_window.0);

            #[cfg(windows)]
            if let Err(e) = picker.destroy_window(picker_window) {
                append_global_error(e);
            }

            self.picker_window = None;
        }
    }

    #[cfg(target_os = "linux")]
    fn handle_zoom_picker(&mut self, _ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        if let Some((window, gc)) = self.picker_window {
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
            if let Ok(img) = picker.get_image(
                picker.screen().root,
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET) as i16,
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET) as i16,
                ZOOM_IMAGE_WIDTH,
                ZOOM_IMAGE_HEIGHT,
            ) {
                use image::Pixel;
                let white = image::Rgba::from_slice(&[255, 255, 255, 255]);
                let black = image::Rgba::from_slice(&[0, 0, 0, 255]);
                let img = display_picker::x11::resize_image(&img, ZOOM_SCALE);
                let img = display_picker::x11::add_border(&img, white, ZOOM_WIN_BORDER_WIDTH / 2)
                    .unwrap();
                let img = display_picker::x11::add_border(&img, black, ZOOM_WIN_BORDER_WIDTH / 2)
                    .unwrap();

                if let Err(e) = img.put(picker.conn(), window, gc, 0, 0) {
                    append_global_error(e);
                    return;
                };

                if let Err(e) = picker.draw_circle(
                    window,
                    gc,
                    (ZOOM_WIN_WIDTH / 2) as i16,
                    (ZOOM_WIN_HEIGHT / 2) as i16,
                    ZOOM_WIN_POINTER_DIAMETER,
                ) {
                    append_global_error(e);
                };
            }
            if let Err(e) = picker.update_window_pos(
                window,
                cursor_pos.0 + ZOOM_WIN_OFFSET,
                cursor_pos.1 + ZOOM_WIN_OFFSET,
            ) {
                append_global_error(e);
                return;
            }
            if let Err(e) = picker.flush() {
                append_global_error(e);
            }
        }
    }

    #[cfg(windows)]
    fn handle_zoom_picker(&mut self, _ui: &mut Ui, picker: Rc<dyn DisplayPickerExt>) {
        if let Some(window) = self.picker_window {
            let cursor_pos = picker.get_cursor_pos().unwrap_or_default();
            match picker.get_screenshot(
                (cursor_pos.0 - ZOOM_IMAGE_X_OFFSET),
                (cursor_pos.1 - ZOOM_IMAGE_Y_OFFSET),
                (ZOOM_WIN_WIDTH as f32 / ZOOM_SCALE) as i32,
                (ZOOM_WIN_HEIGHT as f32 / ZOOM_SCALE) as i32,
            ) {
                Ok(bitmap) => {
                    if let Err(e) = picker.render_bitmap(&bitmap, window, 0, 0, ZOOM_SCALE) {
                        append_global_error(e);
                    }
                    let left = ((ZOOM_WIN_WIDTH / 2) - ZOOM_WIN_POINTER_RADIUS) as i32;
                    let top = ((ZOOM_WIN_HEIGHT / 2) - ZOOM_WIN_POINTER_RADIUS) as i32;
                    if let Err(e) = picker.draw_rectangle(
                        window,
                        left,
                        top,
                        left + ZOOM_WIN_POINTER_DIAMETER as i32,
                        top + ZOOM_WIN_POINTER_DIAMETER as i32,
                        true,
                    ) {
                        append_global_error(e);
                    }
                }
                Err(e) => {
                    append_global_error(e);
                }
            }
            if let Err(e) = picker.move_window(
                window,
                (cursor_pos.0 + ZOOM_WIN_OFFSET),
                (cursor_pos.1 + ZOOM_WIN_OFFSET),
                ZOOM_WIN_WIDTH as i32,
                ZOOM_WIN_HEIGHT as i32,
            ) {
                append_global_error(e);
            }
        }
    }

    #[cfg(any(target_os = "linux", windows))]
    fn zoom_picker_impl(
        &mut self,
        ctx: &mut FrameCtx<'_>,
        ui: &mut Ui,
        picker: Rc<dyn DisplayPickerExt>,
    ) {
        let btn = Button::new(icon::ZOOM_PICKER).sense(egui::Sense::drag());
        let btn = ui
            .add(btn)
            .on_hover_cursor(CursorIcon::ZoomIn)
            .on_hover_text("Drag to enable zoomed window");

        if btn.dragged() {
            ctx.app.zoom_window_dragged = true;
            self.display_zoom_window(ctx, &picker);
        }
        if !btn.dragged() && !btn.has_focus() {
            self.hide_zoom_window(&picker);
            ctx.app.zoom_window_dragged = false;
        }

        self.handle_zoom_picker(ui, picker);
    }

    #[cfg(not(any(target_os = "linux", windows)))]
    fn zoom_picker_impl(&mut self, _: &mut FrameCtx<'_>, _: &mut Ui, _: Rc<dyn DisplayPickerExt>) {}
}
