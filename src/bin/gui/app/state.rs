
use super::{
    render_config::RenderMode,
    settings::AppSettings,
};

/// 計算処理の状態
#[derive(Debug, Clone, PartialEq)]
pub enum ComputeStatus {
    Idle,
    Running,
    Cancelling,
    Failed(String),
}

/// 描画済み画像データ
#[derive(Debug, Clone)]
pub struct RenderedImage {
    pub resolution: (usize, usize),
    pub rgba: Vec<u8>,
}

impl RenderedImage {
    /// RGBAデータのサイズが解像度と一致するか確認する
    pub fn is_valid(&self) -> bool {
        let Some(expected) = self.resolution.0
            .checked_mul(self.resolution.1)
            .and_then(|n| n.checked_mul(4))
        else {
            return false;
        };

        self.rgba.len() == expected
    }
}

/// エクスポート処理の状態
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ExportState {
    #[default]
    Idle,
    Running,
    Completed(String),
    Failed(String),
}

/// 設定履歴
#[derive(Debug, Default)]
pub struct History {
    entries: Vec<SettingsSnapshot>,
}

#[derive(Debug)]
struct SettingsSnapshot {
    settings: AppSettings,
    render_mode: RenderMode,
}

impl History {
    pub fn push(&mut self, settings: &AppSettings, render_mode: RenderMode) {
        self.entries.push(SettingsSnapshot {
            settings: settings.clone(),
            render_mode,
        });
    }

    pub fn undo(&mut self) -> Option<(AppSettings, RenderMode)> {
        self.entries
            .pop()
            .map(|snapshot| (snapshot.settings, snapshot.render_mode))
    }
}

/// アプリケーション全体の状態
pub struct AppState {
    pub settings: AppSettings,
    pub render_mode: RenderMode,
    pub compute_status: ComputeStatus,
    pub image: Option<RenderedImage>,
    pub history: History,
    pub export_state: ExportState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: AppSettings::default(),
            render_mode: RenderMode::Survey,
            compute_status: ComputeStatus::Idle,
            image: None,
            history: History::default(),
            export_state: ExportState::Idle,
        }
    }
}
