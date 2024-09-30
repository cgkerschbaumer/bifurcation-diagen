use std::sync::Mutex;

use indicatif::ProgressBar;
use rayon::prelude::*;

use bifur::fractals::mandelbrot;

use crate::image_helpers::RGBImage;

pub fn generate(
    width: usize,
    height: usize,
    domain: ((f64, f64), (f64, f64)),
    iteration_limit: usize,
) -> RGBImage {
    let mut image = RGBImage::new(width as u32, height as u32);

    let progress_bar = Mutex::new(ProgressBar::new((width * height) as u64));

    let (x_interval, y_interval) = domain;

    image
        .par_enumerate_pixels_mut()
        .for_each(|(col, row, pixel)| {
            let t = (row as f64) / (height as f64);
            let y = y_interval.0 * (1.0 - t) + y_interval.1 * t;

            let t = (col as f64) / (width as f64);
            let x = x_interval.0 * (1.0 - t) + x_interval.1 * t;

            if let Some(_iters) = mandelbrot::diverges_within_max_iterations(x, y, iteration_limit)
            {
                *pixel = image::Rgb([255, 255, 255]);
            } else {
                *pixel = image::Rgb([0, 0, 0]);
            }

            progress_bar.lock().unwrap().inc(1);
        });

    image
}
