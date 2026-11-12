use etfra::prelude::*;

use super::settings::{AppSettings, BURST_LONG_EDGE};

/// プレビューとエクスポートで使う解像度の選択方法。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderTarget {
    Preview,
    /// `None`は選択中の比率でBurst解像度、`Some`は比率も上書きする幅・高さ指定。
    Export {
        resolution: Option<(usize, usize)>,
    },
}

/// 設定値から描画時に使う画像設定を解決する。
///
/// 保存されたBurst基準の表示範囲を保ちながら、出力解像度に応じて
/// 1ピクセルあたりの座標スケールを再計算する。
pub fn resolve_image_config(
    settings: &AppSettings,
    target: RenderTarget,
) -> Result<ImageConfig, RenderConfigError> {
    let resolution = match target {
        RenderTarget::Preview => settings
            .aspect_ratio
            .dimensions_for_long_edge(settings.render_mode.long_edge())
            .ok_or(RenderConfigError::InvalidResolution)?,
        RenderTarget::Export { resolution: None } => settings
            .aspect_ratio
            .dimensions_for_long_edge(BURST_LONG_EDGE)
            .ok_or(RenderConfigError::InvalidResolution)?,
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
    use crate::app::settings::{AspectRatio, RenderMode};

    #[test]
    fn preview_resolution_uses_quality_mode_and_aspect_ratio() {
        let mut settings = AppSettings::default();
        settings.aspect_ratio = AspectRatio::LANDSCAPE_16_9;
        settings.image.resolution = (512, 288);
        settings.image.scale = (3.0 / 512.0, 3.0 / 512.0);

        let survey = resolve_image_config(&settings, RenderTarget::Preview).unwrap();
        assert_eq!(survey.resolution, (256, 144));
        assert_eq!(survey.view_size(), (3.0, 3.0 * 144.0 / 256.0));

        settings.render_mode = RenderMode::Burst;
        let burst = resolve_image_config(&settings, RenderTarget::Preview).unwrap();
        assert_eq!(burst.resolution, (512, 288));
        assert_eq!(burst.view_size(), (3.0, 3.0 * 288.0 / 512.0));
    }

    #[test]
    fn export_defaults_to_burst_and_accepts_custom_dimensions() {
        let mut settings = AppSettings::default();
        settings.aspect_ratio = AspectRatio::PORTRAIT_9_16;
        settings.image.resolution = (288, 512);
        settings.image.scale = (3.0 / 288.0, 3.0 / 288.0);

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
}
