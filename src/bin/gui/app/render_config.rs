use etfra::prelude::*;
use serde::{de, Deserialize, Deserializer, Serialize};

use super::settings::{AppSettings, BURST_LONG_EDGE};

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
        dimensions_for_ratio(self.width, self.height, long_edge)
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

fn dimensions_for_ratio(
    width_ratio: u32,
    height_ratio: u32,
    long_edge: usize,
) -> Option<(usize, usize)> {
    if long_edge == 0 {
        return None;
    }

    let (long_ratio, short_ratio) = if width_ratio >= height_ratio {
        (width_ratio, height_ratio)
    } else {
        (height_ratio, width_ratio)
    };
    let short_edge = (long_edge as u128)
        .checked_mul(short_ratio as u128)?
        .checked_add(long_ratio as u128 / 2)?
        .checked_div(long_ratio as u128)?;
    let short_edge = usize::try_from(short_edge).ok()?.max(1);

    if width_ratio >= height_ratio {
        Some((long_edge, short_edge))
    } else {
        Some((short_edge, long_edge))
    }
}

/// プレビュー品質を指定する。解像度自体はAppSettings.image.resolutionに保持する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenderMode {
    Survey,
    Burst,
}

impl RenderMode {
    /// SurveyはBurstの長辺解像度の1/4で描画する。
    pub const fn long_edge(self) -> usize {
        match self {
            Self::Survey => BURST_LONG_EDGE / 4,
            Self::Burst => BURST_LONG_EDGE,
        }
    }
}

/// 既存解像度の比率を保ったまま、長辺を指定値に変更する。
pub fn dimensions_for_long_edge(
    resolution: (usize, usize),
    long_edge: usize,
) -> Option<(usize, usize)> {
    if resolution.0 == 0 || resolution.1 == 0 {
        return None;
    }

    let (long_ratio, short_ratio) = if resolution.0 >= resolution.1 {
        (resolution.0, resolution.1)
    } else {
        (resolution.1, resolution.0)
    };
    if long_edge == 0 {
        return None;
    }

    let short_edge = (long_edge as u128)
        .checked_mul(short_ratio as u128)?
        .checked_add(long_ratio as u128 / 2)?
        .checked_div(long_ratio as u128)?;
    let short_edge = usize::try_from(short_edge).ok()?.max(1);

    if resolution.0 >= resolution.1 {
        Some((long_edge, short_edge))
    } else {
        Some((short_edge, long_edge))
    }
}

/// プレビューとエクスポートで使う解像度の選択方法。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTarget {
    Preview,
    /// `None`は現在の解像度から導出したBurst解像度、`Some`は任意の幅・高さ。
    Export {
        resolution: Option<(usize, usize)>,
    },
}

/// 保存された実解像度と表示範囲から描画時の画像設定を解決する。
pub fn resolve_image_config(
    settings: &AppSettings,
    target: RenderTarget,
) -> Result<ImageConfig, RenderConfigError> {
    let resolution = match target {
        RenderTarget::Preview => settings.image.resolution,
        RenderTarget::Export { resolution: None } => {
            dimensions_for_long_edge(settings.image.resolution, BURST_LONG_EDGE)
                .ok_or(RenderConfigError::InvalidResolution)?
        }
        RenderTarget::Export {
            resolution: Some(resolution),
        } => resolution,
    };

    if resolution.0 == 0 || resolution.1 == 0 {
        return Err(RenderConfigError::InvalidResolution);
    }

    let view_width = settings.image.view_size().0;
    if !view_width.is_finite() || view_width <= 0.0 {
        return Err(RenderConfigError::InvalidViewWidth);
    }

    let pixel_scale = view_width / resolution.0 as f64;
    if !pixel_scale.is_finite() || pixel_scale <= 0.0 {
        return Err(RenderConfigError::InvalidViewWidth);
    }

    Ok(ImageConfig::new(
        resolution,
        (pixel_scale, pixel_scale),
        settings.image.center,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderConfigError {
    InvalidResolution,
    InvalidViewWidth,
}

impl std::fmt::Display for RenderConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidResolution => {
                write!(formatter, "出力解像度は幅・高さともに1以上である必要があります")
            }
            Self::InvalidViewWidth => write!(formatter, "表示範囲の幅が不正です"),
        }
    }
}

impl std::error::Error for RenderConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_uses_the_stored_resolution() {
        let mut settings = AppSettings::default();
        settings.image.resolution = (256, 144);
        settings.image.scale = (3.0 / 256.0, 3.0 / 256.0);

        let preview = resolve_image_config(&settings, RenderTarget::Preview).unwrap();
        assert_eq!(preview.resolution, (256, 144));
        assert_eq!(preview.view_size(), (3.0, 3.0 * 144.0 / 256.0));
    }

    #[test]
    fn export_defaults_to_burst_and_accepts_custom_dimensions() {
        let mut settings = AppSettings::default();
        settings.image.resolution = (144, 256);
        settings.image.scale = (3.0 / 144.0, 3.0 / 144.0);

        let default_export = resolve_image_config(
            &settings,
            RenderTarget::Export { resolution: None },
        )
        .unwrap();
        assert_eq!(default_export.resolution, (288, 512));
        assert!((default_export.view_size().0 - 3.0).abs() < 1e-12);

        let custom_export = resolve_image_config(
            &settings,
            RenderTarget::Export {
                resolution: Some((1920, 1080)),
            },
        )
        .unwrap();
        assert_eq!(custom_export.resolution, (1920, 1080));
        assert!((custom_export.view_size().0 - 3.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_zero_sized_custom_export() {
        let result = resolve_image_config(
            &AppSettings::default(),
            RenderTarget::Export {
                resolution: Some((0, 1080)),
            },
        );

        assert_eq!(result, Err(RenderConfigError::InvalidResolution));
    }

    #[test]
    fn survey_uses_one_quarter_of_burst_long_edge() {
        assert_eq!(RenderMode::Survey.long_edge(), BURST_LONG_EDGE / 4);
        assert_eq!(RenderMode::Burst.long_edge(), BURST_LONG_EDGE);
    }
}
