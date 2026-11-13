use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use image::ImageEncoder;
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
    let (width, height) = image_dimensions(rendered)?;

    image::save_buffer(
        path,
        &rendered.rgba,
        width,
        height,
        image::ColorType::Rgba8,
    )
    .map_err(ExportError::SaveFailed)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportFiles {
    pub image_path: PathBuf,
    pub settings_path: Option<PathBuf>,
}

/// 画像と任意の設定JSONを、同一タイムスタンプのファイル名で保存する。
/// 既存ファイルは上書きせず、設定保存に失敗した場合は画像を取り除く。
pub fn save_export_bundle(
    rendered: &RenderedImage,
    settings: &AppSettings,
    directory: impl AsRef<Path>,
    timestamp: &str,
    save_settings: bool,
) -> Result<ExportFiles, ExportError> {
    if timestamp.len() != 14
        || !timestamp.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(ExportError::InvalidTimestamp);
    }

    let image_bytes = encode_png(rendered)?;
    let settings_bytes = if save_settings {
        Some(
            serde_json::to_vec_pretty(settings)
                .map_err(ExportError::SettingsSerializationFailed)?,
        )
    } else {
        None
    };

    let directory = directory.as_ref();
    let image_path = directory.join(format!("fractal_{timestamp}.png"));
    let settings_path = settings_bytes
        .as_ref()
        .map(|_| directory.join(format!("settings_{timestamp}.json")));

    write_new_file(&image_path, &image_bytes)?;

    if let (Some(path), Some(bytes)) = (&settings_path, settings_bytes) {
        if let Err(error) = write_new_file(path, &bytes) {
            let operation_error = error.to_string();
            return match fs::remove_file(&image_path) {
                Ok(()) => Err(error),
                Err(cleanup_error) => Err(ExportError::RollbackFailed {
                    path: image_path,
                    operation_error,
                    cleanup_error,
                }),
            };
        }
    }

    Ok(ExportFiles {
        image_path,
        settings_path,
    })
}

fn image_dimensions(
    rendered: &RenderedImage,
) -> Result<(u32, u32), ExportError> {
    if !rendered.is_valid() {
        return Err(ExportError::InvalidImage);
    }

    let width = u32::try_from(rendered.resolution.0)
        .map_err(|_| ExportError::DimensionsTooLarge)?;
    let height = u32::try_from(rendered.resolution.1)
        .map_err(|_| ExportError::DimensionsTooLarge)?;
    Ok((width, height))
}

fn encode_png(rendered: &RenderedImage) -> Result<Vec<u8>, ExportError> {
    let (width, height) = image_dimensions(rendered)?;
    let mut bytes = Vec::new();
    image::codecs::png::PngEncoder::new(&mut bytes)
        .write_image(
            &rendered.rgba,
            width,
            height,
            image::ColorType::Rgba8.into(),
        )
        .map_err(ExportError::SaveFailed)?;
    Ok(bytes)
}

fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), ExportError> {
    let mut file = match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            return Err(ExportError::OutputAlreadyExists(path.to_owned()));
        }
        Err(source) => {
            return Err(ExportError::FileCreateFailed {
                path: path.to_owned(),
                source,
            });
        }
    };

    if let Err(source) = file.write_all(bytes) {
        drop(file);
        return match fs::remove_file(path) {
            Ok(()) => Err(ExportError::FileWriteFailed {
                path: path.to_owned(),
                source,
            }),
            Err(cleanup_error) => Err(ExportError::RollbackFailed {
                path: path.to_owned(),
                operation_error: format!("ファイル書き込みに失敗しました: {source}"),
                cleanup_error,
            }),
        };
    }

    Ok(())
}

