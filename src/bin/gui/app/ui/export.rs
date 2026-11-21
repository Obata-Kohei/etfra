use eframe::egui;

use crate::app::{
    action::AppAction,
    export::ExportOptions,
    settings::BURST_LONG_EDGE,
    state::ExportState,
};

pub struct ExportDialog {
    pub open: bool,
    directory: String,
    save_settings: bool,
    width: String,
    height: String,
}

impl Default for ExportDialog {
    fn default() -> Self {
        Self {
            open: false,
            directory: ".".to_owned(),
            save_settings: false,
            width: BURST_LONG_EDGE.to_string(),
            height: BURST_LONG_EDGE.to_string(),
        }
    }
}

impl ExportDialog {
    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn set_default_resolution(&mut self, resolution: (usize, usize)) {
        self.width = resolution.0.to_string();
        self.height = resolution.1.to_string();
    }

    pub fn options(&self) -> ExportOptions {
        let width = self
            .width
            .trim()
            .parse::<usize>()
            .ok()
            .filter(|value| *value > 0)
            .unwrap_or(BURST_LONG_EDGE);
        let height = self
            .height
            .trim()
            .parse::<usize>()
            .ok()
            .filter(|value| *value > 0)
            .unwrap_or(BURST_LONG_EDGE);

        ExportOptions {
            resolution: Some((width, height)),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        export_busy: bool,
        export_state: &ExportState,
    ) -> Vec<AppAction> {
        let mut actions = Vec::new();
        let mut open = self.open;
        let mut close_dialog = false;

        if open {
            egui::Window::new("Export image")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Save directory");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.directory)
                            .desired_width(300.0),
                    );

                    ui.checkbox(&mut self.save_settings, "Save settings");

                    ui.horizontal(|ui| {
                        ui.label("Width");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.width)
                                .desired_width(90.0),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.label("Height");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.height)
                                .desired_width(90.0),
                        );
                    });
                    ui.separator();
                    ui.label("Common resolution references:");
                    ui.small("HD (720p): 1280 × 720 px");
                    ui.small("Full HD (1080p): 1920 × 1080 px");
                    ui.small("4K UHD: 3840 × 2160 px");

                    match export_state {
                        ExportState::Idle => {}
                        ExportState::Running => {
                            ui.label("Rendering export...");
                        }
                        ExportState::Completed(message) => {
                            ui.label(message);
                        }
                        ExportState::Failed(error) => {
                            ui.colored_label(
                                ui.visuals().error_fg_color,
                                error,
                            );
                        }
                    }

                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(!export_busy, egui::Button::new("Save"))
                            .clicked()
                        {
                            actions.push(AppAction::ExportImage);
                            if self.save_settings {
                                actions.push(AppAction::ExportSettings);
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            close_dialog = true;
                        }
                    });
                });
        }

        self.open = open && !close_dialog;
        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_resolution_can_be_synchronized_with_app_settings() {
        let mut dialog = ExportDialog::default();
        dialog.set_default_resolution((1920, 1080));

        assert_eq!(dialog.options().resolution, Some((1920, 1080)));
    }
}