use eframe::egui;
use num::Complex;
use crate::prelude::*;

pub struct AppState {
    img_cfg: ImageConfig,
    mode: RenderMode,
    recomp: bool,
    move_ratio: Float,
    zoom_ratio: Float,
    history: History,

    image: Option<egui::TextureHandle>,
}

#[derive(Clone)]
struct ImageConfig {
    resolusion: (usize, usize),
    center: Complex<Float>,
    scale: Float
}

pub enum RenderMode {
    Survey,
    Burst,
}

impl RenderMode {
    pub fn resolusion(&self) -> (usize, usize) {
        match self {
            RenderMode::Survey => (64, 64),
            RenderMode::Burst => (1024, 1024),
        }
    }
    //pub fn config_resolusion(mode: RenderMode, reso: (usize, usize)) {}
}

struct History {
    stack: Vec<ImageConfig>,
}

impl AppState {
    pub fn new(
        img_cfg: ImageConfig,
        mode: RenderMode,
        recomp: bool,
        move_ratio: Float,
        zoom_ratio: Float,
        history: History,

        image: Option<egui::TextureHandle>,
    ) -> Self {
        Self {img_cfg, mode, recomp, move_ratio, zoom_ratio, history, image}
    }

    pub fn with_preset_values() -> Self {
        Self {
            img_cfg: ImageConfig { resolusion: RenderMode::Survey.resolusion(), center: Complex::ZERO, scale: 0.001, },
            mode: RenderMode::Survey,
            recomp: false,
            move_ratio: 0.1,
            zoom_ratio: 0.1,
            history: History { stack: Vec::new() },
            image: None,
        }
    }

    pub fn move_left(&mut self) {
        self.img_cfg.center.re -= (self.img_cfg.scale * self.img_cfg.resolusion.0 as Float) * self.move_ratio;
    }

    pub fn move_right(&mut self) {
        self.img_cfg.center.re += (self.img_cfg.scale * self.img_cfg.resolusion.0 as Float) * self.move_ratio;
    }

    pub fn move_up(&mut self) {
        self.img_cfg.center.im += (self.img_cfg.scale * self.img_cfg.resolusion.1 as Float) * self.move_ratio;
    }

    pub fn move_down(&mut self) {
        self.img_cfg.center.im -= (self.img_cfg.scale * self.img_cfg.resolusion.1 as Float) * self.move_ratio;
    }

    pub fn zoom_in(&mut self) {
        self.img_cfg.scale *= self.zoom_ratio;
    }

    pub fn zoom_out(&mut self) {
        self.img_cfg.scale /= self.zoom_ratio;
    }

    pub fn set_resolusion(&mut self, reso: (usize, usize)) {
        self.img_cfg.resolusion = reso;
    }

    pub fn set_mode(&mut self, mode: RenderMode) {
        self.mode = mode;
        self.img_cfg.resolusion = self.mode.resolusion();
    }

    pub fn set_recomp(&mut self, recomp: bool) {
        self.recomp = recomp;
    }

    pub fn set_move_ratio(&mut self, move_ratio: Float) {
        self.move_ratio = move_ratio;
    }

    pub fn set_zoom_ratio(&mut self, zoom_ratio: Float) {
        self.zoom_ratio = zoom_ratio;
    }

    pub fn push_history(&mut self) {
        self.history.stack.push(self.img_cfg.clone());
    }

    pub fn undo(&mut self) {
        let img_cfg = self.history.stack.pop();
        if let Some(ic) = img_cfg {
            self.img_cfg = ic;
        }
    }
}

/*
resolusion: 画像の縦横pixel数 [px] 
center: 中心の複素数座標 [complex]
scale: 複素数平面の長さ/1pixel [complex_length/px]

mode: survey or burst
recomp: 再計算
move_ratio: 平行移動の量 = (scale * resolusion) * move_ratio
zoom_ratio: 拡大縮小の量．scale *(or /)= zoom_ratio
image: Option<egui::TextureHandle
*/