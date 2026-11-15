use std::{
    fs,
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
    ui::{
        export::ExportDialog, formula::FormulaEditor,
        import::ImportDialog, panels,
    },
};

struct PendingExport {
    settings: AppSettings,
    directory: PathBuf,
    timestamp: String,
    save_settings: bool,
}

#[derive(Debug, Clone, Copy)]
struct PreviewCursor {
    pixel: (usize, usize),
    xy: (f64, f64),
}

#[derive(Debug, Default)]
struct PreviewInteraction {
    cursor: Option<PreviewCursor>,
    clicked_xy: Option<(f64, f64)>,
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
    import_dialog: ImportDialog,
    import_status: Option<String>,
    active_settings_panel: SettingsPanel,
    formula_editor: FormulaEditor,
    texture: Option<egui::TextureHandle>,
    coordinate_x: String,
    coordinate_y: String,
    coordinate_error: Option<String>,
    synchronized_center: (f64, f64),
}

impl EtfraApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut state = AppState::default();
        let mut compute = ComputeManager::new(Arc::new(FractalBackend));
        let active_generation = Some(compute.request(state.settings.clone()));
        state.compute_status = ComputeStatus::Running;
        let synchronized_center = state.settings.image.center;

        Self {
            state,
            controller: Controller,
            compute,
            active_generation,
            export_manager: ExportManager::new(),
            export_generation: None,
            pending_export: None,
            export_dialog: ExportDialog::default(),
            import_dialog: ImportDialog::default(),
            import_status: None,
            active_settings_panel: SettingsPanel::Dynamics,
            formula_editor: FormulaEditor::default(),
            texture: None,
            coordinate_x: format!("{:.12}", synchronized_center.0),
            coordinate_y: format!("{:.12}", synchronized_center.1),
            coordinate_error: None,
            synchronized_center,
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
            if !self.export_dialog.open {
                self.export_dialog
                    .set_default_resolution(self.state.settings.image.resolution);
            }
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

    fn show_preview(&self, ui: &mut egui::Ui) -> PreviewInteraction {
        let mut interaction = PreviewInteraction::default();
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
            return interaction;
        };

        let available = ui.available_size();
        let texture_size = texture.size_vec2();
        let scale = (available.x / texture_size.x)
            .min(available.y / texture_size.y)
            .max(0.0);
        ui.centered_and_justified(|ui| {
            let response = ui.add(
                egui::Image::new(texture)
                    .fit_to_exact_size(texture_size * scale)
                    .sense(egui::Sense::click()),
            );
            if self.state.show_center_lines {
                let center = response.rect.center();
                let stroke = egui::Stroke::new(
                    1.0,
                    egui::Color32::from_white_alpha(180),
                );
                let painter = ui.painter_at(response.rect);
                painter.line_segment(
                    [
                        egui::pos2(response.rect.left(), center.y),
                        egui::pos2(response.rect.right(), center.y),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        egui::pos2(center.x, response.rect.top()),
                        egui::pos2(center.x, response.rect.bottom()),
                    ],
                    stroke,
                );
            }

            if self.state.image.is_some() {
                if let Some(position) = response.hover_pos() {
                    interaction.cursor = preview_cursor(
                        response.rect,
                        position,
                        &self.state.settings.image,
                    );
                }
                if response.clicked() {
                    interaction.clicked_xy = response
                        .interact_pointer_pos()
                        .and_then(|position| {
                            preview_cursor(
                                response.rect,
                                position,
                                &self.state.settings.image,
                            )
                        })
                        .map(|cursor| cursor.xy);
                }
            }
        });
        interaction
    }
}

