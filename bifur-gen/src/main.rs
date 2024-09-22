use std::path::PathBuf;

use clap::Parser;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive};

type RGBImage = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

#[derive(Parser, Debug)]
enum Subcommand {
    Feigenbaum,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long)]
    width: u32,

    #[arg(short = 'H', long)]
    height: u32,

    #[arg(short, long)]
    output_file: PathBuf,

    #[clap(subcommand)]
    command: Subcommand,
}

fn generate_feigenbau_diagram(image: &mut RGBImage) {
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([255, 255, 255]);
    }
}

fn main() {
    let args = Args::parse();
    assert!(args.width > 0);
    assert!(args.height > 0);

    let mut imgbuf = RGBImage::new(args.width, args.height);

    match args.command {
        Subcommand::Feigenbaum => generate_feigenbau_diagram(&mut imgbuf),
    }

    match imgbuf.save(args.output_file) {
        Ok(_) => println!("Image successfully created"),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
