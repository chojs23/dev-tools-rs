// use crate::color::Gradient;
//
// use eframe::egui::{
//     pos2, Color32, ColorImage, CursorIcon, ImageButton, ImageData, Rect, Response, TextureId, Ui,
//     Vec2,
// };
// use std::{collections::HashMap, sync::Arc};
//
// pub type TextureAllocator =
//     Option<Arc<eframe::egui::mutex::RwLock<eframe::epaint::TextureManager>>>;
//
// pub fn render_color(
//     ui: &mut Ui,
//     tex_allocator: &mut TextureAllocator,
//     tex_mngr: &mut TextureManager,
//     color: Color32,
//     size: Vec2,
//     on_hover: Option<&str>,
//     border: bool,
// ) -> Option<Response> {
//     let gradient = Gradient::one_color(color);
//     render_gradient(
//         ui,
//         tex_allocator,
//         tex_mngr,
//         &gradient,
//         size,
//         on_hover,
//         border,
//     )
// }
//
// pub fn render_gradient(
//     ui: &mut Ui,
//     tex_allocator: &mut TextureAllocator,
//     tex_mngr: &mut TextureManager,
//     gradient: &Gradient,
//     size: Vec2,
//     on_hover: Option<&str>,
//     border: bool,
// ) -> Option<Response> {
//     if let Some(tex_allocator) = tex_allocator {
//         let resp = ui.horizontal(|ui| {
//             let tex = tex_mngr.get(tex_allocator, gradient);
//             let texel_offset = 0.5 / (gradient.0.len() as f32);
//             let uv = Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
//             let image = ImageButton::new(tex).frame(border).uv(uv);
//             let mut resp = ui.add(image).on_hover_cursor(CursorIcon::PointingHand);
//
//             if let Some(on_hover) = on_hover {
//                 resp = resp.on_hover_text(on_hover);
//             }
//
//             resp
//         });
//         return Some(resp.inner);
//     }
//     None
// }
//
// #[derive(Default, Debug)]
// pub struct TextureManager(HashMap<Gradient, TextureId>);
//
// impl TextureManager {
//     fn get(
//         &mut self,
//         tex_allocator: &mut std::sync::Arc<
//             eframe::egui::mutex::RwLock<eframe::epaint::TextureManager>,
//         >,
//         gradient: &Gradient,
//     ) -> TextureId {
//         *self.0.entry(gradient.clone()).or_insert_with(|| {
//             let pixels = gradient.to_pixel_row();
//             let width = pixels.len();
//             let height = 1;
//             let color_image = ColorImage {
//                 size: [width, height],
//                 pixels,
//             };
//             let image_data = ImageData::Color(color_image.into());
//             tex_allocator.write().alloc(
//                 "image".into(),
//                 image_data,
//                 eframe::egui::TextureOptions::default(),
//             )
//         })
//     }
// }
