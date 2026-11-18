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

    let in_repo = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(&args.target)
        .status()?
        .success();

    if !in_repo {
        Command::new("git")
            .arg("init")
            .current_dir(&args.target)
            .status()?;
    }
    
    Ok(())
}



