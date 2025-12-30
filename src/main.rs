use eframe::egui;
use etfra::prelude::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "etfra-viewer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

