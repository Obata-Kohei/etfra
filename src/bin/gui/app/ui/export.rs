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
    custom_resolution: bool,
    width: usize,
    height: usize,
}

impl Default for ExportDialog {
    fn default() -> Self {
        Self {
            open: false,
            directory: ".".to_owned(),
            save_settings: false,
            custom_resolution: false,
            width: BURST_LONG_EDGE,
            height: BURST_LONG_EDGE,
        }
    }
}

impl ExportDialog {
    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub fn options(&self) -> ExportOptions {
        ExportOptions {
            resolution: self
                .custom_resolution
                .then_some((self.width, self.height)),
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
                    ui.checkbox(
                        &mut self.custom_resolution,
                        "Custom resolution",
                    );
                    if self.custom_resolution {
                        ui.horizontal(|ui| {
                            ui.label("Width");
                            ui.add(
                                egui::DragValue::new(&mut self.width)
                                    .range(1..=u32::MAX as usize),
                            );
                            ui.label("Height");
                            ui.add(
                                egui::DragValue::new(&mut self.height)
                                    .range(1..=u32::MAX as usize),
                            );
                        });
                    } else {
                        ui.label(
                            "Default: Burst resolution, preserving the current aspect ratio.",
                        );
                    }

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