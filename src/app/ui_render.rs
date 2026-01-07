use crate::prelude::*;

pub trait RenderEngine {
    // ラスタースキャン順でrgbargba...の順で
    fn compute(&self) -> Vec<u8>;
    fn compute_par(&self) -> Vec<u8>;
}

impl<D, E, C> RenderEngine for EscapeTimeFractal<D, E, C>
where
    D: ComplexDynamics + Sync + 'static,
    E: EscapeEvaluator<D> + Sync + 'static,
    C: Coloring<E::Output> + Sync + 'static,
    E::Output: Sync + Send,
{
    fn compute(&self) -> Vec<u8> {
        let values = self.escape_values();
        let colors = self.colors_from_values(&values);
        self.rgba_buf_from_colors(&colors)
    }
    fn compute_par(&self) -> Vec<u8> {
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