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

    if input.key_pressed(egui::Key::ArrowUp) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Up)
        } else {
            AppAction::Move(Direction::Up)
        });
    } else if input.key_pressed(egui::Key::W) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Up)
        } else {
            AppAction::Move(Direction::Up)
        });
    }

    if input.key_pressed(egui::Key::ArrowLeft) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Left)
        } else {
            AppAction::Move(Direction::Left)
        });
    } else if input.key_pressed(egui::Key::A) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Left)
        } else {
            AppAction::Move(Direction::Left)
        });
    }

    if input.key_pressed(egui::Key::ArrowDown) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Down)
        } else {
            AppAction::Move(Direction::Down)
        });
    } else if input.key_pressed(egui::Key::S) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Down)
        } else {
            AppAction::Move(Direction::Down)
        });
    }

    if input.key_pressed(egui::Key::ArrowRight) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Right)
        } else {
            AppAction::Move(Direction::Right)
        });
    } else if input.key_pressed(egui::Key::D) {
        actions.push(if input.modifiers.shift {
            AppAction::MoveHalf(Direction::Right)
        } else {
            AppAction::Move(Direction::Right)
        });
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

    if input.key_pressed(egui::Key::L) {
        actions.push(AppAction::ToggleCenterLines);
    }
    if input.key_pressed(egui::Key::Z) {
        actions.push(AppAction::StopAndUndo);
    }

    if input.key_pressed(egui::Key::X)
    && render_mode != RenderMode::SuperLight
    {
        actions.push(AppAction::SetRenderMode(RenderMode::SuperLight));
    }
    if input.key_pressed(egui::Key::C)
        && render_mode != RenderMode::Light
    {
        actions.push(AppAction::SetRenderMode(RenderMode::Light));
    }
    if input.key_pressed(egui::Key::V)
        && render_mode != RenderMode::Survey
    {
        actions.push(AppAction::SetRenderMode(RenderMode::Survey));
    }
    if input.key_pressed(egui::Key::B)
        && render_mode != RenderMode::Burst
    {
        actions.push(AppAction::SetRenderMode(RenderMode::Burst));
    }

    actions
}