fn preview_cursor(
    image_rect: egui::Rect,
    position: egui::Pos2,
    image_config: &etfra::util::image_config::ImageConfig,
) -> Option<PreviewCursor> {
    let (width, height) = image_config.resolution;
    if width == 0
        || height == 0
        || image_rect.width() <= 0.0
        || image_rect.height() <= 0.0
    {
        return None;
    }

    let x = (((position.x - image_rect.left()) / image_rect.width())
        * width as f32)
        .floor()
        .max(0.0) as usize;
    let y = (((position.y - image_rect.top()) / image_rect.height())
        * height as f32)
        .floor()
        .max(0.0) as usize;
    let pixel = (x.min(width - 1), y.min(height - 1));
    let view_size = image_config.view_size();
    let view_bounds = image_config.view_bounds(view_size);
    let xy = image_config.pixel_to_xyplane(pixel, view_bounds);

    Some(PreviewCursor { pixel, xy })
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
        if self.state.settings.image.center != self.synchronized_center {
            self.synchronized_center = self.state.settings.image.center;
            self.coordinate_x = format!("{:.12}", self.synchronized_center.0);
            self.coordinate_y = format!("{:.12}", self.synchronized_center.1);
        }

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
                if ui.button("Export").clicked() {
                    actions.push(AppAction::OpenExportDialog);
                }
                if ui.button("Import").clicked() {
                    self.import_dialog.open = true;
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

        egui::SidePanel::right("app_settings")
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("AppSettings");
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(format!(
                        "Dynamics: {:#?}",
                        self.state.settings.dynamics
                    ));
                    ui.label(format!(
                        "Resolution: {} × {} px",
                        self.state.settings.image.resolution.0,
                        self.state.settings.image.resolution.1
                    ));
                    ui.label(format!(
                        "Center: ({:.6}, {:.6})",
                        self.state.settings.image.center.0,
                        self.state.settings.image.center.1
                    ));
                    ui.label(format!(
                        "Scale: ({:.6}, {:.6})",
                        self.state.settings.image.scale.0,
                        self.state.settings.image.scale.1
                    ));
                    ui.label(format!(
                        "Max iterations: {}",
                        self.state.settings.max_iter
                    ));
                    ui.label(format!(
                        "Escape radius: {}",
                        self.state.settings.escape_radius
                    ));
                    ui.label(format!(
                        "Color map: {:?}",
                        self.state.settings.color_map
                    ));
                    ui.label(format!(
                        "Normalization: {:?}",
                        self.state.settings.normalization
                    ));

                    ui.separator();
                    ui.label("Go to coordinates (x, y)");
                    let mut coordinate_input_changed = false;
                    ui.horizontal(|ui| {
                        ui.label("x");
                        coordinate_input_changed |= ui
                            .add(
                                egui::TextEdit::singleline(
                                    &mut self.coordinate_x,
                                )
                                .desired_width(65.0),
                            )
                            .changed();
                        ui.label("y");
                        coordinate_input_changed |= ui
                            .add(
                                egui::TextEdit::singleline(
                                    &mut self.coordinate_y,
                                )
                                .desired_width(65.0),
                            )
                            .changed();
                    });
                    if coordinate_input_changed {
                        self.coordinate_error = None;
                    }
                    if ui.button("Go to coordinates").clicked() {
                        let x = self.coordinate_x.trim().parse::<f64>();
                        let y = self.coordinate_y.trim().parse::<f64>();
                        match (x, y) {
                            (Ok(x), Ok(y)) if x.is_finite() && y.is_finite() => {
                                actions.push(AppAction::SetCenter(x, y));
                                self.coordinate_error = None;
                            }
                            _ => {
                                self.coordinate_error = Some(
                                    "Enter finite numeric x and y coordinates."
                                        .to_owned(),
                                );
                            }
                        }
                    }
                    if let Some(error) = &self.coordinate_error {
                        ui.colored_label(ui.visuals().error_fg_color, error);
                    }
                });
            });

        let preview_interaction = egui::CentralPanel::default()
            .show(ctx, |ui| self.show_preview(ui))
            .inner;
        if let Some((x, y)) = preview_interaction.clicked_xy {
            actions.push(AppAction::SetCenter(x, y));
        }

        actions.extend(self.export_dialog.show(
            ctx,
            self.export_manager.is_busy(),
            &self.state.export_state,
        ));
        if let Some(path) = self.import_dialog.show(ctx) {
            if path.as_os_str().is_empty() {
                self.import_dialog
                    .set_error("JSONファイルのパスを入力してください".to_owned());
            } else {
                match fs::read(&path)
                    .map_err(|error| error.to_string())
                    .and_then(|contents| {
                        serde_json::from_slice::<AppSettings>(&contents)
                            .map_err(|error| error.to_string())
                    })
                {
                    Ok(settings) => {
                        self.import_dialog.close();
                        self.import_status = Some(format!(
                            "Imported settings from {}",
                            path.display()
                        ));
                        actions.push(AppAction::ApplySettings(settings));
                        actions.push(AppAction::Redraw);
                    }
                    Err(error) => {
                        self.import_dialog.set_error(format!(
                            "設定ファイルを読み込めません: {error}"
                        ));
                    }
                }
            }
        }
        self.process_actions(actions);

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    if let Some(cursor) = preview_interaction.cursor {
                        ui.label(format!(
                            "Pixel ({}, {})   x: {:.8}, y: {:.8}",
                            cursor.pixel.0,
                            cursor.pixel.1,
                            cursor.xy.0,
                            cursor.xy.1
                        ));
                    }
                },
            );
            match &self.state.compute_status {
                ComputeStatus::Idle => {
                    ui.label(
                        "WASD/Arrow keys: move, Shift+Arrow: half move, Q/E: zoom, R: redraw, L: center lines, Z: undo, X/C: Survey/Burst",
                    );
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

            if let Some(status) = &self.import_status {
                ui.separator();
                ui.label(status);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_cursor_maps_pointer_to_pixel_and_plane_coordinates() {
        let image = etfra::util::image_config::ImageConfig::new(
            (2, 2),
            (1.0, 1.0),
            (0.0, 0.0),
        );
        let rect = egui::Rect::from_min_max(
            egui::pos2(0.0, 0.0),
            egui::pos2(2.0, 2.0),
        );

        let cursor = preview_cursor(rect, egui::pos2(1.5, 1.5), &image)
            .unwrap();

        assert_eq!(cursor.pixel, (1, 1));
        assert_eq!(cursor.xy, (0.0, 0.0));
    }
}
