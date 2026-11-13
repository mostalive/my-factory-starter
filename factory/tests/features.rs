use cucumber::{World as _, given};
use std::{fs, io};
use tempfile::TempDir;

#[derive(Debug, Default, cucumber::World)]
struct FactoryWorld {
    workspace: Option<TempDir>,
    target: Option<std::path::PathBuf>,
    seed: Option<std::path::PathBuf>,


}

impl FactoryWorld {
    fn root(&self) -> &std::path::Path {
        self.workspace.as_ref().unwrap().path()
    }
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
    let target = world.root().join("target");

    fs::create_dir(&target)?;
    world.target = Some(target);
    Ok(())
}

#[given("a seed describing a game of Tetris")]
fn create_seed(world: &mut FactoryWorld) -> io::Result<()> {
    let seed = world.root().join("seed.md");

    fs::write(&seed, "Build a game of tetris in the terminal.\n")?;
    world.seed = Some(seed);
    Ok(())
}


#[given("the agent plans the tasks alpha and beta, and does one task a pass")]
fn prepare_agent(world: &mut FactoryWorld) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let pi = world.root().join("pi");
    fs::write(&pi, include_str!("fixtures/pi.sh"))?;
    fs::set_permissions(&pi, fs::Permissions::from_mode(0o755))?;
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
