use eframe::egui;
use crate::app::state::AppState;
use crate::app::state::RenderMode;

pub fn handle_key_input(
    ctx: &egui::Context,
    state: &mut AppState,
) {
    ctx.input(|i| {
        // wasd: 平行移動
        if i.key_pressed(egui::Key::W) || i.key_pressed(egui::Key::ArrowUp) {
            state.push_history();
            state.move_up();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }
        if i.key_pressed(egui::Key::A) || i.key_pressed(egui::Key::ArrowLeft) {
            state.push_history();
            state.move_left();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }
        if i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::ArrowDown) {
            state.push_history();
            state.move_down();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }
        if i.key_pressed(egui::Key::D) || i.key_pressed(egui::Key::ArrowRight) {
            state.push_history();
            state.move_right();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }

        // e: 拡大
        if i.key_pressed(egui::Key::E) {
            state.push_history();
            state.zoom_in();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }

        // q: 縮小
        if i.key_pressed(egui::Key::Q) {
            state.push_history();
            state.zoom_out();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }

        // r: 再描画
        if i.key_pressed(egui::Key::R) {
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }

        // z:ひとつ前の状態へ戻る
        if i.key_pressed(egui::Key::Z) {
            state.undo();
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }

        // x: Survey modeへ移行(低いresolusionで軽量に描画するmode．初期状態はSurvey mode)
        if i.key_pressed(egui::Key::X) {
            state.set_mode(RenderMode::Survey);
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }

        // c: Burst modeへ移行(高いresolutionで精細な描画をするmode．)
        if i.key_pressed(egui::Key::C) {
            state.set_mode(RenderMode::Burst);
            state.set_recomp(true);
            state.set_buf_dirty(true);
        }
    });
}