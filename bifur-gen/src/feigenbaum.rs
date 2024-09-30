use std::sync::Mutex;

use bifur::{histograms::HistogramR1, Histogram};
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::image_helpers::{transpose_image, RGBImage};

pub fn generate(
    width: usize,
    height: usize,
    bifurcation_param_interval: (f64, f64),
    iteration_limit: usize,
    early_exit_eps: f64,
    early_exit_batch: usize,
) -> RGBImage {
    let mut image = RGBImage::new(height as u32, width as u32);

    let progress_bar = Mutex::new(ProgressBar::new(width as u64));

    image
        .rows_mut()
        .enumerate()
        .par_bridge()
        .for_each(|(row_idx, row)| {
            let t = (row_idx as f64) / (width as f64);
            let a = bifurcation_param_interval.0 * (1.0 - t) + bifurcation_param_interval.1 * t;

            let mut hist = HistogramR1::new((0.0, 1.0), height);

            let samples = 1000;
            for i in 0..samples {
                let initial_point = (1.0 / (samples as f64)) * (i as f64);

                bifur::Orbit::<f64>::trace_with_early_exit(
                    &mut hist,
                    |x| a * x * (1.0 - x),
                    initial_point,
                    iteration_limit,
                    early_exit_eps,
                    early_exit_batch,
                );
            }

            for (y, pixel) in row.enumerate() {
                let y = height - 1 - y;
                if let Some(bifur::HistValue::NormalizedValue(val)) =
                    hist.get(y, bifur::HistFormat::DivideByMax)
                {
                    let shade = (255.0 * (1.0 - val)) as u8;
                    *pixel = image::Rgb([shade, shade, shade])
                }
            }

            progress_bar.lock().unwrap().inc(1);
        });

    transpose_image(&image)
}
