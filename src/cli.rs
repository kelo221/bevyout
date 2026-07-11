use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "bevyout",
    about = "Fallout 3 scene preparation and Bevy viewer"
)]
pub(crate) struct Cli {
    /// Optional project/user configuration file.
    #[arg(long, global = true)]
    pub(crate) config: Option<PathBuf>,
    #[command(subcommand)]
    pub(crate) command: CommandLine,
}

#[derive(Subcommand, Debug)]
pub(crate) enum CommandLine {
    /// Extract a Fallout cell, stage its assets, and create a Bevy manifest.
    #[command(name = "prepare")]
    Prepare(PrepareArgs),
    /// Render a preview or bake lightmaps for a prepared scene.
    #[command(name = "bake")]
    Bake(BakeArgs),
    /// Open a prepared scene manifest in the Bevy viewer.
    #[command(name = "view")]
    View(ViewArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct PrepareArgs {
    /// Fallout 3 installation directory (normally supplied by config.toml).
    #[arg(long)]
    pub(crate) game_root: Option<PathBuf>,
    /// Plugin filename under Data, or an absolute plugin path.
    #[arg(long)]
    pub(crate) plugin: Option<PathBuf>,
    /// Cell FormID, in hexadecimal.
    #[arg(long)]
    pub(crate) cell: String,
    /// Blender executable path.
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    /// Output cache directory.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Rebuild assets even when the cache key is unchanged.
    #[arg(long)]
    pub(crate) force: bool,
    /// Fail instead of recording recoverable asset diagnostics.
    #[arg(long)]
    pub(crate) strict: bool,
}

#[derive(Parser, Debug)]
pub(crate) struct ViewArgs {
    /// Prepared scene manifest to open.
    #[arg(long)]
    pub(crate) manifest: PathBuf,
}

#[derive(Parser, Debug)]
pub(crate) struct BakeArgs {
    /// Prepared scene manifest to bake. The final bake metadata is written back to it.
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    /// Preview, quick direct-light bake, or final indirect-light bake.
    #[arg(long, value_enum, default_value_t = BakeQuality::Preview)]
    pub(crate) quality: BakeQuality,
    /// Cycles device for quick/final modes; preview always uses Eevee.
    #[arg(long, value_enum, default_value_t = BakeDevice::Cpu)]
    pub(crate) device: BakeDevice,
    /// Blender executable path.
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    /// KTX-Software `ktx.exe` or legacy `toktx.exe` path.
    #[arg(long)]
    pub(crate) toktx: Option<PathBuf>,
    /// Replace an existing baked output directory.
    #[arg(long)]
    pub(crate) force: bool,
    /// Keep the generated Blender job, script, result, and EXR files.
    #[arg(long)]
    pub(crate) keep_intermediate: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum BakeQuality {
    /// Fast Eevee lighting preview; does not produce a lightmap manifest.
    Preview,
    /// Low-resolution direct-light bake for static architecture and large surfaces.
    Quick,
    /// Full-resolution direct and indirect Cycles bake for all mesh objects.
    Final,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum BakeDevice {
    /// Run Cycles on the CPU.
    Cpu,
    /// Run Cycles on an NVIDIA GPU through OptiX.
    Optix,
    /// Run Cycles on an NVIDIA GPU through CUDA.
    Cuda,
    /// Run Cycles on a supported AMD GPU through HIP.
    Hip,
}
