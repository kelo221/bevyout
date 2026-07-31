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
        CommandLine::RagdollLab(args) => {
            if args.cache_dir.is_none() {
                args.cache_dir = config.output.cache_dir.clone();
            }
        }
        CommandLine::AnimationZoo(args) => {
            if args.cache_dir.is_none() {
                args.cache_dir = config.output.cache_dir.clone();
            }
        }
        CommandLine::View(_)
        | CommandLine::Script(_)
        | CommandLine::ExteriorConversionReport(_)
        | CommandLine::ExteriorCatalog(_) => {}
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
#[path = "config/tests.rs"]
mod tests;
