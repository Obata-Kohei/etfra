use crate::util::color::Color;

/// 色補間を行う色空間。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorInterpolation {
    /// sRGBをlinear RGBへ変換してから補間する。
    LinearRgb,
    /// OKLab色空間で補間する。
    Oklab,
}

#[derive(Debug)]
pub struct Palette(Vec<Color>);

impl Palette {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn from_vec(v: Vec<Color>) -> Self { Self(v) }

    /// 複数の基準色を等間隔に配置し、その間を指定した色空間で補間する。
    ///
    /// `colors` は空でないこと、`n` は1以上であること。
    /// 基準色は色列全体の両端を含む等間隔の位置に配置される。
    /// `n == 1` の場合は最初の基準色を返す。
    pub fn from_colors(n: usize, colors: &[Color], interpolation: ColorInterpolation) -> Self {
        assert!(n >= 1, "palette length must be at least 1");
        assert!(!colors.is_empty(), "at least one anchor color is required");
        if n == 1 || colors.len() == 1 {
            return Self(vec![colors[0]; n]);
        }

        Self::abst_gradation(n, |t| {
            let scaled = t * (colors.len() - 1) as f32;
            let segment = (scaled.floor() as usize).min(colors.len() - 2);
            let local_t = (scaled - segment as f32).clamp(0.0, 1.0);
            colors[segment].interpolate(colors[segment + 1], local_t, interpolation)
        })
    }

    /// クロージャ等で定義されたfに応じてグラデーションを作成
    ///
    /// t = i / (n-1)として[0, n-1](usize)を[0, 1](f32)に写像してtに格納する
    /// f: f32(=t) -> Color
    /// f(t)で作った色を順に格納する

    /// 例:
    ///    Palette::abst_gradation(n, |t| {
    ///        Color::from_hsb(h, s, b_min + (b_max - b_min) * t)
    ///    })
    pub fn abst_gradation<F>(n: usize, mut f: F) -> Self
    where F: FnMut(f32) -> Color,
    {
        assert!(n >= 1);
        let mut pal = Self::from_vec(Vec::with_capacity(n));
        if n == 1 {
            pal.0.push(f(0.0));
            return pal;
        }
        for i in 0..n {
            let t = i as f32 / (n as f32 - 1.0);
            pal.0.push(f(t));
        }
        pal
    }

    /// グレイスケールのパレット作成．n: 色数
    pub fn grayscale(n: usize) -> Palette {
        Palette::abst_gradation(n, |t| {
            let v = (t * 255.0).round() as u8;
            Color::from_rgb(v, v, v)
        })
    }

    // hue値を循環させる
    /*
    fn lerp_hue(h0: f32, h1: f32, t: f32) -> f32 {
        let mut dh = h1 - h0;

        // 0.5（180度）より遠回りしている場合、逆方向に補正する
        if dh > 0.5 {
            dh -= 1.0;
        } else if dh < -0.5 {
            dh += 1.0;
        }

        // 計算結果を 0.0 ~ 1.0 の範囲に収める
        (h0 + dh * t).rem_euclid(1.0)
    }
    */

    /// HSBのhueを元にグラデーションする
    pub fn gradation_by_hue(n: usize, h_min: f32, h_max: f32, s: f32, b: f32) -> Palette {
        Palette::abst_gradation(n, |t| {
            Color::from_hsb(h_min + (h_max - h_min) * t, s, b)
        })
    }

    // HSBのsaturationを元にグラデーションする
    pub fn gradation_by_saturation(n: usize, h: f32, s_min: f32, s_max: f32, b: f32) -> Palette {
        Palette::abst_gradation(n, |t| Color::from_hsb(h, s_min + (s_max - s_min) * t, b))
    }

    // HSBのbrightnessを元にグラデーションする
    pub fn gradation_by_brightness(n: usize, h: f32, s: f32, b_min: f32, b_max: f32) -> Palette {
        Palette::abst_gradation(n, |t| Color::from_hsb(h, s, b_min + (b_max - b_min) * t))
    }

    /// 暗い赤から白へのパレット。
    pub fn red_white(n: usize, interpolation: ColorInterpolation) -> Self {
        Self::from_colors(n, &[Color::from_rgb(0x7f, 0x1d, 0x1d), Color::WHITE], interpolation)
    }

