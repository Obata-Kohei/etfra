use crate::prelude::*;
use rayon::prelude::*;
use image::RgbaImage;


pub struct EscapeTimeFractal<D, E, N, M>
where
    D: Dynamics,
    E: EscapeEvaluator<D>,
    N: NormalizeEscInfo<EscapeResult>,
    M: ColorMap,
{
    pub dynamics: D,
    pub escape_evaluator: E,
    pub coloring: Coloring<N, M>
}

impl<D, E, N, M> EscapeTimeFractal<D, E, N, M>
where
    D: Dynamics + Sync,
    E: EscapeEvaluator<D> + Sync,
    N: NormalizeEscInfo<EscapeResult> + Sync,
    M: ColorMap + Sync,
{
    pub fn new(dynamics: D, escape_evaluator: E, coloring: Coloring<N, M>) -> Self {
        Self {
            dynamics,
            escape_evaluator,
            coloring,
        }
    }

    pub fn escape_results(&self, image_config: &ImageConfig) -> Vec<EscapeResult> {
        let (w, h) = image_config.resolution;
        let view_size = image_config.view_size();
        let view_bounds = image_config.view_bounds(view_size);

        (0..w*h)
            .into_iter()
            .map(|i| {
                let col = i % w;
                let row = i / w;
                let xy = image_config.pixel_to_xyplane((col, row), view_bounds);
                let p = self.dynamics.param_from_xy(xy);
                self.escape_evaluator.evaluate(&self.dynamics, &p)
            })
            .collect()
    }

    pub fn colors_from_escape_results(&self, escape_results: &[EscapeResult]) -> Vec<Color> {
        escape_results
            .iter()
            .map(|esc_res| self.coloring.apply(&esc_res))
            .collect()
    }

    pub fn rgba_buf_from_colors(&self, colors: &[Color]) -> Vec<u8> {
        colors
            .iter()
            .flat_map(|c| c.as_rgba().iter().copied())
            .collect()
    }

    pub fn rgba_image_from_colors(&self, colors: &[Color], image_config: &ImageConfig) -> RgbaImage {
        let rgba = self.rgba_buf_from_colors_par(colors);
        RgbaImage::from_raw(image_config.resolution.0 as u32, image_config.resolution.1 as u32, rgba)
            .expect("RgbImage should be made. size or buf error.")
    }

    // escapeにかかったiter回数をu8へ写像し，Vec<u8>とする
    pub fn u8buf(&self, escape_results:  &[EscapeResult]) -> Vec<u8> {
        let max_iter = self.coloring.normalizer.max_iter();
        escape_results
            .iter()
            .map(|e| {
                let v = (e.iter as f64 / max_iter as f64) * 255.0;
                v.min(255.0) as u8
            })
            .collect()
    }


    /// par functions ///

    pub fn escape_results_par(&self, image_config: &ImageConfig) -> Vec<EscapeResult> {
        let (w, h) = image_config.resolution;
        let view_size = image_config.view_size();
        let view_bounds = image_config.view_bounds(view_size);

        (0..w*h)
            .into_par_iter()
            .map(|i| {
                let col = i % w;
                let row = i / w;
                let xy = image_config.pixel_to_xyplane((col, row), view_bounds);
                let p = self.dynamics.param_from_xy(xy);
                self.escape_evaluator.evaluate(&self.dynamics, &p)
            })
            .collect()
    }

    pub fn colors_from_escape_results_par(&self, escape_results: &[EscapeResult]) -> Vec<Color> {
        escape_results
            .par_iter()
            .map(|esc_res| self.coloring.apply(&esc_res))
            .collect()
    }

    pub fn rgba_buf_from_colors_par(&self, colors: &[Color]) -> Vec<u8> {
        colors
            .par_iter()
            .flat_map_iter(|c| c.as_rgba().iter().copied())
            .collect()
    }

    pub fn u8buf_par(&self, escape_results:  &[EscapeResult]) -> Vec<u8> {
        let max_iter = self.coloring.normalizer.max_iter();
        escape_results
            .par_iter()
            .map(|e| {
                let v = (e.iter as f64 / max_iter as f64) * 255.0;
                v.min(255.0) as u8
            })
            .collect()
    }
}
