use std::process::Command;

#[test]
fn missing_target_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_factory"))
        .args(["--seed", "seed.md"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--target"), "{stderr}");
}

#[test]
fn missing_seed_file_is_rejected() {
    let workspace = tempfile::tempdir().unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_factory"))
        .current_dir(workspace.path())
        .args(["--seed", "missing-seed.md", "--target", "target"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("seed"), "{stderr}");
    assert!(stderr.contains("missing-seed.md"), "{stderr}");
}
