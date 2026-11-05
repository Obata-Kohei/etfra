mod app;
use app::app::EtfraApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "etfra",
        options,
        Box::new(|cc| Ok(Box::new(EtfraApp::new(cc)))),
    )
}