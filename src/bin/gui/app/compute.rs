
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use num_complex::Complex;
use rayon::prelude::*;

use etfra::{
    prelude::*,
    dynamics::Dynamics, escape_evaluator::EscapeEvaluator, escape_time_fractal::EscapeTimeFractal,
    presets::{
        complex_dynamics_presets::{BurningShip, Julia, Mandelbrot},
        custom_complex_dynamics::dynamics::CustomComplexDynamics,
    },
};

use super::{
    settings::{
        DynamicsSettings,
        ColorMapSettings,
        NormalizationSettings,
        AppSettings,
    },
    render_config::{resolve_image_config, RenderTarget},
    state::RenderedImage,
};

/// 計算要求。
///
/// 計算開始時点の設定を保持する。
#[derive(Debug, Clone)]
pub struct ComputeRequest {
    pub generation: u64,
    pub settings: AppSettings,
    pub target: RenderTarget,
}

/// 計算結果。
#[derive(Debug, Clone)]
pub struct ComputeResult {
    pub generation: u64,
    pub result: Result<RenderedImage, ComputeError>,
}

/// 計算エラー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeError {
    /// キャンセルによる終了。通常の計算エラーとは区別する。
    Cancelled,

    /// 計算エンジンで発生したエラー。
    Failed(String),

    /// 出力画像のRGBAデータが不正。
    InvalidImage,
}

/// 計算エンジンとの接続部分。
///
/// 実際のフラクタル計算処理は、このtraitを実装して提供する。
pub trait ComputeBackend: Send + Sync + 'static {
    fn compute(
        &self,
        settings: &AppSettings,
        target: RenderTarget,
        cancel: &AtomicBool,
    ) -> Result<RenderedImage, ComputeError>;
}

/// 実行中の計算。
struct ActiveJob {
    generation: u64,
    cancel: Arc<AtomicBool>,
    handle: JoinHandle<ComputeResult>,
}

/// 計算要求を非同期に管理する。
///
/// 同時に実行する計算は最大1件とする。
/// 実行中に新しい要求が来た場合は、最新の要求だけを保留する。
pub struct ComputeManager {
    backend: Arc<dyn ComputeBackend>,
    active: Option<ActiveJob>,
    pending: Option<ComputeRequest>,
    next_generation: u64,
}

impl ComputeManager {
    pub fn new(backend: Arc<dyn ComputeBackend>) -> Self {
        Self {
            backend,
            active: None,
            pending: None,
            next_generation: 0,
        }
    }

    /// 新しい計算要求を登録する。
    ///
    /// 実行中の計算があればキャンセルを要求し、
    /// その終了後に最新の要求を開始する。
    ///
    /// 戻り値は要求のgeneration。
    pub fn request(&mut self, settings: AppSettings) -> u64 {
        self.request_for_target(settings, RenderTarget::Preview)
    }

    /// 描画対象を指定して計算を登録する。
    pub(crate) fn request_for_target(
        &mut self,
        settings: AppSettings,
        target: RenderTarget,
    ) -> u64 {
        self.next_generation = self
            .next_generation
            .checked_add(1)
            .expect("compute generation overflow");

        let request = ComputeRequest {
            generation: self.next_generation,
            settings,
            target,
        };

        let generation = request.generation;

        if let Some(active) = &self.active {
            // 実行中の計算を停止要求する。
            active.cancel.store(true, Ordering::Relaxed);

            // 保留要求は最新のものだけを残す。
            self.pending = Some(request);
        } else {
            self.start(request);
        }

        generation
    }

    /// 実行中の計算にキャンセルを要求する。
    pub fn cancel(&mut self) {
        self.pending = None;

        if let Some(active) = &self.active {
            active.cancel.store(true, Ordering::Relaxed);
        }
    }

    /// 計算中、または次の計算要求が保留されているか。
    pub fn is_busy(&self) -> bool {
        self.active.is_some() || self.pending.is_some()
    }

    /// 現在実行中のgenerationを取得する。
    pub fn active_generation(&self) -> Option<u64> {
        self.active.as_ref().map(|job| job.generation)
    }

