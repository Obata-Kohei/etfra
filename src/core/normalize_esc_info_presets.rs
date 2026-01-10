use crate::prelude::*;

#[derive(Debug)]
pub struct NormalizeWithMaxIter {
    pub max_iter: usize
}

impl NormalizeEscInfo<EscapeResult> for NormalizeWithMaxIter {
    fn max_iter(&self) -> usize {
        self.max_iter
    }

    fn prepare(&mut self, _: &[EscapeResult]) {}

    fn normalize(&self, esc_res: &EscapeResult) -> f64 {
        esc_res.iter as f64 / self.max_iter as f64
    }
}


#[derive(Debug)]
pub struct NormalizeWithHistgram {
    max_iter: usize,
    cdf: Vec<f64>,
}

impl NormalizeWithHistgram {
    pub fn new(max_iter: usize) -> Self {
        Self {
            max_iter,
            cdf: vec![0.0; max_iter + 1],
        }
    }
}

impl NormalizeEscInfo<EscapeResult> for NormalizeWithHistgram {
    fn max_iter(&self) -> usize {
        self.max_iter
    }

    fn prepare(&mut self, values: &[EscapeResult]) {
        let max_iter = self.cdf.len() - 1;
        let mut hist = vec![0usize; max_iter + 1];

        for r in values {
            if r.escaped {
                hist[r.iter] += 1;
            }
        }

        let total: usize = hist.iter().sum();
        let mut acc = 0.0;

        for (i, h) in hist.iter().enumerate() {
            acc += *h as f64 / total as f64;
            self.cdf[i] = acc;
        }
    }

    fn normalize(&self, value: &EscapeResult) -> f64 {
        if value.iter >= self.max_iter {
            1.0
        } else {
            self.cdf[value.iter]
        }
    }
}
