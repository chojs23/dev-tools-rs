#![cfg(windows)]

use std::ptr::null;

use crate::{color::Color, display_picker::DisplayPicker};
use anyhow::{Context, Error, Result};
use egui::Color32;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, LPARAM, LRESULT, POINT, WPARAM},
        Graphics::Gdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDC, GetPixel,
            Rectangle, ReleaseDC, SelectObject, SetStretchBltMode, StretchBlt, UpdateWindow,
            CLR_INVALID, COLORONCOLOR, HBITMAP, HDC, SRCCOPY,
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, GetCursorPos, GetDesktopWindow,
            MoveWindow, RegisterClassExW, ShowWindow, WNDCLASSEXW,
        },
    },
};

pub use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        SHOW_WINDOW_CMD, SW_SHOWDEFAULT, WINDOW_EX_STYLE, WINDOW_STYLE, WS_BORDER, WS_POPUP,
    },
};

macro_rules! handle_winapi_call {
    ($result:expr, $msg:expr) => {
        if !unsafe { $result }.as_bool() {
            return Err(Error::msg($msg));
        }
    };
}

pub struct Bitmap {
    hbitmap: HBITMAP,
    width: i32,
    height: i32,
}

// pub struct DeviceContext {
//     window: HWND,
//     device: HDC,
// }

// impl Drop for DeviceContext {
//     fn drop(&mut self) {
//         unsafe { ReleaseDC(self.window, self.device) };
//     }
// }

pub trait DisplayPickerExt: DisplayPicker {
    #[allow(clippy::too_many_arguments)]
    fn spawn_window(
        &self,
        class_name: &str,
        window_name: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        style: WINDOW_STYLE,
    ) -> Result<HWND>;

    fn show_window(&self, hwnd: HWND, display_mode: SHOW_WINDOW_CMD) -> Result<()>;
    fn destroy_window(&self, hwnd: HWND) -> Result<()>;
    fn update_window(&self, hwnd: HWND) -> Result<()>;
    fn move_window(&self, hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()>;
    fn get_image(
        &self,
        hwnd: Option<HWND>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<Bitmap>;
    fn get_screenshot(&self, x: i32, y: i32, width: i32, height: i32) -> Result<Bitmap>;
    fn render_bitmap(
        &self,
        bitmap: &Bitmap,
        hwnd: HWND,
        xdest: i32,
        ydest: i32,
        scale: f32,
    ) -> Result<()>;
    fn draw_rectangle(
        &self,
        hwnd: HWND,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        transparent_bg: bool,
    ) -> Result<()>;
}

#[derive(Default, Debug, Clone)]
pub struct WinConn {
    device_context: HDC,
    hinstance: HINSTANCE,
}

impl WinConn {
    pub fn new() -> Result<Self> {
        Ok(WinConn {
            device_context: unsafe { GetDC(None) },
            hinstance: unsafe {
                GetModuleHandleW(PCWSTR::null()).context("failed to get module handle")?
            },
        })
    }

    pub fn get_cursor_pos(&self) -> Result<POINT> {
        let mut cursor_pos = POINT::default();

        handle_winapi_call!(
            GetCursorPos(&mut cursor_pos as *mut POINT),
            "failed to get cursor position"
        );

        Ok(cursor_pos)
    }

    pub fn get_pixel(&self, pos: POINT) -> Result<u32> {
        let color = unsafe { GetPixel(self.device_context, pos.x, pos.y) };
        if color == CLR_INVALID {
            return Err(Error::msg("failed to get pixel"));
        }
        Ok(color)
    }
}

impl Drop for WinConn {
    fn drop(&mut self) {
        unsafe { ReleaseDC(GetDesktopWindow(), self.device_context) };
    }
}

impl DisplayPicker for WinConn {
    fn get_cursor_pos(&self) -> Result<(i32, i32)> {
        self.get_cursor_pos().map(|pos| (pos.x, pos.y))
    }

    fn get_color_under_cursor(&self) -> Result<Color> {
        let cursor_pos = self.get_cursor_pos()?;

        let color = self.get_pixel(cursor_pos)?;

        let r = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let b = ((color >> 16) & 0xff) as u8;

        Ok(Color::from(Color32::from_rgb(r, g, b)))
    }
}

impl DisplayPickerExt for WinConn {
    fn spawn_window(
        &self,
        class_name: &str,
        window_name: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        style: WINDOW_STYLE,
    ) -> Result<HWND> {
        let class_name = class_name.encode_utf16().collect::<Vec<_>>();

        let mut class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            ..Default::default()
        };
        class.lpszClassName = PCWSTR::from_raw(class_name.as_ptr());
        class.hInstance = self.hinstance;
        class.lpfnWndProc = Some(wnd_proc);

        let window_name = window_name.encode_utf16().collect::<Vec<_>>();

        unsafe { RegisterClassExW(&class as *const WNDCLASSEXW) };

        unsafe {
            Ok(CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PCWSTR::from_raw(class_name.as_ptr()),
                PCWSTR::from_raw(window_name.as_ptr()),
                style,
                x,
                y,
                width,
                height,
                None,
                None,
                self.hinstance,
                null(),
            ))
        }
    }