    /// 暗い青から白へのパレット。
    pub fn blue_white(n: usize, interpolation: ColorInterpolation) -> Self {
        Self::from_colors(n, &[Color::from_rgb(0x17, 0x3b, 0x70), Color::WHITE], interpolation)
    }

    /// 暗いオレンジから白へのパレット。
    pub fn orange_white(n: usize, interpolation: ColorInterpolation) -> Self {
        Self::from_colors(n, &[Color::from_rgb(0x8a, 0x3f, 0x0d), Color::WHITE], interpolation)
    }

    /// 暗い緑から白へのパレット。
    pub fn green_white(n: usize, interpolation: ColorInterpolation) -> Self {
        Self::from_colors(n, &[Color::from_rgb(0x17, 0x4d, 0x35), Color::WHITE], interpolation)
    }

    /// 暗い青から白を経由して暗い赤へ向かう発散型パレット。
    pub fn blue_white_red(n: usize, interpolation: ColorInterpolation) -> Self {
        Self::from_colors(
            n,
            &[
                Color::from_rgb(0x17, 0x3b, 0x70),
                Color::WHITE,
                Color::from_rgb(0x7a, 0x1f, 0x2b),
            ],
            interpolation,
        )
    }

    /// 要素数
    pub fn len(&self) -> usize { self.0.len() }
    /// 空かどうか
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    /// indexの色を読み取る
    pub fn get(&self, index: usize) -> Option<&Color> { self.0.get(index) }
    /// 最後尾の要素を返す
    pub fn back(&self) -> Option<&Color> { self.0.last() }
    /// iterを返す
    pub fn iter(&self) -> impl Iterator<Item = &Color> { self.0.iter() }
    /// 最後尾に要素追加
    pub fn push(&mut self, color: Color) { self.0.push(color) }
    /// indexの要素を削除
    pub fn remove(&mut self, index: usize) -> Option<Color> {
        if index < self.len() { Some(self.0.remove(index)) } else { None }
    }
    /// 逆順にする
    pub fn reverse(&mut self) { self.0.reverse(); }
    /// 非破壊的なガンマ補正
    pub fn with_gamma(&self, gamma: f32) -> Self {
        assert!(gamma > 0.0);
        Palette(self.0.iter().map(|c| c.with_gamma(gamma)).collect())
    }
    /// 破壊的なガンマ補正
    pub fn apply_gamma(&mut self, gamma: f32) {
        assert!(gamma > 0.0);
        for c in &mut self.0 { c.apply_gamma(gamma); }
    }
}

impl Default for Palette {
    fn default() -> Self { Self::new() }
}

// for c in &paletteみたいに書くためのTrait
impl<'a> IntoIterator for &'a Palette {
    type Item = &'a Color;
    type IntoIter = std::slice::Iter<'a, Color>;
    fn into_iter(self) -> Self::IntoIter { self.0.iter() }
}

// ::fromやinto()を使うためのTrait
impl From<Vec<Color>> for Palette {
    fn from(v: Vec<Color>) -> Self { Self(v) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolation_preserves_endpoints() {
        let start = Color::from_rgb(12, 34, 56);
        let end = Color::from_rgb(210, 190, 170);
        for mode in [ColorInterpolation::LinearRgb, ColorInterpolation::Oklab] {
            let palette = Palette::from_colors(17, &[start, end], mode);
            assert_eq!(*palette.get(0).unwrap(), start);
            assert_eq!(*palette.back().unwrap(), end);
        }
    }

    #[test]
    fn three_anchor_palette_contains_middle_anchor_when_sampled_at_midpoint() {
        let start = Color::from_rgb(0, 0, 100);
        let palette = Palette::from_colors(3, &[start, Color::WHITE, Color::from_rgb(100, 0, 0)], ColorInterpolation::Oklab);
        assert_eq!(*palette.get(1).unwrap(), Color::WHITE);
    }

    #[test]
    fn one_anchor_is_repeated() {
        let color = Color::from_rgb(20, 30, 40);
        let palette = Palette::from_colors(4, &[color], ColorInterpolation::LinearRgb);
        assert!(palette.iter().all(|c| *c == color));
    }

    #[test]
    fn diverging_preset_has_white_center_for_odd_length() {
        let palette = Palette::blue_white_red(5, ColorInterpolation::Oklab);
        assert_eq!(*palette.get(2).unwrap(), Color::WHITE);
    }
}
