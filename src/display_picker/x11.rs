#![cfg(unix)]
use std::borrow::Cow;

use crate::{color::Color, display_picker::DisplayPicker};
use anyhow::{Context, Result};
use eframe::egui::Color32;
use image::{imageops, ImageBuffer, Rgba};
use x11rb::{
    connection::Connection,
    cursor::Handle as CursorHandle,
    image::Image,
    protocol::xproto::{
        Arc, AtomEnum, ConfigureWindowAux, ConnectionExt, CreateGCAux, CreateWindowAux, EventMask,
        Gcontext, Gravity, PropMode, Screen, Window, WindowClass,
    },
    resource_manager::Database,
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

pub fn resize_image<'a>(img: &'a Image, scale: f32) -> Image<'a> {
    let data = img.data();
    let buffer: ImageBuffer<Rgba<u8>, &[u8]> =
        ImageBuffer::from_raw(img.width() as u32, img.height() as u32, data).unwrap();

    let width = (img.width() as f32 * scale) as u32;
    let height = (img.height() as f32 * scale) as u32;

    let resized = imageops::resize(&buffer, width, height, image::imageops::FilterType::Nearest);

    Image::new(
        width as u16,
        height as u16,
        img.scanline_pad(),
        img.depth(),
        img.bits_per_pixel(),
        img.byte_order(),
        Cow::Owned(resized.into_raw()),
    )
    .unwrap()
}

pub fn add_border<'a>(img: &'a Image, color: &Rgba<u8>, width: u32) -> Result<Image<'a>> {
    let data = img.data();
    let border_width = width;
    let width = img.width() as u32;
    let height = img.height() as u32;

    let base_image: ImageBuffer<Rgba<u8>, &[u8]> =
        ImageBuffer::from_raw(width, height, data).context("failed to initialize image buffer")?;

    let mut frame_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(width + border_width * 2, height + border_width * 2);

    imageops::vertical_gradient(&mut frame_buffer, color, color);

    imageops::replace(
        &mut frame_buffer,
        &base_image,
        border_width as i64,
        border_width as i64,
    );

    Image::new(
        (width + border_width * 2) as u16,
        (height + border_width * 2) as u16,
        img.scanline_pad(),
        img.depth(),
        img.bits_per_pixel(),
        img.byte_order(),
        Cow::Owned(frame_buffer.into_raw()),
    )
    .context("failed to create a new image with border")
}

pub enum WindowType {
    Desktop,
    Dock,
    Toolbar,
    Menu,
    Utility,
    Splash,
    Dialog,
    Normal,
    Notification,
}

impl WindowType {
    fn wm_property(&self) -> &[u8] {
        match &self {
            WindowType::Desktop => b"_NET_WM_WINDOW_TYPE_DESKTOP",
            WindowType::Dock => b"_NET_WM_WINDOW_TYPE_DOCK",
            WindowType::Toolbar => b"_NET_WM_WINDOW_TYPE_TOOLBAR",
            WindowType::Menu => b"_NET_WM_WINDOW_TYPE_MENU",
            WindowType::Utility => b"_NET_WM_WINDOW_TYPE_UTILITY",
            WindowType::Splash => b"_NET_WM_WINDOW_TYPE_SPLASH",
            WindowType::Dialog => b"_NET_WM_WINDOW_TYPE_DIALOG",
            WindowType::Normal => b"_NET_WM_WINDOW_TYPE_NORMAL",
            WindowType::Notification => b"_NET_WM_WINDOW_TYPE_NOTIFICATION",
        }
    }
}

pub trait DisplayPickerExt: DisplayPicker {
    fn conn(&self) -> &RustConnection;
    fn flush(&self) -> Result<()>;
    #[allow(clippy::too_many_arguments)]
    fn spawn_window(
        &self,
        title: &str,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        screen_num: usize,
        window_type: WindowType,
    ) -> Result<(Window, Gcontext)>;
    fn destroy_window(&self, window: Window) -> Result<()>;
    fn update_window_pos(&self, win_id: Window, x: i32, y: i32) -> Result<()>;
    fn screen_num(&self) -> usize;
    fn get_image(&self, window: Window, x: i16, y: i16, width: u16, height: u16) -> Result<Image>;
    fn screen(&self) -> &Screen;
    fn draw_circle(
        &self,
        window: Window,
        gc: Gcontext,
        x: i16,
        y: i16,
        diameter: u16,
    ) -> Result<()>;
}

#[derive(Debug)]
pub struct X11Conn {
    conn: RustConnection,
    screen_num: usize,
}

