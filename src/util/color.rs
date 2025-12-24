#[derive (Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub enum ColorParseError {
    InvalidLength,
    InvalidHex,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {r, g, b, a}
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color {r, g, b, a: 255}
    }

    pub fn from_hsb(h: f32, s: f32, b: f32) -> Self {
        // 値を 0.0 ~ 1.0 の範囲にクランプ（正規化）
        let s = s.clamp(0.0, 1.0);
        let b = b.clamp(0.0, 1.0);

        // 色相を 0.0 ~ 360.0 の範囲に収める
        let h = h.rem_euclid(360.0);

        if s == 0.0 {
            // 彩度が0の場合はグレー（明度のみ反映）
            let val = (b * 255.0).round() as u8;
            return Color::new(val, val, val, 255);
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

    pub fn from_hex(color_code: &str) -> Result<Self, ColorParseError> {
        let color_code = color_code.strip_prefix('#').unwrap_or(color_code);
        let len = color_code.len();

        // 16進 → u8 に変換する小さなヘルパ
        fn to_byte(s: &str) -> Result<u8, ColorParseError> {
            u8::from_str_radix(s, 16).map_err(|_| ColorParseError::InvalidHex)
        }

        match len {
            3 => {
                // RGB -> RRGGBB
                let r = to_byte(&color_code[0..1].repeat(2))?;
                let g = to_byte(&color_code[1..2].repeat(2))?;
                let b = to_byte(&color_code[2..3].repeat(2))?;
                Ok(Color::new(r, g, b, 255))
            }
            4 => {
                // RGBA → RRGGBBAA
                let r = to_byte(&color_code[0..1].repeat(2))?;
                let g = to_byte(&color_code[1..2].repeat(2))?;
                let b = to_byte(&color_code[2..3].repeat(2))?;
                let a = to_byte(&color_code[3..4].repeat(2))?;
                Ok(Color { r, g, b, a })
            }
            6 => {
                // RRGGBB
                let r = to_byte(&color_code[0..2])?;
                let g = to_byte(&color_code[2..4])?;
                let b = to_byte(&color_code[4..6])?;
                Ok(Color { r, g, b, a: 255 })
            }
            8 => {
                // RRGGBBAA
                let r = to_byte(&color_code[0..2])?;
                let g = to_byte(&color_code[2..4])?;
                let b = to_byte(&color_code[4..6])?;
                let a = to_byte(&color_code[6..8])?;
                Ok(Color { r, g, b, a })
            }
            _ => Err(ColorParseError::InvalidLength),
        }
    }

    // color preset
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255};
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255};
    pub const GREEN: Color = Color {r: 0, g: 255, b: 0, a: 255};
    pub const BLUE: Color = Color  {r: 0, g: 0, b: 255, a: 255};

    // getter
    pub fn get_r(&self) -> u8 { self.r }
    pub fn get_g(&self) -> u8 { self.g }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_a(&self) -> u8 { self.a }

    // setter
    pub fn set_r(&mut self, v: u8) {self.r = v}
    pub fn set_g(&mut self, v: u8) {self.g = v}
    pub fn set_b(&mut self, v: u8) {self.b = v}
    pub fn set_a(&mut self, v: u8) {self.a = v}

    // ガンマ補正．非破壊的
    pub fn with_gamma(&self, gamma: f32) -> Self {
        fn corr(v: u8, inv: f32) -> u8 {
            let x = v as f32 / 255.0;
            (x.powf(inv) * 255.0).round().clamp(0.0, 255.0) as u8
        }

        let gamma_inv = 1.0 / gamma;

        Color {
            r: corr(self.r, gamma_inv),
            g: corr(self.g, gamma_inv),
            b: corr(self.b, gamma_inv),
            a: self.a,
        }
    }

    // ガンマ補正．破壊的
    pub fn apply_gamma(&mut self, gamma: f32) {
        fn corr(v: u8, inv: f32) -> u8 {
            let x = v as f32 / 255.0;
            (x.powf(inv) * 255.0).round().clamp(0.0, 255.0) as u8
        }
        let gamma_inv = 1.0 / gamma;
        self.r = corr(self.r, gamma_inv);
        self.g = corr(self.g, gamma_inv);
        self.b = corr(self.b, gamma_inv);
    }

}