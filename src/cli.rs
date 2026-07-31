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
    /// Compare one prepared actor ragdoll in an isolated physics laboratory.
    #[command(name = "ragdoll-lab")]
    RagdollLab(RagdollLabArgs),
    /// Cycle every compatible prepared animation on one isolated actor.
    #[command(name = "animation-zoo")]
    AnimationZoo(AnimationZooArgs),
    /// Generate a deterministic compatibility report for a plugin's records.
    #[command(name = "report")]
    Report(ReportArgs),
    /// List cells discovered in the resolved Fallout plugin load order.
    #[command(name = "cells")]
    Cells(CellsArgs),
    /// Run deterministic Gamebryo-style console scripts.
    #[command(name = "script")]
    Script(ScriptArgs),
    /// Experimentally convert one FO3/FNV NIF 20.2.0.7 asset to a self-contained GLB.
    #[command(name = "nif-convert")]
    NifConvert(NifConvertArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(group(
    clap::ArgGroup::new("source")
        .required(true)
        .args(["input", "asset"])
))]
pub struct NifConvertArgs {
    /// Read a NIF directly from this filesystem path.
    #[arg(long, value_name = "FILE", conflicts_with = "asset")]
    pub(crate) input: Option<PathBuf>,
    /// Resolve a Data-relative NIF path from loose files or Fallout BSAs.
    #[arg(long, value_name = "meshes/PATH.nif", conflicts_with = "input")]
    pub(crate) asset: Option<String>,
    /// Write the self-contained binary glTF here.
    #[arg(long, value_name = "FILE.glb")]
    pub(crate) output: PathBuf,
    /// Fallout 3 / New Vegas installation root; required by --asset and used for textures.
    #[arg(long, value_name = "DIR")]
    pub(crate) game_root: Option<PathBuf>,
    /// Optional authored-collision sidecar output path.
    #[arg(long, value_name = "FILE.physics.json.gz")]
    pub(crate) physics_output: Option<PathBuf>,
    /// Optional deterministic JSON conversion report.
    #[arg(long, value_name = "FILE.json")]
    pub(crate) report: Option<PathBuf>,
    /// Vertex-color conversion policy.
    #[arg(long, value_enum, default_value_t = NifConversionMode::Preserve)]
    pub(crate) conversion: NifConversionMode,
    /// Emit the usable subset while reporting unsupported or missing content.
    #[arg(long)]
    pub(crate) allow_lossy: bool,
    /// Replace existing output files.
    #[arg(long)]
    pub(crate) force: bool,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NifConversionMode {
    Preserve,
    QuickAo,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PrepareConverter {
    Blender,
    Native,
}

#[derive(ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ActorAnimationConverter {
    #[default]
    Disabled,
    Native,
    Blender,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RagdollLabBackend {
    Avian,
    Boxddd,
}

impl std::fmt::Display for RagdollLabBackend {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Avian => "avian",
            Self::Boxddd => "boxddd",
        })
    }
}

impl Default for PrepareConverter {
    fn default() -> Self {
        match crate::converter_policy::resolve_converter_backend(None) {
            crate::converter_policy::ConverterBackend::Native => Self::Native,
            crate::converter_policy::ConverterBackend::Blender => Self::Blender,
        }
    }
}

impl std::fmt::Display for PrepareConverter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let backend = match self {
            Self::Blender => crate::converter_policy::ConverterBackend::Blender,
            Self::Native => crate::converter_policy::ConverterBackend::Native,
        };
        formatter.write_str(backend.as_str())
    }
}

impl std::fmt::Display for ActorAnimationConverter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let backend = match self {
            Self::Disabled => crate::converter_policy::ActorAnimationBackend::Disabled,
            Self::Native => crate::converter_policy::ActorAnimationBackend::Native,
            Self::Blender => crate::converter_policy::ActorAnimationBackend::Blender,
        };
        formatter.write_str(backend.as_str())
    }
}