    /// 計算結果を確認する。
    ///
    /// 計算が終了していれば結果を返す。
    /// 次の要求が保留されていれば、その場で次の計算を開始する。
    ///
    /// 戻り値のgenerationが現在の設定と一致するかは、
    /// 呼び出し側で確認する。
    pub fn poll_result(&mut self) -> Option<ComputeResult> {
        let finished = self
            .active
            .as_ref()
            .is_some_and(|job| job.handle.is_finished());

        if !finished {
            return None;
        }

        let job = self.active.take()?;

        let result = match job.handle.join() {
            Ok(result) => result,
            Err(_) => ComputeResult {
                generation: job.generation,
                result: Err(ComputeError::Failed(
                    "計算スレッドが異常終了しました".to_string(),
                )),
            },
        };

        // 古い計算の終了を確認してから次の計算を開始する。
        if let Some(request) = self.pending.take() {
            self.start(request);
        }

        Some(result)
    }

    fn start(&mut self, request: ComputeRequest) {
        let backend = Arc::clone(&self.backend);
        let cancel = Arc::new(AtomicBool::new(false));
        let worker_cancel = Arc::clone(&cancel);

        let generation = request.generation;

        let handle = thread::spawn(move || {
            let mut result = backend.compute(
                &request.settings,
                request.target,
                &worker_cancel,
            );

            // キャンセル要求後に計算が完了した場合も、
            // その結果は採用しない。
            if worker_cancel.load(Ordering::Relaxed) {
                result = Err(ComputeError::Cancelled);
            }

            // 画像データの整合性を確認する。
            if let Ok(image) = &result {
                if !image.is_valid() {
                    result = Err(ComputeError::InvalidImage);
                }
            }

            ComputeResult {
                generation,
                result,
            }
        });

        self.active = Some(ActiveJob {
            generation,
            cancel,
            handle,
        });
    }
}

/// ComputeManagerのDrop実装．アプリ終了時に計算スレッドを放置せず始末する．
impl Drop for ComputeManager {
    fn drop(&mut self) {
        if let Some(job) = self.active.take() {
            job.cancel.store(true, Ordering::Relaxed);
            let _ = job.handle.join();
        }
    }
}

/// フラクタルを計算するComputeBackend実装。
///
/// 設定を具体的な描画処理へ変換し、RenderedImageを返す。
/// スレッド管理やGUIへの表示は行わない。
pub struct FractalBackend;

impl ComputeBackend for FractalBackend {
    fn compute(
        &self,
        settings: &AppSettings,
        target: RenderTarget,
        cancel: &AtomicBool,
    ) -> Result<RenderedImage, ComputeError> {
        if cancel.load(Ordering::Relaxed) {
            return Err(ComputeError::Cancelled);
        }

        let image_config = resolve_image_config(settings, target)
            .map_err(|error| ComputeError::Failed(error.to_string()))?;
        compute_dynamics(settings, &image_config, cancel)
    }
}

