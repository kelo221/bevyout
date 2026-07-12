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

pub(crate) fn apply(cli: &mut Cli) -> Result<()> {
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
            if args.cache_dir.is_none() {
                args.cache_dir = config.output.cache_dir;
            }
        }
        CommandLine::View(_) => {}
    }
    Ok(())
}

fn config_path(explicit: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = explicit {
        return Some(path.to_path_buf());
    }
    let project = PathBuf::from(".bevyout/config.toml");
    if project.exists() {
        return Some(project);
    }
    let appdata = env::var_os("APPDATA")?;
    let user = PathBuf::from(appdata).join("bevyout/config.toml");
    user.exists().then_some(user)
}
