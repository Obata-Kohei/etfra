
use super::{
    action::{AppAction, Direction, SettingsPanel},
    settings::AppSettings,
    state::{AppState, ComputeStatus},
};

/// 設定変更によって必要となる更新処理。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpdateRequired {
    #[default]
    None,
    Recompute,
}

impl UpdateRequired {
    /// 複数の更新要求を統合する。
    /// Recompute > None
    pub fn merge(self, other: Self) -> Self {
        use UpdateRequired::*;

        match (self, other) {
            (Recompute, _) | (_, Recompute) => Recompute,
            _ => None,
        }
    }
}

/// ControllerからAppへ返す処理結果。
#[derive(Debug, Default)]
pub struct ControllerOutput {
    pub update_required: UpdateRequired,
    pub open_settings: Option<SettingsPanel>,
    pub open_export_dialog: bool,
    pub export_image: bool,
    pub export_settings: bool,
}

/// Actionを受け取り、AppStateを更新する。
#[derive(Debug, Default)]
pub struct Controller;

impl Controller {
    /// 複数のActionを順番に処理する。
    pub fn process_actions(
        &mut self,
        actions: impl IntoIterator<Item = AppAction>,
        state: &mut AppState,
    ) -> ControllerOutput {
        let mut output = ControllerOutput::default();

        for action in actions {
            let result = self.process_action(action, state);

            output.update_required = output
                .update_required
                .merge(result.update_required);

            if result.open_settings.is_some() {
                output.open_settings = result.open_settings;
            }

            output.open_export_dialog |= result.open_export_dialog;
            output.export_image |= result.export_image;
            output.export_settings |= result.export_settings;
        }

        output
    }

    fn process_action(
        &mut self,
        action: AppAction,
        state: &mut AppState,
    ) -> ControllerOutput {
        let mut output = ControllerOutput::default();

        match action {
            AppAction::Move(direction) => {
                let mut settings = state.settings.clone();
                move_view(&mut settings, direction);
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::ZoomIn => {
                let mut settings = state.settings.clone();
                zoom_view(&mut settings, 0.8);
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::ZoomOut => {
                let mut settings = state.settings.clone();
                zoom_view(&mut settings, 1.25);
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::Redraw => {
                output.update_required = UpdateRequired::Recompute;
            }

            AppAction::Undo => {
                output.update_required = self.undo(state);
            }

            AppAction::StopAndUndo => {
                if state.compute_status == ComputeStatus::Running {
                    state.compute_status = ComputeStatus::Cancelling;
                }

                output.update_required = self.undo(state);
            }

            AppAction::OpenSettings(panel) => {
                output.open_settings = Some(panel);
            }

            AppAction::SetDynamics(value) => {
                let mut settings = state.settings.clone();
                settings.dynamics = value;
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::SetColorMap(value) => {
                let mut settings = state.settings.clone();
                settings.color_map = value;
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::SetNormalization(value) => {
                let mut settings = state.settings.clone();
                settings.normalization = value;
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::SetRenderMode(value) => {
                let mut settings = state.settings.clone();
                settings.render_mode = value;
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::SetMaxIterations(value) => {
                let mut settings = state.settings.clone();
                settings.max_iter = value;
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::SetEscapeRadius(value) => {
                let mut settings = state.settings.clone();
                settings.escape_radius = value;
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::ApplySettings(settings) => {
                output.update_required =
                    self.apply_settings(settings, state);
            }

            AppAction::OpenExportDialog => {
                output.open_export_dialog = true;
            }

            AppAction::ExportImage => {
                output.export_image = true;
            }

            AppAction::ExportSettings => {
                output.export_settings = true;
            }
        }

        output
    }

    /// 設定を反映し、必要な更新種別を返す。
    fn apply_settings(
        &self,
        new_settings: AppSettings,
        state: &mut AppState,
    ) -> UpdateRequired {
        if state.settings == new_settings {
            return UpdateRequired::None;
        }

        let update = classify_update(&state.settings, &new_settings);

        // 変更前の設定を履歴に保存する。
        state.history.push(&state.settings);
        state.settings = new_settings;

        // 再計算が必要な場合、古い画像を現在の画像として扱わない。
        if update == UpdateRequired::Recompute {
            state.image = None;

            // 計算中にRecompute要求がなされた場合，実行中の計算をキャンセル
            if state.compute_status == ComputeStatus::Running {
                state.compute_status = ComputeStatus::Cancelling;
            }
        }

        update
    }

    fn undo(&self, state: &mut AppState) -> UpdateRequired {
        let Some(previous) = state.history.undo() else {
            return UpdateRequired::None;
        };

        let update = classify_update(&state.settings, &previous);

        state.settings = previous;

        if update == UpdateRequired::Recompute {
            state.image = None;
        }

        update
    }
}

/// 変更前後の設定から必要な更新を判定する。
fn classify_update(
    old: &AppSettings,
    new: &AppSettings,
) -> UpdateRequired {
    use UpdateRequired::*;

    // 計算結果そのものに影響する設定。
    if old.dynamics != new.dynamics
        || old.image != new.image
        || old.max_iter != new.max_iter
        || old.escape_radius != new.escape_radius
        || old.render_mode != new.render_mode
        || old.color_map != new.color_map
        || old.normalization != new.normalization
    {
        return Recompute;
    }

    None
}

/// 表示範囲を移動する。
fn move_view(settings: &mut AppSettings, direction: Direction) {
    let (width, height) = settings.image.resolution;
    let (sx, sy) = settings.image.scale;

    // 画面サイズの10%を移動量とする。
    let dx = sx * width as f64 * 0.1;
    let dy = sy * height as f64 * 0.1;

    match direction {
        Direction::Up => settings.image.center.1 += dy,
        Direction::Down => settings.image.center.1 -= dy,
        Direction::Left => settings.image.center.0 -= dx,
        Direction::Right => settings.image.center.0 += dx,
    }
}

/// 表示範囲を拡大・縮小する。
fn zoom_view(settings: &mut AppSettings, factor: f64) {
    settings.image.scale.0 *= factor;
    settings.image.scale.1 *= factor;
}
