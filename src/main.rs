use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "widget minimal",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp))),
    )
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello egui");          // ラベル
            ui.button("Button");             // ボタン
        });
    }
}
