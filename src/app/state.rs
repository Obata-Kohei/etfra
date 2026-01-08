use num::Complex;
use crate::{app::ui_render::RenderEngine, prelude::*};

pub struct AppState {
    pub img_cfg: ImageConfig,
    pub mode: RenderMode,
    pub recomp: bool,  // 再計算の必要があるか
    pub buf_dirty: bool,  // バッファが更新されたが表示が更新されていないときにtrue
    pub move_ratio: Float,
    pub zoom_ratio: Float,
    pub history: History,

    pub engine: Box<dyn RenderEngine>,  // フラクタル描画エンジン．変更があればself.engine = Box::new(EscapeTimeFractal::new(...));と新しく作り直す

    pub rgba_buf: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct ImageConfig {
    pub resolution: (usize, usize),
    pub center: Complex<Float>,
    pub scale: Float
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
            RenderMode::Burst => (1024, 1024),
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
        recomp: bool,
        buf_dirty: bool,
        move_ratio: Float,
        zoom_ratio: Float,
        history: History,

        engine: Box<dyn RenderEngine>,

        rgba_buf: Option<Vec<u8>>,
    ) -> Self {
        Self {img_cfg, mode, recomp, buf_dirty, move_ratio, zoom_ratio, history, engine, rgba_buf}
    }

    pub fn with_preset_values() -> Self {
        let resolution = RenderMode::Survey.resolusion();
        let center = Complex::new(-0.5, 0.0);
        let view_size = (3., 3.); //(resolution.0 as Float * scale, resolution.1 as Float * scale);
        let scale = view_size.0 / resolution.0 as Float;
        let img_cfg = ImageConfig {resolution, center, scale};
        let mode = RenderMode::Survey;
        let max_iter = 300;
        let move_ratio = 0.1;
        let zoom_ratio = 0.5;
        let dynamics = Mandelbrot::new();
        let escape_radius = 2.0;
        let escape = EscapeByCount::new(max_iter, escape_radius);
        let palette = Palette::grayscale(256);
        let coloring = PaletteColoring::new(palette, max_iter);

        Self {
            img_cfg,
            mode,
            recomp: true,
            buf_dirty: true,
            move_ratio,
            zoom_ratio,
            history: History { stack: Vec::new() },
            engine: Box::new(
                EscapeTimeFractal::new(
                    dynamics,
                    escape,
                    coloring,
                    resolution,
                    center,
                    view_size
                )
            ),
            rgba_buf: None,
        }
    }

    pub fn compute_if_needed(&mut self) {
        if self.recomp {
            let buf = self.engine.compute(&self.img_cfg);
            self.rgba_buf = Some(buf);
            self.recomp = false;
            self.buf_dirty = true;
        }
    }

    pub fn compute_if_needed_par(&mut self) {
        if self.recomp {
            let buf = self.engine.compute_par(&self.img_cfg);
            self.rgba_buf = Some(buf);
            self.recomp = false;
            self.buf_dirty = true;
        }
    }

    pub fn move_left(&mut self) {
        self.img_cfg.center.re -= (self.img_cfg.scale * self.img_cfg.resolution.0 as Float) * self.move_ratio;
    }

    pub fn move_right(&mut self) {
        self.img_cfg.center.re += (self.img_cfg.scale * self.img_cfg.resolution.0 as Float) * self.move_ratio;
    }

    pub fn move_up(&mut self) {
        self.img_cfg.center.im += (self.img_cfg.scale * self.img_cfg.resolution.1 as Float) * self.move_ratio;
    }

    pub fn move_down(&mut self) {
        self.img_cfg.center.im -= (self.img_cfg.scale * self.img_cfg.resolution.1 as Float) * self.move_ratio;
    }

    pub fn zoom_in(&mut self) {
        self.img_cfg.scale *= self.zoom_ratio;
    }

    pub fn zoom_out(&mut self) {
        self.img_cfg.scale /= self.zoom_ratio;
    }

    pub fn get_resolution(&self) -> (usize, usize) {
        self.img_cfg.resolution
    }

    pub fn set_resolution(&mut self, reso: (usize, usize)) {
        self.img_cfg.resolution = reso;
    }

    pub fn set_mode(&mut self, mode: RenderMode) {
        self.mode = mode;
        self.img_cfg.resolution = self.mode.resolusion();
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