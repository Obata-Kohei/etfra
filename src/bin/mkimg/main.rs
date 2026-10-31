use etfra::prelude::*;

fn main() {
    let dynamics = BurningShip::new();

    let max_iter = 300;
    let escape_radius = 2.0;
    let escape_condition = EscapeByNorm { escape_radius };
    let escape_evaluator = EscapeByCount {
        max_iter,
        condition: escape_condition,
    };

    let palette = Palette::grayscale(256);
    let normalizer = NormalizeWithMaxIter { max_iter };
    let color_map = ColorMapLinear { palette };
    let coloring = Coloring {
        normalizer,
        color_map,
    };

    let mut etf = EscapeTimeFractal::new(dynamics, escape_evaluator, coloring);

    let image_config = &ImageConfig { resolution: (300, 300), scale: (0.01, 0.01), center: (0., 0.) };
    let escape_results = etf.escape_results_par(&image_config);
    let colors = etf.colors_from_escape_results(&escape_results);
    let img = etf.rgba_image_from_colors_par(&colors, image_config);

    img.save("./foo.png")
        .expect("Image should be saved...");

    println!("An fractal made!");
}