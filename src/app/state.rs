use crate::prelude::*;
use num_traits::FromPrimitive;
use crate::app::ui_render::RenderEngine;

pub struct AppState {
    pub img_cfg: ImageConfig,
    pub mode: RenderMode,
    pub is_computing: bool,
    pub recomp: bool,  // 再計算の必要があるか
    pub buf_dirty: bool,  // バッファが更新されたが表示が更新されていないときにtrue
    pub move_ratio: Float,
    pub zoom_ratio: Float,
    pub history: History,

    pub engine: Option<Box<dyn RenderEngine>>,  // フラクタル描画エンジン．変更があればself.engine = Box::new(EscapeTimeFractal::new(...));と新しく作り直す

    pub rgba_buf: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum RenderMode {
    Survey,
    Burst,
}

impl RenderMode {
    pub fn resolusion(&self) -> (usize, usize) {
        match self {
            RenderMode::Survey => (64, 64),
            RenderMode::Burst => (512, 512),
        }
    }
    //pub fn config_resolusion(mode: RenderMode, reso: (usize, usize)) {}
}

#[derive(Debug)]
pub struct History {
    pub stack: Vec<ImageConfig>,
}

impl AppState {
    pub fn new(
        img_cfg: ImageConfig,
        mode: RenderMode,
        is_computing: bool,
        recomp: bool,
        buf_dirty: bool,
        move_ratio: Float,
        zoom_ratio: Float,
        history: History,

        engine: Option<Box<dyn RenderEngine>>,

        rgba_buf: Option<Vec<u8>>,
    ) -> Self {
        Self {img_cfg, mode, is_computing, recomp, buf_dirty, move_ratio, zoom_ratio, history, engine, rgba_buf}
    }

    pub fn with_preset_values() -> Self {
        let mode = RenderMode::Survey;
        let resolution = mode.resolusion();
        let view_size = (3.0, 3.0);
        let w = Float::from_usize(resolution.0).expect("Float should be converted from usize");
        let h = Float::from_usize(resolution.1).expect("Float should be converted from usize");
        let scale = (view_size.0 / w, view_size.1 / h);
        let center = (-0.5, 0.0);
        let img_cfg = ImageConfig::new(
            resolution,
            scale,
            center,
        );
        let is_computing = false;
        let recomp = true;
        let buf_dirty = true;
        let move_ratio = 0.1;
        let zoom_ratio = 0.5;
        let history = History { stack: Vec::new() };

        let dynamics = Mandelbrot::new();

        let max_iter = 300;
        let escape_radius = 2.0;
        let escape_condition = EscapeByNorm {escape_radius};
        let escape_evaluator = EscapeByCount {max_iter, condition: escape_condition};

        let palette = Palette::grayscale(256);
        let normalizer = NormalizeWithMaxIter {max_iter};
        let color_map = ColorMapLinear {palette};
        let coloring = Coloring {normalizer, color_map};

        let etf = EscapeTimeFractal::new(dynamics, escape_evaluator, coloring);

        Self {
            img_cfg,
            mode,
            is_computing,
            recomp,
            buf_dirty,
            move_ratio,
            zoom_ratio,
            history,
            engine: Some(Box::new(etf)),
            rgba_buf: None,
        }
    }

    pub fn move_left(&mut self) {
        let w = Float::from_usize(self.get_resolution().0).expect("Float should be converted from usize");
        self.img_cfg.center.0 -= (self.img_cfg.scale.0 * w) * self.move_ratio;
    }

    pub fn move_right(&mut self) {
        let w = Float::from_usize(self.get_resolution().0).expect("Float should be converted from usize");
        self.img_cfg.center.0 += (self.img_cfg.scale.0 * w) * self.move_ratio;
    }

    pub fn move_up(&mut self) {
        let h = Float::from_usize(self.get_resolution().1).expect("Float should be converted from usize");
        self.img_cfg.center.1 += (self.img_cfg.scale.1 * h) * self.move_ratio;
    }

    pub fn move_down(&mut self) {
        let h = Float::from_usize(self.get_resolution().1).expect("Float should be converted from usize");
        self.img_cfg.center.1 -= (self.img_cfg.scale.1 * h) * self.move_ratio;
    }

    pub fn zoom_in(&mut self) {
        self.img_cfg.scale.0 *= self.zoom_ratio;
        self.img_cfg.scale.1 *= self.zoom_ratio;
    }

    pub fn zoom_out(&mut self) {
        self.img_cfg.scale.0 /= self.zoom_ratio;
        self.img_cfg.scale.1 /= self.zoom_ratio;
    }

    pub fn get_resolution(&self) -> (usize, usize) {
        self.img_cfg.resolution
    }

    pub fn set_resolution(&mut self, reso: (usize, usize)) {
        self.img_cfg.resolution = reso;
    }

    pub fn set_mode(&mut self, mode: RenderMode) {
        let view_size = self.img_cfg.view_size();


        self.mode = mode;
        self.img_cfg.resolution = self.mode.resolusion();

        let w = Float::from_usize(self.mode.resolusion().0).expect("Float should be converted from usize.");
        let h = Float::from_usize(self.mode.resolusion().1).expect("Float should be converted from usize.");
        self.img_cfg.scale.0 = view_size.0 / w;
        self.img_cfg.scale.1 = view_size.1 / h;
    }

    pub fn set_recomp(&mut self, recomp: bool) {
        self.recomp = recomp;
    }

    pub fn set_buf_dirty(&mut self, buf_dirty: bool) {
        self.buf_dirty = buf_dirty;
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