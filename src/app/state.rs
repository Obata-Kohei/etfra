use eframe::egui;
use num::Complex;
use crate::prelude::*;

pub struct AppState {
    view_state: ViewState,
    render_state: RenderState,
    history: History,
}

struct ViewState {
    center: Complex<Float>,
    scale: Float,
    move_ratio: Float,
    zoom_ratio: Float,
}

impl ViewState {
    pub fn new(center: Complex<Float>, scale: Float, move_ratio: Float, zoom_ratio: Float) -> Self {
        Self { center, scale, move_ratio, zoom_ratio }
    }

    pub fn get_center(&self) -> Complex<Float> {
        self.center
    }
    pub fn get_scale(&self) -> Float {
        self.scale
    }

    pub fn set_center(&mut self, center: Complex<Float>) {
        self.center = center;
    }
    pub fn set_scale(&mut self, scale: Float) {
        self.scale = scale;
    }

    pub fn left(&mut self) {
        self.center.re -= 
    }
}

struct RenderState {
    resolution: (usize, usize),
    mode: RenderMode,
    recomp: bool,
    image: Option<egui::TextureHandle>,
}

enum RenderMode {
    Survey,
    Burst,
}

struct History {
    stack: Vec<ViewState>,
}

/*
resolusion: 画像の縦横pixel数 [px] 
center: 中心の複素数座標 [complex]
scale: 複素数平面の長さ/1pixel [complex_length/px]
mode: survey or burst
recomp: 再計算
move_ratio: 平行移動の量 = (scale * resolusion) * move_ratio
zoom_ratio: 拡大縮小の量．scale *(or /)= zoom_ratio
*/