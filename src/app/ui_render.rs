use eframe::egui;
use egui::Image;
use crate::prelude::*;
use crate::app::state::AppState;
use std::sync::atomic::AtomicBool;

pub trait RenderEngine: Send + 'static {
    fn compute(
        &mut self,
        image_config: &ImageConfig,
        cancel: &AtomicBool,
    ) -> Option<Vec<u8>>;
    fn compute_par(
        &mut self,
        image_config: &ImageConfig,
        cancel: &AtomicBool,
    ) -> Option<Vec<u8>>;
}


impl<D, E, N, M> RenderEngine for EscapeTimeFractal<D, E, N, M>
where
    D: Dynamics + Sync + Send + 'static,
    E: EscapeEvaluator<D> + Sync + Send + 'static,
    N: NormalizeEscInfo<EscapeResult> + Sync + Send + 'static,
    M: ColorMap + Sync + Send + 'static,
{
    fn compute(&mut self, image_config: &ImageConfig, cancel: &AtomicBool) -> Option<Vec<u8>> {
        let escape_results = self.escape_results_interruptible(image_config, cancel);
        match escape_results {
            Some(esc_vec) => {
                let colors = self.colors_from_escape_results(&esc_vec);
                Some(self.rgba_buf_from_colors(&colors))
            },
            None => None,
        }
    }

    fn compute_par(&mut self, image_config: &ImageConfig, cancel: &AtomicBool) -> Option<Vec<u8>> {
        let escape_results = self.escape_results_par_interruptible(image_config, cancel);
        match escape_results {
            Some(esc_vec) => {
                let colors = self.colors_from_escape_results_par(&esc_vec);
                Some(self.rgba_buf_from_colors_par(&colors))
            },
            None => None,
        }
    }
}


pub fn show_side_panel(ctx: &egui::Context, state: &mut AppState) {
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
        if state.is_computing {
            ui.label("Computing...");
        } else {
            if ui.button("Recompute").clicked() {
                state.set_recomp(true);
            }
        }
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