impl ActorAnimationConverter {
    pub(crate) const fn backend(self) -> crate::converter_policy::ActorAnimationBackend {
        match self {
            Self::Disabled => crate::converter_policy::ActorAnimationBackend::Disabled,
            Self::Native => crate::converter_policy::ActorAnimationBackend::Native,
            Self::Blender => crate::converter_policy::ActorAnimationBackend::Blender,
        }
    }
}

#[derive(Parser, Debug)]
pub struct ScriptArgs {
    #[command(subcommand)]
    pub(crate) command: ScriptCommand,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ScriptCommand {
    /// Run a .bscript file through the headless console harness.
    #[command(name = "run")]
    Run(ScriptRunArgs),
}

#[derive(Parser, Debug)]
pub struct ScriptRunArgs {
    /// Console script containing one command per line.
    #[arg(value_name = "FILE")]
    pub(crate) file: PathBuf,
    /// Explicitly select the deterministic headless runner (currently the only script runtime).
    #[arg(long)]
    pub(crate) headless: bool,
    /// Write the stable JSONL transcript to this file instead of stdout.
    #[arg(long)]
    pub(crate) transcript: Option<PathBuf>,
    /// Continue after structured command or expectation failures.
    #[arg(long)]
    pub(crate) keep_going: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct PrepareArgs {
    /// GECK EditorID, or an eight-digit hexadecimal FormID. May be repeated to
    /// prepare several cells in one run.
    #[arg(value_name = "EDITOR_ID", conflicts_with_all = ["cell", "all"])]
    pub(crate) selectors: Vec<String>,
    /// Prepare every cell in the resolved plugin chain.
    #[arg(
        long,
        conflicts_with_all = ["all_interiors", "worldspace", "selectors", "cell"]
    )]
    pub(crate) all: bool,
    /// Prepare every interior cell. Combinable with `--worldspace` and
    /// explicit selectors.
    #[arg(long)]
    pub(crate) all_interiors: bool,
    /// Prepare every cell belonging to this worldspace (EditorID or FormID).
    /// Combinable with `--all-interiors` and explicit selectors.
    #[arg(long, value_name = "WORLDSPACE")]
    pub(crate) worldspace: Option<String>,
    /// Print the resolved cell selection (`formid<TAB>editor_id` per line,
    /// sorted) and exit before any extraction or Blender work.
    #[arg(long, conflicts_with = "check_fingerprints")]
    pub(crate) list_only: bool,
    /// Report-only: validate each selected cell's recorded plugin/converter/
    /// physics/prepare-pipeline fingerprints against the current toolchain
    /// and print per-cell status, without preparing anything. Exits with a
    /// nonzero status if any selected cell's fingerprints are stale.
    #[arg(long)]
    pub(crate) check_fingerprints: bool,
    /// Fallout 3 installation directory (normally supplied by config.toml).
    #[arg(long)]
    pub(crate) game_root: Option<PathBuf>,
    /// Plugin filename under Data, or an absolute plugin path.
    #[arg(long)]
    pub(crate) plugin: Option<PathBuf>,
    /// Legacy hexadecimal cell FormID input.
    #[arg(long, hide = true, conflicts_with = "selectors")]
    pub(crate) cell: Option<String>,
    /// Blender executable path.
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    /// NIF-to-GLB backend. Native is the default; use `blender` for compatibility.
    #[arg(long, value_enum, default_value_t = PrepareConverter::default())]
    pub(crate) converter: PrepareConverter,
    /// External-KF clip-pack backend. Disabled by default; select `native` to
    /// decode KF files with Nifty or `blender` for the NIFTools comparison path.
    #[arg(long, value_enum, default_value_t = ActorAnimationConverter::default())]
    pub(crate) actor_animation_converter: ActorAnimationConverter,
    /// KTX-Software `ktx.exe` path used for prepared point-shadow cubemaps.
    #[arg(long)]
    pub(crate) toktx: Option<PathBuf>,
    /// Cubemap face resolution for prepared static point shadows.
    #[arg(long, default_value_t = 512, value_parser = parse_shadow_resolution)]
    pub(crate) shadow_resolution: u32,
    /// Rebuild prepared point-shadow cubemaps even when their fingerprint matches.
    #[arg(long)]
    pub(crate) rebuild_shadows: bool,
    /// Rebuild prepared reflection-probe cubemaps even when their fingerprint matches.
    #[arg(long)]
    pub(crate) rebuild_reflection_probes: bool,
    /// Output cache directory.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Authored Yarn dialogue source to include in the prepared bundle. May
    /// be repeated; no dialogue sources are discovered implicitly.
    #[arg(long = "dialogue-source", value_name = "PATH")]
    pub(crate) dialogue_sources: Vec<PathBuf>,
    /// Refresh the manifest and scene metadata.
    #[arg(long)]
    pub(crate) force: bool,
    /// Rebuild cached NIF-to-GLB assets even when the cache key is unchanged.
    #[arg(long)]
    pub(crate) rebuild_assets: bool,
    /// Fail instead of recording recoverable asset diagnostics.
    #[arg(long)]
    pub(crate) strict: bool,
    /// Number of cells to prepare concurrently in a batch run. Defaults to
    /// the machine's available parallelism.
    #[arg(long, value_name = "N")]
    pub(crate) jobs: Option<usize>,
    /// Retry only cells currently recorded `failed` in the resumable job
    /// manifest, intersected with any other selector given. Alone (no
    /// `--all`/`--all-interiors`/`--worldspace`/selectors), retries every
    /// failed cell recorded in the manifest.
    #[arg(long)]
    pub(crate) retry_failed: bool,
}

