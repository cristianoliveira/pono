use assert_cmd::Command;
use predicates::prelude::predicate;

fn cleanup() {
    match std::fs::remove_dir_all("examples/target") {
        Ok(_) => {
            std::fs::create_dir("examples/target").expect("Failed to recreate target directory");
        }
        err => {
            println!("Failed to cleanup target directory: {:?}", err);
        }
    }
}

#[test]
fn it_list_the_packages_declared_in_the_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("slot")?;

    cmd.arg("-c").arg("examples/basic.toml").arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Packages:"))
        .stdout(predicate::str::contains("nvim: ./examples/source/nvim"))
        .stdout(predicate::str::contains("zsh: ./examples/source/zshrc"));

    Ok(())
}

#[test]
fn it_link_the_packages() -> Result<(), Box<dyn std::error::Error>> {
    cleanup();
    let mut cmd = Command::cargo_bin("slot")?;

    cmd.arg("-c").arg("examples/basic.toml").arg("link");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Linking packages"))
        .stdout(predicate::str::contains(
            "nvim: ./examples/target/nvim (new link)",
        ))
        .stdout(predicate::str::contains(
            "zsh: ./examples/target/.zshrc (new link)",
        ));

    let list_files_in_nvim = std::fs::read_dir("examples/target/nvim")?;
    let list_files_in_source_nvim = std::fs::read_dir("examples/source/nvim")?;
    assert_eq!(
        list_files_in_nvim.count(),
        list_files_in_source_nvim.count()
    );

    let zsh_target_content = std::fs::read_to_string("examples/target/.zshrc")?;
    let zsh_source_content = std::fs::read_to_string("examples/source/zshrc")?;
    assert_eq!(zsh_target_content, zsh_source_content);

    cleanup();
    Ok(())
}