impl X11Conn {
    pub fn new() -> Result<Self> {
        let (conn, screen_num) = x11rb::connect(None).context("failed to connect to x11 server")?;

        Ok(Self { conn, screen_num })
    }

    pub fn conn(&self) -> &RustConnection {
        &self.conn
    }

    pub fn screen_num(&self) -> usize {
        self.screen_num
    }

    pub fn screen(&self) -> &Screen {
        &self.conn.setup().roots[self.screen_num]
    }

    pub fn flush(&self) -> Result<()> {
        self.conn
            .flush()
            .context("failed to flush connection")
            .map(|_| ())
    }

    pub fn get_image(
        &self,
        window: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Image> {
        Image::get(&self.conn, window, x, y, width, height).context("failed to get image")
    }

    pub fn get_cursor_xy(&self, window: Window) -> Result<(i16, i16)> {
        self.conn
            .query_pointer(window)
            .context("connection failed")?
            .reply()
            .context("failed to query pointer")
            .map(|reply| (reply.root_x, reply.root_y))
    }

    pub fn get_color(&self, window: Window, x: i16, y: i16) -> Result<(u8, u8, u8)> {
        let img = self.get_image(window, x, y, 1, 1)?;
        let pixel = img.get_pixel(0, 0);

        let red = (pixel >> 8) & 0xff;
        let green = (pixel >> 16) & 0xff;
        let blue = (pixel >> 24) & 0xff;

        Ok((red as u8, green as u8, blue as u8))
    }

    pub fn get_color_for_screen(&self, screen: &Screen) -> Result<(u8, u8, u8)> {
        let (x, y) = self.get_cursor_xy(screen.root)?;
        self.get_color(screen.root, x, y)
    }

    pub fn get_color_for_conn(&self) -> Result<(u8, u8, u8)> {
        self.get_color_for_screen(self.screen())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spawn_window(
        &self,
        title: &str,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        screen_num: usize,
        window_type: WindowType,
    ) -> Result<(Window, Gcontext)> {
        let screen = self.screen();
        let win_id = self
            .conn
            .generate_id()
            .context("failed to generate window ID")?;
        let gc_id = self
            .conn
            .generate_id()
            .context("failed to generate gc ID")?;
        let resource_db = Database::new_from_default(&self.conn)
            .context("failed to initialize resource database")?;
        let cursor_handle = CursorHandle::new(&self.conn, screen_num, &resource_db)
            .context("failed to aquire cursor handle")?;

        let wm_protocols = self
            .conn
            .intern_atom(false, b"WM_PROTOCOLS")
            .context("initializing atom WM_PROTOCOLS failed")?;
        let wm_delete_window = self
            .conn
            .intern_atom(false, b"WM_DELETE_WINDOW")
            .context("initializing atom WM_DELETE_WINDOW failed")?;
        let net_wm_name = self
            .conn
            .intern_atom(false, b"_NET_WM_NAME")
            .context("initializing atom _NET_WM_NAME failed")?;
        let utf8_string = self
            .conn
            .intern_atom(false, b"UTF8_STRING")
            .context("initializing atom UTF8_STRING failed")?;
        let net_wm_window_type = self
            .conn
            .intern_atom(false, b"_NET_WM_WINDOW_TYPE")
            .context("initializing atom _NET_WM_WINDOW_TYPE failed")?;
        let net_wm_window_value = self
            .conn
            .intern_atom(false, window_type.wm_property())
            .context("initializing atom _NET_WM_WINDOW_TYPE_ failed")?;
        let wm_protocols = wm_protocols
            .reply()
            .context("failed to get reply for WM_PROTOCOLS atom")?
            .atom;
        let wm_delete_window = wm_delete_window
            .reply()
            .context("failed to get reply for WM_DELETE_WINDOW atom")?
            .atom;
        let net_wm_name = net_wm_name
            .reply()
            .context("failed to get reply for _NET_WM_NAME atom")?
            .atom;
        let utf8_string = utf8_string
            .reply()
            .context("failed to get reply for UTF8_STRING atom")?
            .atom;
        let net_wm_window_type = net_wm_window_type
            .reply()
            .context("failed to get reply for _NET_WM_WINDOW_TYPE atom")?
            .atom;
        let net_wm_window_value = net_wm_window_value
            .reply()
            .context("failed to get reply for _NET_WM_WINDOW_TYPE_DIALOG atom")?
            .atom;
        let cursor_handle = cursor_handle
            .reply()
            .context("failed to get reply for cursor handle")?;

        let win_aux = CreateWindowAux::new()
            .event_mask(EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY | EventMask::NO_EVENT)
            .background_pixel(screen.white_pixel)
            .win_gravity(Gravity::NORTH_WEST)
            // Just because, we set the cursor to "wait"
            .cursor(
                cursor_handle
                    .load_cursor(&self.conn, "wait")
                    .context("failed to load cursor")?,
            );

        let gc_aux = CreateGCAux::new().foreground(screen.white_pixel);

        self.conn
            .create_window(
                screen.root_depth,
                win_id,
                screen.root,
                x,
                y,
                width,
                height,
                0,
                WindowClass::INPUT_OUTPUT,
                0,
                &win_aux,
            )
            .context("failed to create window")?;

        self.conn
            .change_property8(
                PropMode::REPLACE,
                win_id,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                title.as_bytes(),
            )
            .context("failed to change WM_NAME property")?;
        self.conn
            .change_property8(
                PropMode::REPLACE,
                win_id,
                net_wm_name,
                utf8_string,
                title.as_bytes(),
            )
            .context("failed to change _NET_WM_NAME property")?;
        self.conn
            .change_property32(
                PropMode::REPLACE,
                win_id,
                wm_protocols,
                AtomEnum::ATOM,
                &[wm_delete_window],
            )
            .context("failed to change WM_PROTOCOLS property")?;
        self.conn
            .change_property32(
                PropMode::REPLACE,
                win_id,
                net_wm_window_type,
                AtomEnum::ATOM,
                &[net_wm_window_value],
            )
            .context("failed to change _NET_WM_WINDOW_TYPE property")?;

        self.conn
            .create_gc(gc_id, win_id, &gc_aux)
            .context("failed to create a gc")?;

        self.conn
            .map_window(win_id)
            .context("faield to map window ID")?;

        self.flush()?;

        Ok((win_id, gc_id))
    }

    pub fn destroy_window(&self, window: Window) -> Result<()> {
        self.conn
            .destroy_window(window)
            .context("failed to destroy window")
            .map(|_| ())
    }

    pub fn update_window_pos(&self, win_id: Window, x: i32, y: i32) -> Result<()> {
        self.conn
            .configure_window(win_id, &ConfigureWindowAux::new().x(Some(x)).y(Some(y)))
            .context("failed to reconfigure window position")
            .map(|_| ())
    }

    pub fn draw_circle(
        &self,
        window: Window,
        gc: Gcontext,
        x: i16,
        y: i16,
        diameter: u16,
    ) -> Result<()> {
        self.conn
            .poly_arc(
                window,
                gc,
                &[Arc {
                    x,
                    y,
                    width: diameter,
                    height: diameter,
                    angle1: 0,
                    angle2: 360 * 64,
                }],
            )
            .context("failed to draw a circle")
            .map(|_| ())
    }
}

impl DisplayPicker for X11Conn {
    fn get_cursor_pos(&self) -> Result<(i32, i32)> {
        self.get_cursor_xy(self.screen().root)
            .map(|(x, y)| (x as i32, y as i32))
    }

    fn get_color_under_cursor(&self) -> Result<Color> {
        self.get_color_for_conn()
            .map(|color| Color32::from_rgb(color.0, color.1, color.2).into())
    }
}

impl DisplayPickerExt for X11Conn {
    fn conn(&self) -> &RustConnection {
        self.conn()
    }
    fn flush(&self) -> Result<()> {
        self.flush()
    }
    fn spawn_window(
        &self,
        title: &str,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        screen_num: usize,
        window_type: WindowType,
    ) -> Result<(Window, Gcontext)> {
        self.spawn_window(title, x, y, width, height, screen_num, window_type)
    }
    fn destroy_window(&self, window: Window) -> Result<()> {
        self.destroy_window(window)
    }
    fn update_window_pos(&self, win_id: Window, x: i32, y: i32) -> Result<()> {
        self.update_window_pos(win_id, x, y)
    }
    fn screen_num(&self) -> usize {
        self.screen_num()
    }
    fn get_image(&self, window: Window, x: i16, y: i16, width: u16, height: u16) -> Result<Image> {
        self.get_image(window, x, y, width, height)
    }
    fn screen(&self) -> &Screen {
        self.screen()
    }
    fn draw_circle(
        &self,
        window: Window,
        gc: Gcontext,
        x: i16,
        y: i16,
        diameter: u16,
    ) -> Result<()> {
        self.draw_circle(window, gc, x, y, diameter)
    }
}
