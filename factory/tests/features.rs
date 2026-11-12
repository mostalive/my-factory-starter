use cucumber::{World as _, given};
use std::{fs, io};
use tempfile::TempDir;

#[derive(Debug, Default, cucumber::World)]
struct FactoryWorld {
    workspace: Option<TempDir>,
    target: Option<std::path::PathBuf>,
    seed: Option<std::path::PathBuf>,
}

#[given("a copy of the factory")]
fn copy_factory(world: &mut FactoryWorld) -> io::Result<()> {
    let workspace = tempfile::tempdir()?;

    fs::copy(
        env!("CARGO_BIN_EXE_factory"),
        workspace.path().join("factory"),
    )?;

    world.workspace = Some(workspace);
    Ok(())
}

#[given("a new target")]
fn new_target(world: &mut FactoryWorld) -> io::Result<()> {
    let workspace = world.workspace.as_ref().unwrap();
    let target = workspace.path().join("target");

    fs::create_dir(&target)?;
    world.target = Some(target);
    Ok(())
}

#[given("a seed describing a game of Tetris")]
fn create_seed(world: &mut FactoryWorld) -> io::Result<()> {
    let workspace = world.workspace.as_ref().unwrap();
    let seed = workspace.path().join("seed.md");

    fs::write(&seed, "Build a game of tetris in the terminal.\n")?;
    world.seed = Some(seed);
    Ok(())
}

#[tokio::main]
async fn main() {
    FactoryWorld::cucumber()
        .fail_on_skipped()
        .filter_run_and_exit("spec/features", |feature, rule, scenario| {
            let real_agent = feature
                .tags
                .iter()
                .chain(rule.into_iter().flat_map(|rule| &rule.tags))
                .chain(&scenario.tags)
                .any(|tag| tag == "real-agent");
            real_agent == cfg!(feature = "real-agent")
        })
        .await;
}
