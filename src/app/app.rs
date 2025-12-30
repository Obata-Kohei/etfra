use eframe::egui;

struct App {
}

impl App {
    pub fn new() -> Self {
        Self {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello egui");          // ラベル
            ui.button("Button");             // ボタン
        });
    }
}