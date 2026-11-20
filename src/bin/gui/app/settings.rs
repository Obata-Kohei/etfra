use etfra::prelude::*;
use serde::{Deserialize, Serialize};

pub const BURST_LONG_EDGE: usize = 512;
pub const SURVEY_LONG_EDGE: usize = 128;
pub const LIGHT_LONG_EDGE: usize = 64;
pub const SUPER_LIGHT_LONG_EDGE: usize = 32;

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
    GrayscaleInv,
    HueGradient,
    HueGradientInv,
    Red,
    Blue,
    Orange,
    Green,
    BlueAndRed,
}

impl ColorMapSettings {
    pub fn create_color_map(&self) -> Palette {
        match &self {
            ColorMapSettings::Grayscale => Palette::grayscale(256),
            ColorMapSettings::HueGradient => {
                Palette::gradation_by_hue(256, 0.0, 360.0, 1.0, 1.0)
            },
            ColorMapSettings::GrayscaleInv => {
                let mut p = Palette::grayscale(256);
                p.reverse();
                p
            }
            ColorMapSettings::HueGradientInv => {
                let mut p = Palette::gradation_by_hue(256, 0.0, 360.0, 1.0, 1.0);
                p.reverse();
                p
            },
            ColorMapSettings::Red => Palette::red_white(256, ColorInterpolation::Oklab),
            ColorMapSettings::Blue => Palette::blue_white(256, ColorInterpolation::Oklab),
            ColorMapSettings::Orange => Palette::orange_white(256, ColorInterpolation::Oklab),
            ColorMapSettings::Green => Palette::green_white(256, ColorInterpolation::Oklab),
            ColorMapSettings::BlueAndRed => Palette::blue_white_red(256, ColorInterpolation::Oklab),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NormalizationSettings {
    MaxIterations,
    Histogram,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub dynamics: DynamicsSettings,
    pub image: ImageConfig,
    pub max_iter: usize,
    pub escape_radius: f64,
    pub color_map: ColorMapSettings,
    pub normalization: NormalizationSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        let resolution = (SURVEY_LONG_EDGE, SURVEY_LONG_EDGE);
        Self {
            dynamics: DynamicsSettings::Mandelbrot,
            image: ImageConfig {
                resolution,
                center: (-0.5, 0.0),
                scale: (
                    3.0 / resolution.0 as f64,
                    3.0 / resolution.1 as f64,
                ),
            },
            max_iter: 500,
            escape_radius: 2.0,
            color_map: ColorMapSettings::Grayscale,
            normalization: NormalizationSettings::MaxIterations,
        }
    }
}