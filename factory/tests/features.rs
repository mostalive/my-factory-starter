use cucumber::{World as _, cli, given, when, then};
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

#[given("no seed is chosen")]
fn no_seed_is_chosen(world: &mut FactoryWorld)-> io::Result<()> {
    world.seed = None;
    Ok(())
}

#[given("no plan")]
#[then("there is no plan")]
fn no_plan(world: &mut FactoryWorld ) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");

    assert!(!plan.exists(), "expected no plan at {}", plan.display());
    Ok(())
}

#[then("there is a plan")]
fn a_plan(world: &mut FactoryWorld ) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");

    assert!(plan.exists(), "expected plan at {}", plan.display());
    Ok(())
}

#[then("the plan shows every task as not done")] 
fn plan_shows_every_task_as_not_done(world: &mut FactoryWorld) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");
    let plan_contents = fs::read_to_string(&plan)?;
    let tasks: Vec<&str> = plan_contents .lines().filter(|line| !line.trim().is_empty()).collect();

    assert!(!tasks.is_empty(), "expected at least one task in {}", plan.display());
    let non_tasks: Vec<&str> = tasks.into_iter().filter(|line| !line.contains("- [ ]")).collect();
    assert!(non_tasks.is_empty(), "expected all non-empty lines to be not done tasks '- [ ]', but got: {} ",
            plan.display());
    Ok(())
}

#[then("there are no new work commits")]
fn no_new_work_commits(world: &mut FactoryWorld) -> io::Result<()> {
    let work: Vec<String> = changed_files(world)
        .into_iter()
        .filter(|path| path != ".factory/plan.md")
        .collect();

    assert!(work.is_empty(), "expected no work commits, but found: {work:?}");
    Ok(())
}

#[then("the committed plan matches the plan on disk")]
fn committed_plan_matches_plan_on_disk(world: &mut FactoryWorld) -> io::Result<()> {
    let target = world.target.as_ref().unwrap();

    let committed = Command::new("git")
        .current_dir(target)
        .args(["show", "HEAD:.factory/plan.md"])
        .output()?;
    assert!(
        committed.status.success(),
        "expected the plan to be committed, but git show said: {}",
        String::from_utf8_lossy(&committed.stderr)
    );

    let on_disk = fs::read(target.join(".factory/plan.md"))?;
    assert_eq!(
        committed.stdout, on_disk,
        "committed plan differs from the plan on disk"
    );
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

fn changed_files(world: &FactoryWorld) -> Vec<String> {
    let target = world.target.as_ref().unwrap();
    let out = Command::new("git")
        .current_dir(target)
        .args(["log", "--format=", "--name-only"])
        .output()
        .expect("git did not run");
    if !out.status.success() {
        return Vec::new();          // no repository yet, so no commits
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|line| !line.is_empty()
        ).map(str::to_string)
         .collect()
}                 
