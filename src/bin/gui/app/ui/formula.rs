use eframe::egui;

#[derive(Default)]
pub struct FormulaEditor {
    buffer: String,
    editing: bool,
}

impl FormulaEditor {
    pub fn edit(
        &mut self,
        ui: &mut egui::Ui,
        expression: &str,
    ) -> Option<String> {
        if !self.editing {
            self.buffer = expression.to_owned();
        }

        ui.label("Custom formula");
        let response = ui.add(
            egui::TextEdit::singleline(&mut self.buffer)
                .desired_width(f32::INFINITY)
                .hint_text("z^2 + c"),
        );
        let enter_pressed =
            ui.input(|input| input.key_pressed(egui::Key::Enter));
        let changed = self.buffer != expression;
        let submitted = changed
            && (response.lost_focus()
                || (response.has_focus() && enter_pressed));

        self.editing = response.has_focus() && !enter_pressed;
        submitted.then(|| self.buffer.clone())
    }

    pub fn stop_editing(&mut self) {
        self.editing = false;
    }
}