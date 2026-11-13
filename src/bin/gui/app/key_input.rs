use eframe::egui;

use super::{
    action::{AppAction, Direction, SettingsPanel},
    render_config::RenderMode,
    settings::{
        ColorMapSettings, DynamicsSettings,
    },
};

pub fn actions_from_input(
    input: &egui::InputState,
    render_mode: RenderMode,
    keyboard_enabled: bool,
) -> Vec<AppAction> {
    actions_from_input_with_panel(
        input,
        render_mode,
        keyboard_enabled,
        SettingsPanel::Dynamics,
    )
}

pub fn actions_from_input_with_panel(
    input: &egui::InputState,
    render_mode: RenderMode,
    keyboard_enabled: bool,
    active_panel: SettingsPanel,
) -> Vec<AppAction> {
    if !keyboard_enabled {
        return Vec::new();
    }

    let mut actions = Vec::new();

    if input.key_pressed(egui::Key::W)
        || input.key_pressed(egui::Key::ArrowUp)
    {
        actions.push(AppAction::Move(Direction::Up));
    }
    if input.key_pressed(egui::Key::A)
        || input.key_pressed(egui::Key::ArrowLeft)
    {
        actions.push(AppAction::Move(Direction::Left));
    }
    if input.key_pressed(egui::Key::S)
        || input.key_pressed(egui::Key::ArrowDown)
    {
        actions.push(AppAction::Move(Direction::Down));
    }
    if input.key_pressed(egui::Key::D)
        || input.key_pressed(egui::Key::ArrowRight)
    {
        actions.push(AppAction::Move(Direction::Right));
    }
    if input.key_pressed(egui::Key::E) {
        actions.push(AppAction::ZoomIn);
    }
    if input.key_pressed(egui::Key::Q) {
        actions.push(AppAction::ZoomOut);
    }
    if input.key_pressed(egui::Key::R) {
        actions.push(AppAction::Redraw);
    }
    if input.key_pressed(egui::Key::Z) {
        actions.push(AppAction::StopAndUndo);
    }
    if input.key_pressed(egui::Key::X) && render_mode != RenderMode::Survey {
        actions.push(AppAction::SetRenderMode(RenderMode::Survey));
    }
    if input.key_pressed(egui::Key::C) && render_mode != RenderMode::Burst {
        actions.push(AppAction::SetRenderMode(RenderMode::Burst));
    }

    let digit_keys = [
        (egui::Key::Num1, 1),
        (egui::Key::Num2, 2),
        (egui::Key::Num3, 3),
        (egui::Key::Num4, 4),
        (egui::Key::Num5, 5),
        (egui::Key::Num6, 6),
        (egui::Key::Num7, 7),
        (egui::Key::Num8, 8),
        (egui::Key::Num9, 9),
    ];
    for (key, value) in digit_keys {
        if input.key_pressed(key) {
            let panel = match value {
                1 => Some(SettingsPanel::Dynamics),
                2 => Some(SettingsPanel::Coloring),
                3 => Some(SettingsPanel::Resolution),
                4 => Some(SettingsPanel::Iterations),
                5 => Some(SettingsPanel::RenderMode),
                _ => None,
            };
            if let Some(panel) = panel {
                if active_panel != panel {
                    actions.push(AppAction::OpenSettings(panel));
                }
            }

            match active_panel {
                SettingsPanel::Dynamics => match value {
                    1 => actions.push(AppAction::SetDynamics(DynamicsSettings::Mandelbrot)),
                    2 => actions.push(AppAction::SetDynamics(DynamicsSettings::Julia { c_re: -0.8, c_im: 0.156 })),
                    3 => actions.push(AppAction::SetDynamics(DynamicsSettings::BurningShip)),
                    4 => actions.push(AppAction::SetDynamics(DynamicsSettings::Tricorn)),
                    5 => actions.push(AppAction::SetDynamics(DynamicsSettings::Phoenix { feedback_re: -0.5, feedback_im: 0.0 })),
                    6 => actions.push(AppAction::SetDynamics(DynamicsSettings::Multibrot { power: 3.0 })),
                    7 => actions.push(AppAction::SetDynamics(DynamicsSettings::Custom { expression: "z^2 + c".to_owned() })),
                    _ => {}
                },
                SettingsPanel::Coloring => match value {
                    1 => actions.push(AppAction::SetColorMap(ColorMapSettings::Grayscale)),
                    2 => actions.push(AppAction::SetColorMap(ColorMapSettings::HueGradient)),
                    _ => {}
                },
                SettingsPanel::RenderMode => match value {
                    1 => actions.push(AppAction::SetRenderMode(RenderMode::Survey)),
                    2 => actions.push(AppAction::SetRenderMode(RenderMode::Burst)),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    actions
}
