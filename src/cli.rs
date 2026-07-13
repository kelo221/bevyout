use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "bevyout",
    about = "Fallout 3 scene preparation and Bevy viewer"
)]
pub struct Cli {
    /// Optional project/user configuration file.
    #[arg(long, global = true)]
    pub(crate) config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: CommandLine,
}

#[derive(Subcommand, Debug)]
pub enum CommandLine {
    /// Extract a Fallout cell, stage its assets, and create a Bevy manifest.
    #[command(name = "prepare")]
    Prepare(PrepareArgs),
    /// Render or bake a prepared scene.
    #[command(name = "bake")]
    Bake(BakeArgs),
    /// Render a prepared scene selected by GECK EditorID.
    #[command(name = "render")]
    Render(RenderArgs),
    /// Open a prepared scene manifest in the Bevy viewer.
    #[command(name = "view")]
    View(ViewArgs),
}

#[derive(Parser, Debug)]
pub struct PrepareArgs {
    /// GECK EditorID, or an eight-digit hexadecimal FormID.
    #[arg(value_name = "EDITOR_ID", conflicts_with = "cell")]
    pub(crate) selector: Option<String>,
    /// Fallout 3 installation directory (normally supplied by config.toml).
    #[arg(long)]
    pub(crate) game_root: Option<PathBuf>,
    /// Plugin filename under Data, or an absolute plugin path.
    #[arg(long)]
    pub(crate) plugin: Option<PathBuf>,
    /// Legacy hexadecimal cell FormID input.
    #[arg(long, hide = true, conflicts_with = "selector")]
    pub(crate) cell: Option<String>,
    /// Blender executable path.
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    /// Output cache directory.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Refresh the manifest and scene metadata.
    #[arg(long)]
    pub(crate) force: bool,
    /// Rebuild cached NIF-to-GLB assets even when the cache key is unchanged.
    #[arg(long)]
    pub(crate) rebuild_assets: bool,
    /// Fail instead of recording recoverable asset diagnostics.
    #[arg(long)]
    pub(crate) strict: bool,
}

#[derive(Parser, Debug)]
pub struct ViewArgs {
    /// Prepared scene manifest to open.
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    /// Skip BoxDDD collider construction for render-only performance testing.
    #[arg(long)]
    pub(crate) disable_physics: bool,
    /// Exit after this many seconds; useful for bounded trace captures.
    #[arg(long)]
    pub(crate) trace_seconds: Option<f32>,
}

#[derive(Parser, Debug)]
pub struct RenderArgs {
    /// GECK EditorID, or an eight-digit hexadecimal FormID.
    #[arg(value_name = "EDITOR_ID")]
    pub(crate) selector: String,
    /// Fallout 3 installation directory used if render needs to prepare the cell.
    #[arg(long, hide = true)]
    pub(crate) game_root: Option<PathBuf>,
    /// Plugin filename used if render needs to prepare the cell.
    #[arg(long, hide = true)]
    pub(crate) plugin: Option<PathBuf>,
    /// Blender executable used if render needs to prepare the cell.
    #[arg(long, hide = true)]
    pub(crate) blender: Option<PathBuf>,
    /// Blender 4.5 LTS executable used if render needs to bake irradiance.
    #[arg(long, hide = true)]
    pub(crate) irradiance_blender: Option<PathBuf>,
    /// KTX-Software executable used if render needs to bake irradiance.
    #[arg(long, hide = true)]
    pub(crate) toktx: Option<PathBuf>,
    /// Prepared scene cache directory; defaults to .bevyout/cache.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Skip BoxDDD collider construction for render-only performance testing.
    #[arg(long)]
    pub(crate) disable_physics: bool,
    /// Exit after this many seconds; useful for bounded trace captures.
    #[arg(long)]
    pub(crate) trace_seconds: Option<f32>,
}

#[derive(Parser, Debug)]
pub struct BakeArgs {
    /// Prepared scene manifest to bake. The final bake metadata is written back to it.
    #[arg(long, conflicts_with = "selector")]
    pub(crate) manifest: Option<PathBuf>,
    /// GECK EditorID, or an eight-digit hexadecimal FormID.
    #[arg(value_name = "EDITOR_ID", conflicts_with = "manifest")]
    pub(crate) selector: Option<String>,
    /// Prepared scene cache directory used by selector-based baking.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Fast Eevee preview or Blender 4.5 irradiance-volume bake.
    #[arg(long, value_enum, default_value_t = BakeQuality::Irradiance)]
    pub(crate) quality: BakeQuality,
    /// World-space distance between irradiance probes, in metres.
    #[arg(
        long,
        default_value_t = 8.0,
        value_parser = parse_irradiance_spacing_meters
    )]
    pub(crate) irradiance_spacing_meters: f32,
    /// Eevee irradiance samples per probe.
    #[arg(
        long,
        default_value_t = 64,
        value_parser = parse_irradiance_samples
    )]
    pub(crate) irradiance_samples: u32,
    /// World-space size of material-compatible static geometry batches, in metres.
    #[arg(
        long,
        default_value_t = 64.0,
        value_parser = parse_static_batch_chunk_meters
    )]
    pub(crate) static_batch_chunk_meters: f32,
    /// Blender executable path.
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    /// Blender 4.5 LTS executable used only for irradiance-volume baking.
    #[arg(long)]
    pub(crate) irradiance_blender: Option<PathBuf>,
    /// KTX-Software `ktx.exe` or legacy `toktx.exe` path.
    #[arg(long)]
    pub(crate) toktx: Option<PathBuf>,
    /// Legacy compatibility flag; bake already replaces existing outputs.
    #[arg(long, hide = true)]
    pub(crate) force: bool,
    /// Keep the generated Blender job, script, result, and Blender cache files.
    #[arg(long)]
    pub(crate) keep_intermediate: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BakeQuality {
    /// Fast Eevee lighting preview; does not produce a baked-GI manifest.
    Preview,
    /// Bake one Blender 4.5 Eevee irradiance volume for the cell.
    Irradiance,
}

fn parse_static_batch_chunk_meters(value: &str) -> Result<f32, String> {
    let value = value
        .parse::<f32>()
        .map_err(|error| format!("invalid batch chunk size: {error}"))?;
    if value.is_finite() && (8.0..=256.0).contains(&value) {
        Ok(value)
    } else {
        Err("batch chunk size must be between 8 and 256 metres".into())
    }
}

fn parse_irradiance_spacing_meters(value: &str) -> Result<f32, String> {
    let value = value
        .parse::<f32>()
        .map_err(|error| format!("invalid irradiance spacing: {error}"))?;
    if value.is_finite() && (2.0..=32.0).contains(&value) {
        Ok(value)
    } else {
        Err("irradiance spacing must be between 2 and 32 metres".into())
    }
}

fn parse_irradiance_samples(value: &str) -> Result<u32, String> {
    let value = value
        .parse::<u32>()
        .map_err(|error| format!("invalid irradiance sample count: {error}"))?;
    if (1..=512).contains(&value) {
        Ok(value)
    } else {
        Err("irradiance samples must be between 1 and 512".into())
    }
}

#[cfg(test)]
#[path = "cli/tests/mod.rs"]
mod tests;
