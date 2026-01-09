use crate::prelude::*;

#[derive(Debug)]
pub struct ColoringByPalette {
    pub palette: Palette,
}

impl Coloring<usize> for ColoringByPalette {
    fn prepare(&mut self, _values: &[usize]) {}

    fn color(&self, n: usize) -> Color {
        let idx = n * self.palette.len() / self.max_iter;
        self.palette
            .get(idx.min(self.palette.len() - 1))
            .copied()
            .unwrap_or(Color::BLACK)
    }
}



#[derive(Debug)]
pub struct PaletteColoring {
    pub palette: Palette,
    pub max_iter: usize
}

impl PaletteColoring {
    pub fn new(palette: Palette, max_iter: usize) -> Self {
        Self { palette, max_iter }
    }
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


#[derive(Debug)]
pub struct ColoringByHistgram {
    cdf: Vec<Float>,
    palette: Palette,
}

impl Coloring<usize> for ColoringByHistgram {
    fn prepare(&mut self, values: &[usize]) {
        let max_iter = self.cdf.len() - 1;
        let mut hist = vec![0usize; max_iter + 1];

        for &v in values {
            hist[v] += 1;
        }
    }
}

