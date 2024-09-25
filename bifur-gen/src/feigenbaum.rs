use bifur::Histogram;
use indicatif::ProgressIterator;

use crate::image_helpers::{transpose_image, RGBImage};

pub fn generate(width: usize, height: usize, bifurcation_param_interval: (f64, f64)) -> RGBImage {
    let mut image = RGBImage::new(height as u32, width as u32);

    for (row_idx, row) in image.enumerate_rows_mut().progress() {
        let t = (row_idx as f64) / (width as f64);
        let a = bifurcation_param_interval.0 * (1.0 - t) + bifurcation_param_interval.1 * t;

        let mut hist = bifur::HistogramR1::new((0.0, 1.0), height);

        let samples = 1000;
        for i in 0..samples {
            let initial_point = (1.0 / (samples as f64)) * (i as f64);
            let orb = bifur::Orbit::<f64>::trace(|x| a * x * (1.0 - x), initial_point, 3000);

            orb.update_histogram(&mut hist);
        }

        for (y, _, pixel) in row {
            let y = height - 1 - y as usize;
            match hist.get(y, bifur::HistFormat::DivideByMax) {
                Some(bifur::HistValue::NormalizedValue(val)) => {
                    let shade = (255.0 * (1.0 - val)) as u8;
                    *pixel = image::Rgb([shade, shade, shade])
                }
                _ => {}
            }
        }
    }

    transpose_image(&image)
}
