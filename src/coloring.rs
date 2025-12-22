use crate::color::Color;
use crate::palette::Palette;
use crate::escape_time_fractal::Float;

pub trait Coloring<T>
where T: Copy
{
    fn color(&self, value: T) -> Color;
}

pub struct PaletteColoring {
    pub palette: Palette,
    pub max_iter: usize
}

impl Coloring<usize> for PaletteColoring {
    fn color(&self, n: usize) -> Color {
        let idx = n * self.palette.len() / self.max_iter;
        self.palette
            .get(idx.min(self.palette.len() - 1))
            .copied()
            .unwrap_or(Color::BLACK)
    }
}

pub struct HistogramColoring {
    cdf: Vec<Float>,
    palette: Vec<Color>,
}

impl HistogramColoring {
    pub fn prepare(values: &[usize], max_iter: usize, palette: Vec<Color>) -> Self {
        let mut hist = vec![0usize; max_iter + 1];
        for &v in values {
            hist[v] += 1;
        }

        let total = values.len() as Float;
        let mut cdf = Vec::with_capacity(hist.len());

        let mut acc = 0.0;
        for h in hist {
            acc += h as Float / total;
            cdf.push(acc);
        }

        Self { cdf, palette }
    }
}

impl Coloring<usize> for HistogramColoring {
    fn color(&self, n: usize) -> Color {
        let t = self.cdf[n];
        let idx = (t * (self.palette.len() - 1) as Float) as usize;
        self.palette[idx]
    }
}
