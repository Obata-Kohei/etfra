#[derive(Clone, Copy, Debug)]
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

    /* ===== presets ===== */

    pub const BLACK: Color = Color([0, 0, 0, 255]);
    pub const WHITE: Color = Color([255, 255, 255, 255]);
    pub const RED:   Color = Color([255, 0, 0, 255]);
    pub const GREEN: Color = Color([0, 255, 0, 255]);
    pub const BLUE:  Color = Color([0, 0, 255, 255]);

    /* ===== getters ===== */

    pub fn get_r(&self) -> u8 { self.0[Self::R] }
    pub fn get_g(&self) -> u8 { self.0[Self::G] }
    pub fn get_b(&self) -> u8 { self.0[Self::B] }
    pub fn get_a(&self) -> u8 { self.0[Self::A] }

    /* ===== setters ===== */

    pub fn set_r(&mut self, v: u8) { self.0[Self::R] = v; }
    pub fn set_g(&mut self, v: u8) { self.0[Self::G] = v; }
    pub fn set_b(&mut self, v: u8) { self.0[Self::B] = v; }
    pub fn set_a(&mut self, v: u8) { self.0[Self::A] = v; }

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
}
