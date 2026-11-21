#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color([u8; 4]);

pub enum ColorParseError {
    InvalidLength,
    InvalidHex,
}

impl Color {
    const R: usize = 0;
    const G: usize = 1;
    const B: usize = 2;
    const A: usize = 3;

    /* ===== constructors ===== */

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color([r, g, b, a])
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color([r, g, b, 255])
    }

    pub fn from_hsb(h: f32, s: f32, b: f32) -> Self {
        let s = s.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);
        let h = h.rem_euclid(360.0);

        if s == 0.0 {
            let v = (b * 255.0).round() as u8;
            return Color::new(v, v, v, 255);
        }

        let sector = (h / 60.0).floor() as i32;
        let f = (h / 60.0) - sector as f32;

        let p = b * (1.0 - s);
        let q = b * (1.0 - s * f);
        let t = b * (1.0 - s * (1.0 - f));

        let (r, g, b_val) = match sector {
            0 | 6 => (b, t, p),
            1 => (q, b, p),
            2 => (p, b, t),
            3 => (p, q, b),
            4 => (t, p, b),
            _ => (b, p, q),
        };

        Color::new(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b_val * 255.0).round() as u8,
            255,
        )
    }

    pub fn from_hex(code: &str) -> Result<Self, ColorParseError> {
        let code = code.strip_prefix('#').unwrap_or(code);

        fn to_byte(s: &str) -> Result<u8, ColorParseError> {
            u8::from_str_radix(s, 16).map_err(|_| ColorParseError::InvalidHex)
        }

        match code.len() {
            3 => Ok(Color::new(
                to_byte(&code[0..1].repeat(2))?,
                to_byte(&code[1..2].repeat(2))?,
                to_byte(&code[2..3].repeat(2))?,
                255,
            )),
            4 => Ok(Color::new(
                to_byte(&code[0..1].repeat(2))?,
                to_byte(&code[1..2].repeat(2))?,
                to_byte(&code[2..3].repeat(2))?,
                to_byte(&code[3..4].repeat(2))?,
            )),
            6 => Ok(Color::new(
                to_byte(&code[0..2])?,
                to_byte(&code[2..4])?,
                to_byte(&code[4..6])?,
                255,
            )),
            8 => Ok(Color::new(
                to_byte(&code[0..2])?,
                to_byte(&code[2..4])?,
                to_byte(&code[4..6])?,
                to_byte(&code[6..8])?,
            )),
            _ => Err(ColorParseError::InvalidLength),
        }
    }

    fn to_oklab(self) -> [f32; 3] {
        let r = srgb_to_linear(self.get_r() as f32 / 255.0);
        let g = srgb_to_linear(self.get_g() as f32 / 255.0);
        let b = srgb_to_linear(self.get_b() as f32 / 255.0);

        let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
        let m = 0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b;
        let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b;
        let l = l.cbrt();
        let m = m.cbrt();
        let s = s.cbrt();

        [
            0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
            1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
            0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
        ]
    }

    fn oklab_to_linear_rgb(lab: [f32; 3]) -> [f32; 3] {
        let [l, a, b] = lab;
        let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
        let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
        let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;
        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        [
            4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s,
            -1.268_438 * l + 2.609_757_4 * m - 0.341_319_4 * s,
            -0.004_196_086_3 * l - 0.703_418_6 * m + 1.707_614_7 * s,
        ]
    }

    /* ===== presets ===== */

    pub const BLACK: Color = Color([0, 0, 0, 255]);
    pub const WHITE: Color = Color([255, 255, 255, 255]);
    pub const RED: Color = Color([255, 0, 0, 255]);
    pub const GREEN: Color = Color([0, 255, 0, 255]);
    pub const BLUE: Color = Color([0, 0, 255, 255]);

    /* ===== getters ===== */

    pub fn get_r(&self) -> u8 {
        self.0[Self::R]
    }
    pub fn get_g(&self) -> u8 {
        self.0[Self::G]
    }
    pub fn get_b(&self) -> u8 {
        self.0[Self::B]
    }
    pub fn get_a(&self) -> u8 {
        self.0[Self::A]
    }

    /* ===== setters ===== */

    pub fn set_r(&mut self, v: u8) {
        self.0[Self::R] = v;
    }
    pub fn set_g(&mut self, v: u8) {
        self.0[Self::G] = v;
    }
    pub fn set_b(&mut self, v: u8) {
        self.0[Self::B] = v;
    }
    pub fn set_a(&mut self, v: u8) {
        self.0[Self::A] = v;
    }

    /* ===== gamma ===== */

    pub fn with_gamma(&self, gamma: f32) -> Self {
        fn corr(v: u8, inv: f32) -> u8 {
            let x = v as f32 / 255.0;
            (x.powf(inv) * 255.0).round().clamp(0.0, 255.0) as u8
        }

        let inv = 1.0 / gamma;

        let mut out = *self;
        out.0[Self::R] = corr(self.get_r(), inv);
        out.0[Self::G] = corr(self.get_g(), inv);
        out.0[Self::B] = corr(self.get_b(), inv);
        out
    }

    pub fn apply_gamma(&mut self, gamma: f32) {
        fn corr(v: u8, inv: f32) -> u8 {
            let x = v as f32 / 255.0;
            (x.powf(inv) * 255.0).round().clamp(0.0, 255.0) as u8
        }

        let inv = 1.0 / gamma;
        self.0[Self::R] = corr(self.get_r(), inv);
        self.0[Self::G] = corr(self.get_g(), inv);
        self.0[Self::B] = corr(self.get_b(), inv);
    }

    /* ===== raw access ===== */

    pub fn as_rgba(&self) -> &[u8; 4] {
        &self.0
    }

        /// 2色を指定した色空間で補間する。
    pub fn interpolate(
        self,
        other: Self,
        t: f32,
        mode: crate::util::palette::ColorInterpolation,
    ) -> Self {
        let t = t.clamp(0.0, 1.0);
        if t == 0.0 { return self; }
        if t == 1.0 { return other; }

        let alpha = lerp(self.get_a() as f32 / 255.0, other.get_a() as f32 / 255.0, t);
        let (r, g, b) = match mode {
            crate::util::palette::ColorInterpolation::LinearRgb => {
                let a = [
                    srgb_to_linear(self.get_r() as f32 / 255.0),
                    srgb_to_linear(self.get_g() as f32 / 255.0),
                    srgb_to_linear(self.get_b() as f32 / 255.0),
                ];
                let b = [
                    srgb_to_linear(other.get_r() as f32 / 255.0),
                    srgb_to_linear(other.get_g() as f32 / 255.0),
                    srgb_to_linear(other.get_b() as f32 / 255.0),
                ];
                (
                    linear_to_srgb(lerp(a[0], b[0], t)),
                    linear_to_srgb(lerp(a[1], b[1], t)),
                    linear_to_srgb(lerp(a[2], b[2], t)),
                )
            }
            crate::util::palette::ColorInterpolation::Oklab => {
                let a = self.to_oklab();
                let b = other.to_oklab();
                let lab = [
                    lerp(a[0], b[0], t),
                    lerp(a[1], b[1], t),
                    lerp(a[2], b[2], t),
                ];
                let rgb = Self::oklab_to_linear_rgb(lab);
                (
                    linear_to_srgb(rgb[0]),
                    linear_to_srgb(rgb[1]),
                    linear_to_srgb(rgb[2]),
                )
            }
        };

        Self::new(to_byte(r), to_byte(g), to_byte(b), to_byte(alpha))
    }
}



fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 { value / 12.92 }
    else { ((value + 0.055) / 1.055).powf(2.4) }
}

fn linear_to_srgb(value: f32) -> f32 {
    let value = value.clamp(0.0, 1.0);
    if value <= 0.003_130_8 { 12.92 * value }
    else { 1.055 * value.powf(1.0 / 2.4) - 0.055 }
}

fn to_byte(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

