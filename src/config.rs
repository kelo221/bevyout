use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::{Cli, CommandLine};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ConfigFile {
    fallout3: FalloutConfig,
    tools: ToolsConfig,
    output: OutputConfig,
    world: WorldConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct FalloutConfig {
    game_root: Option<PathBuf>,
    plugin: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ToolsConfig {
    blender: Option<PathBuf>,
    irradiance_blender: Option<PathBuf>,
    ktx: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct OutputConfig {
    cache_dir: Option<PathBuf>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct WorldConfig {
    /// Issue #51's predictive neighbor preloader: how many cells (active +
    /// preloaded neighbors) may be resident at once before the farthest
    /// (by door-graph distance) is evicted.
    resident_cell_limit: Option<usize>,
}

/// Default for `[world] resident_cell_limit` when unset or no config file is
/// found (F51.4).
pub(crate) const DEFAULT_RESIDENT_CELL_LIMIT: usize = 4;

pub fn apply(cli: &mut Cli) -> Result<()> {
    let Some(path) = config_path(cli.config.as_deref()) else {
        return Ok(());
    };
    let text = fs::read_to_string(&path)
        .with_context(|| format!("could not read config file {}", path.display()))?;
    let config: ConfigFile = toml::from_str(&text)
        .with_context(|| format!("invalid TOML config file {}", path.display()))?;
    match &mut cli.command {
        CommandLine::Prepare(args) => {
            if args.game_root.is_none() {
                args.game_root = config.fallout3.game_root;
            }
            if args.plugin.is_none() {
                args.plugin = config.fallout3.plugin;
            }
            if args.cache_dir.is_none() {
                args.cache_dir = config.output.cache_dir;
            }
            if args.blender.is_none() {
                args.blender = config.tools.blender;
            }
            if args.toktx.is_none() {
                args.toktx = config.tools.ktx;
            }
        }
        CommandLine::Bake(args) => {
            if args.cache_dir.is_none() {
                args.cache_dir = config.output.cache_dir.clone();
            }
            if args.blender.is_none() {
                args.blender = config.tools.blender;
            }
            if args.irradiance_blender.is_none() {
                args.irradiance_blender = config.tools.irradiance_blender;
            }
            if args.toktx.is_none() {
                args.toktx = config.tools.ktx;
            }
        }
        CommandLine::Render(args) => {
            if args.game_root.is_none() {
                args.game_root = config.fallout3.game_root.clone();
            }
            if args.plugin.is_none() {
                args.plugin = config.fallout3.plugin.clone();
            }
            if args.blender.is_none() {
                args.blender = config.tools.blender.clone();
            }
            if args.irradiance_blender.is_none() {
                args.irradiance_blender = config.tools.irradiance_blender.clone();
            }
            if args.toktx.is_none() {
                args.toktx = config.tools.ktx.clone();
            }
            if args.cache_dir.is_none() {
                args.cache_dir = config.output.cache_dir.clone();
            }
        }
        CommandLine::View(_) | CommandLine::Script(_) => {}
        CommandLine::NifConvert(args) => {
            if args.game_root.is_none() {
                args.game_root = config.fallout3.game_root.clone();
            }
        }
        CommandLine::Cells(args) => {
            if args.game_root.is_none() {
                args.game_root = config.fallout3.game_root.clone();
            }
            if args.plugin.is_none() {
                args.plugin = config.fallout3.plugin.clone();
            }
        }
        CommandLine::Report(args) => {
            if args.game_root.is_none() {
                args.game_root = config.fallout3.game_root.clone();
            }
            if args.plugin.is_none() {
                args.plugin = config.fallout3.plugin.clone();
            }
        }
    }
    Ok(())
}

/// Resolves `[world] resident_cell_limit` for the viewer (F51.4).
///
/// Unlike `apply`, this does not thread through `Cli`'s `--config` override:
/// `ViewArgs`/`RenderArgs` have no config-plumbing seam today (`apply`'s
/// `CommandLine::View(_) => {}` arm is a deliberate no-op, see the test
/// `view_command_is_untouched_by_config` below), and wiring one up touches
/// `src/cli.rs`, which is out of scope for this slice. This still finds the
/// same project (`.bevyout/config.toml`) or user config file `apply` would,
/// just without the explicit `--config` override; a missing file, unreadable
/// file, invalid TOML, or unset key all fall back to
/// `DEFAULT_RESIDENT_CELL_LIMIT` rather than erroring, since `apply` already
/// validates the config file's TOML syntax up front for every command
/// (including `view`) before this ever runs.
pub(crate) fn resident_cell_limit() -> usize {
    resident_cell_limit_from_path(config_path(None))
}

fn resident_cell_limit_from_path(path: Option<PathBuf>) -> usize {
    let Some(path) = path else {
        return DEFAULT_RESIDENT_CELL_LIMIT;
    };
    let Ok(text) = fs::read_to_string(&path) else {
        return DEFAULT_RESIDENT_CELL_LIMIT;
    };
    let Ok(config) = toml::from_str::<ConfigFile>(&text) else {
        return DEFAULT_RESIDENT_CELL_LIMIT;
    };
    config
        .world
        .resident_cell_limit
        .unwrap_or(DEFAULT_RESIDENT_CELL_LIMIT)
}

fn config_path(explicit: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = explicit {
        return Some(path.to_path_buf());
    }
    let project = PathBuf::from(".bevyout/config.toml");
    if project.exists() {
        return Some(project);
    }
    let user = user_config_path()?;
    user.exists().then_some(user)
}

/// User-level config path: `%APPDATA%` on Windows, else
/// `$XDG_CONFIG_HOME/bevyout/config.toml`, else `$HOME/.config/bevyout/config.toml`.
fn user_config_path() -> Option<PathBuf> {
    user_config_path_from_env(
        env::var_os("APPDATA"),
        env::var_os("XDG_CONFIG_HOME"),
        env::var_os("HOME"),
    )
}

fn user_config_path_from_env(
    appdata: Option<std::ffi::OsString>,
    xdg_config_home: Option<std::ffi::OsString>,
    home: Option<std::ffi::OsString>,
) -> Option<PathBuf> {
    if let Some(appdata) = appdata {
        return Some(PathBuf::from(appdata).join("bevyout/config.toml"));
    }
    if let Some(xdg_config_home) = xdg_config_home {
        return Some(PathBuf::from(xdg_config_home).join("bevyout/config.toml"));
    }
    Some(PathBuf::from(home?).join(".config/bevyout/config.toml"))
}

#[cfg(test)]
mod tests {
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
}
