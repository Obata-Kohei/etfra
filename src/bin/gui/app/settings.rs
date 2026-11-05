use etfra::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub dynamics: DynamicsSettings,
    pub image: ImageConfig,
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
                resolution: (512, 512),
                center: (-0.5, 0.0),
                scale: (3.0 / 512.0, 3.0 / 512.0),
            },
            max_iter: 500,
            escape_radius: 2.0,
            color_map: ColorMapSettings::HueGradient,
            normalization: NormalizationSettings::MaxIterations,
            render_mode: RenderMode::Survey,
        }
    }
}