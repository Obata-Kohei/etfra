use std::{
    path::PathBuf,
    sync::Arc,
};

use chrono::Local;
use eframe::egui;

use super::{
    action::{AppAction, SettingsPanel},
    compute::{ComputeError, ComputeManager, ComputeResult, FractalBackend},
    controller::{Controller, UpdateRequired},
    export::{ExportManager, ExportOptions, save_export_bundle},
    key_input,
    settings::AppSettings,
    state::{AppState, ComputeStatus, ExportState},
    ui::{export::ExportDialog, formula::FormulaEditor, panels},
};

struct PendingExport {
    settings: AppSettings,
    directory: PathBuf,
    timestamp: String,
    save_settings: bool,
}

pub struct EtfraApp {
    state: AppState,
    controller: Controller,
    compute: ComputeManager,
    active_generation: Option<u64>,
    export_manager: ExportManager,
    export_generation: Option<u64>,
    pending_export: Option<PendingExport>,
    export_dialog: ExportDialog,
    active_settings_panel: SettingsPanel,
    formula_editor: FormulaEditor,
    texture: Option<egui::TextureHandle>,
}

impl EtfraApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut state = AppState::default();
        let mut compute = ComputeManager::new(Arc::new(FractalBackend));
        let active_generation = Some(compute.request(state.settings.clone()));
        state.compute_status = ComputeStatus::Running;

        Self {
            state,
            controller: Controller,
            compute,
            active_generation,
            export_manager: ExportManager::new(),
            export_generation: None,
            pending_export: None,
            export_dialog: ExportDialog::default(),
            active_settings_panel: SettingsPanel::Dynamics,
            formula_editor: FormulaEditor::default(),
            texture: None,
        }
    }

    fn poll_compute(&mut self, ctx: &egui::Context) {
        let Some(result) = self.compute.poll_result() else {
            return;
        };

        if self.active_generation != Some(result.generation) {
            return;
        }

        self.active_generation = None;
        match result.result {
            Ok(image) => {
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [image.resolution.0, image.resolution.1],
                    &image.rgba,
                );
                self.texture = Some(ctx.load_texture(
                    "fractal-preview",
                    color_image,
                    egui::TextureOptions::LINEAR,
                ));
                self.state.image = Some(image);
                self.state.compute_status = ComputeStatus::Idle;
            }
            Err(ComputeError::Cancelled) => {
                self.state.compute_status = ComputeStatus::Idle;
            }
            Err(ComputeError::Failed(error)) => {
                self.state.compute_status = ComputeStatus::Failed(error);
            }
            Err(ComputeError::InvalidImage) => {
                self.state.compute_status = ComputeStatus::Failed(
                    "計算結果の画像データが不正です".to_owned(),
                );
            }
        }
    }

    fn poll_export(&mut self) {
        let Some(result) = self.export_manager.poll_result() else {
            return;
        };

        if self.export_generation != Some(result.generation) {
            return;
        }

        self.export_generation = None;
        let Some(pending) = self.pending_export.take() else {
            self.state.export_state = ExportState::Failed(
                "エクスポートの保存情報が見つかりません".to_owned(),
            );
            return;
        };

        match result {
            ComputeResult {
                result: Ok(image),
                ..
            } => match save_export_bundle(
                &image,
                &pending.settings,
                &pending.directory,
                &pending.timestamp,
                pending.save_settings,
            ) {
                Ok(files) => {
                    let message = match files.settings_path {
                        Some(settings_path) => format!(
                            "Saved {} and {}",
                            files.image_path.display(),
                            settings_path.display(),
                        ),
                        None => format!("Saved {}", files.image_path.display()),
                    };
                    self.state.export_state = ExportState::Completed(message);
                }
                Err(error) => {
                    self.state.export_state =
                        ExportState::Failed(error.to_string());
                }
            },
            ComputeResult {
                result: Err(ComputeError::Cancelled),
                ..
            } => {
                self.state.export_state = ExportState::Idle;
            }
            ComputeResult {
                result: Err(ComputeError::Failed(error)),
                ..
            } => {
                self.state.export_state = ExportState::Failed(error);
            }
            ComputeResult {
                result: Err(ComputeError::InvalidImage),
                ..
            } => {
                self.state.export_state = ExportState::Failed(
                    "計算結果の画像データが不正です".to_owned(),
                );
            }
        }
    }

    fn begin_export(&mut self, save_settings: bool) {
        if self.export_manager.is_busy() {
            return;
        }

        let directory_text = self.export_dialog.directory().trim();
        if directory_text.is_empty() {
            self.state.export_state =
                ExportState::Failed("保存先を入力してください".to_owned());
            return;
        }

        let directory = PathBuf::from(directory_text);
        if !directory.is_dir() {
            self.state.export_state = ExportState::Failed(format!(
                "保存先ディレクトリが存在しません: {}",
                directory.display()
            ));
            return;
        }

        let settings = self.state.settings.clone();
        let options: ExportOptions = self.export_dialog.options();
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let generation = self
            .export_manager
            .request(settings.clone(), options);

        self.pending_export = Some(PendingExport {
            settings,
            directory,
            timestamp,
            save_settings,
        });
        self.export_generation = Some(generation);
        self.state.export_state = ExportState::Running;
    }

    fn process_actions(
        &mut self,
        actions: impl IntoIterator<Item = AppAction>,
    ) {
        let output = self
            .controller
            .process_actions(actions, &mut self.state);

        if let Some(panel) = output.open_settings {
            self.active_settings_panel = panel;
        }
        if output.open_export_dialog {
            self.export_dialog.open = true;
        }

        if output.update_required == UpdateRequired::Recompute {
            self.active_generation =
                Some(self.compute.request(self.state.settings.clone()));
            self.state.compute_status = ComputeStatus::Running;
        } else if self.state.compute_status == ComputeStatus::Cancelling {
            self.compute.cancel();
            self.active_generation = self.compute.active_generation();
        }

        if output.export_image {
            self.begin_export(output.export_settings);
        }
    }

    fn show_preview(&self, ui: &mut egui::Ui) {
        let Some(texture) = &self.texture else {
            ui.centered_and_justified(|ui| match &self.state.compute_status {
                ComputeStatus::Running | ComputeStatus::Cancelling => {
                    ui.label("Calculating");
                }
                ComputeStatus::Failed(error) => {
                    ui.label(format!("Preview failed: {error}"));
                }
                ComputeStatus::Idle => {
                    ui.label("No preview available");
                }
            });
            return;
        };

        let available = ui.available_size();
        let texture_size = texture.size_vec2();
        let scale = (available.x / texture_size.x)
            .min(available.y / texture_size.y)
            .max(0.0);
        ui.centered_and_justified(|ui| {
            ui.add(
                egui::Image::new(texture)
                    .fit_to_exact_size(texture_size * scale),
            );
        });

        if matches!(
            self.state.compute_status,
            ComputeStatus::Running | ComputeStatus::Cancelling
        ) {
            let rect = ui.max_rect();
            ui.painter().text(
                egui::pos2(rect.right() - 18.0, rect.top() + 12.0),
                egui::Align2::RIGHT_TOP,
                "Calculating",
                egui::FontId::proportional(14.0),
                ui.visuals().strong_text_color(),
            );
        }
    }
}

