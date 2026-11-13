use eframe::egui;

use super::formula::FormulaEditor;
use crate::app::{
    action::{AppAction, SettingsPanel},
    render_config::{AspectRatio, RenderMode},
    settings::{
        AppSettings, ColorMapSettings, DynamicsSettings,
        NormalizationSettings,
    },
};

pub fn show_panel(
    ui: &mut egui::Ui,
    panel: SettingsPanel,
    settings: &AppSettings,
    render_mode: RenderMode,
    formula_editor: &mut FormulaEditor,
) -> Vec<AppAction> {
    let mut actions = Vec::new();

    match panel {
        SettingsPanel::Dynamics => {
            show_dynamics(ui, settings, formula_editor, &mut actions);
        }
        SettingsPanel::Coloring => {
            formula_editor.stop_editing();
            show_coloring(ui, settings, &mut actions);
        }
        SettingsPanel::Resolution => {
            formula_editor.stop_editing();
            show_resolution(ui, settings, &mut actions);
        }
        SettingsPanel::Iterations => {
            formula_editor.stop_editing();
            show_iterations(ui, settings, &mut actions);
        }
        SettingsPanel::RenderMode => {
            formula_editor.stop_editing();
            show_render_mode(ui, render_mode, &mut actions);
        }
    }

    actions
}

fn show_dynamics(
    ui: &mut egui::Ui,
    settings: &AppSettings,
    formula_editor: &mut FormulaEditor,
    actions: &mut Vec<AppAction>,
) {
    let mut dynamics = settings.dynamics.clone();

    egui::ComboBox::from_label("1. Fractal")
        .selected_text(dynamics_name(&dynamics))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::Mandelbrot,
                "Mandelbrot",
            );
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::Julia {
                    c_re: -0.8,
                    c_im: 0.156,
                },
                "Julia",
            );
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::BurningShip,
                "Burning Ship",
            );
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::Tricorn,
                "Tricorn",
            );
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::Phoenix {
                    feedback_re: -0.5,
                    feedback_im: 0.0,
                },
                "Phoenix",
            );
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::Multibrot { power: 3.0 },
                "Multibrot",
            );
            ui.selectable_value(
                &mut dynamics,
                DynamicsSettings::Custom {
                    expression: "z^2 + c".to_owned(),
                },
                "Custom",
            );
        });

    match &mut dynamics {
        DynamicsSettings::Julia { c_re, c_im } => {
            ui.add(
                egui::DragValue::new(c_re)
                    .speed(0.01)
                    .prefix("c real: "),
            );
            ui.add(
                egui::DragValue::new(c_im)
                    .speed(0.01)
                    .prefix("c imag: "),
            );
        }
        DynamicsSettings::Phoenix {
            feedback_re,
            feedback_im,
        } => {
            ui.add(
                egui::DragValue::new(feedback_re)
                    .speed(0.01)
                    .prefix("Feedback real: "),
            );
            ui.add(
                egui::DragValue::new(feedback_im)
                    .speed(0.01)
                    .prefix("Feedback imag: "),
            );
        }
        DynamicsSettings::Multibrot { power } => {
            ui.add(
                egui::DragValue::new(power)
                    .range(2.0..=16.0)
                    .speed(0.1)
                    .prefix("Power: "),
            );
        }
        DynamicsSettings::Custom { expression } => {
            if let Some(value) =
                formula_editor.edit(ui, expression)
            {
                *expression = value;
            }
        }
        DynamicsSettings::Mandelbrot
        | DynamicsSettings::BurningShip
        | DynamicsSettings::Tricorn => {
            formula_editor.stop_editing();
        }
    }

    if dynamics != settings.dynamics {
        actions.push(AppAction::SetDynamics(dynamics));
    }
}

fn dynamics_name(dynamics: &DynamicsSettings) -> &'static str {
    match dynamics {
        DynamicsSettings::Mandelbrot => "Mandelbrot",
        DynamicsSettings::Julia { .. } => "Julia",
        DynamicsSettings::BurningShip => "Burning Ship",
        DynamicsSettings::Tricorn => "Tricorn",
        DynamicsSettings::Phoenix { .. } => "Phoenix",
        DynamicsSettings::Multibrot { .. } => "Multibrot",
        DynamicsSettings::Custom { .. } => "Custom",
    }
}