fn parse_shadow_resolution(value: &str) -> Result<u32, String> {
    match value.parse::<u32>() {
        Ok(value @ (128 | 256 | 512)) => Ok(value),
        _ => Err("shadow resolution must be 128, 256, or 512".into()),
    }
}

fn parse_day_night_cycle_seconds(value: &str) -> Result<f32, String> {
    let value = value
        .parse::<f32>()
        .map_err(|error| format!("invalid day/night cycle duration: {error}"))?;
    if value.is_finite() && (1.0..=86_400.0).contains(&value) {
        Ok(value)
    } else {
        Err("day/night cycle duration must be between 1 and 86400 seconds".into())
    }
}

#[derive(Parser, Debug)]
pub struct ViewArgs {
    /// Prepared scene manifest to open.
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    /// Skip BoxDDD collider construction for render-only performance testing.
    #[arg(long)]
    pub(crate) disable_physics: bool,
    /// Enable the bounded native realtime point-shadow pass at startup.
    #[arg(long)]
    pub(crate) realtime_shadows: bool,
    /// Exit after this many seconds; useful for bounded trace captures.
    #[arg(long)]
    pub(crate) trace_seconds: Option<f32>,
    /// Preview a complete Fallout day in this many real seconds.
    #[arg(long, value_name = "SECONDS", value_parser = parse_day_night_cycle_seconds)]
    pub(crate) day_night_cycle_seconds: Option<f32>,
    /// Expose the running viewer to a local agent through Bevy Remote Protocol.
    #[arg(long)]
    pub(crate) agent_bridge: bool,
    /// Loopback HTTP port used by the agent bridge.
    #[arg(long, default_value_t = 15_702, requires = "agent_bridge")]
    pub(crate) agent_port: u16,
    /// Open the window without taking OS input focus or raising it above
    /// other windows. Automatically implied by `--agent-bridge`, so
    /// automated/agent launches never interrupt whatever the human is doing.
    #[arg(long)]
    pub(crate) unfocused: bool,
    /// Load this save slot at startup and apply it to the launch cell.
    #[arg(long, value_name = "SLOT")]
    pub(crate) save_slot: Option<String>,
}

