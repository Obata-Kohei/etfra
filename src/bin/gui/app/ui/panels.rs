use eframe::egui;

use super::{
    formula::FormulaEditor,
    settings,
};
use crate::app::{
    action::{AppAction, SettingsPanel},
    render_config::RenderMode,
    settings::AppSettings,
};

pub fn show_settings_panel(
    ui: &mut egui::Ui,
    active_panel: SettingsPanel,
    app_settings: &AppSettings,
    render_mode: RenderMode,
    formula_editor: &mut FormulaEditor,
) -> Vec<AppAction> {
    let mut actions = Vec::new();

    ui.heading("Settings");
    for (panel, label) in [
        (SettingsPanel::Dynamics, "Dynamics"),
        (SettingsPanel::Coloring, "Coloring"),
        (SettingsPanel::Resolution, "Resolution"),
        (SettingsPanel::Iterations, "Iterations"),
        (SettingsPanel::RenderMode, "Render mode"),
    ] {
        if ui
            .selectable_label(active_panel == panel, label)
            .clicked()
        {
            actions.push(AppAction::OpenSettings(panel));
        }
    }

    ui.separator();
    actions.extend(settings::show_panel(
        ui,
        active_panel,
        app_settings,
        render_mode,
        formula_editor,
    ));

    actions
}