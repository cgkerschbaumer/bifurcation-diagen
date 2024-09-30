use clap::Parser;
use std::path::PathBuf;

mod image_helpers;

mod feigenbaum;
mod mandelbrot;

#[derive(Parser, Debug)]
enum Subcommand {
    Feigenbaum(FeigenbaumArgs),
    Mandelbrot(MandelbrotArgs),
}

#[derive(Parser, Debug)]
struct FeigenbaumArgs {
    #[arg(long, default_value_t = 0.0f64)]
    from: f64,

    #[arg(long, default_value_t = 4.0f64)]
    to: f64,

    #[arg(long, default_value_t = 10000usize)]
    iter_limit: usize,

    #[arg(long, default_value_t = 1e-4f64)]
    early_exit_threshold: f64,

    #[arg(long, default_value_t = 100usize)]
    early_exit_batch: usize,
}

#[derive(Parser, Debug)]
struct MandelbrotArgs {
    #[arg(long, default_value_t = -2.5f64)]
    from_x: f64,

    #[arg(long, default_value_t = 0.5f64)]
    to_x: f64,

    #[arg(long, default_value_t = -1.5f64)]
    from_y: f64,

    #[arg(long, default_value_t = 1.5f64)]
    to_y: f64,

    #[arg(long, default_value_t = 20000usize)]
    iter_limit: usize,
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

fn main() {
    let args = Args::parse();
    assert!(args.width > 0);
    assert!(args.height > 0);

    let img = match args.command {
        Subcommand::Feigenbaum(sub_cmd_args) => feigenbaum::generate(
            args.width,
            args.height,
            (sub_cmd_args.from, sub_cmd_args.to),
            sub_cmd_args.iter_limit,
            sub_cmd_args.early_exit_threshold,
            sub_cmd_args.early_exit_batch,
        ),
        Subcommand::Mandelbrot(sub_cmd_args) => mandelbrot::generate(
            args.width,
            args.height,
            (
                (sub_cmd_args.from_x, sub_cmd_args.to_x),
                (sub_cmd_args.from_y, sub_cmd_args.to_y),
            ),
            sub_cmd_args.iter_limit,
        ),
    };

    if let Err(e) = img.save(args.output_file) {
        eprintln!("Failed to save image: {}", e)
    }
}
