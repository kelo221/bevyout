use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "bevyout",
    about = "Fallout 3 scene preparation and Bevy viewer"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: CommandLine,
}

#[derive(Subcommand, Debug)]
pub(crate) enum CommandLine {
    Prepare(PrepareArgs),
    View(ViewArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct PrepareArgs {
    #[arg(
        long,
        default_value = r"C:\Program Files (x86)\Steam\steamapps\common\Fallout 3 goty"
    )]
    pub(crate) game_root: PathBuf,
    #[arg(long, default_value = "Fallout3.esm")]
    pub(crate) plugin: PathBuf,
    #[arg(long)]
    pub(crate) cell: String,
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    #[arg(long, default_value = ".bevyout/cache")]
    pub(crate) cache_dir: PathBuf,
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
