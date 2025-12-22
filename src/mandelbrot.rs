use num_complex::{self, Complex};
use image::{Rgb, RgbImage};
use crate::{color::Color, palette::Palette};

type MandelFloat = f64;

pub struct Mandelbrot {
    image_size: (usize, usize),  // width, heightの順
    center: Complex<MandelFloat>,
    extent: (MandelFloat, MandelFloat),
    mandel_count_max: usize,
    palette: Palette,
}

impl Mandelbrot {
    #[inline]
    fn re_extent(&self) -> (MandelFloat, MandelFloat) {
        (
            self.center.re - self.extent.0 / 2.0,
            self.center.re + self.extent.0 / 2.0
        )
    }

    #[inline]
    fn im_extent(&self) -> (MandelFloat, MandelFloat) {
        (
            self.center.im - self.extent.1 / 2.0,
            self.center.im + self.extent.1 / 2.0
        )
    }

    #[inline]
    fn extent(&self) -> (MandelFloat, MandelFloat, MandelFloat, MandelFloat) {
        (
            self.center.re - self.extent.0 / 2.0,
            self.center.re + self.extent.0 / 2.0,
            self.center.im - self.extent.1 / 2.0,
            self.center.im + self.extent.1 / 2.0
        )
    }

    pub fn new(
        image_size: (usize, usize),
        center: Complex<f64>,
        extent: (MandelFloat, MandelFloat),
        max_count: usize,
        palette: Palette,
    ) -> Self {
        Self {
            image_size,
            center,
            extent,
            mandel_count_max: max_count,
            palette,
        }
    }

    fn get_complex_at(
        &self,
        point: (usize, usize),
        re_extent: (MandelFloat, MandelFloat),
        im_extent: (MandelFloat, MandelFloat)
    ) -> Complex<MandelFloat> {
        let (x, y) = point;
        let x = x as MandelFloat;
        let y = y as MandelFloat;
        let (re_min, re_max) = re_extent;
        let (im_min, im_max) = im_extent;
        let (w_px, h_px) = self.image_size;

        let t = x / w_px as MandelFloat;
        let re = re_min + t * (re_max - re_min);

        let t = y / h_px as MandelFloat;
        let im = im_max + t * (im_min - im_max);

        Complex { re: re, im: im }
    }

    fn count_divergence(&self, c: Complex<MandelFloat>) -> usize {
        let mut z: Complex<MandelFloat> = Complex::ZERO;

        for n in 1..=self.mandel_count_max {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return n;
            }
        }

        self.mandel_count_max
    }

    fn n_to_color(&self, n: usize) -> Option<&Color> {
        let idx = n * self.palette.len() / self.mandel_count_max;
        self.palette.get(idx.min(self.palette.len() - 1))
    }

    pub fn make_count_vec(&self) -> Vec<usize> {
        let (w, h) = self.image_size;
        let mut ret = vec![0usize; w * h];

        let re_extent = self.re_extent();
        let im_extent = self.im_extent();

        for y in 0..h {
            for x in 0..w {
                let z = self.get_complex_at((x, y), re_extent, im_extent);
                let n = self.count_divergence(z);
                ret[y * w + x] = n;
            }
        }

        ret
    }

    pub fn make_color_vec(&self) -> Vec<Color> {
        let (w, h) = self.image_size;
        let mut ret: Vec<Color> = vec![Color::BLACK; w * h];

        let re_extent = self.re_extent();
        let im_extent = self.im_extent();

        for y in 0..h {
            for x in 0..w {
                let z = self.get_complex_at((x, y), re_extent, im_extent);
                let n = self.count_divergence(z);
                let c_opt = self.n_to_color(n);
                if let Some(&c) = c_opt {
                    ret[y * w + x] = c;
                }
            }
        }

        ret
    }

    pub fn make_image(&self) -> RgbImage {
        let (w, h) = self.image_size;
        let mut img = RgbImage::new(w as u32, h as u32);

        let re_extent = self.re_extent();
        let im_extent = self.im_extent();

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let z = self.get_complex_at(
                (x as usize, y as usize),
                re_extent,
                im_extent,
            );
            let n = self.count_divergence(z);
            let c_opt = self.n_to_color(n);

            if let Some(&c) = c_opt {
                *pixel = Rgb([c.get_r(), c.get_g(), c.get_b()]);
            }
        }

        img
    }
}