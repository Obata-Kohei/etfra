use crate::color::Color;

pub struct Palette (Vec<Color>);

impl Palette {
    // constructor
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_vec(v: Vec<Color>) -> Self {
        Self(v)
    }

    // クロージャ等で定義されたfに応じてグラデーションを作成
    /*
    t = i / (n-1)として[0, n-1](usize)を[0, 1](f32)に写像してtに格納する
    f: f32(=t) -> Color
    f(t)で作った色を順に格納する

    例:
        Palette::abst_gradation(n, |t| {
            Color::from_hsb(h, s, b_min + (b_max - b_min) * t)
        })

    */
    pub fn abst_gradation<F>(n: usize, mut f: F) -> Self
    where
        F: FnMut(f32) -> Color,
    {
        assert!(n >= 1);

        let mut pal = Self::from_vec(Vec::with_capacity(n));

        if n == 1 {
            pal.0.push(f(0.0));
            return  pal;
        }

        for i in 0..n {
            let t = i as f32 / (n as f32 - 1.0);
            pal.0.push(f(t));
        }

        pal
    }

    // グレイスケール
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

    // HSBのhueを元にグラデーションする
    pub fn gradation_by_hue(n: usize, h_min: f32, h_max: f32, s: f32, b: f32) -> Palette {
        Palette::abst_gradation(n, |t| {
            //let h = Palette::lerp_hue(h_min, h_max, t);
            let h = h_min + (h_max - h_min) * t;
            Color::from_hsb(h, s, b)
        })
    }

    // HSBのsaturationを元にグラデーションする
    pub fn gradation_by_saturation(n: usize, h: f32, s_min: f32, s_max: f32, b: f32) -> Palette {
        Palette::abst_gradation(n, |t| {
            Color::from_hsb(h, s_min + (s_max - s_min) * t, b)
        })
    }

    // HSBのbrightnessを元にグラデーションする
    pub fn gradation_by_brightness(n: usize, h: f32, s: f32, b_min: f32, b_max: f32) -> Palette {
        Palette::abst_gradation(n, |t| {
            Color::from_hsb(h, s, b_min + (b_max - b_min) * t)
        })
    }

    // 要素数
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // 空かどうか
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // 読み取り
    pub fn get(&self, index: usize) -> Option<&Color> {
        self.0.get(index)
    }

    // 最終要素
    pub fn back(&self) -> Option<&Color> {
        self.get(self.len() - 1)
    }

    // iterを返す
    pub fn iter(&self) -> impl Iterator<Item = &Color> {
        self.0.iter()
    }

    // 最後尾に要素追加
    pub fn push(&mut self, color: Color) {
        self.0.push(color)
    }

    // indexの要素を削除
    pub fn remove(&mut self, index: usize) -> Option<Color> {
        if index < self.len() {
            Some(self.0.remove(index))
        } else {
            None
        }
    }

    // 逆にする
    pub fn reverse(&mut self) {
        self.0.reverse();
    }

    // ガンマ補正．非破壊的
    pub fn with_gamma(&self, gamma: f32) -> Self {
        assert!(gamma > 0.0);

        Palette(
            self.0
                .iter()
                .map(|c| c.with_gamma(gamma))
                .collect(),
        )
    }

    // ガンマ補正．破壊的
    pub fn apply_gamma(&mut self, gamma: f32) {
        assert!(gamma > 0.0);

        for c in &mut self.0 {
            c.apply_gamma(gamma);
        }
    }
}


// for c in &paletteみたいに書くためのTrait
impl<'a> IntoIterator for &'a Palette {
    type Item = &'a Color;
    type IntoIter = std::slice::Iter<'a, Color>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// ::fromやinto()を使うためのTrait
impl From<Vec<Color>> for Palette {
    fn from(v: Vec<Color>) -> Self {
        Self(v)
    }
}

