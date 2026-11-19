//! Steps for `planning.feature`: how the plan is made and who keeps it true.

use std::{fs, io, process::Command};
use std::path::Path;
    
use cucumber::{given, then};

use crate::FactoryWorld;

#[given("no seed is chosen")]
fn no_seed_is_chosen(world: &mut FactoryWorld) -> io::Result<()> {
    world.seed = None;
    Ok(())
}

const THREE_TASK_PLAN: &str = "- [ ] task 1\n- [ ] task 2\n- [ ] task 3";

#[given("a plan with three tasks, none of them done")]
fn a_plan_with_three_tasks_none_done(world: &mut FactoryWorld) -> io::Result<()> {
    world.ensure_factory_path()?;
    let plan_path = world.plan_path();
    fs::write(&plan_path, &THREE_TASK_PLAN)?;

    Ok(())
}

#[then("there is a plan")]
fn a_plan(world: &mut FactoryWorld) -> io::Result<()> {
    let plan = world.plan_path();

    assert!(plan.exists(), "expected plan at {}", plan.display());
    Ok(())
}

#[then("the plan shows every task as not done")]
fn plan_shows_every_task_as_not_done(world: &mut FactoryWorld) -> io::Result<()> {
    let plan = world.plan_path();
    let contents = world.plan_contents()?;
    let tasks: Vec<&str> = contents
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

#[then("the plan still has those three tasks")]
fn plan_still_has_three_tasks(world: &mut FactoryWorld) -> io::Result<()> {
    assert_eq!(world.plan_contents()?, THREE_TASK_PLAN);
    Ok(())
}

#[then("the target has no uncommitted changes")]
fn target_has_no_uncommited_changes(world: &mut FactoryWorld) -> io::Result<()> {
    let status = status(&world.target.as_ref().unwrap());
    assert!(status.is_empty(), "Expected `git status` to have no uncommited files, but got {:?}", status);
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

fn status(repo: &Path) -> Vec<u8> {
    Command::new("git")
        .current_dir(repo)
        .args(["status", "--porcelain=v1", "-z", "--untracked-files=all"])
        .output()
        .expect("git did not run")
        .stdout

}
