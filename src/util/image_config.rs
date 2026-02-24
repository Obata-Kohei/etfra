use crate::prelude::*;
use crate::util::types;

pub enum PlaneMode {
    ParameterPlane,  // Mandelbrotなど．各ピクセルがc
    DynamicalPlane,  // Julia, Newton fractalなど．各ピクセルがz0
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub resolution: (usize, usize),  // (w, h)
    pub scale: (types::Real, types::Real),  // 1 pxあたりの座標平面の長さ．(x, y).
    pub center: (types::Real, types::Real),  // 描画の中心となる座標平面上の座標. 
    pub precision_bit: u32,  // rugの精度
    pub plane_mode: PlaneMode  // どういう平面か
}

impl Plane {
    pub fn new(
        resolution: (usize, usize),
        scale: (types::Real, types::Real),
        center: (types::Real, types::Real),
        precision_bit: u32,
        plane_mode: PlaneMode,
    ) -> Self {
        Self {
            resolution,
            scale,
            center,
            precision_bit,
            plane_mode,
        }
    }

    pub fn view_size(&self) -> (types::Real, types::Real) {
        //
    }
}



#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub resolution: (usize, usize), // (w, h)
    pub scale: (Float, Float),      // 1 pxあたりの座標平面の長さ．(x, y).
    pub center: (Float, Float),     // 描画の中心となる座標平面上の座標. Complexや(f64, f64)?
}

impl ImageConfig {
    pub fn new(resolution: (usize, usize), scale: (Float, Float), center: (Float, Float)) -> Self {
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
        &self,
        point: (usize, usize),
        view_bouds: (Float, Float, Float, Float),
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
