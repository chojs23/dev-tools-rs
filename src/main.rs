fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Dev-tools-rs",
        options,
        Box::new(|ctx| Ok(dev_tools_rs::DevToolsRsApp::init(ctx))),
    )
}
