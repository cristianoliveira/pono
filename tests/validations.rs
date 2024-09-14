use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn it_fails_when_config_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("slot")?;

    cmd.arg("-c").arg("examples/configs/unknown.toml").arg("list");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Failed to read the examples/configs/unknown.toml file"))
        .stdout(predicate::str::contains("Reason: No such file or directory"))
        .stdout(predicate::str::contains("Debugging:"));

    Ok(())
}
