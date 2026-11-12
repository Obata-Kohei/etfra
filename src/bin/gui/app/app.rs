use eframe::egui;

pub struct EtfraApp {
    // 今後、アプリケーションの状態を保持する
}

impl EtfraApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for EtfraApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        // UI実装時は設定操作をAppActionへ変換する。
        // 画像エクスポートはプレビューと別のExportManagerで実行する。
        // ExportOptions::default()はBurst解像度、resolution指定時は任意サイズ。
        /*
        // 上部ツールバー
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("etfra");
                ui.separator();
                ui.button("Redraw");
                ui.button("Save");
            });
        });

        // 左側設定パネル
        egui::SidePanel::left("settings")
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Settings");
                ui.label("Dynamics");
                ui.label("Coloring");
                ui.label("Resolution");
            });

        // 中央画像表示領域
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Fractal View");
            });
        });

        // 下部ステータスバー
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label("Ready");
        });
        */
    }
}