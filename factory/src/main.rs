use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    seed: PathBuf,
    #[arg(long)]
    target: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if !args.seed.exists() {
        eprintln!("seed file does not exist: {}", args.seed.display());
        std::process::exit(1);
    }

    Command::new("pi").current_dir(&args.target).status()?;

    Ok(())
}
