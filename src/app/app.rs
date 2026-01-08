use eframe::egui;
use egui::Image;

use crate::app::{key_input::handle_key_input, state::AppState};

pub struct App {
    pub state: AppState,
    pub texture: Option<egui::TextureHandle>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::with_preset_values(),
            texture: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        handle_key_input(ctx, &mut self.state);
        self.state.compute_if_needed_par();

        if self.state.buf_dirty {
            if let Some(buf) = &self.state.rgba_buf {
                let (w, h) = self.state.get_resolution();

                /*
                let w = 256;
                let h = 256;
                let mut buf = vec![0u8; w * h * 4];
                for i in 0..(w * h) {
                    buf[4*i + 0] = 255; // R
                    buf[4*i + 1] = 0;   // G
                    buf[4*i + 2] = 0;   // B
                    buf[4*i + 3] = 255; // A
                }
                */

                let img = egui::ColorImage::from_rgba_unmultiplied([w, h], &buf);
                self.texture = Some(ctx.load_texture(
                    "rendered_image",
                    img,
                    egui::TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Nearest,
                        ..Default::default()
                    },
                ));
                self.state.buf_dirty = false;
            }
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("State");

            let (w, h) = self.state.img_cfg.resolution;
            ui.label(format!("resolution: {}x{}", w, h));
            ui.label(format!("center: {}", self.state.img_cfg.center));
            ui.label(format!("scale: {}", self.state.img_cfg.scale));

            ui.label(format!("mode: {:?}", self.state.mode));
            ui.label(format!("recomp: {}", self.state.recomp));
            ui.label(format!("buf_dirty: {}", self.state.buf_dirty));

            ui.label(format!("history length: {}", self.state.history.stack.len()));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.label("Hello egui");
            //ui.button("Button");

            let display_size = ui.available_size();

            if let Some(tex) = &self.texture {
                //let size = tex.size_vec2();
                ui.add(
                    Image::new(tex)
                    .fit_to_exact_size(display_size)
                );
            }
        });

    }
}
