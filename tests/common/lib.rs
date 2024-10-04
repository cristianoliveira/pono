#![allow(dead_code)]
pub fn cleanup() {
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
