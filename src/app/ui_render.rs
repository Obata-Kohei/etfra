use crate::{app::state::ImageConfig, prelude::*};

pub trait RenderEngine {
    // ラスタースキャン順でrgbargba...の順で
    fn compute(&mut self, img_cfg: &ImageConfig) -> Vec<u8>;
    fn compute_par(&mut self, img_cfg: &ImageConfig) -> Vec<u8>;
}

impl<D, E, C> RenderEngine for EscapeTimeFractal<D, E, C>
where
    D: ComplexDynamics + Sync + 'static,
    E: EscapeEvaluator<D> + Sync + 'static,
    C: Coloring<E::Output> + Sync + 'static,
    E::Output: Sync + Send,
{
    fn compute(&mut self, img_cfg: &ImageConfig) -> Vec<u8> {
        self.resolution = img_cfg.resolution;
        self.center = img_cfg.center;
        self.view_size.0 = img_cfg.scale * img_cfg.resolution.0 as Float;
        self.view_size.1 = img_cfg.scale * img_cfg.resolution.1 as Float;
        let values = self.escape_values();
        let colors = self.colors_from_values(&values);
        self.rgba_buf_from_colors(&colors)
    }
    fn compute_par(&mut self, img_cfg: &ImageConfig) -> Vec<u8> {
        self.resolution = img_cfg.resolution;
        self.center = img_cfg.center;
        self.view_size.0 = img_cfg.scale * img_cfg.resolution.0 as Float;
        self.view_size.1 = img_cfg.scale * img_cfg.resolution.1 as Float;
        let values = self.escape_values_par();
        let colors = self.colors_from_values_par(&values);
        self.rgba_buf_from_colors_par(&colors)
    }
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