use eframe::egui;
use egui::Image;
use crate::prelude::*;
use crate::app::state::AppState;

pub trait RenderEngine {
    // ラスタースキャン順でrgbargba...の順で
    fn compute(&mut self, image_config: &ImageConfig) -> Vec<u8>;
    fn compute_par(&mut self, image_config: &ImageConfig) -> Vec<u8>;
}


impl<D, E, N, M> RenderEngine for EscapeTimeFractal<D, E, N, M>
where
    D: Dynamics + Sync + 'static,
    E: EscapeEvaluator<D> + Sync + 'static,
    N: NormalizeEscInfo<EscapeResult> + Sync + 'static,
    M: ColorMap + Sync + Send + 'static,
{
    fn compute(&mut self, image_config: &ImageConfig) -> Vec<u8> {
        let escape_results = self.escape_results(image_config);
        let colors = self.colors_from_escape_results(&escape_results);
        self.rgba_buf_from_colors(&colors)
    }

    fn compute_par(&mut self, image_config: &ImageConfig) -> Vec<u8> {
        let escape_results = self.escape_results_par(image_config);
        let colors = self.colors_from_escape_results_par(&escape_results);
        self.rgba_buf_from_colors_par(&colors)
    }
}


pub fn show_side_panel(ctx: &egui::Context, state: &AppState) {
    egui::SidePanel::left("side_panel")
        .default_width(200.)
        .width_range(200.0..=400.0)
        .show(ctx, |ui| {
        ui.heading("State");

        let (w, h) = state.img_cfg.resolution;
        ui.label(format!("resolution: {}x{}", w, h));
        ui.label(format!("center: ({}, {})", state.img_cfg.center.0, state.img_cfg.center.1));
        ui.label(format!("scale: ({}, {})", state.img_cfg.scale.0, state.img_cfg.scale.1));
        ui.label(format!("view size: ({}, {})", state.img_cfg.view_size().0, state.img_cfg.view_size().1));

        ui.label(format!("mode: {:?}", state.mode));
        ui.label(format!("recomp: {}", state.recomp));
        ui.label(format!("buf_dirty: {}", state.buf_dirty));

        ui.label(format!(
            "history len: {}",
            state.history.stack.len()
        ));
    });
}


pub fn show_central_panel(ctx: &egui::Context, texture: &Option<egui::TextureHandle>,) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let display_size = ui.available_size();

        if let Some(tex) = texture {
            ui.add(
                Image::new(tex)
                    .fit_to_exact_size(display_size),
            );
        }
    });
}

/*
pub struct Renderer {
    texture: Option<egui::TextureHandle>,
}

impl Renderer {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        image: &egui::ColorImage,
    ) {
        let tex = self.texture.get_or_insert_with(|| {
            ctx.load_texture(
                "fractal",
                image.clone(),
                egui::TextureOptions::NEAREST,
            )
        });

        tex.set(image.clone(), egui::TextureOptions::NEAREST);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(tex);
        });
    }
}
*/