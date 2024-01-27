mod menu;

use eframe::{
    egui::FontFamily::Proportional,
    egui::TextStyle::{Body, Button, Heading, Monospace, Small},
    egui::{self, CentralPanel},
    epaint::FontId,
};

use menu::{Menu, MenuTrait};

#[derive(Default)]
struct MyApp {
    current_menu: Menu,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();

        ctx.set_style(style);

        self.init_menu(ctx);

        CentralPanel::default().show(ctx, |ui| match self.current_menu {
            Menu::Home => {
                ui.label("Home");
            }
            Menu::ColorPicker => {
                ui.label("ColorPicker");
            }
            Menu::Jwt => {
                ui.label("JWT");
            }
            Menu::Settings => {
                ui.label("Settings");
            }
            Menu::About => {
                ui.label("About");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Dev tools",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}
