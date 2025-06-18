fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Dev tools",
        options,
        Box::new(|ctx| Ok(dev_tools::DevToolsApp::init(ctx))),
    )
}