    fn show_window(&self, hwnd: HWND, display_mode: SHOW_WINDOW_CMD) -> Result<()> {
        handle_winapi_call!(ShowWindow(hwnd, display_mode), "failed to show window");
        Ok(())
    }

    fn destroy_window(&self, hwnd: HWND) -> Result<()> {
        handle_winapi_call!(DestroyWindow(hwnd), "failed to destroy window");
        Ok(())
    }

    fn update_window(&self, hwnd: HWND) -> Result<()> {
        handle_winapi_call!(UpdateWindow(hwnd), "failed to update window");
        Ok(())
    }

    fn move_window(&self, hwnd: HWND, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        handle_winapi_call!(
            MoveWindow(hwnd, x, y, width, height, true),
            "failed to move window"
        );
        Ok(())
    }

    fn get_image(
        &self,
        hwnd: Option<HWND>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<Bitmap> {
        // Initialize device contexts
        let hwdc = unsafe { GetDC(hwnd) };
        let hwdc_mem = unsafe { CreateCompatibleDC(hwdc) };

        // Set stretch mode of the memory device context to color
        unsafe { SetStretchBltMode(hwdc_mem, COLORONCOLOR) };

        // Initialize bitmap compatible with the device
        let screen_bitmap = unsafe { CreateCompatibleBitmap(hwdc, width, height) };

        // Assign screen_bitmap to the device
        let old_bitmap = unsafe { SelectObject(hwdc_mem, screen_bitmap) };

        // Do a bit transfer of a window to to the bitmap device
        handle_winapi_call!(
            BitBlt(hwdc_mem, 0, 0, width, height, hwdc, x, y, SRCCOPY),
            "failed to block-transfer bitmap to device context"
        );

        unsafe { SelectObject(hwdc_mem, old_bitmap) };
        handle_winapi_call!(DeleteDC(hwdc_mem), "failed to delete device context");
        unsafe { ReleaseDC(hwnd, hwdc) };

        Ok({
            Bitmap {
                hbitmap: screen_bitmap,
                width,
                height,
            }
        })
    }

    fn get_screenshot(&self, x: i32, y: i32, width: i32, height: i32) -> Result<Bitmap> {
        self.get_image(None, x, y, width, height)
    }

    fn render_bitmap(
        &self,
        bitmap: &Bitmap,
        hwnd: HWND,
        xdest: i32,
        ydest: i32,
        scale: f32,
    ) -> Result<()> {
        let hwdc = unsafe { GetDC(hwnd) };
        let hwdc_mem = unsafe { CreateCompatibleDC(hwdc) };

        // Select the new bitmap into the memory device returning the old bitmap
        let old_bitmap = unsafe { SelectObject(hwdc_mem, bitmap.hbitmap) };

        let new_width = (bitmap.width as f32 * scale) as i32;
        let new_height = (bitmap.height as f32 * scale) as i32;

        // Transfer new bitmap to the window device
        handle_winapi_call!(
            StretchBlt(
                hwdc,
                xdest,
                ydest,
                new_width,
                new_height,
                hwdc_mem,
                0,
                0,
                bitmap.width,
                bitmap.height,
                SRCCOPY,
            ),
            "failed to block-transfer bitmap to device context"
        );

        unsafe {
            // Cleanup old bitmap and devices
            SelectObject(hwdc_mem, old_bitmap);
            DeleteDC(hwdc_mem);
            ReleaseDC(hwnd, hwdc);
        }

        Ok(())
    }

    fn draw_rectangle(
        &self,
        hwnd: HWND,
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
        _transparent_bg: bool,
    ) -> Result<()> {
        let hdc = unsafe { GetDC(hwnd) };

        // figure out transparency

        handle_winapi_call!(
            Rectangle(hdc, left, top, right, bottom),
            "failed to draw rectangle"
        );
        Ok(())
    }
}

unsafe extern "system" fn wnd_proc(
    param0: HWND,
    param1: u32,
    param2: WPARAM,
    param3: LPARAM,
) -> LRESULT {
    DefWindowProcW(param0, param1, param2, param3)
}