/// Dynamics、Normalizer、ColorMapを使って描画する。
fn render_with<D, N>(
    dynamics: D,
    settings: &AppSettings,
    image_config: &ImageConfig,
    cancel: &AtomicBool,
    normalizer: N,
) -> Result<RenderedImage, ComputeError>
where
    D: Dynamics + Send + Sync,
    EscapeByCount<EscapeByNorm>: EscapeEvaluator<D> + Send + Sync,
    N: NormalizeEscInfo<EscapeResult> + Send + Sync,
{
    if cancel.load(Ordering::Relaxed) {
        return Err(ComputeError::Cancelled);
    }

    let palette = match settings.color_map {
        ColorMapSettings::Grayscale => Palette::grayscale(256),
        ColorMapSettings::HueGradient => {
            Palette::gradation_by_hue(256, 0.0, 360.0, 1.0, 1.0)
        }
    };

    let coloring = Coloring {
        normalizer,
        color_map: ColorMapLinear { palette },
    };

    // EscapeByNormはComplex<Float>とState2ndOrderの両方に対応するため、
    // Phoenixを含む各Dynamicsで同じEvaluatorを利用できる。
    let evaluator = EscapeByCount::new(
        settings.max_iter,
        EscapeByNorm::new(settings.escape_radius),
    );

    let mut fractal = EscapeTimeFractal::new(dynamics, evaluator, coloring);

    // 全ピクセルの脱出結果を先に計算する。
    let escape_results = fractal
        .escape_results_par_interruptible(image_config, cancel)
        .ok_or(ComputeError::Cancelled)?;

    if cancel.load(Ordering::Relaxed) {
        return Err(ComputeError::Cancelled);
    }

    // Normalizerの準備は全ピクセルの結果を使って一度行い、その後に色変換する。
    let colors = fractal
        .colors_from_escape_results_par_interruptible(&escape_results, cancel)
        .ok_or(ComputeError::Cancelled)?;

    if cancel.load(Ordering::Relaxed) {
        return Err(ComputeError::Cancelled);
    }

    // RGBA変換中も各ピクセルでキャンセルを確認する。
    let pixels: Option<Vec<[u8; 4]>> = colors
        .par_iter()
        .map(|color| {
            if cancel.load(Ordering::Relaxed) {
                None
            } else {
                Some(*color.as_rgba())
            }
        })
        .collect();

    let pixels = pixels.ok_or(ComputeError::Cancelled)?;
    let rgba = pixels
        .into_iter()
        .flat_map(|pixel| pixel.into_iter())
        .collect();

    if cancel.load(Ordering::Relaxed) {
        return Err(ComputeError::Cancelled);
    }

    let image = RenderedImage {
        resolution: image_config.resolution,
        rgba,
    };

    if !image.is_valid() {
        return Err(ComputeError::InvalidImage);
    }

    Ok(image)
}

/// NormalizationSettingsに応じたNormalizerを選んで描画する。
fn render_with_normalization<D>(
    dynamics: D,
    settings: &AppSettings,
    image_config: &ImageConfig,
    cancel: &AtomicBool,
) -> Result<RenderedImage, ComputeError>
where
    D: Dynamics + Send + Sync,
    EscapeByCount<EscapeByNorm>: EscapeEvaluator<D> + Send + Sync,
{
    match settings.normalization {
        NormalizationSettings::MaxIterations => render_with(
            dynamics,
            settings,
            image_config,
            cancel,
            NormalizeWithMaxIter {
                max_iter: settings.max_iter,
            },
        ),

        NormalizationSettings::Histogram => render_with(
            dynamics,
            settings,
            image_config,
            cancel,
            NormalizeWithHistgram::new(settings.max_iter),
        ),
    }
}

/// DynamicsSettingsから具体的なDynamicsを選んで描画する。
fn compute_dynamics(
    settings: &AppSettings,
    image_config: &ImageConfig,
    cancel: &AtomicBool,
) -> Result<RenderedImage, ComputeError> {
    if cancel.load(Ordering::Relaxed) {
        return Err(ComputeError::Cancelled);
    }

    match &settings.dynamics {
        DynamicsSettings::Mandelbrot => {
            render_with_normalization(Mandelbrot::new(), settings, image_config, cancel)
        }

        DynamicsSettings::Julia { c_re, c_im } => render_with_normalization(
            Julia::new(Complex::new(*c_re, *c_im)),
            settings,
            image_config,
            cancel,
        ),

        DynamicsSettings::BurningShip => {
            render_with_normalization(BurningShip::new(), settings, image_config, cancel)
        }

        DynamicsSettings::Tricorn => {
            render_with_normalization(Tricorn::new(), settings, image_config, cancel)
        }

        DynamicsSettings::Phoenix {
            feedback_re,
            feedback_im,
        } => render_with_normalization(
            Phoenix::with_feedback(Complex::new(*feedback_re, *feedback_im)),
            settings,
            image_config,
            cancel,
        ),

        DynamicsSettings::Multibrot { power } => {
            render_with_normalization(Multibrot::new(*power), settings, image_config, cancel)
        }

        DynamicsSettings::Custom { expression } => {
            let dynamics = CustomComplexDynamics::new(expression)
                .map_err(|error| ComputeError::Failed(error.to_string()))?;

            render_with_normalization(dynamics, settings, image_config, cancel)
        }
    }
}