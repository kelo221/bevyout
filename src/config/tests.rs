use super::*;
use crate::cli::Cli;
use clap::Parser;
use std::ffi::OsString;
use std::sync::atomic::{AtomicU32, Ordering};

/// A config file at a unique path under the system temp dir, removed on drop.
struct TempConfigFile {
    path: PathBuf,
}

impl TempConfigFile {
    fn new(contents: &str) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!(
            "bevyout-config-test-{}-{unique}.toml",
            std::process::id()
        ));
        fs::write(&path, contents).expect("write temp config file");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempConfigFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

const SAMPLE_CONFIG: &str = r#"
    [fallout3]
    game_root = "/config/game-root"
    plugin = "config-plugin.esm"

    [tools]
    blender = "/config/blender"
    irradiance_blender = "/config/irradiance-blender"
    ktx = "/config/ktx"

    [output]
    cache_dir = "/config/cache"
"#;

#[test]
fn prepare_cli_value_wins_and_none_is_filled_from_config() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "prepare",
        "--config",
        config.path().to_str().unwrap(),
        "--game-root",
        "/cli/game-root",
    ])
    .unwrap();

    apply(&mut cli).unwrap();

    let CommandLine::Prepare(args) = &cli.command else {
        panic!("expected prepare command");
    };
    // CLI-provided value is not overwritten.
    assert_eq!(args.game_root.as_deref(), Some(Path::new("/cli/game-root")));
    // None is filled from config.
    assert_eq!(args.plugin.as_deref(), Some(Path::new("config-plugin.esm")));
    assert_eq!(args.cache_dir.as_deref(), Some(Path::new("/config/cache")));
    assert_eq!(args.blender.as_deref(), Some(Path::new("/config/blender")));
}

#[test]
fn bake_cli_value_wins_and_none_is_filled_from_config() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--config",
        config.path().to_str().unwrap(),
        "--blender",
        "/cli/blender",
    ])
    .unwrap();

    apply(&mut cli).unwrap();

    let CommandLine::Bake(args) = &cli.command else {
        panic!("expected bake command");
    };
    // CLI-provided value is not overwritten.
    assert_eq!(args.blender.as_deref(), Some(Path::new("/cli/blender")));
    // None is filled from config.
    assert_eq!(args.cache_dir.as_deref(), Some(Path::new("/config/cache")));
    assert_eq!(
        args.irradiance_blender.as_deref(),
        Some(Path::new("/config/irradiance-blender"))
    );
    assert_eq!(args.toktx.as_deref(), Some(Path::new("/config/ktx")));
}

#[test]
fn render_cli_value_wins_and_none_is_filled_from_config() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "render",
        "SuperDuperMart",
        "--config",
        config.path().to_str().unwrap(),
        "--game-root",
        "/cli/game-root",
    ])
    .unwrap();

    apply(&mut cli).unwrap();

    let CommandLine::Render(args) = &cli.command else {
        panic!("expected render command");
    };
    // CLI-provided value is not overwritten.
    assert_eq!(args.game_root.as_deref(), Some(Path::new("/cli/game-root")));
    // None is filled from config.
    assert_eq!(args.plugin.as_deref(), Some(Path::new("config-plugin.esm")));
    assert_eq!(args.blender.as_deref(), Some(Path::new("/config/blender")));
    assert_eq!(
        args.irradiance_blender.as_deref(),
        Some(Path::new("/config/irradiance-blender"))
    );
    assert_eq!(args.toktx.as_deref(), Some(Path::new("/config/ktx")));
    assert_eq!(args.cache_dir.as_deref(), Some(Path::new("/config/cache")));
}

#[test]
fn view_command_is_untouched_by_config() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "view",
        "--manifest",
        "scene.ron",
        "--config",
        config.path().to_str().unwrap(),
    ])
    .unwrap();

    apply(&mut cli).unwrap();

    let CommandLine::View(args) = &cli.command else {
        panic!("expected view command");
    };
    assert_eq!(args.manifest, Path::new("scene.ron"));
}

#[test]
fn ragdoll_lab_uses_configured_cache_without_touching_prepared_data() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "ragdoll-lab",
        "SuperDuperMart",
        "--actor",
        "00041606",
        "--config",
        config.path().to_str().unwrap(),
    ])
    .unwrap();

    apply(&mut cli).unwrap();

    let CommandLine::RagdollLab(args) = &cli.command else {
        panic!("expected ragdoll-lab command");
    };
    assert_eq!(args.cache_dir.as_deref(), Some(Path::new("/config/cache")));
}

