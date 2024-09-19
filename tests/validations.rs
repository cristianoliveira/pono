use assert_cmd::Command;
use predicates::prelude::predicate;

const BINARY_NAME: &str = "pono";

#[test]
fn it_fails_when_config_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg("examples/configs/unknown.toml")
        .arg("list");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(
            "Failed to read the examples/configs/unknown.toml file",
        ))
        .stdout(predicate::str::contains(
            "Reason: (config-error) No such file or directory (os error 2)",
        ))
        .stdout(predicate::str::contains("Debugging:"));

    Ok(())
}

#[test]
fn it_fails_when_config_lacks_packages() -> Result<(), Box<dyn std::error::Error>> {
    let pono_config = "examples/configs/invalid-missing-packages.toml";
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c").arg(pono_config).arg("list");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(
            "Failed to read the examples/configs/invalid-missing-packages.toml file",
        ))
        .stdout(predicate::str::contains(
            "Reason: (config-error) TOML parse error at line 1, column 1",
        ))
        .stdout(predicate::str::contains("Debugging:"));

    Ok(())
}

#[test]
fn it_fails_when_contains_invalid_package() -> Result<(), Box<dyn std::error::Error>> {
    let pono_config = "examples/configs/invalid-package.toml";
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c").arg(pono_config).arg("list");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains(
            "Failed to read the examples/configs/invalid-package.toml file",
        ))
        .stdout(predicate::str::contains(
            "Reason: (config-error) TOML parse error at line 2, column 7",
        ))
        .stdout(predicate::str::contains("Debugging:"));

    Ok(())
}

#[test]
fn it_fails_when_target_exist_and_isnt_a_symbolic_link() -> Result<(), Box<dyn std::error::Error>> {
    let pono_config = "examples/configs/invalid-target-is-not-link.toml";
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c").arg(pono_config).arg("enable").arg("notlink");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Reason: (not-available)"))
        .stdout(predicate::str::contains(
            "Target path 'examples/to/.gitkeep' already exists and is a file.",
        ));

    Ok(())
}

#[test]
fn it_fails_when_source_is_missing() -> Result<(), Box<dyn std::error::Error>> {
    let pono_config = "examples/configs/invalid-target-is-not-link.toml";
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg(pono_config)
        .arg("enable")
        .arg("doesnexist");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Reason: (not-found)"))
        .stdout(predicate::str::contains("Pono source does not exist"));

    Ok(())
}

#[test]
fn it_does_not_fail_when_missing_config_toml_for_completion(
) -> Result<(), Box<dyn std::error::Error>> {
    // change the current directory to the root of the project
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c")
        .arg("examples/configs/unknown.toml")
        .arg("completions")
        .arg("bash");

    cmd.assert().success();

    Ok(())
}
