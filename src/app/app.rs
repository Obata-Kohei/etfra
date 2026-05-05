use std::sync::mpsc::Receiver;
use std::sync::{atomic::AtomicBool, Arc};
use eframe::egui;
use std::thread::JoinHandle;

use crate::app::{
    key_input::handle_key_input,
    state::AppState,
    ui_render::{self, RenderEngine},
};
use crate::prelude::*;

pub struct App {
    pub state: AppState,
    pub texture: Option<egui::TextureHandle>,

    // メイン計算スレッド
    compute_handle: Option<JoinHandle<()>>,
    result_rx: Option<Receiver<(Box<dyn RenderEngine>, ImageConfig, Option<Vec<u8>>)>>,

    // エクスポート用スレッド（画面描画とは別に走らせる）
    export_handle: Option<JoinHandle<()>>,
    export_rx: Option<Receiver<Box<dyn RenderEngine>>>, // engineの返却受け取り用
    export_status: ExportStatus,
}

#[derive(Default)]
enum ExportStatus {
    #[default]
    Idle,
    Running,
    Done(String),   // 保存したファイルパス
    Failed(String), // エラーメッセージ
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::with_preset_values(),
            texture: None,
            compute_handle: None,
            result_rx: None,
            export_handle: None,
            export_rx: None,
            export_status: ExportStatus::Idle,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        handle_key_input(ctx, &mut self.state);

        if self.state.is_computing {
            ctx.request_repaint();
        }

        // 再計算要求が来たが、すでに計算中 -> キャンセル
        if self.state.recomp && self.compute_handle.is_some() {
            if let Some(cancel) = &self.state.cancel_flag {
                cancel.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }

        // 新規計算開始（engineがある場合のみ）
        if self.state.recomp && self.compute_handle.is_none() && self.state.engine.is_some() {
            self.start_compute_thread();
        }

        self.poll_compute_result();
        self.update_texture(ctx);

        // エクスポートスレッドの完了チェック
        self.poll_export_result();

        ui_render::show_left_panel(ctx, &mut self.state);
        ui_render::show_right_panel(ctx);
        ui_render::show_central_panel(ctx, &self.texture);

        // エクスポートダイアログ表示
        if let Some(req) = ui_render::show_export_dialog(ctx, &mut self.state) {
            self.start_export_thread(req.width, req.height);
        }

        // エクスポート状態の通知
        self.show_export_status(ctx);
    }
}

impl App {
    fn start_compute_thread(&mut self) {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        let img_cfg = self.state.img_cfg.clone();
        let mut engine = self.state.engine.take().expect("engine must exist");

        let cancel = Arc::new(AtomicBool::new(false));
        let cancel_child = cancel.clone();

        self.state.is_computing = true;
        self.state.recomp = false;

        self.compute_handle = Some(std::thread::spawn(move || {
            let result = engine.compute_par(&img_cfg, &cancel_child);
            let _ = tx.send((engine, img_cfg, result));
        }));

        self.state.cancel_flag = Some(cancel);
        self.result_rx = Some(rx);
    }

    fn poll_compute_result(&mut self) {
        let Some(rx) = &self.result_rx else { return };

        if let Ok((engine, img_cfg, result)) = rx.try_recv() {
            self.state.engine = Some(engine);

            match result {
                Some(buf) => {
                    self.state.img_cfg = img_cfg;
                    self.state.rgba_buf = Some(buf);
                    self.state.buf_dirty = true;
                }
                None => {
                    self.state.rgba_buf = None;
                    self.state.buf_dirty = false;
                }
            }

            self.state.is_computing = false;
            self.compute_handle = None;
            self.result_rx = None;
            self.state.cancel_flag = None;
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        if !self.state.buf_dirty {
            return;
        }

        let Some(buf) = &self.state.rgba_buf else {
            self.state.buf_dirty = false;
            return;
        };

        let (w, h) = self.state.img_cfg.resolution;
        let expected = w * h * 4;

        if buf.len() != expected {
            self.state.buf_dirty = false;
            return;
        }

        let img = egui::ColorImage::from_rgba_unmultiplied([w, h], buf);
        self.texture = Some(ctx.load_texture("rendered_image", img, egui::TextureOptions::NEAREST));
        self.state.buf_dirty = false;
    }

    /// 指定解像度でフラクタルを計算し PNG として保存するスレッドを起動する。
    /// scale は現在の view_size をそのまま使い、解像度に合わせて再スケールする。
    fn start_export_thread(&mut self, width: usize, height: usize) {
        // すでにエクスポート中なら無視
        if self.export_handle.is_some() {
            return;
        }

        // engine を一時的に取り出す。エクスポート完了後にチャンネルで返却される。
        let Some(mut engine) = self.state.engine.take() else {
            return;
        };

        // 現在の scale をそのまま使い、指定解像度の ImageConfig を作る。
        // （view_size を保ちたい場合は scale を再計算するが、ここでは scale 固定）
        let scale = self.state.img_cfg.scale;
        let export_cfg = ImageConfig::new(
            (width, height),
            scale,
            self.state.img_cfg.center,
        );

        self.export_status = ExportStatus::Running;

        // engineを返却するためのチャンネル
        // compute スレッドと同じパターン：
        //   スレッド側で tx.send(engine) → メイン側で rx.try_recv() で受け取る
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel::<Box<dyn RenderEngine>>();
        self.export_rx = Some(rx);

        let cancel = Arc::new(AtomicBool::new(false));

        self.export_handle = Some(std::thread::spawn(move || {
            // フラクタル計算（並列）
            let rgba_buf = engine.compute_par(&export_cfg, &cancel);

            // 計算完了後、engineをメインスレッドへ送り返す
            // （computeスレッドが tx.send((engine, img_cfg, result)) するのと同じ構造）
            let _ = tx.send(engine);

            // PNG保存
            let Some(buf) = rgba_buf else { return };

            let filename = {
                use chrono::Local;
                let now = Local::now();
                format!("fractal_{}.png", now.format("%Y%m%d_%H%M%S"))
            };

            let img = image::RgbaImage::from_raw(width as u32, height as u32, buf)
                .expect("buffer size mismatch");
            let _ = img.save(&filename);
        }));
    }

    fn poll_export_result(&mut self) {
        // export_rx がなければexport中ではない
        let Some(rx) = &self.export_rx else { return };

        // engineが送り返されてきたか非ブロッキングで確認
        // （compute スレッドの poll_compute_result と同じパターン）
        if let Ok(engine) = rx.try_recv() {
            // engineをメインの状態に戻す
            self.state.engine = Some(engine);

            // ハンドルとチャンネルをクリア
            self.export_handle = None;
            self.export_rx = None;

            self.export_status = ExportStatus::Done("Save done.".to_string());
        }
    }

    fn show_export_status(&mut self, ctx: &egui::Context) {
        let msg = match &self.export_status {
            ExportStatus::Idle => return,
            ExportStatus::Running => "Caluculating and saving...",
            ExportStatus::Done(_) => "OK: Save done.",
            ExportStatus::Failed(_) => "!!! FAILED TO SAVE !!!",
        };

        egui::Window::new("Export Status")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, [-8.0, -8.0])
            .show(ctx, |ui| {
                ui.label(msg);
                if matches!(self.export_status, ExportStatus::Done(_) | ExportStatus::Failed(_)) {
                    if ui.button("OK").clicked() {
                        self.export_status = ExportStatus::Idle;
                    }
                }
            });

        if matches!(self.export_status, ExportStatus::Running) {
            ctx.request_repaint();
        }
    }
}