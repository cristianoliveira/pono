use assert_cmd::Command;
use predicates::prelude::predicate;

#[path = "./common/lib.rs"]
mod common;

const BINARY_NAME: &str = "pono";

#[test]
fn it_allows_using_environment_variables() -> Result<(), Box<dyn std::error::Error>> {
    common::cleanup();
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg("examples/configs/using-environment-variables.toml")
        .arg("enable")
        .arg("var:env");

    let expected_output = "var:env: $PWD/examples/to/other";

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected_output));

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.arg("-c")
        .arg("examples/configs/using-environment-variables.toml")
        .arg("disable")
        .arg("var:env");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Unlinked pono: var:env"));

    Ok(())
}
