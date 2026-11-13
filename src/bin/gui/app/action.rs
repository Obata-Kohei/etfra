use etfra::util::types::Float;

use super::{
    render_config::{AspectRatio, RenderMode},
    settings::{
        AppSettings, ColorMapSettings, DynamicsSettings, NormalizationSettings,
    },
};

/// キー入力・UI操作をアプリケーションの操作へ変換したもの。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPanel {
    Dynamics,
    Coloring,
    Resolution,
    Iterations,
    RenderMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    // 表示範囲の操作
    Move(Direction),
    ZoomIn,
    ZoomOut,

    // 描画・履歴
    Redraw,
    Undo,
    StopAndUndo,

    // 設定UIを開く。設定そのものは変更しない。
    OpenSettings(SettingsPanel),

    // UIで選択・編集された設定を反映する。
    SetDynamics(DynamicsSettings),
    SetAspectRatio(AspectRatio),
    SetColorMap(ColorMapSettings),
    SetNormalization(NormalizationSettings),
    SetRenderMode(RenderMode),
    SetMaxIterations(usize),
    SetEscapeRadius(Float),

    // 現在の設定をまとめて適用する。
    ApplySettings(AppSettings),

    // エクスポート
    OpenExportDialog,
    ExportImage,
    ExportSettings,
}