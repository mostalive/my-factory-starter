use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    seed: PathBuf,
    #[arg(long)]
    target: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("factory not built yet");

    if !args.seed.exists() {
        eprintln!("seed file does not exist: {}", args.seed.display());
        std::process::exit(1);
    }
}
