use std::time::Instant;

use chrono::{Local, DateTime};
use num_complex::Complex;
use crate::palette::Palette;
use crate::escape_time_fractal::EscapeTimeFractal;
use crate::complex_dynamics as cmpdyn;
use crate::escape_evaluator as esceval;
use crate::coloring as clr;

mod color;
mod palette;
mod complex_dynamics;
mod escape_evaluator;
mod coloring;
mod escape_time_fractal;

fn main() {
    let resolution: (usize, usize) = (2048 as usize, 2048 as usize);  // (w, h)
    let center = Complex::new(-0.5, 0.);
    let view_size = (3., 3.);
    let max_iter = 500;
    let escape_radius = 2.0;
    //let mut palette = Palette::gradation_by_hue(256, 0., 360., 1.0, 1.0);
    let mut palette = Palette::grayscale(256);
    palette.reverse();

    let dynamics = cmpdyn::Mandelbrot {};
    let escape = esceval::EscapeByCount {max_iter, escape_radius};
    let coloring = clr::PaletteColoring {palette, max_iter};

    let frc = EscapeTimeFractal::new(dynamics, escape, coloring, resolution, center, view_size);

    let start = Instant::now();
    let img = frc.render_par();
    let duration = start.elapsed();

    println!("time elapsed: {:?}", duration);

    let now: DateTime<Local> = Local::now();
    let s = now.format("%Y%m%d%H%M%S").to_string();
    img.save(format!("fractal_{s}.png")).expect("image should be saved");
}
