//! Steps for `planning.feature`: how the plan is made and who keeps it true.

use std::{fs, io, process::Command};

use cucumber::{given, then};

use crate::FactoryWorld;

#[given("no seed is chosen")]
fn no_seed_is_chosen(world: &mut FactoryWorld) -> io::Result<()> {
    world.seed = None;
    Ok(())
}

#[then("there is a plan")]
fn a_plan(world: &mut FactoryWorld) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");

    assert!(plan.exists(), "expected plan at {}", plan.display());
    Ok(())
}

#[then("the plan shows every task as not done")]
fn plan_shows_every_task_as_not_done(world: &mut FactoryWorld) -> io::Result<()> {
    let plan = world.target.as_ref().unwrap().join(".factory/plan.md");
    let plan_contents = fs::read_to_string(&plan)?;
    let tasks: Vec<&str> = plan_contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    assert!(
        !tasks.is_empty(),
        "expected at least one task in {}",
        plan.display()
    );
    let non_tasks: Vec<&str> = tasks
        .into_iter()
        .filter(|line| !line.contains("- [ ]"))
        .collect();
    assert!(
        non_tasks.is_empty(),
        "expected all non-empty lines to be not done tasks '- [ ]', but got: {} ",
        plan.display()
    );
    Ok(())
}

#[then("there are no new work commits")]
fn no_new_work_commits(world: &mut FactoryWorld) -> io::Result<()> {
    let work: Vec<String> = changed_files(world)
        .into_iter()
        .filter(|path| path != ".factory/plan.md")
        .collect();

    assert!(
        work.is_empty(),
        "expected no work commits, but found: {work:?}"
    );
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

fn changed_files(world: &FactoryWorld) -> Vec<String> {
    let target = world.target.as_ref().unwrap();
    let out = Command::new("git")
        .current_dir(target)
        .args(["log", "--format=", "--name-only"])
        .output()
        .expect("git did not run");
    if !out.status.success() {
        return Vec::new(); // no repository yet, so no commits
    }
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
