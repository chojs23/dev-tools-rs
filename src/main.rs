use eframe::egui::{self, CentralPanel, SidePanel};

#[derive(Default)]
enum Menu {
    #[default]
    Home,
    Settings,
    About,
}

#[derive(Default)]
struct MyApp {
    current_menu: Menu,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button("Home").clicked() {
                    self.current_menu = Menu::Home;
                }
                if ui.button("Settings").clicked() {
                    self.current_menu = Menu::Settings;
                }
                if ui.button("About").clicked() {
                    self.current_menu = Menu::About;
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| match self.current_menu {
            Menu::Home => {
                ui.label("Home");
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
