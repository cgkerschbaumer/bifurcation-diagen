use std::path::PathBuf;

use indicatif::ProgressIterator;
use clap::Parser;

use bifur::Histogram;
use image::ImageBuffer;

type RGBImage = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

#[derive(Parser, Debug)]
enum Subcommand {
    Feigenbaum,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long)]
    width: usize,

    #[arg(short = 'H', long)]
    height: usize,

    #[arg(short, long)]
    output_file: PathBuf,

    #[clap(subcommand)]
    command: Subcommand,
}

fn transpose_image(img: &RGBImage) -> RGBImage {
    let mut img_transposed = RGBImage::new(img.height(), img.width());

    for (x, y, pixel) in img_transposed.enumerate_pixels_mut() {
        *pixel = *img.get_pixel(y, x);
    }

    img_transposed
}

fn generate_feigenbau_diagram(width: usize, height: usize) -> RGBImage {
    let mut image = RGBImage::new(height as u32, width as u32);

    for (a, row) in image.enumerate_rows_mut().progress() {
        let a = 4.0 * (a as f64) / (width as f64);

        let mut hist = bifur::HistogramR1::new((0.0, 1.0), height as usize);

        let samples = 1000;
        for i in 0..samples {
            let initial_point = (1.0 / (samples as f64)) * (i as f64);
            let orb = bifur::Orbit::<f64>::trace(|x| a * x * (1.0 - x), initial_point, 2000);

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

fn main() {
    let args = Args::parse();
    assert!(args.width > 0);
    assert!(args.height > 0);

    match args.command {
        Subcommand::Feigenbaum => {
            let img = generate_feigenbau_diagram(args.width, args.height);

            if let Err(e) = img.save(args.output_file) {
                eprintln!("Failed to save image: {}", e)
            }
        }
    }
}
