use std::sync::Mutex;

use indicatif::ProgressBar;
use rayon::prelude::*;

use bifur::fractals::mandelbrot;

use crate::image_helpers::RGBImage;

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s; // Chroma
    let h_prime = h / 60.0; // Hue' divided into 6 sectors
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

    let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_prime) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_prime) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_prime) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_prime) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = l - c / 2.0; // Match lightness
    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    (r, g, b)
}

fn color_function(t: f64, z: (f64, f64)) -> (u8, u8, u8) {
    let zn = ((z.0 + 0.5) * (z.0 + 0.5) + z.1 * z.1).sqrt();
    let log_zn = zn.ln();
    let nu = 500.0 * t + 1.0 - (2.0 + log_zn).ln() / 2.0_f64.ln();

    let hue = (nu * 5.0 + 160.0) % 360.0;
    let saturation = 0.6;
    let lightness = 0.7 * zn; // zn.clamp(0.0, 1.0);

    hsl_to_rgb(hue, saturation, lightness)
}

fn color_function_2(t: f64) -> (u8, u8, u8) {
    let cycle_value = (t * 30.0).sin().abs();
    let r = (255.0 * cycle_value) as u8;
    let g = (255.0 * (1.0 - cycle_value)) as u8;
    let b = (255.0 * (cycle_value / 2.0)) as u8;

    (r, g, b)
}

pub fn generate(
    width: usize,
    height: usize,
    domain: ((f64, f64), (f64, f64)),
    iteration_limit: usize,
) -> RGBImage {
    let progress_bar = Mutex::new(ProgressBar::new((width * height) as u64));

    let mut divergence_map = vec![None; width * height];
    let (x_interval, y_interval) = domain;

    divergence_map
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, elem)| {
            let col = idx % width;
            let row = idx / width;

            let t = (row as f64) / (height as f64);
            let y = y_interval.0 * (1.0 - t) + y_interval.1 * t;

            let t = (col as f64) / (width as f64);
            let x = x_interval.0 * (1.0 - t) + x_interval.1 * t;

            if let Some(iterations) =
                mandelbrot::diverges_within_max_iterations(x, y, iteration_limit)
            {
                *elem = Some((x, y, iterations));
            }

            progress_bar.lock().unwrap().inc(1);
        });

    let mut image = RGBImage::new(width as u32, height as u32);
    for ((_, _, pixel), iteration_of_divergence) in
        image.enumerate_pixels_mut().zip(divergence_map.iter())
    {
        if let Some((x, y, iterations)) = *iteration_of_divergence {
            *pixel = image::Rgb(color_function((iterations as f64) / (iteration_limit as f64), (x, y)).into());
            // *pixel = image::Rgb(color_function_2((iterations as f64) / (iteration_limit as f64)).into());
        }
    }

    image
}
