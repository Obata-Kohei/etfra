use std::{
    path::Path,
    sync::Arc,
};

use super::{
    compute::{ComputeManager, ComputeResult, FractalBackend},
    render_config::RenderTarget,
    settings::AppSettings,
    state::RenderedImage,
};

/// エクスポート解像度。未指定なら現在の解像度から導出したBurst解像度を使う。
///
/// 指定時は自動導出される比率にかかわらず、任意の幅・高さを使う。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExportOptions {
    pub resolution: Option<(usize, usize)>,
}

/// プレビューとは独立してエクスポート計算を管理する。
pub struct ExportManager {
    compute: ComputeManager,
}

impl ExportManager {
    pub fn new() -> Self {
        Self {
            compute: ComputeManager::new(Arc::new(FractalBackend)),
        }
    }

    /// 設定スナップショットを使ってエクスポート計算を開始する。
    pub fn request(&mut self, settings: AppSettings, options: ExportOptions) -> u64 {
        self.compute.request_for_target(
            settings,
            RenderTarget::Export {
                resolution: options.resolution,
            },
        )
    }

    pub fn poll_result(&mut self) -> Option<ComputeResult> {
        self.compute.poll_result()
    }

    pub fn is_busy(&self) -> bool {
        self.compute.is_busy()
    }

    pub fn cancel(&mut self) {
        self.compute.cancel();
    }
}

impl Default for ExportManager {
    fn default() -> Self {
        Self::new()
    }
}

/// RGBA画像を拡張子に応じた形式で保存する。
pub fn save_rendered_image(
    rendered: &RenderedImage,
    path: impl AsRef<Path>,
) -> Result<(), ExportError> {
    if !rendered.is_valid() {
        return Err(ExportError::InvalidImage);
    }

    let width = u32::try_from(rendered.resolution.0)
        .map_err(|_| ExportError::DimensionsTooLarge)?;
    let height = u32::try_from(rendered.resolution.1)
        .map_err(|_| ExportError::DimensionsTooLarge)?;

    image::save_buffer(
        path,
        &rendered.rgba,
        width,
        height,
        image::ColorType::Rgba8,
    )
    .map_err(ExportError::SaveFailed)
}

#[derive(Debug)]
pub enum ExportError {
    InvalidImage,
    DimensionsTooLarge,
    SaveFailed(image::ImageError),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidImage => write!(formatter, "画像の解像度とRGBAデータが一致しません"),
            Self::DimensionsTooLarge => {
                write!(formatter, "画像の解像度が保存形式の上限を超えています")
            }
            Self::SaveFailed(error) => {
                write!(formatter, "画像の保存に失敗しました: {error}")
            }
        }
    }
}

impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SaveFailed(error) => Some(error),
            Self::InvalidImage | Self::DimensionsTooLarge => None,
        }
    }
}
