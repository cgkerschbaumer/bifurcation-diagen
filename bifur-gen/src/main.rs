use clap::Parser;
use std::path::PathBuf;

mod feigenbaum;
mod image_helpers;

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

fn main() {
    let args = Args::parse();
    assert!(args.width > 0);
    assert!(args.height > 0);

    match args.command {
        Subcommand::Feigenbaum => {
            let img = feigenbaum::generate(args.width, args.height, (0.0, 4.0));

            if let Err(e) = img.save(args.output_file) {
                eprintln!("Failed to save image: {}", e)
            }
        }
    }
}
