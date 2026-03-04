use std::clone;

use crate::core::real::Real;

#[derive(Debug, Clone)]
pub struct Plane<R: Real> {
    resolution: (usize, usize),
    center: (R, R),
    scale: (R, R),
    precision_bit: u32,
}

impl Plane {
    pub fn builder() -> PlaneBuilder {
        PlaneBuilder::default()
    }
}

impl<R: Real> Plane<R> {
    // 座標平面上でx/y方向それぞれについて，描画される長さ
    pub fn view_size(&self) -> (R, R) {
        let mut w = self.scale.0.clone();
        let rw = R::from_usize(self.precision_bit, self.resolution.0);
        w.mul_assign_ref(&rw);

        let mut h = self.scale.1.clone();
        let rh = R::from_usize(self.precision_bit, self.resolution.1);
        h.mul_assign_ref(&rh);

        (w, h)
    }

    // 描画範囲のうち，(xmin, xmax, ymin, ymax)をこの順番にタプルで返す
    pub fn view_bounds(
        &self,
    ) -> (R, R, R, R) {
        let two = R::from_usize(self.precision_bit, 2);

        let (view_w, view_h) = self.view_size();

        let mut half_w = view_w.clone();
        half_w.div_assign_ref(&two);
        let mut half_h = view_h.clone();
        half_h.div_assign_ref(&two);

        let mut xmin = self.center.0.clone();
        xmin.sub_assign_ref(&half_w);

        let mut xmax = self.center.0.clone();
        xmax.add_assign_ref(&half_w);

        let mut ymin = self.center.1.clone();
        ymin.sub_assign_ref(&half_h);

        let mut ymax = self.center.1.clone();
        ymax.add_assign_ref(&half_h);

        (xmin, xmax, ymin, ymax)
    }

    // 画面のピクセルの位置から，座標平面の座標を算出
    pub fn pixel_to_point(
        &self,
        pixel: (usize, usize),
        view_bounds: &(R, R, R, R),
    ) -> (R, R) {
        let w = R::from_usize(self.precision_bit, self.resolution.0);
        let h = R::from_usize(self.precision_bit, self.resolution.1);
        let (xmin, xmax, ymin, ymax) = view_bounds;

        // tx = px / w
        let mut tx = R::from_usize(self.precision_bit, pixel.0);
        tx.div_assign_ref(&w);

        // dx = xmax - xmin
        let mut x = xmax.clone();
        x.sub_assign_ref(&xmin);

        // x = xmin + tx * dx
        x.mul_assign_ref(&tx);
        x.add_assign_ref(&xmin);

        // ty = py / h
        let mut ty = R::from_usize(self.precision_bit, pixel.1);
        ty.div_assign_ref(&h);

        // dy = ymin - ymax （画像座標は上→下が正）
        let mut y = ymin.clone();
        y.sub_assign_ref(&ymax);

        // y = ymax + ty * dy
        y.mul_assign_ref(&ty);
        y.add_assign_ref(&ymax);

        (x, y)
    }
}


pub struct PlaneBuilder<R: Real> {
    resolution: (usize, usize),
    center: (f64, f64),
    scale: (f64, f64),
    precision_bit: u32,
}

impl Default for PlaneBuilder {
    fn default() -> Self {
        Self {
            resolution: (100, 100),
            center: (0.0, 0.0),
            scale: (0.01, 0.01),
            precision_bit: 64,
        }
    }
}

impl<R: Real> PlaneBuilder<R> {
    pub fn resolution(mut self, w: usize, h: usize) -> Self {
        self.resolution = (w, h);
        self
    }

    pub fn precision(mut self, bits: u32) -> Self {
        self.precision_bit = bits;
        self
    }

    pub fn center_f64(mut self, x: f64, y: f64) -> Self {
        self.center = (x, y);
        self
    }

    pub fn scale_f64(mut self, x: f64, y: f64) -> Self {
        self.scale = (x, y);
        self
    }

    pub fn build(self) -> Plane<R> {
        let (cx, cy) = self.center;
        let (hx, hy) = self.scale;

        let center = (
            R::from_f64(self.precision_bit, cx),
            R::from_f64(self.precision_bit, cy),
        );

        let scale = (
            R::from_f64(self.precision_bit, hx),
            R::from_f64(self.precision_bit, hy),
        );

        Plane {
            resolution: self.resolution,
            center,
            scale,
            precision_bit: self.precision_bit,
        }
    }
}
