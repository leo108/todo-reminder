use std::process::Command;

#[test]
fn test_main_with_table_format() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_files/config.toml")
        .arg("--format=table")
        .arg("--no-tty")
        .arg("--exit-zero")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    insta::assert_debug_snapshot!(stdout.lines().collect::<Vec<&str>>());
}

#[test]
fn test_main_with_json_format() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/test_files/config.toml")
        .arg("--format=json")
        .arg("--exit-zero")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    insta::assert_debug_snapshot!(stdout.lines().collect::<Vec<&str>>());
}
