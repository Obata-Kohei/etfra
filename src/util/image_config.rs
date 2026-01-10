use crate::prelude::*;

pub struct ImageConfig {
    pub resolution: (usize, usize),   // (w, h)
    pub scale: (Float, Float),  // 1 pxあたりの座標平面の長さ．(x, y). F: Float?
    pub center: (Float, Float),  // 描画の中心となる座標平面上の座標. Complexや(f64, f64)?
}

impl ImageConfig {
    pub fn new(
        resolution: (usize, usize),
        scale: (Float, Float),
        center: (Float, Float),
    ) -> Self {
        Self {
            resolution,
            scale,
            center,
        }
    }

    pub fn view_size(&self) -> (Float, Float) {
        (
            self.resolution.0 as Float * self.scale.0,
            self.resolution.1 as Float * self.scale.1,
        )
    }

    pub fn view_bounds(&self, view_size: (Float, Float)) -> (Float, Float, Float, Float) {
        let (w, h) = view_size;
        (
            self.center.0 - w / 2.0,
            self.center.0 + w / 2.0,
            self.center.1 - h / 2.0,
            self.center.1 + h / 2.0,
        )
    }

    pub fn pixel_to_xyplane(
        &self, point: (usize, usize), view_bouds: (Float, Float, Float, Float)
    ) -> (Float, Float) {
        let (x, y) = point;
        let (xmin, xmax, ymin, ymax) = view_bouds;
        let (w, h) = self.resolution;

        let t = x as Float / w as Float;
        let x = xmin + t * (xmax - xmin);

        let t = y as Float / h as Float;
        let y = ymax + t * (ymin - ymax);

        (x, y)
    }
}