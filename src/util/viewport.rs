use crate::util::types;

#[derive(Debug, Clone, Copy)]
pub enum PlaneMode {
    ParameterPlane,  // Mandelbrotなど．各ピクセルがc
    DynamicalPlane,  // Julia, Newton fractalなど．各ピクセルがz0
}

#[derive(Debug, Clone)]
pub struct Viewport {
    resolution: (usize, usize),
    center: (types::Real, types::Real),
    half_extent: (types::Real, types::Real),
    precision_bit: u32,
    plane_mode: PlaneMode,
}

impl Viewport {
    pub fn builder() -> ViewportBuilder {
        ViewportBuilder::default()
    }

    pub fn precision(&self) -> u32 {
        self.precision_bit
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.resolution
    }

    pub fn plane_mode(&self) -> PlaneMode {
        self.plane_mode
    }

    // 座標平面上でx/y方向それぞれについて，描画される長さ
    pub fn view_size(&self) -> (types::Real, types::Real) {
        let mut w = self.half_extent.0.clone();
        w *= 2;

        let mut h = self.half_extent.1.clone();
        h *= 2;

        (w, h)
    }

    // 描画範囲のうち，(xmin, xmzx, ymin, ymax)をこの順番にタプルで返す
    pub fn view_bounds(
        &self,
    ) -> (types::Real, types::Real, types::Real, types::Real) {

        let mut xmin = self.center.0.clone();
        xmin -= &self.half_extent.0;

        let mut xmax = self.center.0.clone();
        xmax += &self.half_extent.0;

        let mut ymin = self.center.1.clone();
        ymin -= &self.half_extent.1;

        let mut ymax = self.center.1.clone();
        ymax += &self.half_extent.1;

        (xmin, xmax, ymin, ymax)
    }

    // 画面のピクセルの位置から，座標平面の座標を算出
    pub fn pixel_to_point(
        &self,
        point: (usize, usize),
        view_bounds: (types::Real, types::Real, types::Real, types::Real),
    ) -> (types::Real, types::Real) {

        let (px, py) = point;
        let (xmin, xmax, ymin, ymax) = view_bounds;
        let (w, h) = self.resolution();

        let prec = self.precision_bit;

        // --- t_x = px / w (高精度で計算)
        let mut tx = types::Real::with_val(prec, px);
        tx /= w;

        // --- dx = xmax - xmin
        let mut dx = xmax.clone();
        dx -= &xmin;

        // --- x = xmin + tx * dx
        let mut x = dx;
        x *= &tx;
        x += xmin;

        // --- t_y = py / h
        let mut ty = types::Real::with_val(prec, py);
        ty /= h;

        // --- dy = ymin - ymax （画像座標は上→下が正）
        let mut dy = ymin.clone();
        dy -= &ymax;

        // --- y = ymax + ty * dy
        let mut y = dy;
        y *= &ty;
        y += ymax;

        (x, y)
    }
}


pub struct ViewportBuilder {
    resolution: (usize, usize),
    center: (f64, f64),
    half_extent: (f64, f64),
    precision_bit: u32,
    plane_mode: PlaneMode,
}

impl Default for ViewportBuilder {
    fn default() -> Self {
        Self {
            resolution: (100, 100),
            center: (0.0, 0.0),
            half_extent: (1.0, 1.0),
            precision_bit: 32,
            plane_mode: PlaneMode::ParameterPlane,
        }
    }
}

impl ViewportBuilder {
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

    pub fn half_extent_f64(mut self, x: f64, y: f64) -> Self {
        self.half_extent = (x, y);
        self
    }

    pub fn plane_mode(mut self, mode: PlaneMode) -> Self {
        self.plane_mode = mode;
        self
    }

        pub fn build(self) -> Viewport {
        let (cx, cy) = self.center;
        let (hx, hy) = self.half_extent;

        let center = (
            types::Real::with_val(self.precision_bit, cx),
            types::Real::with_val(self.precision_bit, cy),
        );

        let half_extent = (
            types::Real::with_val(self.precision_bit, hx),
            types::Real::with_val(self.precision_bit, hy),
        );

        Viewport {
            resolution: self.resolution,
            center,
            half_extent,
            precision_bit: self.precision_bit,
            plane_mode: self.plane_mode,
        }
    }
}