fn show_coloring(
    ui: &mut egui::Ui,
    settings: &AppSettings,
    actions: &mut Vec<AppAction>,
) {
    let mut color_map = settings.color_map;
    egui::ComboBox::from_label("2. Color map")
        .selected_text(match color_map {
            ColorMapSettings::Grayscale => "Grayscale",
            ColorMapSettings::HueGradient => "Hue gradient",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut color_map,
                ColorMapSettings::Grayscale,
                "Grayscale",
            );
            ui.selectable_value(
                &mut color_map,
                ColorMapSettings::HueGradient,
                "Hue gradient",
            );
        });
    if color_map != settings.color_map {
        actions.push(AppAction::SetColorMap(color_map));
    }

    let mut normalization = settings.normalization;
    egui::ComboBox::from_label("Normalization")
        .selected_text(match normalization {
            NormalizationSettings::MaxIterations => "Max iterations",
            NormalizationSettings::Histogram => "Histogram",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut normalization,
                NormalizationSettings::MaxIterations,
                "Max iterations",
            );
            ui.selectable_value(
                &mut normalization,
                NormalizationSettings::Histogram,
                "Histogram",
            );
        });
    if normalization != settings.normalization {
        actions.push(AppAction::SetNormalization(normalization));
    }
}

fn show_resolution(
    ui: &mut egui::Ui,
    settings: &AppSettings,
    actions: &mut Vec<AppAction>,
) {
    let (width, height) = settings.image.resolution;
    let current_ratio = AspectRatio::PRESETS.iter().copied().find(|ratio| {
        width as u128 * ratio.height() as u128
            == height as u128 * ratio.width() as u128
    });
    let selected_text = current_ratio.map_or_else(
        || format!("{width}:{height} (custom)"),
        |ratio| format!("{}:{}", ratio.width(), ratio.height()),
    );
    let mut selected_ratio = current_ratio;

    egui::ComboBox::from_label("3. Aspect ratio")
        .selected_text(selected_text)
        .show_ui(ui, |ui| {
            for ratio in AspectRatio::PRESETS {
                ui.selectable_value(
                    &mut selected_ratio,
                    Some(*ratio),
                    format!("{}:{}", ratio.width(), ratio.height()),
                );
            }
        });

    if let Some(ratio) = selected_ratio {
        if Some(ratio) != current_ratio {
            actions.push(AppAction::SetAspectRatio(ratio));
        }
    }
    ui.label(format!("{} × {} px", width, height));
}

fn show_iterations(
    ui: &mut egui::Ui,
    settings: &AppSettings,
    actions: &mut Vec<AppAction>,
) {
    let mut max_iterations = settings.max_iter;
    ui.label("4. Maximum iterations");
    ui.add(
        egui::DragValue::new(&mut max_iterations)
            .range(1..=100_000)
            .speed(1),
    );
    if max_iterations != settings.max_iter {
        actions.push(AppAction::SetMaxIterations(max_iterations));
    }

    let mut escape_radius = settings.escape_radius;
    ui.label("4. Escape radius");
    ui.add(
        egui::DragValue::new(&mut escape_radius)
            .range(0.1..=1_000_000.0)
            .speed(0.1),
    );
    if escape_radius != settings.escape_radius {
        actions.push(AppAction::SetEscapeRadius(escape_radius));
    }
}

fn show_render_mode(
    ui: &mut egui::Ui,
    render_mode: RenderMode,
    actions: &mut Vec<AppAction>,
) {
    ui.label("5. Preview quality");
    for (mode, label) in [
        (RenderMode::Survey, "Survey"),
        (RenderMode::Burst, "Burst"),
    ] {
        if ui
            .selectable_label(render_mode == mode, label)
            .clicked()
        {
            actions.push(AppAction::SetRenderMode(mode));
        }
    }
}