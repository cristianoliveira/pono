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

#[test]
fn it_executes_pre_enable_and_disable_hook_when_configured(
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg("examples/basic.toml")
        .arg("enable")
        .arg("with-hooks");

    cmd.assert().success().stdout(predicate::str::contains(
        "Running pre_enable hook for with-hooks",
    ));

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg("examples/basic.toml")
        .arg("disable")
        .arg("with-hooks");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Unlinked pono: with-hooks"))
        .stdout(predicate::str::contains(
            "Running pre_disable hook for with-hooks",
        ));

    Ok(())
}

#[test]
#[cfg(feature = "test-all")] // use: `make tests`
fn it_allows_using_tilda_to_express_homedir() -> Result<(), Box<dyn std::error::Error>> {
    common::cleanup();
    std::fs::remove_file(
        std::path::Path::new(std::env::var("HOME")?.as_str()).join("__pono_test__"),
    )
    .ok();
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg("examples/configs/using-environment-variables.toml")
        .arg("enable")
        .arg("home");

    cmd.assert().success().stdout(predicate::str::contains(
        "home  $PWD/examples/from/other -> ~/__pono_test__ (linking)",
    ));

    let mut cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.arg("-c")
        .arg("examples/configs/using-environment-variables.toml")
        .arg("disable")
        .arg("home");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Unlinked pono: home"));

    Ok(())
}
