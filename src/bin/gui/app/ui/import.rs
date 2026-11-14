use std::path::PathBuf;

use eframe::egui;

pub struct ImportDialog {
    pub open: bool,
    path: String,
    error: Option<String>,
}

impl Default for ImportDialog {
    fn default() -> Self {
        Self {
            open: false,
            path: String::new(),
            error: None,
        }
    }
}

impl ImportDialog {
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn close(&mut self) {
        self.open = false;
        self.error = None;
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<PathBuf> {
        let mut path_to_import = None;
        let mut open = self.open;
        let mut close_dialog = false;

        if open {
            egui::Window::new("Import settings")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("JSON file path");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.path)
                            .desired_width(300.0),
                    );

                    if let Some(error) = &self.error {
                        ui.colored_label(ui.visuals().error_fg_color, error);
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Import").clicked() {
                            path_to_import = Some(PathBuf::from(self.path.trim()));
                        }
                        if ui.button("Cancel").clicked() {
                            close_dialog = true;
                        }
                    });
                });
        }

        self.open = open && !close_dialog;
        path_to_import
    }
}
