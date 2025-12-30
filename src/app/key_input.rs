use eframe::egui;
use crate::app::state::AppState;

pub fn handle_key_input(
    ctx: &egui::Context,
    state: AppState,
) {
    if ctx.wants_keyboard_input() {
        return;
    }

    ctx.input(|i| {
        // wasd: 平行移動
        if i.key_pressed(egui::Key::A) {
            state.left_shift()
        }

        // e: 拡大

        // q: 縮小

        // r: 再描画

        // z:ひとつ前の状態へ戻る

        // x: Survey modeへ移行(低いresolusionで軽量に描画するmode．初期状態はSurvey mode)

        // c: Burst modeへ移行(高いresolutionで精細な描画をするmode．)

    });
}