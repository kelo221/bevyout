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
    Prepare(PrepareArgs),
    Bake(BakeArgs),
    View(ViewArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct PrepareArgs {
    #[arg(long)]
    pub(crate) game_root: Option<PathBuf>,
    #[arg(long)]
    pub(crate) plugin: Option<PathBuf>,
    #[arg(long)]
    pub(crate) cell: String,
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    #[arg(long)]
    pub(crate) force: bool,
    #[arg(long)]
    pub(crate) strict: bool,
}

#[derive(Parser, Debug)]
pub(crate) struct ViewArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
}

#[derive(Parser, Debug)]
pub(crate) struct BakeArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    #[arg(long, value_enum, default_value_t = BakeQuality::Preview)]
    pub(crate) quality: BakeQuality,
    #[arg(long, value_enum, default_value_t = BakeDevice::Cpu)]
    pub(crate) device: BakeDevice,
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    #[arg(long)]
    pub(crate) toktx: Option<PathBuf>,
    #[arg(long)]
    pub(crate) force: bool,
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
    Cpu,
    Optix,
    Cuda,
    Hip,
}
