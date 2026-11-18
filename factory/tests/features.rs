use cucumber::{World as _, cli, given, when, then, writer, WriterExt as _};
use std::{fs, io, process::{Command, Output}};
use tempfile::TempDir;

#[path = "support/planning.rs"]
mod planning;
#[path = "support/stats.rs"]
mod stats;


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

    fn agent_calls(&self) -> std::path::PathBuf {
        self.root().join("agent-calls")
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

#[given("no plan")]
#[then("there is no plan")]
fn no_plan(world: &mut FactoryWorld ) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");

    assert!(!plan.exists(), "expected no plan at {}", plan.display());
    Ok(())
}

#[when("the factory runs one pass")]
fn factory_runs_one_pass(world: &mut FactoryWorld) -> io::Result<()> {

    let mut command = Command::new(world.root().join("factory"));
    command.env("PATH", format!("{}:{}", world.root().display(),
                                std::env::var("PATH").unwrap_or_default()));
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

#[then("no agent has been called")]
fn no_agent_has_been_called(world: &mut FactoryWorld) -> io::Result<()> {
    let calls = world.agent_calls();
    let log = fs::read_to_string(&calls).unwrap_or_default();

    assert!(
        log.is_empty(),
        "expected the agent never to be called, but it was called:\n{log}"
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
    let script = include_str!("fixtures/pi.sh")
        .replace("@CALLS@", &world.agent_calls().display().to_string());

    fs::write(&pi, script)?;
    fs::set_permissions(&pi, fs::Permissions::from_mode(0o755))?;
    Ok(())
}

#[tokio::main]
async fn main() {
    FactoryWorld::cucumber()
        .with_writer(writer::Normalize::new(writer::Tee::new(
            writer::Basic::stdout::<FactoryWorld>().summarized(),
            stats::StepStats::new(),
        )))
        .fail_on_skipped()
        .with_cli(cli::Opts::<_, _, _, cli::Empty>::parsed())
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

