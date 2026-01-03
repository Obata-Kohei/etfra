use eframe::egui;
use crate::app::state::AppState;
use crate::app::state::RenderMode;

pub fn handle_key_input(
    ctx: &egui::Context,
    state: &mut AppState,
) {
    if ctx.wants_keyboard_input() {
        return;
    }

    ctx.input(|i| {
        // wasd: 平行移動
        if i.key_pressed(egui::Key::W) {
            state.push_history();
            state.move_up();
            state.set_recomp(true);
        }
        if i.key_pressed(egui::Key::A) {
            state.push_history();
            state.move_left();
            state.set_recomp(true);
        }
        if i.key_pressed(egui::Key::S) {
            state.push_history();
            state.move_down();
            state.set_recomp(true);
        }
        if i.key_pressed(egui::Key::D) {
            state.push_history();
            state.move_right();
            state.set_recomp(true);
        }

        // e: 拡大
        if i.key_pressed(egui::Key::E) {
            state.push_history();
            state.zoom_in();
            state.set_recomp(true);
        }

        // q: 縮小
        if i.key_pressed(egui::Key::Q) {
            state.push_history();
            state.zoom_out();
            state.set_recomp(true);
        }

        // r: 再描画
        if i.key_pressed(egui::Key::R) {
            state.set_recomp(true);
        }

        // z:ひとつ前の状態へ戻る
        if i.key_pressed(egui::Key::Z) {
            state.undo();
            state.set_recomp(true);
        }

        // x: Survey modeへ移行(低いresolusionで軽量に描画するmode．初期状態はSurvey mode)
        if i.key_pressed(egui::Key::X) {
            state.set_mode(RenderMode::Survey);
            state.set_recomp(true);
        }

        // c: Burst modeへ移行(高いresolutionで精細な描画をするmode．)
        if i.key_pressed(egui::Key::C) {
            state.set_mode(RenderMode::Burst);
            state.set_recomp(true);
        }
    });
}