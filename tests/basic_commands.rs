use assert_cmd::Command;
use predicates::prelude::predicate;

const BINARY_NAME: &str = "pono";

fn cleanup() {
    let current_dir = std::env::current_dir().unwrap();
    let examples_dir = current_dir.join("examples/to");
    let paths = std::fs::read_dir(&examples_dir).expect(&format!(
        "CLEANUP: Failed to read directory {:?}",
        examples_dir
    ));

    for path in paths.into_iter() {
        let path = path.unwrap().path();
        // ignore .gitkeep file
        if path.ends_with(".gitkeep") {
            continue;
        }
        std::fs::remove_file(&path).expect(&format!("Failed to remove file {:?}", path));
    }
}

#[test]
fn it_list_the_ponos_declared_in_the_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c").arg("examples/basic.toml").arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Ponos:"))
        .stdout(predicate::str::contains("nvim: ./examples/from/nvim"))
        .stdout(predicate::str::contains("zsh: ./examples/from/zshrc"));

    Ok(())
}

#[test]
fn it_link_the_ponos() -> Result<(), Box<dyn std::error::Error>> {
    cleanup();
    let mut cmd = Command::cargo_bin(BINARY_NAME)?;

    cmd.arg("-c").arg("examples/basic.toml").arg("link");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Linking ponos"))
        .stdout(predicate::str::contains(
            "nvim: ./examples/to/nvim (new link)",
        ))
        .stdout(predicate::str::contains(
            "zsh: ./examples/to/.zshrc (new link)",
        ));

    let list_files_in_nvim = std::fs::read_dir("examples/to/nvim")?;
    let list_files_in_source_nvim = std::fs::read_dir("examples/to/nvim")?;
    assert_eq!(
        list_files_in_nvim.count(),
        list_files_in_source_nvim.count()
    );

    let zsh_target_content = std::fs::read_to_string("examples/to/.zshrc")?;
    let zsh_source_content = std::fs::read_to_string("examples/from/zshrc")?;
    assert_eq!(zsh_target_content, zsh_source_content);

    cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.arg("-c")
        .arg("examples/basic.toml")
        .arg("status")
        .arg("nvim");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("nvim ./examples/to/nvim (linked)"))
        .stdout(predicate::str::contains("zsh ./examples/to/.zshrc (linked)").count(0));

    cmd = Command::cargo_bin(BINARY_NAME)?;
    cmd.arg("-c")
        .arg("examples/basic.toml")
        .arg("unlink")
        .arg("nvim");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Unlinked pono: nvim"));

    cleanup();
    Ok(())
}
