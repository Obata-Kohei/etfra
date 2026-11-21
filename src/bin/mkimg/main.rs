use etfra::{prelude::*, presets::custom_complex_dynamics::dynamics::CustomComplexDynamics};

fn main() {
    let dynamics = CustomComplexDynamics::new("z^2 + sin(x^2 - y^2 + 2*x*y*i + c)")
                                                        .expect("Valid expr should be written...");

    let max_iter = 300;
    let escape_radius = 2.0;
    let escape_condition = EscapeByNorm { escape_radius };
    let escape_evaluator = EscapeByCount {
        max_iter,
        condition: escape_condition,
    };

    let normalizer = NormalizeWithMaxIter { max_iter };
    let mut palette = Palette::grayscale(256); palette.reverse();
    let color_map = ColorMapLinear { palette };
    let coloring = Coloring {
        normalizer,
        color_map,
    };

    let mut etf = EscapeTimeFractal::new(dynamics, escape_evaluator, coloring);

    let image_config = &ImageConfig { resolution: (256, 256), scale: (10.0/256.0, 10.0/256.0), center: (0.0, 0.0) };
    let escape_results = etf.escape_results_par(&image_config);
    let colors = etf.colors_from_escape_results(&escape_results);
    let img = etf.rgba_image_from_colors_par(&colors, image_config);

    img.save(format!("./foo.png"))
        .expect("Image should be saved...");

    println!("An fractal made!");
}