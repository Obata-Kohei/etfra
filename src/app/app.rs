use eframe::egui;
use crate::app::{key_input::handle_key_input, state::AppState, ui_render};

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

        ui_render::show_side_panel(ctx, &self.state);
        ui_render::show_central_panel(ctx, &self.texture);
    }
}
