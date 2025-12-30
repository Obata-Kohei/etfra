use eframe::egui;

pub struct Renderer {
    texture: Option<egui::TextureHandle>,
}

impl Renderer {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        image: &egui::ColorImage,
    ) {
        let tex = self.texture.get_or_insert_with(|| {
            ctx.load_texture(
                "fractal",
                image.clone(),
                egui::TextureOptions::NEAREST,
            )
        });

        tex.set(image.clone(), egui::TextureOptions::NEAREST);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(tex);
        });
    }
}
