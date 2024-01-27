use eframe::egui::{self};

#[derive(Default)]
pub enum Menu {
    #[default]
    Home,
    ColorPicker,
    Jwt,
    Settings,
    About,
}

pub trait MenuTrait {
    fn init_menu(&mut self, ctx: &egui::Context);
}

// impl MenuTrait for MyApp {
//     fn init_menu(&mut self, ctx: &egui::Context) {
//         SidePanel::left("side_panel").show(ctx, |ui| {
//             ui.vertical(|ui| {
//                 if ui.button("Home").clicked() {
//                     self.current_menu = Menu::Home;
//                 }
//                 if ui.button("ColorPicker").clicked() {
//                     self.current_menu = Menu::ColorPicker;
//                 }
//                 if ui.button("JWT").clicked() {
//                     self.current_menu = Menu::Jwt;
//                 }
//                 if ui.button("Settings").clicked() {
//                     self.current_menu = Menu::Settings;
//                 }
//                 if ui.button("About").clicked() {
//                     self.current_menu = Menu::About;
//                 }
//             });
//         });
//     }
// }