#[derive(Debug)]
pub enum ExportError {
    InvalidImage,
    DimensionsTooLarge,
    SaveFailed(image::ImageError),
    InvalidTimestamp,
    SettingsSerializationFailed(serde_json::Error),
    OutputAlreadyExists(PathBuf),
    FileCreateFailed {
        path: PathBuf,
        source: io::Error,
    },
    FileWriteFailed {
        path: PathBuf,
        source: io::Error,
    },
    RollbackFailed {
        path: PathBuf,
        operation_error: String,
        cleanup_error: io::Error,
    },
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
            Self::InvalidTimestamp => {
                write!(formatter, "ファイル名用の日時は14桁で指定してください")
            }
            Self::SettingsSerializationFailed(error) => {
                write!(formatter, "設定のJSON変換に失敗しました: {error}")
            }
            Self::OutputAlreadyExists(path) => {
                write!(formatter, "保存先のファイルが既に存在します: {}", path.display())
            }
            Self::FileCreateFailed { path, source } => {
                write!(formatter, "ファイルを作成できませんでした ({}): {source}", path.display())
            }
            Self::FileWriteFailed { path, source } => {
                write!(formatter, "ファイルに書き込めませんでした ({}): {source}", path.display())
            }
            Self::RollbackFailed {
                path,
                operation_error,
                cleanup_error,
            } => {
                write!(
                    formatter,
                    "{operation_error}; 不完全なファイル {} の削除にも失敗しました: {cleanup_error}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ExportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SaveFailed(error) => Some(error),
            Self::SettingsSerializationFailed(error) => Some(error),
            Self::FileCreateFailed { source, .. }
            | Self::FileWriteFailed { source, .. } => Some(source),
            Self::RollbackFailed {
                cleanup_error, ..
            } => Some(cleanup_error),
            Self::InvalidImage
            | Self::DimensionsTooLarge
            | Self::InvalidTimestamp
            | Self::OutputAlreadyExists(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    use super::*;

    static NEXT_TEMP_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TempDirectory(PathBuf);

    impl TempDirectory {
        fn new() -> Self {
            let id = NEXT_TEMP_DIRECTORY.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "etfra-export-{}-{id}",
                std::process::id()
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn saves_png_and_settings_with_the_same_timestamp() {
        let directory = TempDirectory::new();
        let settings = AppSettings::default();
        let image = RenderedImage {
            resolution: (1, 1),
            rgba: vec![12, 34, 56, 255],
        };

        let files = save_export_bundle(
            &image,
            &settings,
            &directory.0,
            "20261113021530",
            true,
        )
        .unwrap();

        assert_eq!(
            files.image_path.file_name().unwrap(),
            "fractal_20261113021530.png"
        );
        assert_eq!(
            files.settings_path.as_ref().unwrap().file_name().unwrap(),
            "settings_20261113021530.json"
        );
        let png_bytes = fs::read(&files.image_path).unwrap();
        assert!(png_bytes.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]));
        let saved_settings: AppSettings =
            serde_json::from_slice(&fs::read(files.settings_path.unwrap()).unwrap())
                .unwrap();
        assert_eq!(saved_settings, settings);
    }

    #[test]
    fn does_not_overwrite_an_existing_export_file() {
        let directory = TempDirectory::new();
        let image_path = directory.0.join("fractal_20261113021530.png");
        fs::write(&image_path, b"existing").unwrap();

        let result = save_export_bundle(
            &RenderedImage {
                resolution: (1, 1),
                rgba: vec![12, 34, 56, 255],
            },
            &AppSettings::default(),
            &directory.0,
            "20261113021530",
            true,
        );

        assert!(matches!(
            result,
            Err(ExportError::OutputAlreadyExists(path)) if path == image_path
        ));
        assert_eq!(fs::read(image_path).unwrap(), b"existing");
        assert!(!directory.0.join("settings_20261113021530.json").exists());
    }

    #[test]
    fn removes_the_image_if_the_matching_settings_file_already_exists() {
        let directory = TempDirectory::new();
        let settings_path = directory.0.join("settings_20261113021530.json");
        fs::write(&settings_path, b"existing").unwrap();

        let result = save_export_bundle(
            &RenderedImage {
                resolution: (1, 1),
                rgba: vec![12, 34, 56, 255],
            },
            &AppSettings::default(),
            &directory.0,
            "20261113021530",
            true,
        );

        assert!(matches!(
            result,
            Err(ExportError::OutputAlreadyExists(path)) if path == settings_path
        ));
        assert_eq!(fs::read(settings_path).unwrap(), b"existing");
        assert!(!directory.0.join("fractal_20261113021530.png").exists());
    }
}
