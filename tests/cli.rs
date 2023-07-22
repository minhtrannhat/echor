use assert_cmd::Command;
use predicates::prelude::*;
use rstest::{fixture, rstest};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[fixture]
#[once]
pub fn pre_test() -> bool {
    let script_path = "./test_script.sh";

    // Ensure the script file has the necessary permissions to be executed
    let mut permissions_cmd = Command::new("chmod");
    permissions_cmd.arg("+x").arg(script_path);
    permissions_cmd
        .output()
        .expect("Cannot make test_script.sh executable");

    // Run the bash script using the `Command` struct
    let output = Command::new("bash")
        .arg(script_path)
        .output()
        .expect("Running bash script failed!");

    if output.status.success() {
        // The script ran successfully
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Script Output:\n{}", stdout);

        true
    } else {
        // The script encountered an error
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Script Error:\n{}", stderr);

        false
    }
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[rstest]
fn hello1(pre_test: &bool) -> TestResult {
    assert_eq!(pre_test, &true);
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[rstest]
fn hello2(pre_test: &bool) -> TestResult {
    assert_eq!(pre_test, &true);
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[rstest]
fn hello1_no_newline(pre_test: &bool) -> TestResult {
    assert_eq!(pre_test, &true);
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[rstest]
fn hello2_no_newline(pre_test: &bool) -> TestResult {
    assert_eq!(pre_test, &true);
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
