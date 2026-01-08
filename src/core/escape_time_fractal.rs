use crate::util::color::Color;
use crate::core::complex_dynamics::ComplexDynamics;
use crate::core::escape_evaluator::EscapeEvaluator;
use crate::core::coloring::Coloring;
use crate::util::types::Float;

use rayon::prelude::*;
use num_complex::{self, Complex};
use image::{Rgb, RgbImage};

pub struct EscapeTimeFractal<D, E, C>
where
    D: ComplexDynamics,
    E: EscapeEvaluator<D>,
    C: Coloring<E::Output>
{
    pub dynamics: D,  // 力学系の定義
    pub escape: E,  // escape評価器
    pub coloring: C,  // 色付け
    pub resolution: (usize, usize),  // 描画画像サイズ(w, h)
    pub center: Complex<Float>,  // 描画の中心の複素数座標
    pub view_size: (Float, Float),  // 描画する範囲(re, im)
}

impl<D, E, C> EscapeTimeFractal<D, E, C>
where
    D: ComplexDynamics + Sync,
    E: EscapeEvaluator<D> + Sync,
    C: Coloring<E::Output> + Sync,
    E::Output: Sync + Send,
{
    pub fn new(
        dynamics: D,
        escape: E,
        coloring: C,
        resolution: (usize, usize),
        center: Complex<Float>,
        view_size: (Float, Float),
    ) -> Self {
        EscapeTimeFractal {
            dynamics,
            escape,
            coloring,
            resolution,
            center,
            view_size,
        }
    }

    // (remin, remax, immin, immax)を返す
    #[inline]
    fn view_bounds(&self) -> (Float, Float, Float, Float) {
        let (w, h) = self.view_size;
        (
            self.center.re - w / 2.0,
            self.center.re + w / 2.0,
            self.center.im - h / 2.0,
            self.center.im + h / 2.0,
        )
    }

    fn pixel_to_complex(
        &self,
        point: (usize, usize),
        view_bounds: (Float, Float, Float, Float),
    ) -> Complex<Float> {
        let (x, y) = point;
        let (re_min, re_max, im_min, im_max) = view_bounds;
        let (w, h) = self.resolution;

        let t = x as Float / w as Float;
        let re = re_min + t * (re_max - re_min);

        let t = y as Float / h as Float;
        let im = im_max + t * (im_min - im_max);

        Complex {re, im}
    }

    pub fn escape_values(&self) -> Vec<E::Output> {
        let (w, h) = self.resolution;
        let bounds = self.view_bounds();

        (0..w*h)
            .into_iter()
            .map(|i| {
                let x = i % w;
                let y = i / w;
                let z = self.pixel_to_complex((x, y), bounds);
                self.escape.evaluate(&self.dynamics, z)
            })
            .collect()
    }

    pub fn escape_values_par(&self) -> Vec<E::Output> {
        let (w, h) = self.resolution;
        let bounds = self.view_bounds();

        (0..w * h)
            .into_par_iter()
            .map(|i| {
                let x = i % w;
                let y = i / w;
                let z = self.pixel_to_complex((x, y), bounds);
                self.escape.evaluate(&self.dynamics, z)
            })
            .collect()
    }

    pub fn colors_from_values(&self, values: &[E::Output]) -> Vec<Color> {
        values
            .iter()
            .map(|&v| {
                self.coloring.color(v)
            })
            .collect()
    }

    pub fn colors_from_values_par(&self, values: &[E::Output]) -> Vec<Color> {
        values
            .par_iter()
            .map(|&v| self.coloring.color(v))
            .collect()
    }

    // ラスタースキャン順のピクセルが，rgbargba...と並ぶbuffer
    pub fn rgba_buf_from_colors(&self, colors: &[Color]) -> Vec<u8> {
        let (w, h) = self.resolution;
        assert_eq!(colors.len(), w * h);
        let mut buf = vec![0u8; w * h * 4];

        buf.chunks_mut(4)
            .enumerate()
            .for_each(|(i, px)| {
                let c = &colors[i];
                px[0] = c.get_r();
                px[1] = c.get_g();
                px[2] = c.get_b();
                px[3] = c.get_a();
            });

        buf
    }

    // ラスタースキャン順のピクセルが，rgbargba...と並ぶbuffer par
    pub fn rgba_buf_from_colors_par(&self, colors: &[Color]) -> Vec<u8> {
        let (w, h) = self.resolution;
        assert_eq!(colors.len(), w * h);
        let mut buf = vec![0u8; w * h * 4];

        buf.par_chunks_mut(4)
            .enumerate()
            .for_each(|(i, px)| {
                let c = &colors[i];
                px[0] = c.get_r();
                px[1] = c.get_g();
                px[2] = c.get_b();
                px[3] = c.get_a();
            });

        buf
    }


    pub fn render_from_colors(&self, colors: &[Color]) -> RgbImage {
        let (w, h) = self.resolution;
        let mut buf = vec![0u8; w * h * 3];

        buf.chunks_mut(3)
            .enumerate()
            .for_each(|(i, px)| {
                let c = &colors[i];
                px[0] = c.get_r();
                px[1] = c.get_g();
                px[2] = c.get_b();
            });

        RgbImage::from_raw(w as u32, h as u32, buf)
            .expect("The image should be made but it failed.")
    }

    pub fn render_from_colors_par(&self, colors: &[Color]) -> RgbImage {
        let (w, h) = self.resolution;
        let mut buf = vec![0u8; w * h * 3];

        buf.par_chunks_mut(3)
            .enumerate()
            .for_each(|(i, px)| {
                let c = &colors[i];
                px[0] = c.get_r();
                px[1] = c.get_g();
                px[2] = c.get_b();
            });

        RgbImage::from_raw(w as u32, h as u32, buf)
            .expect("The image should be made but it failed.")
    }

    pub fn render(&self) -> RgbImage {
        let (w, h) = self.resolution;
        let bounds = self.view_bounds();
        let mut img = RgbImage::new(w as u32, h as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = self.pixel_to_complex((x as usize, y as usize), bounds);
            let escape_value = self.escape.evaluate(&self.dynamics, c);
            let color = self.coloring.color(escape_value);
            *pixel = Rgb([color.get_r(), color.get_g(), color.get_b()]);
        }

        img
    }

    pub fn render_par(&self) -> RgbImage {
        let vs = self.escape_values_par();
        let cs = self.colors_from_values_par(&vs);
        self.render_from_colors_par(&cs)
    }

}