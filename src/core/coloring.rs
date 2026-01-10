use crate::prelude::*;

pub struct Coloring<N, M> {
    pub normalizer: N,
    pub color_map: M,
}

impl<N, M> Coloring<N, M>
where
    N: NormalizeEscInfo<EscapeResult>,
    M: ColorMap,
{
    pub fn apply(&self, esc_res: &EscapeResult) -> Color {
        let t = self.normalizer.normalize(esc_res);
        self.color_map.map(t)
    }
}
