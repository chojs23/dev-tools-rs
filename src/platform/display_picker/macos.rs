use crate::core::color::Color;
use anyhow::{Error, Result};
use core_graphics::{
    base::CGFloat,
    display::{CGDirectDisplayID, CGGetDisplaysWithRect, CGPoint, CGRect, CGSize},
    sys::{CGEventRef, CGEventSourceRef, CGImageRef},
};
use objc::{class, msg_send, rc::autoreleasepool, runtime::Object, sel, sel_impl};
use std::ptr::null;

use crate::core::color::Rgb;

use super::DisplayPicker;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGDisplayCreateImageForRect(display: CGDirectDisplayID, rect: CGRect) -> CGImageRef;
    pub fn CGImageRelease(image: CGImageRef);
    pub fn CGEventCreate(src: *const CGEventSourceRef) -> CGEventRef;
    pub fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
}

#[derive(Debug)]
pub struct MacConn;
impl MacConn {
    pub fn get_cursor_pos(&self) -> CGPoint {
        autoreleasepool(|| unsafe {
            let event = CGEventCreate(null());
            CGEventGetLocation(event)
        })
    }
}

pub trait DisplayPickerExt: DisplayPicker {}

impl DisplayPickerExt for MacConn {}

impl DisplayPicker for MacConn {
    fn get_cursor_pos(&self) -> Result<(i32, i32)> {
        let pos = self.get_cursor_pos();
        Ok((pos.x as i32, pos.y as i32))
    }
    fn get_color_under_cursor(&self) -> Result<Color> {
        autoreleasepool(|| unsafe {
            let location = self.get_cursor_pos();

            let rect = CGRect::new(&location, &CGSize::new(1., 1.));
            let mut id = CGDirectDisplayID::default();
            let mut count = 0u32;
            CGGetDisplaysWithRect(rect, 1, &mut id as *mut _, &mut count as *mut _);

            let image = CGDisplayCreateImageForRect(id, rect);
            if image.is_null() {
                return Err(Error::msg("failed to acquire image of display"));
            }

            let cls = class!(NSBitmapImageRep);
            let img: *mut Object = msg_send![cls, alloc];
            let bitmap: *mut Object = msg_send![img, initWithCGImage: image];
            CGImageRelease(image);
            let color: *mut Object = msg_send![bitmap, colorAtX:0 y:0];

            let r = CGFloat::default();
            let g = CGFloat::default();
            let b = CGFloat::default();
            let a = CGFloat::default();
            let _: *mut Object = msg_send![color, getRed:&r green:&g blue:&b alpha:&a];
            Ok(Color::Rgb(Rgb::new(r as f32, g as f32, b as f32)))
        })
    }
}
