use std::fs;
use std::path::Path;

fn rust_files_below(root: &Path, output: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(root).expect("read architecture directory") {
        let path = entry.expect("read architecture entry").path();
        if path.is_dir() {
            rust_files_below(&path, output);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            output.push(path);
        }
    }
}

#[test]
fn core_crate_has_no_bevy_dependency() {
    let manifest = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/bevyout-core/Cargo.toml"),
    )
    .expect("read bevyout-core manifest");
    assert!(
        !manifest.lines().any(|line| {
            line.trim_start().starts_with("bevy =") || line.trim_start().starts_with("bevy_")
        }),
        "bevyout-core must remain engine-independent"
    );
}

#[test]
fn preparation_does_not_depend_on_viewer() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/vsa");
    let mut files = Vec::new();
    rust_files_below(&root, &mut files);
    let offenders = files
        .into_iter()
        .filter(|path| {
            fs::read_to_string(path)
                .expect("read Rust source")
                .contains("crate::viewer")
        })
        .collect::<Vec<_>>();
    assert!(
        offenders.is_empty(),
        "preparation/runtime dependency inversion in: {offenders:?}"
    );
}