#[test]
fn animation_zoo_uses_configured_cache_without_touching_prepared_data() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "animation-zoo",
        "SuperDuperMart",
        "--actor",
        "00041606",
        "--config",
        config.path().to_str().unwrap(),
    ])
    .unwrap();

    apply(&mut cli).unwrap();

    let CommandLine::AnimationZoo(args) = &cli.command else {
        panic!("expected animation-zoo command");
    };
    assert_eq!(args.cache_dir.as_deref(), Some(Path::new("/config/cache")));
}

#[test]
fn unreadable_config_path_errors() {
    // A directory path is a valid `Path` but fails `fs::read_to_string`.
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir_path = env::temp_dir().join(format!(
        "bevyout-config-test-dir-{}-{unique}",
        std::process::id()
    ));
    fs::create_dir_all(&dir_path).expect("create temp dir");

    let mut cli =
        Cli::try_parse_from(["bevyout", "prepare", "--config", dir_path.to_str().unwrap()])
            .unwrap();

    let error = apply(&mut cli).expect_err("directory path should not be readable as a file");
    assert!(
        format!("{error:#}").contains("could not read config file"),
        "unexpected error: {error:#}"
    );

    let _ = fs::remove_dir_all(&dir_path);
}

#[test]
fn invalid_toml_errors() {
    let config = TempConfigFile::new("this is not [valid toml");
    let mut cli = Cli::try_parse_from([
        "bevyout",
        "prepare",
        "--config",
        config.path().to_str().unwrap(),
    ])
    .unwrap();

    let error = apply(&mut cli).expect_err("invalid TOML should fail to parse");
    assert!(
        format!("{error:#}").contains("invalid TOML config file"),
        "unexpected error: {error:#}"
    );
}

// F51.4: `[world] resident_cell_limit`.

#[test]
fn resident_cell_limit_reads_the_configured_value() {
    let config = TempConfigFile::new(
        r#"
        [world]
        resident_cell_limit = 7
        "#,
    );
    assert_eq!(
        resident_cell_limit_from_path(Some(config.path().to_path_buf())),
        7
    );
}

#[test]
fn resident_cell_limit_defaults_when_no_config_file() {
    assert_eq!(
        resident_cell_limit_from_path(None),
        DEFAULT_RESIDENT_CELL_LIMIT
    );
}

#[test]
fn resident_cell_limit_defaults_when_config_file_has_no_world_section() {
    let config = TempConfigFile::new(SAMPLE_CONFIG);
    assert_eq!(
        resident_cell_limit_from_path(Some(config.path().to_path_buf())),
        DEFAULT_RESIDENT_CELL_LIMIT
    );
}

#[test]
fn resident_cell_limit_defaults_when_config_file_is_invalid_toml() {
    let config = TempConfigFile::new("this is not [valid toml");
    assert_eq!(
        resident_cell_limit_from_path(Some(config.path().to_path_buf())),
        DEFAULT_RESIDENT_CELL_LIMIT
    );
}

#[test]
fn resident_cell_limit_defaults_when_config_path_does_not_exist() {
    let nonexistent = env::temp_dir().join("bevyout-config-test-resident-limit-missing.toml");
    assert!(!nonexistent.exists());
    assert_eq!(
        resident_cell_limit_from_path(Some(nonexistent)),
        DEFAULT_RESIDENT_CELL_LIMIT
    );
}

#[test]
fn config_path_explicit_always_wins_even_if_nonexistent() {
    let nonexistent = env::temp_dir().join("bevyout-config-test-does-not-exist.toml");
    assert!(!nonexistent.exists());

    let resolved = config_path(Some(&nonexistent));

    assert_eq!(resolved.as_deref(), Some(nonexistent.as_path()));
}

#[test]
fn user_config_path_prefers_appdata_when_present() {
    let resolved = user_config_path_from_env(
        Some(OsString::from("/win/appdata")),
        Some(OsString::from("/xdg/config")),
        Some(OsString::from("/home/user")),
    );
    assert_eq!(
        resolved,
        Some(PathBuf::from("/win/appdata/bevyout/config.toml"))
    );
}

#[test]
fn user_config_path_falls_back_to_xdg_config_home_without_appdata() {
    let resolved = user_config_path_from_env(
        None,
        Some(OsString::from("/xdg/config")),
        Some(OsString::from("/home/user")),
    );
    assert_eq!(
        resolved,
        Some(PathBuf::from("/xdg/config/bevyout/config.toml"))
    );
}

#[test]
fn user_config_path_falls_back_to_home_dot_config_without_appdata_or_xdg() {
    let resolved = user_config_path_from_env(None, None, Some(OsString::from("/home/user")));
    assert_eq!(
        resolved,
        Some(PathBuf::from("/home/user/.config/bevyout/config.toml"))
    );
}

#[test]
fn user_config_path_is_none_without_any_env_var() {
    let resolved = user_config_path_from_env(None, None, None);
    assert_eq!(resolved, None);
}
