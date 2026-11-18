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

    // check_seed(&args.seed)
    if !args.seed.exists() {
        eprintln!("seed file does not exist: {}", args.seed.display());
        std::process::exit(1);
    }

    Command::new("pi").current_dir(&args.target).status()?;

    // ensure_repo(&args.target)
    let in_repo = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(&args.target)
        .status()?
        .success();

    let target = &args.target;
    if !in_repo {
        Command::new("git")
            .arg("init")
            .current_dir(target)
            .status()?;


        Command::new("git")
            .args(["add", ".factory/plan.md"])
            .current_dir(target)
            .status()?;

        Command::new("git")
            .args(["commit", "-m", "Initial factory plan committed"])
            .current_dir(target)
            .status()?;
    }

    //TODO commit_changed_plan(&args.plan)

    
    Ok(())
}


