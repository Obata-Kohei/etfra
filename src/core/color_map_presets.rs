use crate::prelude::*;

pub struct ColorMapLinear {
    pub palette: Palette,
}

impl ColorMap for ColorMapLinear {
    fn map(&self, t: f64) -> Color {
        let n = self.palette.len();

        let idx = (t * (n - 1) as f64)
            .clamp(0.0, (n - 1) as f64) as usize;

        let color = self.palette.get(idx);

        color
            .copied()
            .unwrap_or(Color::BLACK)
    }
}