impl eframe::App for EtfraApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        self.poll_compute(ctx);
        self.poll_export();

        let keyboard_enabled = !ctx.wants_keyboard_input();
        let render_mode = self.state.render_mode;
        let mut actions = ctx.input(|input| {
            key_input::actions_from_input(
                input,
                render_mode,
                keyboard_enabled,
            )
        });

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("etfra");
                ui.separator();
                if ui.button("Redraw").clicked() {
                    actions.push(AppAction::Redraw);
                }
                if ui.button("Undo").clicked() {
                    actions.push(AppAction::Undo);
                }
                if ui.button("Export...").clicked() {
                    actions.push(AppAction::OpenExportDialog);
                }
            });
        });

        egui::SidePanel::left("settings")
            .default_width(230.0)
            .show(ctx, |ui| {
                actions.extend(panels::show_settings_panel(
                    ui,
                    self.active_settings_panel,
                    &self.state.settings,
                    self.state.render_mode,
                    &mut self.formula_editor,
                ));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_preview(ui);
        });

        actions.extend(self.export_dialog.show(
            ctx,
            self.export_manager.is_busy(),
            &self.state.export_state,
        ));
        self.process_actions(actions);

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            match &self.state.compute_status {
                ComputeStatus::Idle => {
                    ui.label("Ready");
                }
                ComputeStatus::Running => {
                    ui.label("Calculating...");
                    ctx.request_repaint();
                }
                ComputeStatus::Cancelling => {
                    ui.label("Cancelling...");
                    ctx.request_repaint();
                }
                ComputeStatus::Failed(error) => {
                    ui.label(format!("Rendering failed: {error}"));
                }
            }
            if self.export_manager.is_busy() {
                ui.separator();
                ui.label("Exporting...");
                ctx.request_repaint();
            } else if !self.export_dialog.open {
                if let ExportState::Failed(error) = &self.state.export_state {
                    ui.separator();
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }
            }
        });
    }
}
