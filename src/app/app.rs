use eframe::egui;
use egui::Image;

use crate::app::state::AppState;

pub struct App {
    state: AppState,
    texture: Option<egui::TextureHandle>,
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
        self.state.compute_if_needed();

        if self.state.buf_dirty {
            if let Some(buf) = &self.state.rgba_buf {
                let (w, h) = self.state.get_resolution();
                let img = egui::ColorImage::from_rgba_unmultiplied([w, h], buf);
                self.texture = Some(ctx.load_texture(
                    "rendered_image",
                    img,
                    egui::TextureOptions::default(),
                ));
                self.state.buf_dirty = false;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello egui");
            //ui.button("Button");

            if let Some(tex) = &self.texture {
                let size = tex.size_vec2();
                ui.add(Image::new(tex).fit_to_exact_size(size));
            }
        });
    }
}
