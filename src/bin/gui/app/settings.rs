use etfra::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize};

pub const BURST_LONG_EDGE: usize = 512;

/// 画像の縦横比。値は既約比に正規化する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AspectRatio {
    width: u32,
    height: u32,
}

impl AspectRatio {
    pub const SQUARE: Self = Self {
        width: 1,
        height: 1,
    };
    pub const LANDSCAPE_4_3: Self = Self {
        width: 4,
        height: 3,
    };
    pub const LANDSCAPE_16_9: Self = Self {
        width: 16,
        height: 9,
    };
    pub const PORTRAIT_3_4: Self = Self {
        width: 3,
        height: 4,
    };
    pub const PORTRAIT_9_16: Self = Self {
        width: 9,
        height: 16,
    };

    /// 設定UIに表示する比率プリセット。
    pub const PRESETS: &[Self] = &[
        Self::SQUARE,
        Self::LANDSCAPE_4_3,
        Self::LANDSCAPE_16_9,
        Self::PORTRAIT_3_4,
        Self::PORTRAIT_9_16,
    ];

    pub fn new(width: u32, height: u32) -> Option<Self> {
        if width == 0 || height == 0 {
            return None;
        }

        let divisor = gcd(width, height);
        Some(Self {
            width: width / divisor,
            height: height / divisor,
        })
    }

    pub fn width(self) -> u32 {
        self.width
    }

    pub fn height(self) -> u32 {
        self.height
    }

    /// 長辺を指定したときのピクセル寸法を返す。
    pub fn dimensions_for_long_edge(self, long_edge: usize) -> Option<(usize, usize)> {
        if long_edge == 0 {
            return None;
        }

        let (long_ratio, short_ratio) = if self.width >= self.height {
            (self.width, self.height)
        } else {
            (self.height, self.width)
        };
        let short_edge = (long_edge as u128)
            .checked_mul(short_ratio as u128)?
            .checked_add(long_ratio as u128 / 2)?
            .checked_div(long_ratio as u128)?;
        let short_edge = usize::try_from(short_edge).ok()?;
        let short_edge = short_edge.max(1);

        if self.width >= self.height {
            Some((long_edge, short_edge))
        } else {
            Some((short_edge, long_edge))
        }
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::SQUARE
    }
}

impl<'de> Deserialize<'de> for AspectRatio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SerializedAspectRatio {
            width: u32,
            height: u32,
        }

        let ratio = SerializedAspectRatio::deserialize(deserializer)?;
        Self::new(ratio.width, ratio.height).ok_or_else(|| {
            de::Error::custom("アスペクト比の幅と高さは1以上である必要があります")
        })
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

/// フラクタルの種類と、その計算に必要なパラメータ。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DynamicsSettings {
    Mandelbrot,
    Julia { c_re: Float, c_im: Float },
    BurningShip,
    Tricorn,
    Phoenix { feedback_re: Float, feedback_im: Float },
    Multibrot { power: Float },
    Custom { expression: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorMapSettings {
    Grayscale,
    HueGradient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizationSettings {
    MaxIterations,
    Histogram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderMode {
    Survey,
    Burst,
}

impl RenderMode {
    /// SurveyはBurstの長辺解像度の半分で描画する。
    pub const fn long_edge(self) -> usize {
        match self {
            Self::Survey => BURST_LONG_EDGE / 2,
            Self::Burst => BURST_LONG_EDGE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub dynamics: DynamicsSettings,
    pub image: ImageConfig,
    #[serde(default)]
    pub aspect_ratio: AspectRatio,
    pub max_iter: usize,
    pub escape_radius: f64,
    pub color_map: ColorMapSettings,
    pub normalization: NormalizationSettings,
    pub render_mode: RenderMode,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            dynamics: DynamicsSettings::Mandelbrot,
            image: ImageConfig {
                resolution: (BURST_LONG_EDGE, BURST_LONG_EDGE),
                center: (-0.5, 0.0),
                scale: (
                    3.0 / BURST_LONG_EDGE as f64,
                    3.0 / BURST_LONG_EDGE as f64,
                ),
            },
            aspect_ratio: AspectRatio::SQUARE,
            max_iter: 500,
            escape_radius: 2.0,
            color_map: ColorMapSettings::HueGradient,
            normalization: NormalizationSettings::MaxIterations,
            render_mode: RenderMode::Survey,
        }
    }
}