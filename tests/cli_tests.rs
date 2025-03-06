use assert_cmd::Command;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("ebclup").unwrap();
    let assert = cmd.arg("--version").assert();
    let p = env!("CARGO_PKG_NAME");
    let v = env!("CARGO_PKG_VERSION");
    let expected = format!("{p} {v}\n");
    assert.stdout(expected).success();
}

#[test]
fn test_scratch_template() {
    let mut cmd = Command::cargo_bin("ebclup").unwrap();
    cmd.arg("startproject")
        .arg("scratch")
        .arg("test_project_scratch")
        .current_dir(env!("OUT_DIR"))
        .assert()
        .success();
    let project_dir = std::path::Path::new(env!("OUT_DIR")).join("test_project_scratch");
    assert!(project_dir.exists());
    let readme_path = project_dir.join("README.md");
    assert!(readme_path.exists());
}