#[derive(Parser, Debug)]
pub struct RagdollLabArgs {
    /// Prepared scene GECK EditorID or eight-digit hexadecimal FormID.
    #[arg(value_name = "EDITOR_ID")]
    pub(crate) selector: String,
    /// Actor reference FormID from the prepared scene.
    #[arg(long, value_name = "FORM_ID")]
    pub(crate) actor: String,
    /// Physics solver used only by the isolated laboratory.
    #[arg(long, value_enum, default_value_t = RagdollLabBackend::Avian)]
    pub(crate) backend: RagdollLabBackend,
    /// Prepared scene cache directory; defaults to .bevyout/cache.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Expose the laboratory to a local agent through Bevy Remote Protocol.
    #[arg(long)]
    pub(crate) agent_bridge: bool,
    /// Loopback HTTP port used by the agent bridge.
    #[arg(long, default_value_t = 15_702, requires = "agent_bridge")]
    pub(crate) agent_port: u16,
    /// Exit after this many seconds; useful for bounded solver captures.
    #[arg(long)]
    pub(crate) trace_seconds: Option<f32>,
}

#[derive(Parser, Debug)]
pub struct AnimationZooArgs {
    /// Prepared scene GECK EditorID or eight-digit hexadecimal FormID.
    #[arg(value_name = "EDITOR_ID")]
    pub(crate) selector: String,
    /// Actor reference FormID from the prepared scene.
    #[arg(long, value_name = "FORM_ID")]
    pub(crate) actor: String,
    /// Prepared scene cache directory; defaults to .bevyout/cache.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Normalized clip name to select first.
    #[arg(long, value_name = "NAME")]
    pub(crate) start_clip: Option<String>,
    /// Exit after this many seconds; useful for bounded captures.
    #[arg(long)]
    pub(crate) trace_seconds: Option<f32>,
    /// Expose the zoo to a local agent through Bevy Remote Protocol.
    #[arg(long)]
    pub(crate) agent_bridge: bool,
    /// Loopback HTTP port used by the agent bridge.
    #[arg(long, default_value_t = 15_702, requires = "agent_bridge")]
    pub(crate) agent_port: u16,
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
    /// NIF-to-GLB backend used if render needs to prepare or refresh the cell.
    #[arg(long, value_enum, default_value_t = PrepareConverter::default())]
    pub(crate) converter: PrepareConverter,
    /// Legacy compatibility option; Rust irradiance baking does not invoke Blender.
    #[arg(long, hide = true)]
    pub(crate) irradiance_blender: Option<PathBuf>,
    /// KTX-Software executable used if render needs to bake irradiance.
    #[arg(long, hide = true)]
    pub(crate) toktx: Option<PathBuf>,
    /// Cubemap face resolution used if render needs to prepare static point shadows.
    #[arg(long, default_value_t = 512, value_parser = parse_shadow_resolution)]
    pub(crate) shadow_resolution: u32,
    /// Rebuild prepared point-shadow cubemaps if render refreshes the cell.
    #[arg(long)]
    pub(crate) rebuild_shadows: bool,
    /// Rebuild prepared reflection probes if render refreshes the cell.
    #[arg(long)]
    pub(crate) rebuild_reflection_probes: bool,
    /// Enable the bounded native realtime point-shadow pass at startup.
    #[arg(long)]
    pub(crate) realtime_shadows: bool,
    /// Prepared scene cache directory; defaults to .bevyout/cache.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Skip BoxDDD collider construction for render-only performance testing.
    #[arg(long)]
    pub(crate) disable_physics: bool,
    /// Exit after this many seconds; useful for bounded trace captures.
    #[arg(long)]
    pub(crate) trace_seconds: Option<f32>,
    /// Preview a complete Fallout day in this many real seconds.
    #[arg(long, value_name = "SECONDS", value_parser = parse_day_night_cycle_seconds)]
    pub(crate) day_night_cycle_seconds: Option<f32>,
    /// Expose the running viewer to a local agent through Bevy Remote Protocol.
    #[arg(long)]
    pub(crate) agent_bridge: bool,
    /// Loopback HTTP port used by the agent bridge.
    #[arg(long, default_value_t = 15_702, requires = "agent_bridge")]
    pub(crate) agent_port: u16,
}

