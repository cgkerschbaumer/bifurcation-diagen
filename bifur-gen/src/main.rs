use clap::Parser;
use std::path::PathBuf;

mod feigenbaum;
mod image_helpers;

#[derive(Parser, Debug)]
enum Subcommand {
    Feigenbaum(FeigenbaumArgs),
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

    match args.command {
        Subcommand::Feigenbaum(sub_cmd_args) => {
            let img = feigenbaum::generate(
                args.width,
                args.height,
                (sub_cmd_args.from, sub_cmd_args.to),
                sub_cmd_args.iter_limit,
                sub_cmd_args.early_exit_threshold,
                sub_cmd_args.early_exit_batch,
            );

            if let Err(e) = img.save(args.output_file) {
                eprintln!("Failed to save image: {}", e)
            }
        }
    }
}
