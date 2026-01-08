use eframe::egui;
use etfra::prelude::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([512.0, 512.0])
            .with_min_inner_size([256.0, 256.0]),
        ..Default::default()
    };

    eframe::run_native(
        "etfra-viewer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
