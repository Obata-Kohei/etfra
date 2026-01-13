use eframe::egui;
use etfra::prelude::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "etfra-viewer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