#[derive(Parser, Debug)]
pub struct BakeArgs {
    /// Prepared scene manifest to bake. The final bake metadata is written back to it.
    #[arg(long, conflicts_with = "selector")]
    pub(crate) manifest: Option<PathBuf>,
    /// GECK EditorID, or an eight-digit hexadecimal FormID.
    #[arg(value_name = "EDITOR_ID", conflicts_with = "manifest")]
    pub(crate) selector: Option<String>,
    /// Bake every interior cell in the prepared cell catalogue
    /// (`<cache_dir>/cellmap.ron`, written by `prepare --all-interiors`).
    /// Resumable: per-cell progress is recorded in
    /// `<cache_dir>/bake_jobs.ron`, and already-baked, still-valid cells
    /// are skipped.
    #[arg(long, conflicts_with_all = ["selector", "manifest"])]
    pub(crate) all_interiors: bool,
    /// Retry only cells currently recorded `failed` in the resumable bake
    /// job manifest, intersected with `--all-interiors` when both are
    /// given. Alone, retries every failed cell recorded in the manifest.
    #[arg(long, conflicts_with_all = ["selector", "manifest"])]
    pub(crate) retry_failed: bool,
    /// Prepared scene cache directory used by selector-based and batch baking.
    #[arg(long)]
    pub(crate) cache_dir: Option<PathBuf>,
    /// Fast Blender preview or Rust CPU irradiance-volume bake.
    #[arg(long, value_enum, default_value_t = BakeQuality::Irradiance)]
    pub(crate) quality: BakeQuality,
    /// World-space distance between irradiance probes, in metres.
    #[arg(
        long,
        default_value_t = 8.0,
        value_parser = parse_irradiance_spacing_meters
    )]
    pub(crate) irradiance_spacing_meters: f32,
    /// Deterministic CPU hemisphere samples per probe face.
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
    /// Blender executable path, used only by --quality preview.
    #[arg(long)]
    pub(crate) blender: Option<PathBuf>,
    /// Legacy compatibility option; accepted but ignored by the Rust baker.
    #[arg(long)]
    pub(crate) irradiance_blender: Option<PathBuf>,
    /// Unified KTX-Software `ktx.exe` path (legacy option name).
    #[arg(long)]
    pub(crate) toktx: Option<PathBuf>,
    /// Re-bake every selected cell in a batch run even when its recorded
    /// bake is still valid. Single-cell bake already replaces existing
    /// outputs, so outside `--all-interiors`/`--retry-failed` this remains
    /// the legacy no-op it always was.
    #[arg(long)]
    pub(crate) force: bool,
    /// Keep raw irradiance atlas slices, or Blender preview intermediates.
    #[arg(long)]
    pub(crate) keep_intermediate: bool,
}

#[derive(Parser, Debug)]
pub struct ReportArgs {
    /// Fallout 3 installation directory (normally supplied by config.toml).
    #[arg(long)]
    pub(crate) game_root: Option<PathBuf>,
    /// Plugin filename under Data, or an absolute plugin path.
    #[arg(long)]
    pub(crate) plugin: Option<PathBuf>,
    /// Directory to write the compatibility report and summary into.
    /// Defaults to `.bevyout/reports` (gitignored).
    #[arg(long)]
    pub(crate) out_dir: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct CellsArgs {
    /// Fallout 3 installation directory (normally supplied by config.toml).
    #[arg(long)]
    pub(crate) game_root: Option<PathBuf>,
    /// Plugin filename under Data, or an absolute plugin path.
    #[arg(long)]
    pub(crate) plugin: Option<PathBuf>,
    /// Only print interior cells.
    #[arg(long)]
    pub(crate) interiors_only: bool,
    /// Emit the deterministic `CellMap` RON artifact (grid coordinates,
    /// worldspace membership, content-set-wide door connectivity) instead of
    /// the default cell catalogue.
    #[arg(long)]
    pub(crate) map: bool,
    /// With `--map`, write the RON artifact to this path instead of stdout.
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BakeQuality {
    /// Fast Eevee lighting preview; does not produce a baked-GI manifest.
    Preview,
    /// Bake one deterministic Rust CPU irradiance volume for the cell.
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
