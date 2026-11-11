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
    let _args = Args::parse();
    println!("factory not built yet");
}
