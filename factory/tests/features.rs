use cucumber::{World as _, given, when, then};
use std::{fs, io, process::{Command, Output}};
use tempfile::TempDir;


#[derive(Debug, Default, cucumber::World)]
struct FactoryWorld {
    workspace: Option<TempDir>,
    target: Option<std::path::PathBuf>,
    seed: Option<std::path::PathBuf>,
    output: Option<Output>
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

#[given("no seed is chosen")]
fn no_seed_is_chosen(world: &mut FactoryWorld)-> io::Result<()> {
    world.seed = None;
    Ok(())
}

#[given("no plan")]
fn no_plan(world: &mut FactoryWorld ) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");

    assert!(!plan.exists(), "expected no plan at {}", plan.display());
    Ok(())
}

#[when("the factory runs one pass")]
fn factory_runs_one_pass(world: &mut FactoryWorld) -> io::Result<()> {
    let mut command = Command::new(world.root().join("factory"));
    command.current_dir(world.root());

    if let Some(seed) = &world.seed {
        command.arg("--seed").arg(seed);
    }
    if let Some(target) = &world.target {
        command.arg("--target").arg(target);
    }
    world.output = Some(command.output()?);
   Ok(())
}

#[given("the seed has been deleted")]
fn delete_seed(world: &mut FactoryWorld) -> io::Result<()> {
    let seed = world.seed.as_ref().expect("no seed to delete");

    fs::remove_file(seed)?;
    Ok(())
}

#[then("it reports that there is no seed")]
fn reports_no_seed(world: &mut FactoryWorld) -> io::Result<()> {
    let output = world.output.as_ref().expect("the factory has not run");

    assert!(
        !output.status.success(),
        "expected the factory to fail when there is no seed"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("seed"),
        "expected the factory to report a missing seed, got: {stderr}"
    );
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
