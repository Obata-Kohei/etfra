use std::sync::mpsc::Receiver;
use std::sync::{atomic::AtomicBool, Arc};
//use std::sync::atomic::Ordering;
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

    compute_handle: Option<JoinHandle<()>>,
    result_rx: Option<Receiver<(Box<dyn RenderEngine>, ImageConfig, Option<Vec<u8>>)>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::with_preset_values(),
            texture: None,
            compute_handle: None,
            result_rx: None,
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

        // 新規計算開始
        if self.state.recomp && self.compute_handle.is_none() {
            self.start_compute_thread();
        }

        self.poll_compute_result();
        self.update_texture(ctx);

        ui_render::show_side_panel(ctx, &mut self.state);
        ui_render::show_central_panel(ctx, &self.texture);
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
            // 状態不整合の保険
            self.state.buf_dirty = false;
            return;
        }

        let img = egui::ColorImage::from_rgba_unmultiplied([w, h], buf);
        self.texture = Some(ctx.load_texture("rendered_image", img, egui::TextureOptions::NEAREST));
        self.state.buf_dirty = false;
    }
}
