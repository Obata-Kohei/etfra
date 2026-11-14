use eframe::egui;

use super::{
    action::{AppAction, Direction},
    render_config::RenderMode,
};

pub fn actions_from_input(
    input: &egui::InputState,
    render_mode: RenderMode,
    keyboard_enabled: bool,
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

    actions
}
