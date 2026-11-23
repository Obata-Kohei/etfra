use std::sync::Arc;

use eframe::egui;

/// GUI全体のテーマ設定
pub fn configure(ctx: &egui::Context) {
    configure_fonts(ctx);
    configure_colors(ctx);
}

/// フォント設定
fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // フォントをバイナリに埋め込む
    fonts.font_data.insert(
        "custom".to_owned(),
        Arc::new(egui::FontData::from_static(
            include_bytes!("../assets/IBMPlexMono-Regular.ttf"),
        )),
    );

    // 標準フォントとして登録
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["custom".to_owned()],
    );

    fonts.families.insert(
        egui::FontFamily::Monospace,
        vec!["custom".to_owned()],
    );

    ctx.set_fonts(fonts);
}

/// カラー設定
fn configure_colors(ctx: &egui::Context) {
    use egui::{Color32, Stroke, CornerRadius};

    // --------------------------------------------------
    // Palette
    // --------------------------------------------------

    let background = Color32::from_rgb(36, 36, 40);
    let panel      = Color32::from_rgb(24, 29, 31);
    let surface    = Color32::from_rgb(30, 37, 39);
    let border     = Color32::from_rgb(55, 65, 64);

    let text       = Color32::from_rgb(194, 198, 185);
    let muted      = Color32::from_rgb(130, 139, 132);

    let accent     = Color32::from_rgb(117, 166, 157);
    let hover      = Color32::from_rgb(48, 67, 65);
    let active     = Color32::from_rgb(63, 88, 81);

    let warning    = Color32::from_rgb(190, 151, 91);
    let error      = Color32::from_rgb(178, 95, 86);

    // --------------------------------------------------
    // Base visuals
    // --------------------------------------------------

    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = background;
    visuals.window_fill = panel;

    visuals.override_text_color = Some(text);

    visuals.extreme_bg_color = Color32::from_rgb(11, 14, 16);
    visuals.faint_bg_color = surface;
    visuals.code_bg_color = Color32::from_rgb(20, 25, 27);

    visuals.hyperlink_color = accent;
    visuals.warn_fg_color = warning;
    visuals.error_fg_color = error;

    visuals.selection.bg_fill = Color32::from_rgb(54, 79, 76);
    visuals.selection.stroke = Stroke::new(1.0, accent);

    // --------------------------------------------------
    // Widget appearance
    // --------------------------------------------------

    fn set_widget(
        widget: &mut egui::style::WidgetVisuals,
        fill: Color32,
        border: Color32,
        text: Color32,
    ) {
        widget.bg_fill = fill;
        widget.weak_bg_fill = fill;
        widget.bg_stroke = Stroke::new(1.0, border);
        widget.fg_stroke = Stroke::new(1.0, text);
        widget.corner_radius = CornerRadius::same(2);
        widget.expansion = 0.0;
    }

    set_widget(
        &mut visuals.widgets.noninteractive,
        panel,
        border,
        text,
    );

    set_widget(
        &mut visuals.widgets.inactive,
        surface,
        border,
        text,
    );

    set_widget(
        &mut visuals.widgets.hovered,
        hover,
        accent,
        text,
    );

    set_widget(
        &mut visuals.widgets.active,
        active,
        accent,
        text,
    );

    set_widget(
        &mut visuals.widgets.open,
        hover,
        accent,
        text,
    );

    // --------------------------------------------------
    // General appearance
    // --------------------------------------------------

    visuals.button_frame = true;
    visuals.collapsing_header_frame = true;
    visuals.striped = true;

    // --------------------------------------------------
    // Apply
    // --------------------------------------------------

    ctx.set_theme(egui::Theme::Dark);
    ctx.set_visuals_of(egui::Theme::Dark, visuals);

    // Spacing and layout
    ctx.style_mut(|style| {
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
    });
}