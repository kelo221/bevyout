use super::*;
use crate::cli::progress::ProgressReporter;
use crate::vsa::catalog::{CellCatalog, build_cell_map};
use crate::vsa::paths::CellSelector;
use bevyout_core::actor::{
    ActorAppearanceAvailability, ActorAssemblyBlueprint as CoreActorAssemblyBlueprint,
    ActorAttachmentPoint, ActorFallbackLevel, ActorFallbackReason, ActorKind, ActorMeshRole,
    ActorWeaponCandidate, AssembledApparel, AssembledMeshPart, FaceGenAvailability,
    canonicalize_mesh_parts, hair_visible, resolve_actor_fallback, resolve_actor_root_scale,
    select_starting_weapon,
};
use bevyout_core::actor_state::{ActorSkill, ActorValue};
use bevyout_core::facegen::{FaceGenAssetKind, FaceGenDiagnostic, FaceGenRaw};
use bevyout_core::image_space::IMAGE_SPACE_MODIFIER_CATALOG_REVISION;
use bevyout_core::manifest::CellInfo;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Dispatches `prepare`: a single legacy selector goes straight through
/// `prepare_single` below; `--all`/`--all-interiors`/`--worldspace`/
/// `--all-exteriors`, `--exterior-radius`, `--list-only`, `--check-fingerprints`,
/// `--retry-failed`, or more than one positional selector build a lightweight
/// cell catalogue and resolve the batch through `resolve_selection`.
pub fn prepare(args: PrepareArgs) -> Result<()> {
    let progress = ProgressReporter::new(args.progress.mode);
    let mut explicit = args.selectors.clone();
    explicit.extend(args.cell.clone());

    let is_batch = prepare_requires_batch(&args, explicit.len());

    if !is_batch {
        let selector_input = explicit
            .into_iter()
            .next()
            .context("provide a GECK EditorID/FormID selector or legacy --cell, or pass --all/--all-interiors/--all-exteriors/--worldspace/--exterior-radius/--retry-failed")?;
        return prepare_single(args, selector_input, &progress);
    }

    prepare_batch(args, explicit, &progress)
}

fn prepare_requires_batch(args: &PrepareArgs, explicit_count: usize) -> bool {
    args.list_only
        || args.check_fingerprints
        || args.all
        || args.all_interiors
        || args.all_exteriors
        || args.worldspace.is_some()
        || args.exterior_radius.is_some()
        || args.retry_failed
        || explicit_count > 1
}

/// The single-cell CLI path. Builds a one-cell `BatchSession` (issue #47)
/// rather than duplicating the plugin-chain/BSA/audio/footstep loading
/// `prepare_cell` needs, so its output is identical to what the previous
/// `prepare_one` produced for a single cell (F47.1).
fn prepare_single(
    args: PrepareArgs,
    selector_input: String,
    progress: &ProgressReporter,
) -> Result<()> {
    let game_root = args
        .game_root
        .clone()
        .context("Fallout 3 is not configured; pass --game-root or create .bevyout/config.toml")?;
    let root = fs::canonicalize(&game_root).context("game root does not exist")?;
    let plugin = args
        .plugin
        .clone()
        .unwrap_or_else(|| PathBuf::from("Fallout3.esm"));
    let plugin_path = if plugin.is_absolute() {
        plugin
    } else {
        root.join("Data").join(&plugin)
    };
    let plugin_path = fs::canonicalize(&plugin_path).context("plugin does not exist")?;
    let data_root = root.join("Data");
    let cache_dir = absolutize(
        &args
            .cache_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(".bevyout/cache")),
    )?;

    let loaded_plugins = load_plugin_chain(&plugin_path, &data_root)?;
    let fingerprint = content_set_fingerprint(&loaded_plugins);
    let session = BatchSession::new(
        &plugin_path,
        &data_root,
        &cache_dir,
        loaded_plugins,
        fingerprint,
    )?;
    let selector = parse_cell_selector(&selector_input)?;
    let exterior_worldspace = selected_exterior_worldspace(
        session.parsed_content.cells().map(|(_, cell)| cell),
        &selector,
    );
    let mut output = Vec::new();
    progress.started("Prepare", Some(1));
    progress.phase_started("cell", Some(1));
    let mut result = prepare_cell(&session, args, selector_input, &mut output, progress);
    if result.is_ok()
        && let Some(worldspace_form_id) = exterior_worldspace
    {
        result = finalize_exterior_index(&session, &cache_dir, worldspace_form_id);
    }
    for line in &output {
        println!("{line}");
    }
    progress.unit_completed_in_phase("cell", Some(1), None);
    progress.finished(result.is_ok());
    result
}

fn selected_exterior_worldspace<'a>(
    cells: impl IntoIterator<Item = &'a CellInfo>,
    selector: &CellSelector,
) -> Option<u32> {
    cells
        .into_iter()
        .find(|cell| match selector {
            CellSelector::FormId(form_id) => cell.form_id == *form_id,
            CellSelector::EditorId(editor_id) => cell
                .editor_id
                .as_deref()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(editor_id)),
        })
        .filter(|cell| !cell.interior)
        .and_then(|cell| cell.worldspace_form_id)
}

fn prepare_batch(
    args: PrepareArgs,
    explicit: Vec<String>,
    progress: &ProgressReporter,
) -> Result<()> {
    let spec = SelectionSpec {
        all: args.all,
        all_interiors: args.all_interiors,
        all_exteriors: args.all_exteriors,
        worldspace: args.worldspace.clone(),
        exterior_radius: args.exterior_radius,
        explicit,
    };

    let game_root = args
        .game_root
        .clone()
        .context("Fallout 3 is not configured; pass --game-root or create .bevyout/config.toml")?;
    let root = fs::canonicalize(&game_root).context("game root does not exist")?;
    let plugin = args
        .plugin
        .clone()
        .unwrap_or_else(|| PathBuf::from("Fallout3.esm"));
    let plugin_path = if plugin.is_absolute() {
        plugin
    } else {
        root.join("Data").join(&plugin)
    };
    let plugin_path = fs::canonicalize(&plugin_path).context("plugin does not exist")?;
    let data_root = root.join("Data");
    let cache_dir = absolutize(
        &args
            .cache_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(".bevyout/cache")),
    )?;

    // Read the plugin chain exactly once for the whole batch (F47.2): the
    // catalogue used for `resolve_selection`, the `cellmap.ron` artifact
    // (F47.4), and every cell's `BatchSession` (F47.1) all share this same
    // `loaded_plugins`/`fingerprint`, instead of each cell in the batch
    // re-reading and re-fingerprinting the chain the way `prepare_one` did
    // before this issue.
    let loaded_plugins = load_plugin_chain(&plugin_path, &data_root)?;
    let fingerprint = content_set_fingerprint(&loaded_plugins);
    let sources = loaded_plugins
        .iter()
        .map(|plugin| PluginSource {
            name: &plugin.name,
            bytes: &plugin.bytes,
        })
        .collect::<Vec<_>>();
    let catalog = CellCatalog::build(&sources, fingerprint.clone())?;
    let cells: Vec<CellSummary> = catalog
        .entries
        .iter()
        .map(|entry| CellSummary {
            form_id: entry.form_id,
            editor_id: entry.editor_id.clone(),
            name: entry.name.clone(),
            interior: entry.interior,
            worldspace_form_id: entry.worldspace_form_id,
            grid: entry.grid,
        })
        .collect();
    let worldspace_names = catalog.worldspaces.clone();

    // F48.1: the resumable job manifest for this content set. A manifest on
    // disk built against a different fingerprint is discarded automatically
    // by `load_or_new`.
    let manifest_path = manifest_path(&cache_dir);
    let mut manifest = JobManifest::load_or_new(&manifest_path, &fingerprint)?;

    // F48.3: `--retry-failed` alone (no other selector) means "every failed
    // cell in the manifest"; combined with a selector, it means the
    // intersection of that selection with the failed set.
    let mut resolved = if args.retry_failed {
        if spec.is_empty() {
            manifest.failed_form_ids()
        } else {
            let selection = resolve_selection(&cells, &worldspace_names, &spec)?;
            let failed: HashSet<u32> = manifest.failed_form_ids().into_iter().collect();
            selection
                .into_iter()
                .filter(|form_id| failed.contains(form_id))
                .collect()
        }
    } else {
        resolve_selection(&cells, &worldspace_names, &spec)?
    };
    resolved.sort_unstable();
    resolved.dedup();

    if args.list_only {
        let editor_ids: HashMap<u32, Option<String>> = cells
            .into_iter()
            .map(|cell| (cell.form_id, cell.editor_id))
            .collect();
        for form_id in &resolved {
            let editor_id = editor_ids
                .get(form_id)
                .and_then(|editor_id| editor_id.clone())
                .unwrap_or_default();
            println!("{form_id:08x}\t{editor_id}");
        }
        return Ok(());
    }

    // F49.3: report-only fingerprint check -- lists every selected,
    // previously-`Done` cell's fingerprint status against the current
    // toolchain and exits nonzero on any staleness. Performs no
    // preparation: it returns before the cell map is written or the job
    // manifest's `pending` entries and on-disk copy are touched.
    let selected_scene_converter_revision = NATIVE_PREPARED_CONVERTER_REVISION;
    let selected_actor_animation_backend = crate::converter_policy::resolve_actor_animation_backend(
        Some(args.actor_animation_converter.backend()),
    );
    let selected_converter_revision = crate::converter_policy::prepare_converter_identity(
        selected_scene_converter_revision,
        selected_actor_animation_backend,
        ACTOR_ANIMATION_CATALOG_REVISION,
        actor_animation_converter_revision(selected_actor_animation_backend),
    );
    if args.check_fingerprints {
        return report_fingerprints(
            &manifest,
            &resolved,
            &fingerprint,
            &selected_converter_revision,
        );
    }

    // F47.4: write the deterministic cell map into the cache dir root,
    // reusing the same `ParsedContentSet` -> `CellMap` builder `cells --map`
    // uses, from the content set this run already parsed above.
    let cell_map = build_cell_map(&sources, fingerprint.clone())?;
    let cell_map_path = write_cell_map(&cache_dir, &cell_map)?;
    println!(
        "wrote cell map: {} ({} cells, {} door edges)",
        cell_map_path.display(),
        cell_map.cells.len(),
        cell_map.doors.len()
    );

    // F48.2: every selected cell gets at least a `pending` entry, and the
    // manifest is written once up front -- before any cell runs -- so a
    // crash before the first cell even finishes still leaves a manifest on
    // disk distinguishing "selected, not yet attempted" from "never
    // selected". `--force` reruns everything selected regardless of a
    // recorded `done` status; otherwise cells already `done` under this
    // fingerprint are skipped and reported once.
    //
    // F49.2: `filter_resume_checked` additionally validates each `Done`
    // cell's recorded plugin/converter/physics/prepare-pipeline
    // fingerprints against `current_fingerprints` -- fresh runs, resumes,
    // and `--retry-failed` all go through this same call, so any stale
    // component re-prepares exactly that cell instead of being skipped.
    manifest.ensure_pending(&resolved);
    let current_fingerprints =
        CellFingerprints::current_with_converter(fingerprint.clone(), &selected_converter_revision);
    let (to_run, skipped, stale_cells) =
        filter_resume_checked(&manifest, &resolved, args.force, &current_fingerprints);
    for (form_id, components) in &stale_cells {
        println!("{}", stale_cell_line(*form_id, components));
    }
    println!("{}", summary_line(skipped, stale_cells.len()));
    if skipped > 0 {
        println!("resuming: skipping {skipped} completed cell(s)");
    }
    manifest.write_atomic(&manifest_path)?;

    progress.started("Prepare", Some(resolved.len() as u64));
    progress.phase_started("cell", Some(resolved.len() as u64));
    for _ in 0..skipped {
        progress.unit_completed_in_phase("cell", Some(resolved.len() as u64), None);
    }

    let session = match BatchSession::new(
        &plugin_path,
        &data_root,
        &cache_dir,
        loaded_plugins,
        fingerprint,
    ) {
        Ok(session) => session,
        Err(error) => {
            progress.finished(false);
            return Err(error);
        }
    };

    // F48.4: bounded worker pool. `--jobs N` overrides; otherwise the
    // machine's available parallelism, and never more workers than there
    // are cells to run.
    let worker_count = args
        .jobs
        .unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(std::num::NonZeroUsize::get)
                .unwrap_or(1)
        })
        .max(1)
        .min(to_run.len().max(1));

    let next_index = AtomicUsize::new(0);
    let manifest_mutex = Mutex::new(manifest);
    // Groups each cell's buffered output lines into one atomic print so
    // concurrent workers' lines never interleave mid-line (F48.4).
    let stdout_mutex = Mutex::new(());

    std::thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| {
                loop {
                    let index = next_index.fetch_add(1, Ordering::SeqCst);
                    let Some(&form_id) = to_run.get(index) else {
                        break;
                    };
                    let selector_input = format!("{form_id:08x}");
                    let mut cell_args = args.clone();
                    cell_args.selectors = vec![selector_input.clone()];
                    cell_args.cell = None;
                    cell_args.all = false;
                    cell_args.all_interiors = false;
                    cell_args.all_exteriors = false;
                    cell_args.worldspace = None;
                    cell_args.exterior_radius = None;
                    cell_args.list_only = false;
                    cell_args.retry_failed = false;
                    cell_args.check_fingerprints = false;

                    let mut output = Vec::new();
                    let cell_progress = progress.scoped();
                    let result = prepare_cell(
                        &session,
                        cell_args,
                        selector_input.clone(),
                        &mut output,
                        &cell_progress,
                    );

                    let status = match &result {
                        Ok(()) => JobStatus::Done,
                        Err(error) => JobStatus::Failed(format!("{error:#}")),
                    };

                    {
                        let _stdout_guard = stdout_mutex.lock().unwrap();
                        for line in &output {
                            println!("{line}");
                        }
                        if let JobStatus::Failed(reason) = &status {
                            eprintln!("cell {selector_input} failed: {reason}");
                        }
                    }

                    cell_progress.unit_completed_in_phase(
                        "cell",
                        Some(resolved.len() as u64),
                        None,
                    );

                    {
                        let mut manifest = manifest_mutex.lock().unwrap();
                        // F49.1: a cell that finished `Done` records the
                        // fingerprints it was prepared under, so a later
                        // run can validate them via `filter_resume_checked`
                        // instead of trusting `Done` alone.
                        if let JobStatus::Done = status {
                            manifest.record_fingerprints(form_id, current_fingerprints.clone());
                        }
                        manifest.set_status(form_id, status);
                        // F48.4: rewrite the manifest through after EVERY
                        // cell completion (atomically, see
                        // `JobManifest::write_atomic`) so interrupting the
                        // batch at any point is safe to resume from.
                        if let Err(error) = manifest.write_atomic(&manifest_path) {
                            eprintln!("warning: failed to persist job manifest: {error:#}");
                        }
                    }
                }
            });
        }
    });

    if resolved.iter().any(|form_id| {
        cells
            .iter()
            .find(|cell| cell.form_id == *form_id)
            .is_some_and(|cell| !cell.interior)
    }) {
        finalize_exterior_indexes(&session, &cache_dir)?;
    }

    let manifest = manifest_mutex.into_inner().expect("mutex not poisoned");
    let mut failed_entries: Vec<(u32, String)> = Vec::new();
    let mut done_count = 0usize;
    for &form_id in &to_run {
        match manifest.status(form_id) {
            Some(JobStatus::Done) => done_count += 1,
            Some(JobStatus::Failed(reason)) => failed_entries.push((form_id, reason.clone())),
            _ => {}
        }
    }
    failed_entries.sort_by_key(|(form_id, _)| *form_id);

    // F47.3: one deterministic end-of-batch cache summary line, aggregating
    // every cell's asset cache counts plus the session-level physics
    // sidecar cache's hit/miss totals.
    let asset_totals = *session.asset_totals.lock().unwrap();
    let (physics_reads, physics_hits) = {
        let physics_cache = session.physics_cache.lock().unwrap();
        (physics_cache.accesses(), physics_cache.hits)
    };
    progress.cache_counts(
        physics_hits as u64,
        (physics_reads.saturating_sub(physics_hits)) as u64,
    );
    println!(
        "{}",
        batch_cache_summary_line(asset_totals, physics_reads, physics_hits)
    );

    // F48.3: the deterministic end-of-batch failure summary -- always
    // printed, even with zero failures -- plus one sorted-by-FormID line per
    // failure.
    println!("{done_count} done, {} failed", failed_entries.len());
    for (form_id, reason) in &failed_entries {
        let first_line = reason.lines().next().unwrap_or("");
        println!("  {form_id:08x} {first_line}");
    }

    if !failed_entries.is_empty() {
        progress.finished(false);
        bail!(
            "{} of {} cell(s) failed to prepare",
            failed_entries.len(),
            to_run.len()
        );
    }
    progress.finished(true);
    Ok(())
}

/// `prepare --check-fingerprints` (F49.3): report-only. Lists every
/// selected cell that has previously completed (`JobStatus::Done`) with its
/// fingerprint status (`valid` or `stale (<components>)`) against the
/// current toolchain, then the same deterministic summary line F49.2 prints
/// during a real batch run. Performs no I/O beyond the read-only manifest
/// already loaded by the caller -- no cell map write, no `ensure_pending`,
/// no `BatchSession`, no Blender. Cells never completed are not part of
/// this report (there is nothing recorded to validate); returns an error
/// (nonzero process exit) when any reported cell is stale.
fn report_fingerprints(
    manifest: &JobManifest,
    resolved: &[u32],
    plugin_content_set_fingerprint: &str,
    converter_revision: &str,
) -> Result<()> {
    let current = CellFingerprints::current_with_converter(
        plugin_content_set_fingerprint,
        converter_revision,
    );
    let mut valid_count = 0usize;
    let mut stale_count = 0usize;
    for &form_id in resolved {
        match manifest.status(form_id) {
            Some(JobStatus::Done) => {
                let components = stale_components(manifest.fingerprints_for(form_id), &current);
                if components.is_empty() {
                    println!("fingerprint: cell {form_id:08x} valid");
                    valid_count += 1;
                } else {
                    println!("{}", stale_cell_line(form_id, &components));
                    stale_count += 1;
                }
            }
            // Never-completed cells have nothing recorded to validate;
            // report their status without counting them valid or stale so
            // the summary line matches F49.2's semantics exactly.
            Some(JobStatus::Pending) => println!("fingerprint: cell {form_id:08x} pending"),
            Some(JobStatus::Failed(_)) => println!("fingerprint: cell {form_id:08x} failed"),
            None => println!("fingerprint: cell {form_id:08x} not prepared"),
        }
    }
    println!("{}", summary_line(valid_count, stale_count));
    if stale_count > 0 {
        bail!("{stale_count} cell(s) have stale fingerprints");
    }
    Ok(())
}

/// Prepares one cell using a session's already-loaded plugin chain, BSA/audio
/// archive indexes, and staged footstep set (issue #47): the batch loop in
/// `prepare_batch` calls this once per selected cell against one shared
/// `&BatchSession`, and `prepare_single` calls it once against a one-cell
/// session. `session` never exposes a way to reload the chain (see
/// `session.rs`), so this function structurally cannot repeat that I/O no
/// matter how many cells a batch prepares (F47.1, F47.2).
///
/// Issue #48 runs this concurrently for several cells at once, bounded by
/// `--jobs`, so it takes `&BatchSession` (shared) rather than `&mut`, and
/// writes its progress lines to `output` instead of `println!`ing them
/// directly -- the caller flushes `output` as one atomic block under a
/// mutex so two cells' lines can never interleave mid-line (F48.4).
fn prepare_cell(
    session: &BatchSession,
    args: PrepareArgs,
    selector_input: String,
    output: &mut Vec<String>,
    progress: &ProgressReporter,
) -> Result<()> {
    progress.phase_started(
        format!("cell {selector_input}/resolve and load records"),
        None,
    );
    let selector = parse_cell_selector(&selector_input)?;
    let game_root = args
        .game_root
        .context("Fallout 3 is not configured; pass --game-root or create .bevyout/config.toml")?;
    let root = fs::canonicalize(game_root).context("game root does not exist")?;
    let plugin = args.plugin.unwrap_or_else(|| PathBuf::from("Fallout3.esm"));
    let plugin_path = if plugin.is_absolute() {
        plugin.clone()
    } else {
        root.join("Data").join(&plugin)
    };
    let plugin_path = fs::canonicalize(&plugin_path).context("plugin does not exist")?;
    let data_root = root.join("Data");
    let cache_dir = absolutize(
        &args
            .cache_dir
            .unwrap_or_else(|| PathBuf::from(".bevyout/cache")),
    )?;
    let staging_dir = cache_dir.join("staging");
    let assets_dir = cache_dir.join("assets");
    fs::create_dir_all(&staging_dir)?;
    fs::create_dir_all(&assets_dir)?;

    // Plugin chain, esplugin validation: read/computed once in
    // `BatchSession::new`, not per cell (F47.2).
    let source_fingerprint = session.fingerprint.clone();
    let source_plugins = session
        .loaded_plugins
        .iter()
        .map(|plugin| PreparedPluginSource {
            name: plugin.name.clone(),
            path: plugin.path.to_string_lossy().to_string(),
            fingerprint: fingerprint(&plugin.bytes),
        })
        .collect::<Vec<_>>();
    let mut diagnostics = Vec::new();
    diagnostics.extend(session.plugin_diagnostics.iter().cloned());
    let plugin_sources = session.plugin_sources();
    // The batch session parses the content set once and shares that immutable
    // state with all workers. Selection still returns an owned plugin because
    // preparation mutates cell-local collections.
    let mut parsed = session
        .parsed_content
        .select_shared(&selector)
        .context("failed to select Fallout content set cell")?;
    diagnostics.extend(parsed.diagnostics.drain(..).map(|message| Diagnostic {
        severity: "info".into(),
        message,
    }));
    let mut cell = parsed
        .cell
        .take()
        .with_context(|| format!("requested cell selector '{selector_input}' was not found"))?;
    let cell_id = cell.form_id;
    let scene_dir = cache_dir.join("scenes").join(format!("{cell_id:08x}"));
    fs::create_dir_all(&scene_dir)?;
    if let Some(image_space_form_id) = cell.image_space_form_id {
        if let Some(image_space) = parsed.image_spaces.get(&image_space_form_id).cloned() {
            info_image_space(&mut diagnostics, &image_space);
            cell.image_space = Some(image_space);
        } else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "{} references unresolved ImageSpace {:08x}",
                    super::super::manifest::cell_label(&cell),
                    image_space_form_id
                ),
            });
        }
    }
    // BSA archive indexes: indexed once in `BatchSession::new`, not per cell
    // (F47.2).
    diagnostics.extend(session.archive_diagnostics.iter().cloned());
    // Audio archive indexes: same -- indexed once in `BatchSession::new`.
    diagnostics.extend(session.audio_diagnostics.iter().cloned());
    progress.phase_started(
        format!("cell {selector_input}/textures, interface, audio"),
        None,
    );
    let (cell_audio, mut audio_clips) = stage_audio(
        &data_root,
        &session.audio_archives,
        &parsed,
        &mut diagnostics,
        &cache_dir.join("audio"),
    )?;
    // Footstep clip set: cell-independent (fixed surface families), staged
    // once in `BatchSession::new` and shared verbatim by every cell (F47.2).
    diagnostics.extend(session.footstep_diagnostics.iter().cloned());
    let footstep_sets = session.footstep_sets.clone();
    let hard_landing_clips = session.hard_landing_clips.clone();
    if let Some(metadata) = parsed.cell_metadata.as_ref() {
        cell.lighting_template_form_id = metadata.lighting_template_form_id;
        cell.lighting_template_flags = metadata.lighting_template_flags;
        cell.water_form_id = metadata.water_form_id;
        cell.water_height = metadata
            .water_height
            .filter(|height| height.is_finite() && height.abs() < 1_000_000.0);
        let raw_lighting = metadata.lighting;
        let mut effective_lighting = raw_lighting.unwrap_or_else(|| legacy_lighting(&cell));
        cell.raw_lighting = raw_lighting.map(prepared_lighting);
        if let Some(template_form_id) = metadata.lighting_template_form_id {
            if let Some(template) = parsed.lighting_templates.get(&template_form_id) {
                if let Some(lighting) = template.lighting {
                    effective_lighting.apply_template(lighting, metadata.lighting_template_flags);
                    cell.lighting_template = Some(PreparedLightingTemplate {
                        form_id: template.form_id,
                        editor_id: template.editor_id.clone(),
                        ambient_rgba: lighting.ambient_rgba,
                        directional_rgba: lighting.directional_rgba,
                        fog_rgba: lighting.fog_rgba,
                        fog_near: lighting.fog_near,
                        fog_far: lighting.fog_far,
                        fog_directional_fade: lighting.fog_directional_fade,
                        fog_clip_distance: lighting.fog_clip_distance,
                        fog_power: lighting.fog_power,
                        directional_rotation_xy: lighting.rotation_xy,
                        directional_rotation_z: lighting.rotation_z,
                    });
                } else {
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "{} references LightingTemplate {:08x} without usable DATA; retained CELL lighting",
                            super::super::manifest::cell_label(&cell),
                            template_form_id
                        ),
                    });
                }
            } else {
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "{} references unresolved LightingTemplate {:08x}",
                        super::super::manifest::cell_label(&cell),
                        template_form_id
                    ),
                });
            }
        }
        cell.ambient_rgba = effective_lighting.ambient_rgba;
        cell.directional_rgba = effective_lighting.directional_rgba;
        cell.effective_lighting = Some(prepared_lighting(effective_lighting));
    } else {
        cell.effective_lighting = Some(prepared_lighting(legacy_lighting(&cell)));
    }
    cell.day_night_profile = resolve_authoritative_day_night_profile(&cell, &parsed);
    cell.day_night_preview_profile = resolve_preview_day_night_profile(&parsed);
    // Package point references are scoped to packages reachable from actors
    // in this selected cell, never to every content-wide PACK record. The
    // catalog input is also reused by the interior path below.
    let package_catalog_inputs = build_package_catalog_inputs(&parsed);
    if !cell.interior {
        let static_converter_revision = NATIVE_NIF_CONVERTER_REVISION;
        let actor_converter_revision = NATIVE_ACTOR_CONVERTER_REVISION;
        let actor_animation_backend = args.actor_animation_converter.backend();
        progress.phase_started(
            format!("cell {selector_input}/parse and stage assets"),
            None,
        );
        // Issue #299 (M6 W3-C stage 1): exterior ACHR/ACRE references need the
        // same resolved `ActorAssemblyBlueprint` interior cells already get.
        // Build the actor catalog inputs and appearance models up front, the
        // same way the interior path does below, so `stage_placements_with_
        // package_points` can assemble actor visuals instead of receiving an
        // empty appearance map and silently producing bare, asset-less
        // placements for every NPC/creature reference.
        let actor_references = parsed
            .references
            .iter()
            .filter(|reference| {
                matches!(reference.kind, ReferenceKind::Npc | ReferenceKind::Creature)
            })
            .cloned()
            .collect::<Vec<_>>();
        let actor_catalog_inputs = build_actor_catalog_inputs(&parsed, &actor_references);
        let mut actor_catalog = build_actor_catalog(&actor_catalog_inputs, &source_fingerprint);
        let actor_models = build_actor_appearance_models(
            &parsed,
            &actor_references,
            &actor_catalog,
            &source_fingerprint,
            &data_root,
            &session.archives,
            &mut diagnostics,
        )?;
        let actor_assemblies = actor_models
            .iter()
            .map(|(reference_form_id, appearance)| {
                (*reference_form_id, appearance.blueprint.clone())
            })
            .collect::<HashMap<_, _>>();
        attach_actor_assemblies(&mut actor_catalog, &actor_assemblies);
        // Issue #305 review (issue #301 follow-up): resolve the package-point
        // reference scope from the built actor catalog, the same source of
        // truth the interior branch below uses, so a TPLT-inherited package
        // list is not silently dropped -- `actor_package_lists` only ever
        // read direct `PKID` values off the parsed plugin.
        let exterior_package_linked_reference_ids = package_linked_reference_ids(
            &actor_catalog_package_lists(&actor_catalog),
            &package_catalog_inputs.packages,
        );
        // Exterior workers share both the staging tree and the global
        // recipe-addressed `assets/` outputs. Hold the existing narrow
        // conversion lock before cache decisions are made, otherwise two
        // cells can both schedule and atomically replace the same GLB on
        // Windows.
        let asset_stage_guard = session.asset_stage_lock.lock().unwrap();
        let mut exterior_stage = stage_placements_with_package_points(
            parsed.references.clone(),
            &parsed.bases,
            &actor_models,
            &exterior_package_linked_reference_ids,
            &data_root,
            &session.archives,
            &staging_dir,
            &assets_dir,
            &mut diagnostics,
            args.rebuild_assets,
            static_converter_revision,
            actor_converter_revision,
        )?;
        progress.cache_counts(
            exterior_stage.cache_hits as u64,
            (exterior_stage.cache_missing
                + exterior_stage.cache_invalid
                + exterior_stage.cache_explicit_rebuilds) as u64,
        );
        {
            if let Some(worldspace_form_id) = cell.worldspace_form_id {
                let cached = session
                    .worldspace_lod_cache
                    .lock()
                    .unwrap()
                    .get(&worldspace_form_id)
                    .cloned();
                let _lod_assets = if let Some(cached) = cached {
                    cached
                } else {
                    let index = session
                        .exterior_indexes
                        .iter()
                        .find(|index| index.worldspace_form_id == worldspace_form_id)
                        .cloned();
                    let assets = index
                        .map(|index| {
                            prepare_worldspace_lod(
                                &index,
                                &data_root,
                                &session.archives,
                                &staging_dir,
                                &assets_dir,
                                args.rebuild_assets,
                                args.jobs,
                                args.strict,
                                output,
                                &mut diagnostics,
                            )
                        })
                        .transpose()?
                        .unwrap_or_default();
                    session
                        .worldspace_lod_cache
                        .lock()
                        .unwrap()
                        .insert(worldspace_form_id, assets.clone());
                    assets
                };
            }
        }
        let mut failed_native_assets = HashMap::new();
        progress.phase_started(
            format!("cell {selector_input}/native NIF/KF conversion"),
            Some(exterior_stage.jobs.len() as u64),
        );
        if !exterior_stage.jobs.is_empty() {
            let batch = run_native_batch(
                &exterior_stage.jobs,
                &data_root,
                &session.archives,
                &cache_dir,
                args.jobs,
                args.strict,
            )
            .context("native exterior asset conversion failed")?;
            output.push(batch.summary().line());
            for outcome in &batch.outcomes {
                if outcome.status == NativeJobStatus::Converted {
                    continue;
                }
                output.push(format!(
                    "native conversion {}: model={} job={} stage={} error={}",
                    match outcome.status {
                        NativeJobStatus::Unsupported => "unsupported",
                        NativeJobStatus::Failed => "failed",
                        NativeJobStatus::Converted => unreachable!(),
                    },
                    outcome.model,
                    outcome.index,
                    outcome.stage,
                    outcome.error.as_deref().unwrap_or("unknown error")
                ));
            }
            batch.enforce_strict(args.strict)?;
            for outcome in &batch.outcomes {
                progress.job_completed(outcome.status == NativeJobStatus::Converted);
                progress.unit_completed(Some(exterior_stage.jobs.len() as u64), None);
            }
            failed_native_assets = batch
                .failed_outputs(&exterior_stage.jobs)
                .into_iter()
                .filter_map(|(path, reason)| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| (PathBuf::from(format!("assets/{name}")), reason))
                })
                .collect();
        }
        if !failed_native_assets.is_empty() {
            for placement in &mut exterior_stage.placements {
                let Some(asset_path) = placement.asset_path.as_ref() else {
                    continue;
                };
                let Some(reason) = failed_native_assets.get(Path::new(asset_path)) else {
                    continue;
                };
                placement.asset_path = None;
                placement.physics_asset_path = None;
                placement.error = Some(reason.clone());
                placement.step_support = false;
            }
        }
        drop(asset_stage_guard);

        session.record_persistent_assets(&exterior_stage.placements);

        // Re-attach assemblies after native conversion/failed-asset scrubbing
        // above may have cleared a placement's `asset_path`/`error`: the
        // catalog must reflect the same finalized state the package receives
        // below, mirroring the interior path's `finalized_actor_assemblies`.
        let finalized_actor_assemblies = exterior_stage
            .placements
            .iter()
            .filter_map(|placement| match &placement.semantic {
                PreparedSemantic::Npc(actor) | PreparedSemantic::Creature(actor) => actor
                    .assembly
                    .as_ref()
                    .map(|assembly| (placement.reference_form_id, assembly.clone())),
                _ => None,
            })
            .collect::<HashMap<_, _>>();
        attach_actor_assemblies(&mut actor_catalog, &finalized_actor_assemblies);

        // Exterior NAVM used to be flattened directly into the cell package,
        // which silently discarded authored adjacency, doors, NVEX targets,
        // merge records, and the collision-derived clearance verdict. Reuse
        // the same prepared graph pipeline as interiors before the package is
        // assembled; the package stores the resulting graph artifact pointer
        // while retaining its compact flattened buffers for diagnostics.
        progress.phase_started(
            format!("cell {selector_input}/navigation and physics"),
            None,
        );
        let mut physics_assets = HashMap::new();
        for placement in &mut exterior_stage.placements {
            let Some(relative_path) = placement.physics_asset_path.as_ref() else {
                continue;
            };
            if !physics_assets.contains_key(relative_path) {
                let asset = session.physics_cache.lock().unwrap().get_or_insert_with(
                    relative_path,
                    || {
                        let path = cache_dir
                            .join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
                        read_physics_asset(&path)
                    },
                )?;
                physics_assets.insert(relative_path.clone(), asset);
            }
            let asset = physics_assets
                .get(relative_path)
                .expect("exterior physics asset was inserted above");
            placement.physics_source = Some(asset.source.clone());
            placement.physics_classification = classify_placement(&placement.semantic, asset);
            placement.step_support = retain_static_step_support(
                placement.step_support,
                placement.physics_classification,
            );
        }
        let (navigation_source, navigation_clearance_ready, navigation_summaries) =
            prepare_exterior_navigation(
                &cache_dir,
                &scene_dir,
                cell_id,
                &mut diagnostics,
                &parsed.navmeshes,
                parsed.navigation.as_ref(),
                &exterior_stage.placements,
                &physics_assets,
                cell.grid
                    .zip(parsed.land.as_ref())
                    .map(|((x, y), land)| {
                        crate::vsa::terrain_from_land(
                            land,
                            bevyout_core::manifest::exterior::GridCoordinate::new(x, y),
                        )
                    })
                    .as_ref(),
            )?;
        output.extend(navigation_summaries);

        let mut package = crate::vsa::build_cell_package(
            &parsed,
            &cell,
            &source_fingerprint,
            navigation_source.as_ref(),
            navigation_clearance_ready,
        )
        .with_context(|| {
            format!(
                "exterior cell {} is missing a worldspace or XCLC grid",
                super::super::manifest::cell_label(&cell)
            )
        })?;
        if let Some(terrain) = package.terrain.as_mut() {
            let assignments = parsed
                .land
                .as_ref()
                .map_or(&[][..], |land| land.texture_assignments.as_slice());
            crate::vsa::prepare_terrain_albedo(
                terrain,
                assignments,
                &parsed.landscape_textures,
                &parsed.texture_sets,
                &data_root,
                &session.archives,
                &cache_dir,
                &source_fingerprint,
                cell_id,
                &mut diagnostics,
            )?;
        }
        crate::vsa::apply_staged_assets(
            &mut package,
            &exterior_stage.placements,
            &failed_native_assets
                .into_iter()
                .filter_map(|(path, reason)| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| (format!("assets/{name}"), reason))
                })
                .collect(),
        );
        let actor_catalog_artifact = write_actor_catalog(&cache_dir, cell_id, &actor_catalog)?;
        let actor_catalog_summary = format!(
            "actor catalog: prepared {}, inherited {}, unresolved {}, unsupported {}, skipped {}",
            actor_catalog.counters.prepared,
            actor_catalog.counters.inherited,
            actor_catalog.counters.unresolved,
            actor_catalog.counters.unsupported,
            actor_catalog.counters.skipped
        );
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: actor_catalog_summary.clone(),
        });
        output.push(actor_catalog_summary);
        let actor_catalog_path = Some(actor_catalog_artifact.relative_path);
        let actor_catalog_revision = Some(ACTOR_CATALOG_REVISION.into());
        let actor_catalog_hash = Some(actor_catalog_artifact.hash);
        let mut actor_animation_catalog = discover_actor_animation_catalog(
            &actor_catalog,
            &source_fingerprint,
            &data_root,
            &session.archives,
            &parsed.idles,
        )?;
        let conversion_context = ActorAnimationConversionContext {
            converter: actor_animation_backend,
            converter_revision: actor_animation_converter_revision(actor_animation_backend),
            data_root: &data_root,
            archives: &session.archives,
            staging_dir: &staging_dir,
            assets_dir: &assets_dir,
            rebuild: args.rebuild_assets,
        };
        // Issue #305 review: actor-animation conversion writes to the same
        // shared, identity-addressed staging/GLB/report paths the placement
        // conversion above guards against racing on. Re-acquire the narrow
        // conversion lock for this span rather than holding it across the
        // navigation/terrain/package work in between, which does not touch
        // those shared paths and should stay parallel across cells.
        let actor_animation_stage_guard = session.asset_stage_lock.lock().unwrap();
        let actor_animation_conversion =
            convert_actor_animation_catalog(&mut actor_animation_catalog, &conversion_context)?;
        let actor_animation_catalog_artifact =
            write_actor_animation_catalog(&cache_dir, cell_id, &actor_animation_catalog)?;
        drop(actor_animation_stage_guard);
        let ready_animation_clips = actor_animation_catalog
            .animation_sets
            .iter()
            .flat_map(|set| set.clips.iter())
            .filter(|clip| {
                clip.status
                    == bevyout_core::actor_animation::PreparedActorAnimationClipStatus::Ready
            })
            .count();
        let actor_animation_catalog_summary = format!(
            "actor animation catalog: {} actor mappings, {} sets, {} ready clips, packs built {}, reused {}, failed clips {}, cache {}",
            actor_animation_catalog.actor_mappings.len(),
            actor_animation_catalog.animation_sets.len(),
            ready_animation_clips,
            actor_animation_conversion.built_packs,
            actor_animation_conversion.reused_packs,
            actor_animation_conversion.failed_clips,
            if actor_animation_catalog_artifact.reused {
                "reused"
            } else {
                "written"
            }
        );
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: actor_animation_catalog_summary.clone(),
        });
        output.push(actor_animation_catalog_summary);
        let actor_animation_catalog_path = Some(actor_animation_catalog_artifact.relative_path);
        let actor_animation_catalog_revision = Some(ACTOR_ANIMATION_CATALOG_REVISION.into());
        let actor_animation_catalog_hash = Some(actor_animation_catalog_artifact.hash);
        let package_path = cache_dir
            .join("worldspaces")
            .join(format!("{:08x}", package.worldspace_form_id))
            .join("cells")
            .join(format!("{:08x}.ron", package.cell_form_id));
        if let Some(parent) = package_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(
            &package_path,
            to_string_pretty(&package, PrettyConfig::default())?,
        )?;
        let storage_plan =
            exterior_scene_storage_plan(package.worldspace_form_id, package.cell_form_id);
        debug_assert_eq!(
            package_path,
            cache_dir.join(
                storage_plan
                    .package_path
                    .replace('/', std::path::MAIN_SEPARATOR_STR,)
            )
        );
        let terrain_prepared = package.terrain.is_some();
        let object_count = package.static_objects.len()
            + package.dynamic_objects.len()
            + package.distant_objects.len()
            + package.actors.len();
        let manifest = PreparedSceneManifest {
            schema_version: CURRENT_MANIFEST_SCHEMA_VERSION,
            prepare_revision: Some(CURRENT_PREPARE_REVISION.into()),
            converter_revision: Some(NATIVE_PREPARED_CONVERTER_REVISION.into()),
            physics_schema_version: Some(PHYSICS_ASSET_SCHEMA_VERSION),
            asset_root: cache_dir.to_string_lossy().to_string(),
            source_plugin: plugin_path.to_string_lossy().to_string(),
            source_fingerprint,
            item_catalog_path: None,
            item_catalog_revision: None,
            item_catalog_hash: None,
            recipe_catalog_path: None,
            recipe_catalog_revision: None,
            recipe_catalog_hash: None,
            actor_catalog_path,
            actor_catalog_revision,
            actor_catalog_hash,
            actor_animation_catalog_path,
            actor_animation_catalog_revision,
            actor_animation_catalog_hash,
            image_space_modifier_catalog_path: None,
            image_space_modifier_catalog_revision: None,
            image_space_modifier_catalog_hash: None,
            source_plugins,
            cell,
            placements: Vec::new(),
            lights: Vec::new(),
            diagnostics: if storage_plan.embed_content_diagnostics {
                diagnostics
            } else {
                Vec::new()
            },
            visual_issues: Vec::new(),
            navmeshes: Vec::new(),
            nav_graph: None,
            cell_audio: PreparedCellAudio::default(),
            audio_clips: Vec::new(),
            footstep_sets: Vec::new(),
            hard_landing_clips: Vec::new(),
            bake: None,
            static_point_shadows: None,
            reflection_probes: None,
            mutability_summary: PreparedMutabilitySummary::default(),
            leveled_lists: std::collections::BTreeMap::new(),
            dialogue: None,
            exterior: storage_plan.embed_package.then_some(package),
        };
        progress.phase_started(format!("cell {selector_input}/manifest publication"), None);
        let manifest_path = scene_dir.join("scene.ron");
        fs::write(
            &manifest_path,
            to_string_pretty(&manifest, PrettyConfig::default())?,
        )?;
        output.push(format!(
            "prepared exterior {} terrain={} objects={} -> {}",
            super::super::manifest::cell_label(&manifest.cell),
            terrain_prepared,
            object_count,
            manifest_path.display()
        ));
        return Ok(());
    }
    let static_converter_revision = NATIVE_NIF_CONVERTER_REVISION;
    let actor_converter_revision = NATIVE_ACTOR_CONVERTER_REVISION;
    let prepared_converter_revision = NATIVE_PREPARED_CONVERTER_REVISION;
    let actor_animation_backend = args.actor_animation_converter.backend();
    progress.phase_started(
        format!("cell {selector_input}/navigation and physics"),
        None,
    );
    let (navmeshes, mut nav_graph, mut nav_graph_full, nav_graph_summary) = stage_navmeshes(
        &cache_dir,
        &scene_dir,
        cell_id,
        &mut diagnostics,
        &parsed.navmeshes,
        parsed.navigation.as_ref(),
    )?;
    output.push(nav_graph_summary);
    // Parses the cell's references and stages their NIF/texture files under
    // the shared `staging_dir`/`assets_dir` (F48.4 parallel phase). Every
    // path written here is content-addressed (the NIF by its
    // normalized model path, textures by their own asset path, the
    // eventual GLB/physics pair by a content hash of the NIF bytes), so two
    // cells racing to stage the *same* missing asset write the same bytes
    // to the same path -- redundant in the worst case, not corrupting. That
    // is what makes it safe to run for several cells concurrently, unlike
    // the shared texture-staging/native-conversion step immediately below.
    // Snapshot every ACHR/ACRE reference before `references` is taken below
    // (issue #103, M4 wave 1 task C): the actor catalog needs the raw
    // reference identity/transform/enable state independent of whether
    // `stage_placements` later produces a renderable placement for it.
    let actor_references = parsed
        .references
        .iter()
        .filter(|reference| matches!(reference.kind, ReferenceKind::Npc | ReferenceKind::Creature))
        .cloned()
        .collect::<Vec<_>>();
    diagnostics.extend(session.dialogue_voice_diagnostics.iter().cloned());
    let dialogue_voice_discovery = should_discover_dialogue_voice(args.dialogue_voice_discover)
        .then(|| {
            discover_dialogue_voice(
                cell_id,
                &parsed,
                &plugin_sources,
                &data_root,
                &session.dialogue_voice_archives,
            )
        })
        .transpose()?;
    let actor_catalog_inputs = build_actor_catalog_inputs(&parsed, &actor_references);
    let mut actor_catalog = build_actor_catalog(&actor_catalog_inputs, &source_fingerprint);
    // Issue #175 (M4 wave 11 lane C): the package catalog's `known_form_ids`
    // link-check set needs every base record and placed reference this
    // decode pass saw; it was assembled before the exterior/interior split
    // so package-linked marker eligibility has the same scope on both paths.
    let mut references = std::mem::take(&mut parsed.references);
    let actor_models = build_actor_appearance_models(
        &parsed,
        &actor_references,
        &actor_catalog,
        &source_fingerprint,
        &data_root,
        &session.archives,
        &mut diagnostics,
    )?;
    let actor_assemblies = actor_models
        .iter()
        .map(|(reference_form_id, appearance)| (*reference_form_id, appearance.blueprint.clone()))
        .collect::<HashMap<_, _>>();
    attach_actor_assemblies(&mut actor_catalog, &actor_assemblies);
    let (catalog_references, catalog_reference_ids) =
        catalog_item_references(&parsed.bases, &references);
    references.extend(catalog_references);
    let package_linked_reference_ids = package_linked_reference_ids(
        &actor_catalog_package_lists(&actor_catalog),
        &package_catalog_inputs.packages,
    );
    progress.phase_started(
        format!("cell {selector_input}/parse and stage assets"),
        None,
    );
    let stage = stage_placements_with_package_points(
        references,
        &parsed.bases,
        &actor_models,
        &package_linked_reference_ids,
        &data_root,
        &session.archives,
        &staging_dir,
        &assets_dir,
        &mut diagnostics,
        args.rebuild_assets,
        static_converter_revision,
        actor_converter_revision,
    )?;
    // F47.3: fold this cell's asset cache counts into the batch total.
    session.asset_totals.lock().unwrap().add(
        stage.cache_hits,
        stage.cache_missing,
        stage.cache_invalid,
        stage.cache_explicit_rebuilds,
    );
    let PlacementStage {
        jobs,
        mut visual_assets,
        mut placements,
        lights,
        cache_hits,
        cache_missing,
        cache_invalid,
        cache_explicit_rebuilds,
        leveled_lists,
    } = stage;
    progress.cache_counts(
        cache_hits as u64,
        (cache_missing + cache_invalid + cache_explicit_rebuilds) as u64,
    );
    // F48.4 serialization point: `convert_staged_textures` walks every
    // `.dds` under the *whole* `staging_dir` (not just this cell's), and
    // Native conversion and `convert_staged_textures` walk shared staging
    // paths. Neither operation is content-addressed at the staging boundary,
    // so this block holds `session.asset_stage_lock` while one cell performs
    // its conversion phase; other workers keep parsing and staging around it.
    let item_icons;
    let mut failed_native_assets = HashMap::new();
    {
        progress.phase_started(
            format!("cell {selector_input}/textures and interface"),
            None,
        );
        let _asset_stage_guard = session.asset_stage_lock.lock().unwrap();
        item_icons = stage_item_icons(
            &parsed.bases,
            &data_root,
            &session.archives,
            &staging_dir,
            &source_fingerprint,
            &mut diagnostics,
        )?;
        stage_pipboy_sprites(
            &data_root,
            &session.archives,
            &staging_dir,
            &mut diagnostics,
        )?;
        stage_hud_sprites(
            &data_root,
            &session.archives,
            &staging_dir,
            &mut diagnostics,
        )?;
        {
            convert_staged_textures(
                &staging_dir.join("item-icons").join(&source_fingerprint),
                &mut diagnostics,
            )?;
            convert_staged_textures(&staging_dir.join("interface"), &mut diagnostics)?;
            if !jobs.is_empty() {
                progress.phase_started(
                    format!("cell {selector_input}/native NIF/KF conversion"),
                    Some(jobs.len() as u64),
                );
                let batch = run_native_batch(
                    &jobs,
                    &data_root,
                    &session.archives,
                    &cache_dir,
                    args.jobs,
                    args.strict,
                )
                .context("native NIF batch conversion failed")?;
                let summary = batch.summary();
                let summary_line = summary.line();
                output.push(summary_line.clone());
                diagnostics.push(Diagnostic {
                    severity: "info".into(),
                    message: summary_line,
                });
                for outcome in &batch.outcomes {
                    progress.job_completed(outcome.status == NativeJobStatus::Converted);
                    progress.unit_completed(Some(jobs.len() as u64), None);
                    if outcome.status == NativeJobStatus::Converted {
                        if let Some(warning) = &outcome.error {
                            diagnostics.push(Diagnostic {
                                severity: "warning".into(),
                                message: format!(
                                    "native conversion lossy: model={} job={} {warning}",
                                    outcome.model, outcome.index
                                ),
                            });
                        }
                        continue;
                    }
                    let message = format!(
                        "native conversion {}: model={} job={} stage={} error={}",
                        match outcome.status {
                            NativeJobStatus::Unsupported => "unsupported",
                            NativeJobStatus::Failed => "failed",
                            NativeJobStatus::Converted => unreachable!(),
                        },
                        outcome.model,
                        outcome.index,
                        outcome.stage,
                        outcome.error.as_deref().unwrap_or("unknown error")
                    );
                    output.push(message.clone());
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message,
                    });
                }
                batch.enforce_strict(args.strict)?;
                for (path, reason) in batch.failed_outputs(&jobs) {
                    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                        failed_native_assets.insert(format!("assets/{file_name}"), reason);
                    }
                }
            }
        }
    }
    if !failed_native_assets.is_empty() {
        visual_assets.retain(|asset| !failed_native_assets.contains_key(&asset.asset_path));
        for placement in &mut placements {
            let Some(asset_path) = placement.asset_path.as_ref() else {
                continue;
            };
            let Some(reason) = failed_native_assets.get(asset_path) else {
                continue;
            };
            placement.asset_path = None;
            placement.physics_asset_path = None;
            placement.error = Some(reason.clone());
            placement.step_support = false;
        }
    }
    let mut scene_placements = Vec::new();
    let mut catalog_placements = Vec::new();
    for placement in placements {
        if catalog_reference_ids.contains(&placement.reference_form_id) {
            catalog_placements.push(placement);
        } else {
            scene_placements.push(placement);
        }
    }
    placements = scene_placements;
    let scene_assets = placements
        .iter()
        .filter_map(|placement| placement.asset_path.as_deref())
        .collect::<HashSet<_>>();
    visual_assets.retain(|asset| scene_assets.contains(asset.asset_path.as_str()));
    let additional_audio_form_ids =
        apply_container_animation_audio(&cache_dir, &parsed, &mut placements, &mut diagnostics);
    let staged_form_ids = audio_clips
        .iter()
        .map(|clip| clip.form_id)
        .collect::<HashSet<_>>();
    let additional_audio_form_ids = additional_audio_form_ids
        .into_iter()
        .filter(|form_id| !staged_form_ids.contains(form_id));
    audio_clips.extend(stage_audio_clips(
        &data_root,
        &session.audio_archives,
        &parsed,
        &mut diagnostics,
        &cache_dir.join("audio"),
        additional_audio_form_ids,
    )?);
    audio_clips.sort_by_key(|clip| clip.form_id);
    let cache_summary = format!(
        "asset cache: reused {cache_hits}, missing {cache_missing}, invalid {cache_invalid}, explicitly rebuilt {cache_explicit_rebuilds}; scheduled {} NIF-to-GLB conversion(s)",
        jobs.len()
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: cache_summary.clone(),
    });
    output.push(cache_summary);
    let visual_issues = audit_prepared_visuals(&cache_dir, &visual_assets, &placements)?;
    for issue in &visual_issues {
        output.push(format_visual_issue(issue));
    }
    let invalid_visuals = visual_issues
        .iter()
        .filter(|issue| issue.severity == "error")
        .count();
    let review_visuals = visual_issues.len() - invalid_visuals;
    let visual_summary = format!(
        "visual completeness: {} assets checked, {invalid_visuals} invalid, {review_visuals} review required",
        visual_assets.len()
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: visual_summary.clone(),
    });
    output.push(visual_summary);
    // F47.3: this cell's unique physics assets, sourced through the
    // session-level cache so a sidecar already read for an earlier cell in
    // the batch is reused (a hit) instead of re-read from disk.
    let mut physics_assets = HashMap::new();
    let mut authored_assets = 0_usize;
    let mut fallback_assets = 0_usize;
    let mut dynamic_placements = 0_usize;
    let mut dynamic_rejections = HashSet::new();
    for placement in placements.iter_mut().chain(catalog_placements.iter_mut()) {
        let Some(relative_path) = placement.physics_asset_path.as_ref() else {
            continue;
        };
        if !physics_assets.contains_key(relative_path) {
            let asset =
                session
                    .physics_cache
                    .lock()
                    .unwrap()
                    .get_or_insert_with(relative_path, || {
                        let path = cache_dir
                            .join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
                        read_physics_asset(&path)
                    })?;
            physics_assets.insert(relative_path.clone(), asset);
        }
        let asset = physics_assets
            .get(relative_path)
            .expect("physics asset was inserted above");
        placement.physics_source = Some(asset.source.clone());
        placement.physics_classification = classify_placement(&placement.semantic, asset);
        placement.step_support =
            retain_static_step_support(placement.step_support, placement.physics_classification);
        if !catalog_reference_ids.contains(&placement.reference_form_id)
            && let Some(reason) = dynamic_rejection_reason(&placement.semantic, asset)
        {
            dynamic_rejections.insert(format!(
                "physics body for {} ({:08x}) remains static: {reason}",
                placement
                    .editor_id
                    .as_deref()
                    .or(placement.display_name.as_deref())
                    .unwrap_or("<unnamed>"),
                placement.reference_form_id
            ));
        }
        if placement.physics_classification == PreparedPhysicsClassification::Dynamic {
            dynamic_placements += 1;
        }
    }
    for asset in physics_assets.values() {
        match asset.source {
            super::super::physics::PreparedPhysicsSource::AuthoredHavok => authored_assets += 1,
            super::super::physics::PreparedPhysicsSource::GeneratedRender => fallback_assets += 1,
        }
    }
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: format!(
            "physics sidecars: {authored_assets} authored Havok, {fallback_assets} generated fallback; {dynamic_placements} dynamic placement(s)"
        ),
    });
    for message in dynamic_rejections {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message,
        });
    }
    // Issue #153 (M4 wave 10): collision-derived navmesh validation +
    // clearance. Runs here -- after physics classification -- so the static
    // collision shell it validates the authored NAVM against is the same
    // fixed geometry the player's capsule collides with. Mutates the prepared
    // nav graph in place, rewrites `navgraph.ron`, and refreshes the manifest
    // pointer (`nav_graph`).
    if let Some(graph) = nav_graph_full.as_mut() {
        let collision = cell_static_collision_triangles(&placements, &physics_assets);
        let (_clearance_source, clearance_summary) =
            apply_nav_clearance(&cache_dir, cell_id, graph, &collision, &mut diagnostics)?;
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: clearance_summary.clone(),
        });
        output.push(clearance_summary);

        // Issue #177 (M4 wave 11): collision-derived blocker -> polygon
        // associations. Runs after clearance so triangle indices refer to the
        // post-clip polygons the runtime loads, and against the placements
        // the static shell deliberately excludes (doors, kinematic
        // activators) -- the missing input the authored `NVDP` door list only
        // ever provides for load/travel doors.
        let blockers = cell_blocker_volumes(&placements, &physics_assets);
        let (updated_source, door_summary) = apply_derived_door_associations(
            &cache_dir,
            cell_id,
            graph,
            &blockers,
            &mut diagnostics,
        )?;
        nav_graph = Some(updated_source);
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: door_summary.clone(),
        });
        output.push(door_summary);
    }
    let mut item_catalog = build_item_catalog(
        &parsed.bases,
        &item_icons,
        &catalog_placements,
        &physics_assets,
        &source_fingerprint,
    );
    let (generic_pickup, generic_drop) = item_transfer_audio_defaults(&parsed);
    apply_item_transfer_audio_defaults(&mut item_catalog, generic_pickup, generic_drop);
    apply_actor_weapon_assets(&mut placements, &item_catalog);
    let finalized_actor_assemblies = placements
        .iter()
        .filter_map(|placement| match &placement.semantic {
            PreparedSemantic::Npc(actor) | PreparedSemantic::Creature(actor) => actor
                .assembly
                .as_ref()
                .map(|assembly| (placement.reference_form_id, assembly.clone())),
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    attach_actor_assemblies(&mut actor_catalog, &finalized_actor_assemblies);
    let mut actor_animation_catalog = discover_actor_animation_catalog(
        &actor_catalog,
        &source_fingerprint,
        &data_root,
        &session.archives,
        &parsed.idles,
    )?;
    let conversion_context = ActorAnimationConversionContext {
        converter: actor_animation_backend,
        converter_revision: actor_animation_converter_revision(actor_animation_backend),
        data_root: &data_root,
        archives: &session.archives,
        staging_dir: &staging_dir,
        assets_dir: &assets_dir,
        rebuild: args.rebuild_assets,
    };
    let actor_animation_conversion =
        convert_actor_animation_catalog(&mut actor_animation_catalog, &conversion_context)?;
    let (catalog_path, catalog_hash) = write_item_catalog(&cache_dir, &item_catalog)?;
    output.push(format!(
        "item catalog: {} records, {} icons, {} world assets -> {}",
        item_catalog.items.len(),
        item_catalog
            .items
            .iter()
            .filter(|item| item.icon_asset_path.is_some())
            .count(),
        item_catalog
            .items
            .iter()
            .filter(|item| item.world_asset_path.is_some())
            .count(),
        catalog_path
    ));
    let item_catalog_path = Some(catalog_path);
    let item_catalog_revision = Some(ITEM_CATALOG_REVISION.into());
    let item_catalog_hash = Some(catalog_hash);
    let recipe_catalog_build =
        build_recipe_catalog(&parsed.recipes, &parsed.bases, &source_fingerprint);
    for issue in &recipe_catalog_build.invalid {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!(
                "recipe {:08x} excluded from prepared catalog: {}",
                issue.form_id, issue.message
            ),
        });
    }
    let recipe_artifact = write_recipe_catalog(&cache_dir, &recipe_catalog_build.catalog)?;
    let recipe_cache_state = if recipe_artifact.reused {
        "reused"
    } else {
        "written"
    };
    let recipe_summary = format!(
        "recipe catalog: {} valid, {} invalid, cache {recipe_cache_state} -> {}",
        recipe_catalog_build.catalog.recipes.len(),
        recipe_catalog_build.invalid.len(),
        recipe_artifact.relative_path
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: recipe_summary.clone(),
    });
    output.push(recipe_summary);
    // Issue #175 (M4 wave 11 lane C): content-set-wide like the item/recipe
    // catalogs above (packages are plugin content, not tied to this cell's
    // placements), so it shares their fingerprint-keyed `catalogs/`
    // directory rather than the actor catalog's per-cell path below.
    let package_catalog = build_package_catalog(&package_catalog_inputs, &source_fingerprint);
    // Unlike `item_catalog_path`/`actor_catalog_path`, the manifest carries
    // no dedicated pointer for this catalog: its path is fully deterministic
    // from `source_fingerprint` (`catalogs/<fingerprint>/packages.ron`,
    // exactly what `write_package_catalog` itself constructs) and it is
    // read on demand by the `showpackages` console command rather than
    // preloaded at `view` startup, so there is nothing to validate a
    // manifest-carried hash/revision against.
    let (package_catalog_path_relative, _package_catalog_hash) =
        write_package_catalog(&cache_dir, &package_catalog)?;
    // M4 wave 11 follow-up: real-data acceptance on cell 0001a273 found the
    // original 4-counter line was 100% noise (3021/3021 packages tripped
    // "unsupported subrecord"; 2356/718 "unresolved" were almost entirely
    // out-of-scope PLDT/PTDT FormIDs, not dangling links). `deferred_*`/
    // `out_of_scope_*` carry that volume separately from the two counters
    // that remain genuine signal -- see `package_catalog.rs`'s module doc
    // comment for the measured breakdown.
    let package_catalog_summary = format!(
        "package catalog: {} packages, {} unsupported type, {} unsupported subrecord, {} deferred subrecord, {} unresolved location, {} unresolved target, {} out-of-scope location, {} out-of-scope target -> {}",
        package_catalog.counters.total,
        package_catalog.counters.unsupported_type,
        package_catalog.counters.unsupported_subrecord,
        package_catalog.counters.deferred_subrecord,
        package_catalog.counters.unresolved_location,
        package_catalog.counters.unresolved_target,
        package_catalog.counters.out_of_scope_location,
        package_catalog.counters.out_of_scope_target,
        package_catalog_path_relative
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: package_catalog_summary.clone(),
    });
    output.push(package_catalog_summary);
    // Per-cell artifact next to `scene.ron` -- the actor catalog embeds this
    // cell's ACHR/ACRE placements, so unlike the content-set-wide item/
    // recipe catalogs it must not share one fingerprint-keyed file across
    // cells (each prepare would overwrite the previous cell's actors).
    let actor_catalog_artifact = write_actor_catalog(&cache_dir, cell_id, &actor_catalog)?;
    let actor_catalog_summary = format!(
        "actor catalog: prepared {}, inherited {}, unresolved {}, unsupported {}, skipped {}",
        actor_catalog.counters.prepared,
        actor_catalog.counters.inherited,
        actor_catalog.counters.unresolved,
        actor_catalog.counters.unsupported,
        actor_catalog.counters.skipped
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: actor_catalog_summary.clone(),
    });
    output.push(actor_catalog_summary);
    let actor_catalog_path = Some(actor_catalog_artifact.relative_path);
    let actor_catalog_revision = Some(ACTOR_CATALOG_REVISION.into());
    let actor_catalog_hash = Some(actor_catalog_artifact.hash);
    let actor_animation_catalog_artifact =
        write_actor_animation_catalog(&cache_dir, cell_id, &actor_animation_catalog)?;
    let ready_animation_clips = actor_animation_catalog
        .animation_sets
        .iter()
        .flat_map(|set| set.clips.iter())
        .filter(|clip| {
            clip.status == bevyout_core::actor_animation::PreparedActorAnimationClipStatus::Ready
        })
        .count();
    let actor_animation_catalog_summary = format!(
        "actor animation catalog: {} actor mappings, {} sets, {} ready clips, packs built {}, reused {}, failed clips {}, cache {}",
        actor_animation_catalog.actor_mappings.len(),
        actor_animation_catalog.animation_sets.len(),
        ready_animation_clips,
        actor_animation_conversion.built_packs,
        actor_animation_conversion.reused_packs,
        actor_animation_conversion.failed_clips,
        if actor_animation_catalog_artifact.reused {
            "reused"
        } else {
            "written"
        }
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: actor_animation_catalog_summary.clone(),
    });
    output.push(actor_animation_catalog_summary);
    let actor_animation_catalog_path = Some(actor_animation_catalog_artifact.relative_path);
    let actor_animation_catalog_revision = Some(ACTOR_ANIMATION_CATALOG_REVISION.into());
    let actor_animation_catalog_hash = Some(actor_animation_catalog_artifact.hash);
    let image_space_modifier_catalog =
        build_image_space_modifier_catalog(&parsed.image_space_modifiers, &source_fingerprint);
    let (image_space_modifier_catalog_path, image_space_modifier_catalog_hash) =
        write_image_space_modifier_catalog(&cache_dir, &image_space_modifier_catalog)?;
    let image_space_modifier_summary = format!(
        "ImageSpace modifier catalog: {} records -> {}",
        image_space_modifier_catalog.modifiers.len(),
        image_space_modifier_catalog_path
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: image_space_modifier_summary.clone(),
    });
    output.push(image_space_modifier_summary);
    let dialogue = prepare_authored_dialogue_bundle(
        &cache_dir,
        cell_id,
        &selector_input,
        &args.dialogue_sources,
        &args.dialogue_voice_manifests,
        &placements,
        dialogue_voice_discovery,
        args.dialogue_voice_report.as_deref(),
        &mut diagnostics,
        output,
    )?;
    let mutability_summary = summarize_mutability(&placements);
    let mutability_log = format!(
        "runtime mutability: immutable {}, enable_group {}, script_addressable {}, unknown {}",
        mutability_summary.immutable,
        mutability_summary.enable_group,
        mutability_summary.script_addressable,
        mutability_summary.unknown
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: mutability_log.clone(),
    });
    output.push(mutability_log);
    let failures = placements.iter().filter(|p| p.error.is_some()).count();
    enforce_strict_visual_completeness(args.strict, failures, &visual_issues)?;
    let has_runtime_actor_visual = placements.iter().any(|placement| {
        matches!(
            &placement.semantic,
            PreparedSemantic::Npc(PreparedActor {
                assembly: Some(_),
                ..
            }) | PreparedSemantic::Creature(PreparedActor {
                assembly: Some(_),
                ..
            })
        )
    });
    if placements.iter().all(|p| p.asset_path.is_none())
        && lights.is_empty()
        && !has_runtime_actor_visual
    {
        bail!(
            "no renderable assets were found in {}",
            super::super::manifest::cell_label(&cell)
        )
    }

    progress.phase_started(
        format!("cell {selector_input}/static shadows and reflection probes"),
        None,
    );
    let static_point_shadows = prepare_static_point_shadows(
        StaticShadowPrepareOptions {
            asset_root: &cache_dir,
            scene_dir: &scene_dir,
            resolution: args.shadow_resolution,
            rebuild: args.rebuild_shadows,
            ktx: args.toktx.clone(),
        },
        &placements,
        &lights,
        &mut diagnostics,
        Some(progress),
        &format!("cell {selector_input}/static point shadows"),
    )?;
    let reflection_layouts = if cell.interior {
        nav_graph_full
            .as_ref()
            .map_or_else(Vec::new, reflection_probe_layouts)
    } else {
        Vec::new()
    };
    let reflection_probes = if cell.interior {
        prepare_reflection_probes(
            ReflectionProbePrepareOptions {
                asset_root: &cache_dir,
                scene_dir: &scene_dir,
                rebuild: args.rebuild_reflection_probes,
                ktx: args.toktx,
            },
            &reflection_layouts,
            &placements,
            &lights,
            cell.effective_lighting
                .as_ref()
                .expect("effective cell lighting is prepared before reflection probes"),
            &mut diagnostics,
        )?
    } else {
        None
    };

    progress.phase_started(format!("cell {selector_input}/manifest publication"), None);
    let manifest = PreparedSceneManifest {
        schema_version: CURRENT_MANIFEST_SCHEMA_VERSION,
        prepare_revision: Some(CURRENT_PREPARE_REVISION.into()),
        converter_revision: Some(prepared_converter_revision.into()),
        physics_schema_version: Some(PHYSICS_ASSET_SCHEMA_VERSION),
        asset_root: cache_dir.to_string_lossy().to_string(),
        source_plugin: plugin_path.to_string_lossy().to_string(),
        source_fingerprint,
        item_catalog_path,
        item_catalog_revision,
        item_catalog_hash,
        recipe_catalog_path: Some(recipe_artifact.relative_path),
        recipe_catalog_revision: Some(RECIPE_CATALOG_REVISION.into()),
        recipe_catalog_hash: Some(recipe_artifact.hash),
        actor_catalog_path,
        actor_catalog_revision,
        actor_catalog_hash,
        actor_animation_catalog_path,
        actor_animation_catalog_revision,
        actor_animation_catalog_hash,
        image_space_modifier_catalog_path: Some(image_space_modifier_catalog_path),
        image_space_modifier_catalog_revision: Some(IMAGE_SPACE_MODIFIER_CATALOG_REVISION.into()),
        image_space_modifier_catalog_hash: Some(image_space_modifier_catalog_hash),
        source_plugins,
        cell,
        placements,
        lights,
        diagnostics,
        visual_issues,
        navmeshes,
        nav_graph,
        cell_audio,
        audio_clips,
        footstep_sets,
        hard_landing_clips,
        mutability_summary,
        bake: None,
        static_point_shadows,
        reflection_probes,
        leveled_lists,
        dialogue,
        exterior: None,
    };
    let manifest_path = scene_dir.join("scene.ron");
    fs::write(
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    output.push(format!(
        "prepared {} ({} placements, {} unresolved, {} visual issue(s)) -> {}",
        super::super::manifest::cell_label(&manifest.cell),
        manifest.placements.len(),
        failures,
        manifest.visual_issues.len(),
        manifest_path.display()
    ));
    Ok(())
}

fn finalize_exterior_indexes(session: &BatchSession, cache_dir: &Path) -> Result<()> {
    for template in session.exterior_indexes.iter() {
        publish_exterior_index(session, cache_dir, template)?;
    }
    Ok(())
}

fn finalize_exterior_index(
    session: &BatchSession,
    cache_dir: &Path,
    worldspace_form_id: u32,
) -> Result<()> {
    let template = session
        .exterior_indexes
        .iter()
        .find(|index| index.worldspace_form_id == worldspace_form_id)
        .with_context(|| format!("missing exterior worldspace index {worldspace_form_id:08x}"))?;
    publish_exterior_index(session, cache_dir, template)
}

fn publish_exterior_index(
    session: &BatchSession,
    cache_dir: &Path,
    template: &bevyout_core::manifest::exterior::ExteriorWorldspaceIndex,
) -> Result<()> {
    let persistent_assets = session.persistent_assets.lock().unwrap().clone();
    let worldspace_lod = session.worldspace_lod_cache.lock().unwrap().clone();
    let mut index = template.clone();
    if let Some(lod_assets) = worldspace_lod.get(&index.worldspace_form_id) {
        index.worldspace_lod = lod_assets.clone();
    }
    apply_staged_persistent_assets(&mut index, &persistent_assets);
    let index_path = cache_dir
        .join("worldspaces")
        .join(format!("{:08x}", index.worldspace_form_id))
        .join("index.ron");
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }
    merge_existing_persistent_assets(&index_path, &mut index);
    index.sort_deterministically();
    fs::write(
        &index_path,
        to_string_pretty(&index, PrettyConfig::default())?,
    )?;
    Ok(())
}

fn apply_staged_persistent_assets(
    index: &mut bevyout_core::manifest::exterior::ExteriorWorldspaceIndex,
    updates: &HashMap<u32, PersistentAssetUpdate>,
) {
    let mut unavailable = Vec::new();
    for reference in &mut index.persistent_references {
        reference.asset_path = updates
            .get(&reference.reference_form_id)
            .and_then(|update| update.asset_path.clone());
        if let Some(error) = updates
            .get(&reference.reference_form_id)
            .and_then(|update| update.error.as_deref())
        {
            unavailable.push((reference.reference_form_id, error.to_owned()));
        }
    }
    for (reference_form_id, error) in unavailable {
        index
            .diagnostics
            .push(bevyout_core::manifest::exterior::ExteriorDiagnostic {
                code: "persistent_asset_unavailable".into(),
                form_id: Some(reference_form_id),
                severity: "warning".into(),
                message: error,
            });
    }
}

fn merge_existing_persistent_assets(
    index_path: &Path,
    index: &mut bevyout_core::manifest::exterior::ExteriorWorldspaceIndex,
) {
    let Ok(bytes) = fs::read(index_path) else {
        return;
    };
    let Ok(previous) =
        ron::de::from_bytes::<bevyout_core::manifest::exterior::ExteriorWorldspaceIndex>(&bytes)
    else {
        return;
    };
    if previous.revision != index.revision
        || previous.content_fingerprint != index.content_fingerprint
    {
        return;
    }
    let prepared = previous
        .persistent_references
        .into_iter()
        .filter_map(|reference| {
            reference
                .asset_path
                .filter(|path| path.to_ascii_lowercase().ends_with(".glb"))
                .map(|path| (reference.reference_form_id, path))
        })
        .collect::<HashMap<_, _>>();
    for reference in &mut index.persistent_references {
        if reference.asset_path.is_none() {
            reference.asset_path = prepared.get(&reference.reference_form_id).cloned();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn prepare_authored_dialogue_bundle(
    asset_root: &Path,
    cell_form_id: u32,
    selector: &str,
    source_paths: &[PathBuf],
    voice_manifest_paths: &[PathBuf],
    placements: &[PreparedPlacement],
    discovery: Option<DialogueVoiceDiscovery>,
    report_path: Option<&Path>,
    diagnostics: &mut Vec<Diagnostic>,
    output: &mut Vec<String>,
) -> Result<Option<bevyout_core::dialogue::PreparedDialogueBundleRef>> {
    if source_paths.is_empty() && discovery.is_none() {
        if !voice_manifest_paths.is_empty() {
            bail!("dialogue voice manifests require at least one --dialogue-source");
        }
        return Ok(None);
    }

    let workspace =
        fs::canonicalize(std::env::current_dir().context("resolving dialogue source workspace")?)
            .context("canonicalizing dialogue source workspace")?;
    let mut sources = Vec::with_capacity(source_paths.len());
    for source_path in source_paths {
        if source_path.is_absolute() {
            bail!(
                "dialogue source must be a workspace-relative path: {}",
                source_path.display()
            );
        }
        let requested = source_path.to_string_lossy().replace('\\', "/");
        let relative = requested
            .strip_prefix("dialogue/")
            .unwrap_or(&requested)
            .to_owned();
        if relative.starts_with("../")
            || relative == ".."
            || relative.starts_with('/')
            || Path::new(&relative)
                .components()
                .any(|component| component == std::path::Component::ParentDir)
        {
            bail!(
                "dialogue source must be a workspace-relative path: {}",
                source_path.display()
            );
        }
        let absolute = fs::canonicalize(workspace.join(source_path))
            .with_context(|| format!("reading dialogue source {}", source_path.display()))?;
        if !absolute.starts_with(&workspace) {
            bail!(
                "dialogue source must stay inside the workspace: {}",
                source_path.display()
            );
        }
        let content = fs::read_to_string(&absolute)
            .with_context(|| format!("reading dialogue source {}", absolute.display()))?;
        let relative = if relative.starts_with("authored/") {
            relative
        } else {
            format!("authored/{relative}")
        };
        sources.push(crate::vsa::dialogue::DialogueSource {
            relative_path: relative,
            kind: crate::vsa::dialogue::DialogueSourceKind::Authored,
            content,
        });
    }
    if source_paths.is_empty() {
        let preserved = read_existing_authored_dialogue_sources(asset_root, cell_form_id)?;
        if !preserved.is_empty() {
            let message = format!(
                "dialogue sources preserved: {} authored source(s) from the existing cell bundle",
                preserved.len()
            );
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: message.clone(),
            });
            output.push(message);
            sources.extend(preserved);
        }
    }

    let dialogue_scope = discovery
        .as_ref()
        .map(|discovery| format!("scenes/{:08x}", discovery.demand_report.cell_form_id));
    let dialogue_output_root = dialogue_scope
        .as_deref()
        .map(|scope| asset_root.join(scope))
        .unwrap_or_else(|| asset_root.to_owned());
    let explicit_voice_input = if voice_manifest_paths.is_empty() {
        read_existing_authored_voice_input(asset_root, cell_form_id)?
    } else {
        read_dialogue_voice_input(&workspace, voice_manifest_paths)?
    };
    let (demand_report, discovered_voice_input, actor_bindings) = discovery
        .map(|discovery| {
            sources.extend(discovery.generated_sources);
            (
                Some(discovery.demand_report),
                Some(discovery.voice_input),
                discovery.actor_bindings,
            )
        })
        .unwrap_or((None, None, Vec::new()));
    let voice_input = merge_dialogue_voice_inputs(explicit_voice_input, discovered_voice_input);
    if sources.is_empty() {
        if let Some(report) = demand_report {
            write_dialogue_voice_demand_report(&dialogue_output_root, &report)?;
            write_dialogue_voice_report(report_path, &report)?;
        }
        return Ok(None);
    }
    let prepared =
        crate::vsa::dialogue::prepare_dialogue_bundle_with_voice_demand_and_bindings_scoped(
            asset_root,
            sources,
            voice_input,
            demand_report,
            dialogue_scope.as_deref(),
            actor_bindings,
        )?;
    if let Some(report_path) = report_path
        && let Some(bundle) = prepared.bundle.as_ref()
        && let Some(demand_path) = bundle.voice_demand_path.as_deref()
    {
        let bytes =
            fs::read(asset_root.join(demand_path.replace('/', std::path::MAIN_SEPARATOR_STR)))
                .with_context(|| format!("reading prepared dialogue voice demand {demand_path}"))?;
        write_dialogue_voice_report_bytes(report_path, &bytes)?;
    }
    let bundle = prepared.bundle.clone();
    let binding_count = placements
        .iter()
        .filter(|placement| {
            prepared.catalog.actor_bindings.iter().any(|binding| {
                binding.actor_reference_form_id == placement.reference_form_id
                    && binding.actor_base_form_id == placement.base_form_id
            }) || placement.editor_id.as_ref().is_some_and(|editor_id| {
                prepared
                    .catalog
                    .conversations
                    .keys()
                    .any(|dialogue| dialogue.as_str() == editor_id)
            })
        })
        .count();
    let voice_index = prepared
        .bundle
        .as_ref()
        .and_then(|bundle| bundle.voice_index_path.as_deref())
        .and_then(|path| {
            fs::read(asset_root.join(path.replace('/', std::path::MAIN_SEPARATOR_STR)))
                .ok()
                .and_then(|bytes| ron::de::from_bytes(&bytes).ok())
        });
    let coverage = crate::vsa::dialogue::coverage::assess_voice_coverage(
        asset_root,
        &prepared.catalog,
        voice_index.as_ref(),
    );
    let voice_coverage = coverage.summary();
    diagnostics.push(Diagnostic {
        severity: if coverage.is_ready() {
            "info"
        } else {
            "warning"
        }
        .into(),
        message: voice_coverage.clone(),
    });
    output.push(voice_coverage);
    if !coverage.is_ready() {
        let missing = coverage.missing_labels().join(", ");
        let missing_line = format!("dialogue voice missing keys: {missing}");
        let repair_guidance = crate::vsa::dialogue::coverage::voice_repair_guidance(
            selector,
            &prepared.catalog,
            &coverage,
        );
        let next_line = format!("dialogue voice {repair_guidance}");
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: missing_line.clone(),
        });
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: next_line.clone(),
        });
        output.push(missing_line);
        output.push(next_line);
    }
    let voice_line_count = coverage.mapped_lines;
    let summary = format!(
        "dialogue bundle: {} conversation(s), {} source(s), {} voice line(s), {} stable placement binding(s) -> {}",
        prepared.catalog.conversations.len(),
        prepared.catalog.source_paths.len(),
        voice_line_count,
        binding_count,
        bundle
            .as_ref()
            .map(|bundle| bundle.catalog_path.as_str())
            .unwrap_or("<none>"),
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: summary.clone(),
    });
    output.push(summary);
    Ok(bundle)
}

fn read_existing_authored_dialogue_sources(
    asset_root: &Path,
    cell_form_id: u32,
) -> Result<Vec<crate::vsa::dialogue::DialogueSource>> {
    let dialogue_root = asset_root
        .join("scenes")
        .join(format!("{cell_form_id:08x}"))
        .join("dialogue");
    let catalog_path = dialogue_root.join("catalog.ron");
    if !catalog_path.is_file() {
        return Ok(Vec::new());
    }
    let catalog = ron::de::from_bytes::<crate::vsa::dialogue::PreparedDialogueCatalog>(
        &fs::read(&catalog_path).with_context(|| {
            format!(
                "reading existing dialogue catalog {}",
                catalog_path.display()
            )
        })?,
    )
    .with_context(|| {
        format!(
            "parsing existing dialogue catalog {}",
            catalog_path.display()
        )
    })?;
    let mut sources = Vec::new();
    for source_path in catalog
        .source_paths
        .iter()
        .filter(|path| path.starts_with("authored/"))
    {
        let path = dialogue_root.join(source_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let content = fs::read_to_string(&path)
            .with_context(|| format!("reading preserved dialogue source {}", path.display()))?;
        sources.push(crate::vsa::dialogue::DialogueSource {
            relative_path: source_path.clone(),
            kind: crate::vsa::dialogue::DialogueSourceKind::Authored,
            content,
        });
    }
    sources.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(sources)
}

fn read_existing_authored_voice_input(
    asset_root: &Path,
    cell_form_id: u32,
) -> Result<Option<crate::vsa::dialogue::DialogueVoiceInput>> {
    let index_path = asset_root
        .join("scenes")
        .join(format!("{cell_form_id:08x}"))
        .join("dialogue/voice_index.ron");
    if !index_path.is_file() {
        return Ok(None);
    }
    let index = ron::de::from_bytes::<bevyout_core::dialogue::PreparedDialogueVoiceIndex>(
        &fs::read(&index_path)
            .with_context(|| format!("reading existing voice index {}", index_path.display()))?,
    )
    .with_context(|| format!("parsing existing voice index {}", index_path.display()))?;
    let source_manifest_path = index.source_manifest_path.clone();
    let mut entries = Vec::new();
    for voice in index.entries.into_iter().filter(|voice| {
        voice.speaker_form_id.is_none() && !voice.line_key.as_str().starts_with("fallout:")
    }) {
        let staged_path =
            asset_root.join(voice.asset_path.replace('/', std::path::MAIN_SEPARATOR_STR));
        let bytes = fs::read(&staged_path).with_context(|| {
            format!("reading preserved dialogue voice {}", staged_path.display())
        })?;
        entries.push(crate::vsa::dialogue::DialogueVoiceInputEntry {
            line_key: voice.line_key,
            source_path: voice.source_path.unwrap_or(voice.asset_path),
            bytes,
            source_fingerprint: voice.source_fingerprint,
            source_origin: voice.source_origin,
            speaker_form_id: None,
            voice_type_form_id: voice.voice_type_form_id,
        });
    }
    if entries.is_empty() {
        Ok(None)
    } else {
        Ok(Some(crate::vsa::dialogue::DialogueVoiceInput {
            manifest_path: source_manifest_path,
            cell_form_id: Some(cell_form_id),
            entries,
        }))
    }
}

fn merge_dialogue_voice_inputs(
    explicit: Option<crate::vsa::dialogue::DialogueVoiceInput>,
    discovered: Option<crate::vsa::dialogue::DialogueVoiceInput>,
) -> Option<crate::vsa::dialogue::DialogueVoiceInput> {
    match (explicit, discovered) {
        (None, None) => None,
        (Some(input), None) | (None, Some(input)) => Some(input),
        (Some(mut explicit), Some(discovered)) => {
            let explicit_keys = explicit
                .entries
                .iter()
                .map(|entry| (entry.line_key.clone(), entry.speaker_form_id))
                .collect::<HashSet<_>>();
            explicit
                .entries
                .extend(discovered.entries.into_iter().filter(|entry| {
                    !explicit_keys.contains(&(entry.line_key.clone(), entry.speaker_form_id))
                }));
            explicit.entries.sort_by(|left, right| {
                left.line_key
                    .cmp(&right.line_key)
                    .then_with(|| left.speaker_form_id.cmp(&right.speaker_form_id))
            });
            explicit.manifest_path =
                format!("{}|{}", explicit.manifest_path, discovered.manifest_path);
            explicit.cell_form_id = discovered.cell_form_id.or(explicit.cell_form_id);
            Some(explicit)
        }
    }
}

fn should_discover_dialogue_voice(_legacy_explicit_flag: bool) -> bool {
    true
}

fn write_dialogue_voice_demand_report(
    asset_root: &Path,
    report: &bevyout_core::dialogue::PreparedDialogueVoiceDemandReport,
) -> Result<()> {
    let path = asset_root.join("dialogue/voice_demand.ron");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        path,
        ron::ser::to_string_pretty(report, ron::ser::PrettyConfig::default())?,
    )?;
    Ok(())
}

fn write_dialogue_voice_report(
    report_path: Option<&Path>,
    report: &bevyout_core::dialogue::PreparedDialogueVoiceDemandReport,
) -> Result<()> {
    let Some(report_path) = report_path else {
        return Ok(());
    };
    let bytes = ron::ser::to_string_pretty(report, ron::ser::PrettyConfig::default())?;
    write_dialogue_voice_report_bytes(report_path, bytes.as_bytes())
}

fn write_dialogue_voice_report_bytes(report_path: &Path, bytes: &[u8]) -> Result<()> {
    let report_path = absolutize(report_path)?;
    let bevyout_root = absolutize(Path::new(".bevyout"))?;
    if !report_path.starts_with(&bevyout_root) {
        bail!(
            "dialogue voice report must stay inside .bevyout: {}",
            report_path.display()
        );
    }
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(report_path, bytes)?;
    Ok(())
}

fn read_dialogue_voice_input(
    workspace: &Path,
    manifest_paths: &[PathBuf],
) -> Result<Option<crate::vsa::dialogue::DialogueVoiceInput>> {
    if manifest_paths.is_empty() {
        return Ok(None);
    }

    let mut requested_paths = manifest_paths
        .iter()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect::<Vec<_>>();
    requested_paths.sort();
    requested_paths.dedup();

    let mut entries = Vec::new();
    for requested in &requested_paths {
        let path = Path::new(requested);
        if path.is_absolute()
            || path.components().any(|component| {
                component == std::path::Component::ParentDir
                    || component == std::path::Component::RootDir
            })
        {
            bail!(
                "dialogue voice manifest must be workspace-relative: {}",
                requested
            );
        }
        let absolute = fs::canonicalize(workspace.join(path))
            .with_context(|| format!("reading dialogue voice manifest {requested}"))?;
        if !absolute.starts_with(workspace) {
            bail!(
                "dialogue voice manifest must stay inside the workspace: {}",
                requested
            );
        }
        let content = fs::read_to_string(&absolute)
            .with_context(|| format!("reading dialogue voice manifest {requested}"))?;
        let manifest = crate::vsa::dialogue::parse_voice_manifest(&content)
            .with_context(|| format!("validating dialogue voice manifest {requested}"))?;
        for entry in manifest.entries {
            let source_path = Path::new(&entry.source_path);
            if source_path.is_absolute()
                || source_path.components().any(|component| {
                    component == std::path::Component::ParentDir
                        || component == std::path::Component::RootDir
                })
            {
                bail!(
                    "dialogue voice source must be workspace-relative: {}",
                    entry.source_path
                );
            }
            let source_absolute = fs::canonicalize(workspace.join(source_path))
                .with_context(|| format!("reading dialogue voice asset {}", entry.source_path))?;
            if !source_absolute.starts_with(workspace) {
                bail!(
                    "dialogue voice source must stay inside the workspace: {}",
                    entry.source_path
                );
            }
            entries.push(crate::vsa::dialogue::DialogueVoiceInputEntry {
                line_key: entry.line_key,
                source_path: entry.source_path.replace('\\', "/"),
                bytes: fs::read(&source_absolute).with_context(|| {
                    format!("reading dialogue voice asset {}", entry.source_path)
                })?,
                source_fingerprint: None,
                source_origin: None,
                speaker_form_id: None,
                voice_type_form_id: None,
            });
        }
    }

    Ok(Some(crate::vsa::dialogue::DialogueVoiceInput {
        manifest_path: requested_paths.join("|"),
        cell_form_id: None,
        entries,
    }))
}

fn resolve_authoritative_day_night_profile(
    cell: &super::super::manifest::CellInfo,
    parsed: &ParsedPlugin,
) -> Option<PreparedDayNightProfile> {
    let climate_form_id = parsed
        .cell_metadata
        .as_ref()
        .and_then(|metadata| metadata.climate_form_id)
        .or_else(|| {
            cell.worldspace_form_id.and_then(|worldspace_form_id| {
                resolve_worldspace_climate(worldspace_form_id, &parsed.worldspaces)
            })
        })?;
    let climate = parsed.climates.get(&climate_form_id)?;
    let weather = climate
        .weather_entries
        .iter()
        .find(|entry| entry.chance > 0 && parsed.weathers.contains_key(&entry.weather_form_id))
        .and_then(|entry| parsed.weathers.get(&entry.weather_form_id))?;
    Some(prepared_day_night_profile(
        Some(climate),
        weather,
        PreparedDayNightProfileSource::Authoritative,
    ))
}

fn resolve_worldspace_climate(
    worldspace_form_id: u32,
    worldspaces: &HashMap<u32, WorldspaceRecord>,
) -> Option<u32> {
    let mut current = worldspace_form_id;
    let mut visited = HashSet::new();
    while visited.insert(current) {
        let worldspace = worldspaces.get(&current)?;
        if worldspace.parent_flags & WorldspaceRecord::USE_PARENT_CLIMATE != 0 {
            current = worldspace.parent_form_id?;
            continue;
        }
        return worldspace.climate_form_id;
    }
    None
}

fn resolve_preview_day_night_profile(parsed: &ParsedPlugin) -> Option<PreparedDayNightProfile> {
    let candidates = parsed
        .weathers
        .values()
        .map(|weather| (weather.form_id, weather.editor_id.clone()))
        .collect::<Vec<_>>();
    let weather_form_id = bevyout_core::time_of_day::select_preview_weather(&candidates)?;
    let weather = parsed.weathers.get(&weather_form_id)?;
    let climate = parsed
        .climates
        .values()
        .filter(|climate| {
            climate
                .weather_entries
                .iter()
                .any(|entry| entry.chance > 0 && entry.weather_form_id == weather.form_id)
        })
        .min_by_key(|climate| climate.form_id);
    Some(prepared_day_night_profile(
        climate,
        weather,
        PreparedDayNightProfileSource::PreviewFallback,
    ))
}

fn prepared_day_night_profile(
    climate: Option<&ClimateRecord>,
    weather: &WeatherRecord,
    source: PreparedDayNightProfileSource,
) -> PreparedDayNightProfile {
    PreparedDayNightProfile {
        climate_form_id: climate.map(|value| value.form_id),
        climate_editor_id: climate.and_then(|value| value.editor_id.clone()),
        weather_form_id: weather.form_id,
        weather_editor_id: weather.editor_id.clone(),
        timings: climate.map(|value| value.timings).unwrap_or_default(),
        sky_upper: weather.sky_upper,
        sky_lower: weather.sky_lower,
        ambient: weather.ambient,
        sunlight: weather.sunlight,
        source,
    }
}

// ---------------------------------------------------------------------
// Actor catalog wiring (issue #103, M4 wave 1 task C)
//
// `actor_catalog.rs` is a pure std/serde-only module (no `openmw_esm4`
// imports, so it can be pulled into `tests/features.rs` verbatim -- see its
// module doc comment). The boundary conversion from the parser's
// `ActorData`/`RaceRecord`/`ClassRecord`/`FactionRecord`/`PackageRecord`/
// `ReferenceRecord` types into its plain input types lives here, the same
// way `cell_map::CellMap::build` is fed by `catalog.rs` rather than
// importing the ESM4 reader itself.
// ---------------------------------------------------------------------

/// Computes which of the 10 documented `ACBS.template_flags` bits (FO3
/// fopdoc's `ACBS` page) this actor's own record sets, consuming
/// `ActorBaseConfig::uses_template_flag`/`TEMPLATE_USE_*` so the pure
/// catalog resolver never needs the bitmask constants themselves.
fn actor_template_usage(config: Option<ActorBaseConfig>) -> ActorTemplateUsage {
    let Some(config) = config else {
        return ActorTemplateUsage::default();
    };
    let known_mask = ActorBaseConfig::TEMPLATE_USE_TRAITS
        | ActorBaseConfig::TEMPLATE_USE_STATS
        | ActorBaseConfig::TEMPLATE_USE_FACTIONS
        | ActorBaseConfig::TEMPLATE_USE_ACTOR_EFFECT_LIST
        | ActorBaseConfig::TEMPLATE_USE_AI_DATA
        | ActorBaseConfig::TEMPLATE_USE_AI_PACKAGES
        | ActorBaseConfig::TEMPLATE_USE_MODEL_ANIMATION
        | ActorBaseConfig::TEMPLATE_USE_BASE_DATA
        | ActorBaseConfig::TEMPLATE_USE_INVENTORY
        | ActorBaseConfig::TEMPLATE_USE_SCRIPT;
    ActorTemplateUsage {
        traits: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_TRAITS),
        stats: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_STATS),
        factions: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_FACTIONS),
        actor_effect_list: config
            .uses_template_flag(ActorBaseConfig::TEMPLATE_USE_ACTOR_EFFECT_LIST),
        ai_data: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_AI_DATA),
        ai_packages: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_AI_PACKAGES),
        model_animation: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_MODEL_ANIMATION),
        base_data: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_BASE_DATA),
        inventory: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_INVENTORY),
        script: config.uses_template_flag(ActorBaseConfig::TEMPLATE_USE_SCRIPT),
        unsupported_bits: config.template_flags & !known_mask,
    }
}

/// Converts one `NPC_`/`CREA` `BaseRecord` into the pure module's plain
/// `ActorRecordInput`, or `None` for any other record kind (or an `NPC_`/
/// `CREA` record whose `ActorData` failed to decode, which cannot happen
/// today since task A always attaches `Some(ActorData)` for those kinds,
/// but is handled defensively rather than panicking). `form_id` is filled
/// in by the caller since `BaseRecord` does not carry its own FormID.
fn actor_record_input(
    base: &BaseRecord,
    all_bases: &HashMap<u32, BaseRecord>,
) -> Option<ActorRecordInput> {
    let kind = match base.kind.as_str() {
        "NPC_" => ActorRecordKind::Npc,
        "CREA" => ActorRecordKind::Creature,
        _ => return None,
    };
    let actor = base.actor.as_ref()?;
    let config = actor.base_config;
    let acbs_flags = config.map(|config| config.flags).unwrap_or_default();
    // Bit 0x1 of ACBS.flags is the standard "Female" flag shared by ACBS
    // across Bethesda's ESM/ESM4 games and confirmed against the FO3 fopdoc
    // ACBS page; it travels with the `traits` group per this task's brief
    // ("race/sex-relevant traits"), not `base_data`.
    let female = acbs_flags & 0x1 != 0;
    let traits = ActorTraits {
        race_form_id: actor.race_form_id,
        female,
        height: actor.height,
        weight: actor.weight,
        hair_form_id: actor.hair_form_id,
        eyes_form_id: actor.eyes_form_id,
        head_part_form_ids: actor.head_part_form_ids.clone(),
        voice_form_id: actor.voice_form_id,
        facegen: FaceGenRaw {
            geometry_symmetric: actor.facegen_geometry_symmetric.clone(),
            geometry_asymmetric: actor.facegen_geometry_asymmetric.clone(),
            texture_symmetric: actor.facegen_texture_symmetric.clone(),
        },
        facegen_present: actor.facegen_geometry_symmetric.is_some()
            || actor.facegen_geometry_asymmetric.is_some()
            || actor.facegen_texture_symmetric.is_some(),
    };
    let mut stats = ActorStats {
        level_or_mult: config
            .map(|config| config.level_or_mult)
            .unwrap_or_default(),
        calc_min_level: config
            .map(|config| config.calc_min_level)
            .unwrap_or_default(),
        calc_max_level: config
            .map(|config| config.calc_max_level)
            .unwrap_or_default(),
        speed_multiplier: config
            .map(|config| config.speed_multiplier)
            .unwrap_or_default(),
        karma: config.map(|config| config.karma).unwrap_or_default(),
        disposition_base: config
            .map(|config| config.disposition_base)
            .unwrap_or_default(),
        fatigue: config.map(|config| config.fatigue).unwrap_or_default(),
        barter_gold: config.map(|config| config.barter_gold).unwrap_or_default(),
        ..ActorStats::default()
    };
    if let Some(npc_stats) = actor.base_stats {
        stats.health = Some(npc_stats.base_health);
        stats.special = Some([
            npc_stats.strength,
            npc_stats.perception,
            npc_stats.endurance,
            npc_stats.charisma,
            npc_stats.intelligence,
            npc_stats.agility,
            npc_stats.luck,
        ]);
    }
    stats.npc_skill_values = actor.skills.map(|skills| skills.values);
    if let Some(creature_stats) = actor.creature.as_ref().and_then(|creature| creature.stats) {
        stats.health = Some(i32::from(creature_stats.health));
        stats.special = Some([
            creature_stats.strength,
            creature_stats.perception,
            creature_stats.endurance,
            creature_stats.charisma,
            creature_stats.intelligence,
            creature_stats.agility,
            creature_stats.luck,
        ]);
        stats.creature_type = Some(creature_stats.creature_type);
        stats.combat_skill = Some(creature_stats.combat_skill);
        stats.magic_skill = Some(creature_stats.magic_skill);
        stats.stealth_skill = Some(creature_stats.stealth_skill);
        stats.creature_damage = Some(creature_stats.damage);
    }
    let factions = actor
        .factions
        .iter()
        .map(|membership| ActorFactionInput {
            faction_form_id: membership.faction_form_id,
            rank: membership.rank,
        })
        .collect();
    let model_animation = ActorModelAnimation {
        model_path: base.model.clone(),
        creature_model_list: actor
            .creature
            .as_ref()
            .map(|creature| creature.model_list.clone())
            .unwrap_or_default(),
        animation_files: actor.animation_files.clone(),
        creature_base_scale: actor
            .creature
            .as_ref()
            .and_then(|creature| creature.base_scale),
    };
    let base_data = ActorBaseData {
        name: base.name.clone(),
        acbs_flags,
    };
    let inventory = base
        .inventory
        .iter()
        .map(|item| prepared_inventory_entry(item, all_bases))
        .collect();
    Some(ActorRecordInput {
        form_id: 0,
        kind,
        editor_id: base.editor_id.clone(),
        base_template_form_id: base.base_template_form_id,
        template_usage: actor_template_usage(config),
        traits,
        stats,
        factions,
        actor_effect_form_id: actor.unarmed_attack_effect_form_id,
        ai_data: actor.ai_data.map(|data| ActorAiDataInput {
            aggression: data.aggression,
            confidence: data.confidence,
            energy_level: data.energy_level,
            responsibility: data.responsibility,
            mood: data.mood,
            services: data.services,
            teaches: data.teaches,
            max_training_level: data.max_training_level,
            assistance: data.assistance,
            aggro_radius_behavior: data.aggro_radius_behavior,
            aggro_radius: data.aggro_radius,
        }),
        package_form_ids: actor.package_form_ids.clone(),
        model_animation,
        base_data,
        inventory,
        script_form_id: actor.script_form_id,
        class_form_id: actor.class_form_id,
        combat_style_form_id: actor.combat_style_form_id,
        death_item_form_id: actor.death_item_form_id,
    })
}

/// Assembles the pure `ActorCatalogInputs` for one prepared cell:
/// `NPC_`/`CREA` bases and `LVLN`/`LVLC` leveled lists both come from
/// `parsed.bases` (task A/B's parser puts every decoded base record kind in
/// the same map), `races`/`classes`/`factions`/`packages` come from their
/// own maps for existence checks and faction rank titles, and
/// `known_bases` is every decoded base FormID, including the appearance
/// `HAIR`/`EYES`/`HDPT` records used for model/texture-link checks.
fn build_actor_catalog_inputs(
    parsed: &ParsedPlugin,
    actor_references: &[ReferenceRecord],
) -> ActorCatalogInputs {
    let mut actors = HashMap::new();
    let mut leveled = HashMap::new();
    let known_bases = parsed.bases.keys().copied().collect::<HashSet<_>>();
    for (&form_id, base) in &parsed.bases {
        if let Some(data) = &base.leveled {
            leveled.insert(
                form_id,
                LeveledInput {
                    form_id,
                    entries: data
                        .entries
                        .iter()
                        .map(|entry| entry.item_form_id)
                        .collect(),
                },
            );
            continue;
        }
        if let Some(mut input) = actor_record_input(base, &parsed.bases) {
            input.form_id = form_id;
            actors.insert(form_id, input);
        }
    }
    let races = parsed.races.keys().copied().collect::<HashSet<_>>();
    let race_modifiers = parsed
        .races
        .iter()
        .map(|(&form_id, race)| {
            let mut modifiers = RaceModifierInput::default();
            for boost in &race.skill_boosts {
                let Some(skill) = fo3_skill_actor_value(boost.actor_value) else {
                    modifiers.diagnostics.push(format!(
                        "unsupported race actor value {} with boost {}",
                        boost.actor_value, boost.boost
                    ));
                    continue;
                };
                *modifiers
                    .skill_modifiers
                    .entry(ActorValue::Skill(skill))
                    .or_default() += f32::from(boost.boost);
            }
            modifiers.facegen_male = FaceGenRaw {
                geometry_symmetric: race.face_gen_male.geometry_symmetric.clone(),
                geometry_asymmetric: race.face_gen_male.geometry_asymmetric.clone(),
                texture_symmetric: race.face_gen_male.texture_symmetric.clone(),
            };
            modifiers.facegen_female = FaceGenRaw {
                geometry_symmetric: race.face_gen_female.geometry_symmetric.clone(),
                geometry_asymmetric: race.face_gen_female.geometry_asymmetric.clone(),
                texture_symmetric: race.face_gen_female.texture_symmetric.clone(),
            };
            modifiers.diagnostics.sort();
            modifiers.diagnostics.dedup();
            (form_id, modifiers)
        })
        .collect();
    let classes = parsed.classes.keys().copied().collect::<HashSet<_>>();
    let packages = parsed.packages.keys().copied().collect::<HashSet<_>>();
    let factions = parsed
        .factions
        .iter()
        .map(|(&form_id, faction)| {
            let ranks = faction
                .ranks
                .iter()
                .map(|rank| FactionRankInput {
                    rank_number: rank.rank_number,
                    male_title: rank.male_title.clone(),
                    female_title: rank.female_title.clone(),
                })
                .collect();
            let relations = faction
                .relations
                .iter()
                .map(|relation| FactionRelationInput {
                    faction_form_id: relation.faction_form_id,
                    modifier: relation.modifier,
                    group_combat_reaction: relation.group_combat_reaction,
                })
                .collect();
            (
                form_id,
                FactionInput {
                    form_id,
                    editor_id: faction.editor_id.clone(),
                    name: faction.name.clone(),
                    ranks,
                    relations,
                },
            )
        })
        .collect();
    let placements = actor_references
        .iter()
        .filter_map(|reference| {
            let kind = match reference.kind {
                ReferenceKind::Npc => ActorRecordKind::Npc,
                ReferenceKind::Creature => ActorRecordKind::Creature,
                ReferenceKind::Object => return None,
            };
            let transform = placement_transform(reference);
            Some(ActorPlacementInput {
                reference_form_id: reference.form_id,
                base_form_id: reference.base_form_id,
                kind,
                translation: transform.0,
                rotation_xyzw: transform.1,
                scale: transform.2,
                initially_enabled: reference.initially_enabled,
            })
        })
        .collect();
    ActorCatalogInputs {
        actors,
        leveled,
        races,
        race_modifiers,
        classes,
        factions,
        packages,
        known_bases,
        placements,
    }
}

fn fo3_skill_actor_value(actor_value: i8) -> Option<ActorSkill> {
    Some(match actor_value {
        32 => ActorSkill::Barter,
        33 => ActorSkill::BigGuns,
        34 => ActorSkill::EnergyWeapons,
        35 => ActorSkill::Explosives,
        36 => ActorSkill::Lockpick,
        37 => ActorSkill::Medicine,
        38 => ActorSkill::MeleeWeapons,
        39 => ActorSkill::Repair,
        40 => ActorSkill::Science,
        41 => ActorSkill::SmallGuns,
        42 => ActorSkill::Sneak,
        43 => ActorSkill::Speech,
        44 => ActorSkill::Throwing,
        45 => ActorSkill::Unarmed,
        _ => return None,
    })
}

#[allow(clippy::too_many_arguments)]
fn stage_placements_with_package_points(
    references: Vec<ReferenceRecord>,
    bases: &HashMap<u32, BaseRecord>,
    actor_models: &HashMap<u32, PreparedActorAppearance>,
    package_linked_reference_ids: &HashSet<u32>,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    assets_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    rebuild_assets: bool,
    static_converter_revision: &str,
    actor_converter_revision: &str,
) -> Result<PlacementStage> {
    // `stage_placements` already owns the general eligibility rules. Keep
    // package-point retention as a narrow orchestrator seam: split only the
    // explicitly reachable editor markers, then run the existing stage with
    // a temporary asset-less base map for that marker subset. This preserves
    // the real base FormID and transform while making it impossible for the
    // marker to enqueue a GLB, physics sidecar, static mesh, or shadow role.
    let (package_markers, ordinary): (Vec<_>, Vec<_>) =
        references.into_iter().partition(|reference| {
            package_linked_reference_ids.contains(&reference.form_id)
                && bases
                    .get(&reference.base_form_id)
                    .and_then(|base| base.model.as_deref())
                    .is_some_and(|model| is_editor_marker(&normalize_asset_path(model)))
        });
    let mut stage = stage_placements(
        ordinary,
        bases,
        actor_models,
        data_root,
        archives,
        staging_dir,
        assets_dir,
        diagnostics,
        rebuild_assets,
        static_converter_revision,
        actor_converter_revision,
    )?;
    if package_markers.is_empty() {
        return Ok(stage);
    }

    let mut marker_bases = bases.clone();
    for reference in &package_markers {
        if let Some(base) = marker_bases.get_mut(&reference.base_form_id) {
            base.model = None;
        }
    }
    let marker_stage = stage_placements(
        package_markers,
        &marker_bases,
        actor_models,
        data_root,
        archives,
        staging_dir,
        assets_dir,
        diagnostics,
        rebuild_assets,
        static_converter_revision,
        actor_converter_revision,
    )?;
    stage.jobs.extend(marker_stage.jobs);
    stage.visual_assets.extend(marker_stage.visual_assets);
    stage.placements.extend(marker_stage.placements);
    stage.lights.extend(marker_stage.lights);
    stage.cache_hits += marker_stage.cache_hits;
    stage.cache_missing += marker_stage.cache_missing;
    stage.cache_invalid += marker_stage.cache_invalid;
    stage.cache_explicit_rebuilds += marker_stage.cache_explicit_rebuilds;
    stage.leveled_lists.extend(marker_stage.leveled_lists);
    stage
        .placements
        .sort_by_key(|placement| placement.reference_form_id);
    stage.lights.sort_by_key(|light| light.reference_form_id);
    Ok(stage)
}

fn actor_catalog_package_lists(catalog: &PreparedActorCatalog) -> Vec<Vec<u32>> {
    catalog
        .entries
        .iter()
        .filter_map(|entry| match entry {
            ActorCatalogEntry::Prepared(blueprint) if blueprint.initially_enabled => {
                Some(blueprint.package_form_ids.clone())
            }
            _ => None,
        })
        .collect()
}

// ---------------------------------------------------------------------
// Package catalog wiring (issue #175, M4 wave 11 lane C)
//
// `package_catalog.rs` is a pure std/serde-only module for the same reason
// `actor_catalog.rs` is (see its module doc comment): boundary conversion
// from the parser's `PackageRecord`/`PackageLocation`/`PackageSchedule`/
// `PackageTarget` types into its plain input types lives here.
// ---------------------------------------------------------------------

/// Assembles the pure `PackageCatalogInputs` for this prepare pass: every
/// decoded `PACK` record (content-set-wide, like `parsed.recipes`/
/// `parsed.bases`, not scoped to one cell) plus every FormID this pass
/// decoded (bases and placed references), used only to diagnose an
/// unresolved `PLDT`/`PTDT` FormID -- see `PackageCatalogInputs::
/// known_form_ids`'s doc comment for the same scoping caveat
/// `ActorCatalogInputs::known_bases` already accepts.
fn build_package_catalog_inputs(parsed: &ParsedPlugin) -> PackageCatalogInputs {
    let mut known_form_ids: HashSet<u32> = parsed.bases.keys().copied().collect();
    known_form_ids.extend(parsed.references.iter().map(|reference| reference.form_id));
    let packages = parsed
        .packages
        .iter()
        .map(|(&form_id, record)| {
            (
                form_id,
                PackageInput {
                    form_id,
                    editor_id: record.editor_id.clone(),
                    general_flags: record.general_flags,
                    package_type: record.package_type,
                    location: record.location.map(|location| PackageLocationInput {
                        location_type: location.location_type,
                        form_id: location.form_id,
                        raw_value: location.raw_value,
                        radius: location.radius,
                    }),
                    schedule: record.schedule.map(|schedule| PackageScheduleInput {
                        month: schedule.month,
                        day_of_week: schedule.day_of_week,
                        date: schedule.date,
                        time: schedule.time,
                        duration: schedule.duration,
                    }),
                    target: record.target.map(|target| PackageTargetInput {
                        target_type: target.target_type,
                        form_id: target.form_id,
                        raw_value: target.raw_value,
                        count_or_distance: target.count_or_distance,
                    }),
                    idle_collection: record.idle_collection.as_ref().map(|collection| {
                        PreparedPackageIdleCollection {
                            flags: collection.flags,
                            timer_seconds: collection.timer_seconds,
                            animation_form_ids: collection.animation_form_ids.clone(),
                        }
                    }),
                    conditions: record.conditions.clone(),
                    unsupported_subrecords: record.ignored_subrecords.clone(),
                    diagnostics: record.diagnostics.clone(),
                },
            )
        })
        .collect();
    PackageCatalogInputs {
        packages,
        known_form_ids,
    }
}

/// Selects the visual NIF set for each actor placement.  Fallout NPC records
/// point at a skeleton rather than a render mesh, so NPCs use the sex-specific
/// race body/head parts and deterministic inventory apparel.  CREA
/// records already carry their renderable NIFZ list.  The list is sorted and
/// deduplicated before it reaches the assembler, making cache keys stable
/// across plugin traversal order.
fn build_actor_appearance_models(
    parsed: &ParsedPlugin,
    references: &[ReferenceRecord],
    catalog: &PreparedActorCatalog,
    source_fingerprint: &str,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<HashMap<u32, PreparedActorAppearance>> {
    let mut result = HashMap::new();
    for reference in references {
        let catalog_blueprint = catalog.entries.iter().find_map(|entry| match entry {
            ActorCatalogEntry::Prepared(blueprint)
                if blueprint.reference_form_id == reference.form_id =>
            {
                Some(blueprint.as_ref())
            }
            _ => None,
        });
        let Some(actor) = catalog_blueprint else {
            let kind = match reference.kind {
                ReferenceKind::Npc => ActorKind::Humanoid,
                ReferenceKind::Creature => ActorKind::Creature,
                ReferenceKind::Object => continue,
            };
            let fallback = resolve_actor_fallback(
                &ActorAppearanceAvailability {
                    kind,
                    base_form_id: reference.base_form_id,
                    reference_form_id: reference.form_id,
                    ..Default::default()
                },
                vec![ActorFallbackReason::MissingSkeleton {
                    path: "<unresolved actor blueprint>".into(),
                }],
            );
            result.insert(
                reference.form_id,
                PreparedActorAppearance {
                    descriptor: None,
                    inventory: Vec::new(),
                    blueprint: CoreActorAssemblyBlueprint {
                        source_base_form_id: reference.base_form_id,
                        resolved_base_form_id: reference.base_form_id,
                        reference_form_id: reference.form_id,
                        kind,
                        root_scale: resolve_actor_root_scale(kind, reference.scale, None, None),
                        fallback,
                        ..Default::default()
                    },
                },
            );
            continue;
        };

        let kind = if actor.record_kind == "CREA" {
            ActorKind::Creature
        } else {
            ActorKind::Humanoid
        };
        let source_base_form_id = if actor.source_base_form_id == 0 {
            actor.base_form_id
        } else {
            actor.source_base_form_id
        };
        let resolved_base_form_id = actor.resolved_base_form_id.unwrap_or(actor.base_form_id);
        let mut reasons = Vec::new();
        let mut availability_cache = HashMap::new();
        let race = actor.race_form_id.and_then(|id| parsed.races.get(&id));
        let race_scale = race.map(|race| {
            if actor.female {
                race.female_height
            } else {
                race.male_height
            }
        });
        let root_scale = resolve_actor_root_scale(
            kind,
            actor.scale,
            race_scale,
            if kind == ActorKind::Creature {
                actor.creature_base_scale
            } else {
                actor.height
            },
        );

        let skeleton = if kind == ActorKind::Creature {
            actor
                .model_path
                .as_deref()
                .and_then(normalized_actor_reference_nif)
        } else {
            let mut candidates = actor
                .model_path
                .as_deref()
                .and_then(normalized_actor_reference_nif)
                .into_iter()
                .chain(humanoid_skeleton_candidates(actor.female).map(str::to_owned));
            let mut selected = None;
            for path in candidates.by_ref() {
                if actor_model_available(data_root, archives, &path, &mut availability_cache)? {
                    selected = Some(path);
                    break;
                }
            }
            selected
        };
        let skeleton_available = if let Some(path) = skeleton.as_deref() {
            actor_model_available(data_root, archives, path, &mut availability_cache)?
        } else {
            false
        };
        if !skeleton_available {
            reasons.push(ActorFallbackReason::MissingSkeleton {
                path: skeleton.clone().unwrap_or_else(|| "<none>".into()),
            });
        }

        let mut exact_parts = Vec::new();
        let mut race_sex_parts = Vec::new();
        let mut race_default_parts = Vec::new();
        let mut exact_body_inputs = Vec::new();
        let mut race_body_inputs = Vec::new();
        let mut default_body_inputs = Vec::new();
        let mut apparel_inputs = Vec::new();
        let mut assembled_apparel = Vec::new();
        let mut selected_eye_form_id = None;
        let mut selected_eye_texture_path = None;
        let mut exact_complete = skeleton_available;
        let mut race_sex_complete = false;
        let mut race_default_complete = false;

        if kind == ActorKind::Humanoid {
            let gear = actor
                .inventory
                .iter()
                .flat_map(|entry| {
                    resolve_actor_gear_candidates(
                        parsed,
                        entry.base_form_id,
                        reference.form_id,
                        source_fingerprint,
                    )
                })
                .collect::<Vec<_>>();
            let mut available_apparel = HashSet::new();
            for candidate in &gear {
                let Some(model) = candidate.worn_model(actor.female) else {
                    continue;
                };
                let normalized = normalize_asset_path(model);
                if actor_model_available(data_root, archives, &normalized, &mut availability_cache)?
                {
                    available_apparel.insert(normalized);
                } else {
                    reasons.push(ActorFallbackReason::MissingEquipmentModel {
                        item_form_id: candidate.form_id,
                        path: normalized,
                    });
                }
            }
            let outfit = select_spawn_outfit(&gear, actor.female, |model| {
                available_apparel.contains(&normalize_asset_path(model))
            });
            diagnostics.extend(outfit.diagnostics.iter().map(|message| Diagnostic {
                severity: "warning".into(),
                message: format!("actor {:08x}: {message}", reference.form_id),
            }));
            for item in &outfit.worn {
                let path = normalize_asset_path(&item.model_path);
                apparel_inputs.push(ActorApparelInput {
                    path: path.clone(),
                    form_id: item.form_id,
                    biped_slot_mask: item.biped_slot_mask,
                });
                assembled_apparel.push(AssembledApparel {
                    item_form_id: item.form_id,
                    model_path: Some(path),
                    biped_slot_mask: item.biped_slot_mask,
                    model_available: true,
                });
            }

            if let Some(race) = race {
                let (sex_body, sex_head, default_body, default_head) = if actor.female {
                    (
                        &race.body_parts_female,
                        &race.head_parts_female,
                        &race.body_parts_male,
                        &race.head_parts_male,
                    )
                } else {
                    (
                        &race.body_parts_male,
                        &race.head_parts_male,
                        &race.body_parts_female,
                        &race.head_parts_female,
                    )
                };

                let mut sex_body_complete = required_race_body_parts_present(sex_body);
                for part in sex_body {
                    let Some(path) = part.model_path.as_deref().and_then(normalized_actor_nif)
                    else {
                        if race_body_part_requires_model(part.index) {
                            sex_body_complete = false;
                            reasons.push(ActorFallbackReason::MissingBodyPart {
                                index: part.index,
                                path: "<none>".into(),
                            });
                        }
                        continue;
                    };
                    if actor_model_available(data_root, archives, &path, &mut availability_cache)? {
                        race_body_inputs.push(ActorBodyPartInput {
                            path: path.clone(),
                            index: part.index,
                        });
                        race_sex_parts.push(AssembledMeshPart {
                            name: format!("RaceBody{}", part.index),
                            source_form_id: actor.race_form_id,
                            model_path: path,
                            attachment_point: ActorAttachmentPoint::Root,
                            role: ActorMeshRole::Body(part.index),
                            is_visible: true,
                        });
                    } else {
                        if race_body_part_requires_model(part.index) {
                            sex_body_complete = false;
                            reasons.push(ActorFallbackReason::MissingBodyPart {
                                index: part.index,
                                path,
                            });
                        }
                    }
                }

                let mut sex_head_complete = required_race_head_parts_present(sex_head);
                for part in sex_head {
                    let Some(path) = part.model_path.as_deref().and_then(normalized_actor_nif)
                    else {
                        if race_head_part_requires_model(part.index) {
                            sex_head_complete = false;
                            reasons.push(ActorFallbackReason::MissingHeadModel {
                                path: "<none>".into(),
                            });
                        }
                        continue;
                    };
                    if actor_model_available(data_root, archives, &path, &mut availability_cache)? {
                        race_sex_parts.push(AssembledMeshPart {
                            name: format!("RaceHead{}", part.index),
                            source_form_id: actor.race_form_id,
                            model_path: path,
                            attachment_point: ActorAttachmentPoint::Head,
                            role: race_head_mesh_role(part.index),
                            is_visible: true,
                        });
                    } else {
                        if race_head_part_requires_model(part.index) {
                            sex_head_complete = false;
                            reasons.push(ActorFallbackReason::MissingHeadModel { path });
                        }
                    }
                }
                race_sex_complete = skeleton_available && sex_body_complete && sex_head_complete;

                let mut default_body_complete = required_race_body_parts_present(default_body);
                for part in default_body {
                    let Some(path) = part.model_path.as_deref().and_then(normalized_actor_nif)
                    else {
                        if race_body_part_requires_model(part.index) {
                            default_body_complete = false;
                        }
                        continue;
                    };
                    if actor_model_available(data_root, archives, &path, &mut availability_cache)? {
                        default_body_inputs.push(ActorBodyPartInput {
                            path: path.clone(),
                            index: part.index,
                        });
                        race_default_parts.push(AssembledMeshPart {
                            name: format!("RaceDefaultBody{}", part.index),
                            source_form_id: actor.race_form_id,
                            model_path: path,
                            attachment_point: ActorAttachmentPoint::Root,
                            role: ActorMeshRole::Body(part.index),
                            is_visible: true,
                        });
                    } else {
                        if race_body_part_requires_model(part.index) {
                            default_body_complete = false;
                        }
                    }
                }
                let mut default_head_complete = required_race_head_parts_present(default_head);
                for part in default_head {
                    let Some(path) = part.model_path.as_deref().and_then(normalized_actor_nif)
                    else {
                        if race_head_part_requires_model(part.index) {
                            default_head_complete = false;
                        }
                        continue;
                    };
                    if actor_model_available(data_root, archives, &path, &mut availability_cache)? {
                        race_default_parts.push(AssembledMeshPart {
                            name: format!("RaceDefaultHead{}", part.index),
                            source_form_id: actor.race_form_id,
                            model_path: path,
                            attachment_point: ActorAttachmentPoint::Head,
                            role: race_head_mesh_role(part.index),
                            is_visible: true,
                        });
                    } else {
                        if race_head_part_requires_model(part.index) {
                            default_head_complete = false;
                        }
                    }
                }
                race_default_complete =
                    skeleton_available && default_body_complete && default_head_complete;
                if !race_sex_complete && race_default_complete {
                    reasons.push(ActorFallbackReason::IncompatibleSkin {
                        path: format!(
                            "race:{:08x}:{}",
                            actor.race_form_id.unwrap_or_default(),
                            if actor.female { "female" } else { "male" }
                        ),
                    });
                }

                exact_parts.extend(race_sex_parts.iter().cloned());
                exact_body_inputs.extend(race_body_inputs.iter().cloned());
                if !actor.head_part_form_ids.is_empty() {
                    exact_parts.retain(|part| !matches!(part.role, ActorMeshRole::Head(_)));
                    let mut explicit_head_complete = true;
                    for (index, form_id) in actor.head_part_form_ids.iter().copied().enumerate() {
                        let path = parsed
                            .bases
                            .get(&form_id)
                            .and_then(|base| base.model.as_deref())
                            .and_then(normalized_actor_nif);
                        let Some(path) = path else {
                            explicit_head_complete = false;
                            reasons.push(ActorFallbackReason::MissingHeadModel {
                                path: format!("form:{form_id:08x}"),
                            });
                            continue;
                        };
                        if actor_model_available(
                            data_root,
                            archives,
                            &path,
                            &mut availability_cache,
                        )? {
                            exact_parts.push(AssembledMeshPart {
                                name: format!("HeadPart{index}"),
                                source_form_id: Some(form_id),
                                model_path: path,
                                attachment_point: ActorAttachmentPoint::Head,
                                role: ActorMeshRole::Head(index as u32),
                                is_visible: true,
                            });
                        } else {
                            explicit_head_complete = false;
                            reasons.push(ActorFallbackReason::MissingHeadModel { path });
                        }
                    }
                    exact_complete &= explicit_head_complete;
                }

                let hair_form_id = actor.hair_form_id.or(if actor.female {
                    race.female_default_hair_form_id
                } else {
                    race.male_default_hair_form_id
                });
                if let Some(form_id) = hair_form_id {
                    let path = parsed
                        .bases
                        .get(&form_id)
                        .and_then(|base| base.model.as_deref())
                        .and_then(normalized_actor_nif);
                    match path {
                        Some(path)
                            if actor_model_available(
                                data_root,
                                archives,
                                &path,
                                &mut availability_cache,
                            )? =>
                        {
                            let part = AssembledMeshPart {
                                name: "Hair".into(),
                                source_form_id: Some(form_id),
                                model_path: path,
                                attachment_point: ActorAttachmentPoint::Head,
                                role: ActorMeshRole::Hair,
                                is_visible: hair_visible(outfit.occupied_slots),
                            };
                            exact_parts.push(part.clone());
                            race_sex_parts.push(part.clone());
                            race_default_parts.push(part);
                        }
                        Some(path) => {
                            reasons.push(ActorFallbackReason::MissingHairModel { path });
                        }
                        None => {
                            reasons.push(ActorFallbackReason::MissingHairModel {
                                path: format!("form:{form_id:08x}"),
                            });
                        }
                    }
                }

                let eyes_form_id = actor
                    .eyes_form_id
                    .or_else(|| race.eye_form_ids.first().copied());
                if let Some(form_id) = eyes_form_id {
                    let path = parsed
                        .bases
                        .get(&form_id)
                        .and_then(|base| base.icon.as_deref())
                        .map(normalize_asset_path);
                    match path {
                        Some(path)
                            if actor_model_available(
                                data_root,
                                archives,
                                &path,
                                &mut availability_cache,
                            )? =>
                        {
                            // EYES selects the diffuse texture for the race's
                            // index 6/7 eye geometry; it is not another NIF.
                            selected_eye_form_id = Some(form_id);
                            selected_eye_texture_path = Some(path);
                        }
                        Some(path) => {
                            reasons.push(ActorFallbackReason::MissingEyeModel { path });
                        }
                        None => {
                            reasons.push(ActorFallbackReason::MissingEyeModel {
                                path: format!("form:{form_id:08x}"),
                            });
                        }
                    }
                }
            } else {
                exact_complete = false;
                reasons.push(ActorFallbackReason::IncompatibleSkin {
                    path: actor.race_form_id.map_or_else(
                        || "race:<none>".into(),
                        |form_id| format!("race:{form_id:08x}"),
                    ),
                });
            }
            exact_complete &= race_sex_complete;
        } else if skeleton_available {
            let model_directory = skeleton.as_ref().and_then(|model| {
                model
                    .rfind('\\')
                    .or_else(|| model.rfind('/'))
                    .map(|index| model[..index].to_owned())
            });
            let mut available_parts = Vec::new();
            for (index, model) in actor.creature_model_list.iter().enumerate() {
                let model = if model.contains('\\') || model.contains('/') {
                    model.clone()
                } else if let Some(directory) = model_directory.as_ref() {
                    format!("{directory}\\{model}")
                } else {
                    model.clone()
                };
                let Some(path) = normalized_actor_nif(&model) else {
                    continue;
                };
                if let Some(bytes) = resolve_asset(data_root, archives, &path)? {
                    available_parts.push((
                        index,
                        path.clone(),
                        bytes.len(),
                        creature_path_is_main_model(&path),
                    ));
                    availability_cache.insert(path, true);
                } else {
                    availability_cache.insert(path.clone(), false);
                    reasons.push(ActorFallbackReason::MissingCreatureModel { path });
                }
            }
            let main_part = select_creature_main_model(&available_parts);
            for (part_index, (source_index, path, _, _)) in available_parts.into_iter().enumerate()
            {
                let is_main_model = Some(part_index) == main_part;
                exact_parts.push(AssembledMeshPart {
                    name: if is_main_model {
                        "CreatureRoot".into()
                    } else {
                        format!("CreaturePart{}", source_index + 1)
                    },
                    source_form_id: Some(resolved_base_form_id),
                    model_path: path,
                    attachment_point: ActorAttachmentPoint::Root,
                    role: ActorMeshRole::CreaturePart(if is_main_model {
                        0
                    } else {
                        (source_index + 1) as u32
                    }),
                    is_visible: true,
                });
            }
            let main_model_available = main_part.is_some();
            exact_complete &= main_model_available;
            if !main_model_available {
                reasons.push(ActorFallbackReason::MissingCreatureModel {
                    path: "<main creature model>".into(),
                });
            }
        }

        let mut weapon_candidates = Vec::new();
        for entry in &actor.inventory {
            let mut candidates =
                actor_weapon_model_candidates(parsed, entry.base_form_id, &mut HashSet::new(), 0);
            candidates.sort_by_key(|candidate| candidate.item_form_id);
            candidates.dedup_by_key(|candidate| candidate.item_form_id);
            if !candidates.is_empty() {
                let index = (appearance_selection_seed(
                    source_fingerprint,
                    reference.form_id,
                    entry.base_form_id,
                ) as usize)
                    % candidates.len();
                let mut candidate = candidates.swap_remove(index);
                candidate.available = if let Some(path) = candidate.model_path.as_deref() {
                    actor_model_available(data_root, archives, path, &mut availability_cache)?
                } else {
                    false
                };
                weapon_candidates.push(candidate);
            }
        }
        let equipped_weapon = select_starting_weapon(&weapon_candidates);
        if let Some(weapon) = equipped_weapon.as_ref()
            && !weapon.model_available
        {
            reasons.push(ActorFallbackReason::MissingEquipmentModel {
                item_form_id: weapon.item_form_id,
                path: weapon.model_path.clone().unwrap_or_else(|| "<none>".into()),
            });
        }

        let facegen_head_path = exact_parts
            .iter()
            .chain(race_sex_parts.iter())
            .chain(race_default_parts.iter())
            .find(|part| matches!(part.role, ActorMeshRole::Head(_)))
            .map(|part| part.model_path.clone());
        let (facegen_descriptor, facegen_asset_diagnostic) =
            if let Some(resolved) = actor.facegen.as_ref() {
                match facegen_head_path.as_deref() {
                    Some(head_path) => match prepare_actor_facegen_descriptor(
                        actor, resolved, head_path, data_root, archives,
                    )? {
                        Ok(descriptor) => (Some(descriptor), None),
                        Err(diagnostic) => (None, Some(diagnostic)),
                    },
                    None => (
                        None,
                        Some(FaceGenDiagnostic::MissingAsset {
                            asset: FaceGenAssetKind::GeometryMorph,
                            path: "<selected head>".into(),
                        }),
                    ),
                }
            } else {
                (None, None)
            };
        if let Some(diagnostic) = facegen_asset_diagnostic.as_ref() {
            reasons.push(ActorFallbackReason::MissingFaceGen);
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "actor {:08x} FaceGen fallback: {diagnostic}",
                    reference.form_id
                ),
            });
        }
        let facegen_diagnostics = actor
            .facegen_diagnostics
            .iter()
            .cloned()
            .chain(facegen_asset_diagnostic.iter().cloned())
            .collect::<Vec<_>>();
        let facegen_reconstruction_fingerprint = facegen_descriptor.as_ref().map(|descriptor| {
            fingerprint(
                &serde_json::to_vec(descriptor)
                    .expect("FaceGen descriptor serialization must be deterministic"),
            )
        });
        let assembled_facegen = facegen_descriptor
            .is_some()
            .then(|| actor.facegen.clone())
            .flatten();
        let facegen = if !actor.facegen_diagnostics.is_empty() || facegen_asset_diagnostic.is_some()
        {
            FaceGenAvailability::Incompatible
        } else if facegen_descriptor.is_some() {
            FaceGenAvailability::Compatible
        } else {
            FaceGenAvailability::NotAuthored
        };
        let availability = ActorAppearanceAvailability {
            kind,
            base_form_id: resolved_base_form_id,
            reference_form_id: reference.form_id,
            exact_available: exact_complete,
            race_sex_available: race_sex_complete,
            race_default_available: race_default_complete,
            // This project does not yet ship a skeleton-compatible generic
            // humanoid GLB. Do not mislabel the viewer's bounds proxy as tier
            // four; callers may select GenericProjectBody only when they can
            // provide an actual compatible generic assembly.
            generic_available: false,
            facegen,
        };
        let fallback = resolve_actor_fallback(&availability, reasons);
        let (mut mesh_parts, body_inputs) = match fallback.level {
            ActorFallbackLevel::AuthoredExact => (exact_parts, exact_body_inputs),
            ActorFallbackLevel::RaceSexSpecific => (race_sex_parts, race_body_inputs),
            ActorFallbackLevel::RaceDefault => (race_default_parts, default_body_inputs),
            ActorFallbackLevel::GenericProjectBody | ActorFallbackLevel::ProxyMesh => {
                (Vec::new(), Vec::new())
            }
        };
        canonicalize_mesh_parts(&mut mesh_parts);
        let mut visual_inputs = mesh_parts
            .iter()
            .filter(|part| part.is_visible)
            .map(|part| part.model_path.clone())
            .collect::<Vec<_>>();
        visual_inputs.extend(apparel_inputs.iter().map(|item| item.path.clone()));
        let mut descriptor = if matches!(
            fallback.level,
            ActorFallbackLevel::GenericProjectBody | ActorFallbackLevel::ProxyMesh
        ) {
            None
        } else {
            canonical_actor_assembly(skeleton.clone(), visual_inputs)
        };
        if let Some(assembly) = descriptor.as_mut() {
            assembly.body_parts = body_inputs;
            assembly.apparel = apparel_inputs;
            assembly.head_parts = mesh_parts
                .iter()
                .filter(|part| {
                    part.is_visible && part.attachment_point == ActorAttachmentPoint::Head
                })
                .map(|part| part.model_path.clone())
                .collect();
            assembly.head_anim_parts = mesh_parts
                .iter()
                .filter(|part| part.is_visible && part.role == ActorMeshRole::Hair)
                .map(|part| part.model_path.clone())
                .collect();
            assembly.eye_geometry = mesh_parts
                .iter()
                .filter(|part| part.role == ActorMeshRole::Eyes)
                .map(|part| part.model_path.clone())
                .collect();
            assembly.eye_texture = selected_eye_texture_path.as_ref().map(|path| {
                if path.starts_with("textures/") {
                    path.clone()
                } else {
                    format!("textures/{path}")
                }
            });
            assembly.facegen = facegen_descriptor.clone();
        }

        let blueprint = CoreActorAssemblyBlueprint {
            source_base_form_id,
            resolved_base_form_id,
            reference_form_id: reference.form_id,
            kind,
            female: actor.female,
            race_form_id: actor.race_form_id,
            root_scale,
            skeleton_path: skeleton,
            mesh_parts,
            apparel: assembled_apparel,
            eye_form_id: selected_eye_form_id,
            eye_texture_path: selected_eye_texture_path,
            facegen: assembled_facegen,
            facegen_reconstruction_fingerprint,
            facegen_diagnostics,
            equipped_weapon,
            fallback,
        };
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!(
                "actor assembly ref={:08x} source={source_base_form_id:08x} resolved={resolved_base_form_id:08x} tier={} parts={} apparel={} weapon={} scale={root_scale:.6}",
                reference.form_id,
                blueprint.fallback.level.label(),
                blueprint.mesh_parts.len(),
                blueprint.apparel.len(),
                blueprint
                    .equipped_weapon
                    .as_ref()
                    .map_or_else(|| "none".into(), |weapon| format!("{:08x}", weapon.item_form_id)),
            ),
        });
        diagnostics.extend(blueprint.fallback.reasons.iter().map(|reason| Diagnostic {
            severity: "warning".into(),
            message: format!(
                "actor fallback ref={:08x} base={resolved_base_form_id:08x} tier={} reason={}",
                reference.form_id,
                blueprint.fallback.level.label(),
                reason.code()
            ),
        }));
        result.insert(
            reference.form_id,
            PreparedActorAppearance {
                descriptor,
                inventory: actor.inventory.clone(),
                blueprint,
            },
        );
    }
    Ok(result)
}

fn prepare_actor_facegen_descriptor(
    actor: &ActorBlueprint,
    resolved: &bevyout_core::facegen::FaceGenResolved,
    head_path: &str,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
) -> Result<Result<ActorFaceGenDescriptor, FaceGenDiagnostic>> {
    let head_path = normalize_asset_path(head_path);
    let tri_path = facegen_companion_path(&head_path, "tri");
    let geometry_morph_path = facegen_companion_path(&head_path, "egm");
    let texture_morph_path = facegen_companion_path(&head_path, "egt");
    let Some(tri_bytes) = resolve_asset(data_root, archives, &tri_path)? else {
        return Ok(Err(FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::TriMorph,
            path: tri_path,
        }));
    };
    let Some(geometry_bytes) = resolve_asset(data_root, archives, &geometry_morph_path)? else {
        return Ok(Err(FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::GeometryMorph,
            path: geometry_morph_path,
        }));
    };
    let Some(texture_bytes) = resolve_asset(data_root, archives, &texture_morph_path)? else {
        return Ok(Err(FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::TextureMorph,
            path: texture_morph_path,
        }));
    };
    let tri = match parse_tri_layout(&tri_bytes) {
        Ok(layout) => layout,
        Err(diagnostic) => return Ok(Err(diagnostic)),
    };
    let geometry = match parse_geometry_morph(&geometry_bytes) {
        Ok(morph) => morph,
        Err(diagnostic) => return Ok(Err(diagnostic)),
    };
    let texture = match parse_texture_morph(&texture_bytes) {
        Ok(morph) => morph,
        Err(diagnostic) => return Ok(Err(diagnostic)),
    };
    let Some(head_bytes) = resolve_asset(data_root, archives, &head_path)? else {
        return Ok(Err(FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::GeometryMorph,
            path: head_path,
        }));
    };
    let head_document = match nif::fo3::parse(&head_bytes) {
        Ok(document) => document,
        Err(error) => {
            return Ok(Err(FaceGenDiagnostic::UnsupportedAsset {
                asset: FaceGenAssetKind::GeometryMorph,
                reason: format!("head NIF could not be parsed: {error}"),
            }));
        }
    };
    let head_scene = match nif::fo3::extract_scene(&head_document) {
        Ok(scene) => scene,
        Err(error) => {
            return Ok(Err(FaceGenDiagnostic::UnsupportedAsset {
                asset: FaceGenAssetKind::GeometryMorph,
                reason: format!("head NIF scene could not be extracted: {error}"),
            }));
        }
    };
    let head_vertices = head_scene
        .nodes
        .iter()
        .filter_map(|node| node.mesh.as_ref())
        .map(|mesh| mesh.positions.len())
        .sum::<usize>();
    let head_triangles = head_scene
        .nodes
        .iter()
        .filter_map(|node| node.mesh.as_ref())
        .map(|mesh| mesh.indices.len() / 3)
        .sum::<usize>();
    let head_uvs = head_scene
        .nodes
        .iter()
        .filter_map(|node| node.mesh.as_ref())
        .map(|mesh| mesh.tex_coords.len())
        .sum::<usize>();
    if geometry.vertex_count != tri.combined_vertex_count {
        return Ok(Err(FaceGenDiagnostic::GeometryVertexCountMismatch {
            expected: tri.combined_vertex_count,
            actual: geometry.vertex_count,
        }));
    }
    if head_vertices != tri.base_vertex_count {
        return Ok(Err(FaceGenDiagnostic::GeometryBaseVertexCountMismatch {
            expected: tri.base_vertex_count,
            actual: head_vertices,
        }));
    }
    if tri.texture_coordinate_count != head_uvs {
        return Ok(Err(FaceGenDiagnostic::GeometryUvCountMismatch {
            expected: tri.texture_coordinate_count,
            actual: head_uvs,
        }));
    }
    if tri.triangle_count != head_triangles {
        return Ok(Err(FaceGenDiagnostic::UnsupportedAsset {
            asset: FaceGenAssetKind::TriMorph,
            reason: format!(
                "TRI has {} triangles but the head has {head_triangles}",
                tri.triangle_count
            ),
        }));
    }
    let Some(diffuse_path) = head_scene
        .materials
        .iter()
        .find_map(|material| material.diffuse_texture.clone())
    else {
        return Ok(Err(FaceGenDiagnostic::UnsupportedAsset {
            asset: FaceGenAssetKind::TextureMorph,
            reason: "selected head has no diffuse texture".into(),
        }));
    };
    let Some(diffuse_bytes) = resolve_asset(data_root, archives, &diffuse_path)? else {
        return Ok(Err(FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::TextureMorph,
            path: diffuse_path,
        }));
    };
    let image = match image::load_from_memory(&diffuse_bytes) {
        Ok(image) => image,
        Err(error) => {
            return Ok(Err(FaceGenDiagnostic::UnsupportedAsset {
                asset: FaceGenAssetKind::TextureMorph,
                reason: format!("head diffuse texture could not be decoded: {error}"),
            }));
        }
    };
    if !texture_dimensions_compatible(image.width(), image.height(), texture.width, texture.height)
    {
        return Ok(Err(FaceGenDiagnostic::TextureDimensionsMismatch {
            expected_width: texture.width,
            expected_height: texture.height,
            actual_width: image.width(),
            actual_height: image.height(),
        }));
    }
    Ok(Ok(ActorFaceGenDescriptor {
        base_form_id: actor.base_form_id,
        race_form_id: actor.race_form_id,
        head_path,
        tri_path,
        geometry_morph_path,
        texture_morph_path,
        tri_hash: fingerprint(&tri_bytes),
        geometry_morph_hash: fingerprint(&geometry_bytes),
        texture_morph_hash: fingerprint(&texture_bytes),
        base_vertex_count: tri.base_vertex_count,
        texture_coordinate_count: tri.texture_coordinate_count,
        resolved: resolved.clone(),
    }))
}

fn facegen_companion_path(head_path: &str, extension: &str) -> String {
    let stem = head_path.strip_suffix(".nif").unwrap_or(head_path);
    format!("{stem}.{extension}")
}

fn normalized_actor_nif(path: &str) -> Option<String> {
    let path = normalize_asset_path(path);
    (path.to_ascii_lowercase().ends_with(".nif")
        && !is_editor_marker(&path)
        && !is_non_rendering_effect(&path))
    .then_some(path)
}

fn normalized_actor_reference_nif(path: &str) -> Option<String> {
    let path = normalize_asset_path(path);
    (path.ends_with(".nif") && !is_editor_marker(&path)).then_some(path)
}

fn humanoid_skeleton_candidates(female: bool) -> impl Iterator<Item = &'static str> {
    let sex_specific = if female {
        "characters/_female/skeleton.nif"
    } else {
        "characters/_male/skeleton.nif"
    };
    [
        sex_specific,
        "characters/_male/skeleton.nif",
        "characters/skeleton.nif",
    ]
    .into_iter()
}

fn race_body_part_requires_model(index: u32) -> bool {
    index <= 2
}

fn race_head_part_requires_model(index: u32) -> bool {
    index == 0
}

fn race_head_mesh_role(index: u32) -> ActorMeshRole {
    if matches!(index, 6 | 7) {
        ActorMeshRole::Eyes
    } else {
        ActorMeshRole::Head(index)
    }
}

fn required_race_body_parts_present(parts: &[crate::vsa::openmw_esm4::RacePartEntry]) -> bool {
    (0..=2).all(|required| parts.iter().any(|part| part.index == required))
}

fn required_race_head_parts_present(parts: &[crate::vsa::openmw_esm4::RacePartEntry]) -> bool {
    parts.iter().any(|part| part.index == 0)
}

fn creature_path_is_main_model(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let mut components = normalized.rsplit('/');
    let Some(file) = components.next() else {
        return false;
    };
    let Some(parent) = components.next() else {
        return false;
    };
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    stem.eq_ignore_ascii_case(parent)
}

fn select_creature_main_model(parts: &[(usize, String, usize, bool)]) -> Option<usize> {
    parts
        .iter()
        .enumerate()
        .max_by(|(_, left), (_, right)| {
            left.3
                .cmp(&right.3)
                .then_with(|| left.2.cmp(&right.2))
                // Prefer the lexicographically lowest normalized path when
                // two candidates have the same authored-name and size rank.
                .then_with(|| right.1.cmp(&left.1))
        })
        .map(|(index, _)| index)
}

fn actor_model_available(
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    path: &str,
    cache: &mut HashMap<String, bool>,
) -> Result<bool> {
    let path = normalize_asset_path(path);
    if let Some(available) = cache.get(&path) {
        return Ok(*available);
    }
    let available = resolve_asset(data_root, archives, &path)?.is_some();
    cache.insert(path, available);
    Ok(available)
}

fn apply_actor_weapon_assets(
    placements: &mut [PreparedPlacement],
    item_catalog: &PreparedItemCatalog,
) {
    let world_assets = item_catalog
        .items
        .iter()
        .filter_map(|item| {
            item.world_asset_path
                .as_ref()
                .map(|path| (item.base_form_id, path.clone()))
        })
        .collect::<HashMap<_, _>>();
    for placement in placements {
        let assembly = match &mut placement.semantic {
            PreparedSemantic::Npc(actor) | PreparedSemantic::Creature(actor) => {
                actor.assembly.as_mut()
            }
            _ => None,
        };
        let Some(assembly) = assembly else {
            continue;
        };
        let Some(weapon) = assembly.equipped_weapon.as_mut() else {
            continue;
        };
        let source_path = weapon.model_path.clone().unwrap_or_else(|| "<none>".into());
        weapon.model_path = world_assets.get(&weapon.item_form_id).cloned();
        weapon.model_available = weapon.model_path.is_some();
        if !weapon.model_available {
            assembly
                .fallback
                .reasons
                .push(ActorFallbackReason::MissingEquipmentModel {
                    item_form_id: weapon.item_form_id,
                    path: source_path,
                });
            assembly.fallback.reasons.sort();
            assembly.fallback.reasons.dedup();
        }
    }
}

/// Resolves one authored inventory entry to a deterministic visual item. NPC
/// inventories commonly point at nested LVLI lists, so following only direct
/// ARMO/WEAP records silently drops all equipment from the assembled actor.
fn resolve_actor_gear_candidates(
    parsed: &ParsedPlugin,
    root_form_id: u32,
    reference_form_id: u32,
    source_fingerprint: &str,
) -> Vec<ApparelCandidate> {
    resolve_actor_gear_candidates_inner(
        parsed,
        root_form_id,
        reference_form_id,
        source_fingerprint,
        &mut HashSet::new(),
        0,
    )
}

fn resolve_actor_gear_candidates_inner(
    parsed: &ParsedPlugin,
    form_id: u32,
    reference_form_id: u32,
    source_fingerprint: &str,
    active: &mut HashSet<u32>,
    depth: usize,
) -> Vec<ApparelCandidate> {
    const LEVELED_USE_ALL: u8 = 0x04;

    if depth >= 32 || !active.insert(form_id) {
        return Vec::new();
    }
    let Some(base) = parsed.bases.get(&form_id) else {
        active.remove(&form_id);
        return Vec::new();
    };
    if crate::vsa::assets::actor_visual_gear_kind(&base.kind) {
        let Some(models) = base.apparel_models.as_ref() else {
            active.remove(&form_id);
            return Vec::new();
        };
        let OpenMwItemStats::Apparel {
            armor_rating,
            max_condition,
            biped_slot_mask,
        } = &base.item_stats
        else {
            active.remove(&form_id);
            return Vec::new();
        };
        let candidate = ApparelCandidate {
            form_id,
            male_worn: models.male_worn.clone(),
            female_worn: models.female_worn.clone(),
            male_world: models.male_world.clone(),
            female_world: models.female_world.clone(),
            biped_slot_mask: biped_slot_mask.unwrap_or_default(),
            base_armor_rating: armor_rating.unwrap_or_default(),
            max_condition: *max_condition,
            current_condition: None,
            value: base.value.unwrap_or_default(),
        };
        active.remove(&form_id);
        return vec![candidate];
    }
    let Some(leveled) = base.leveled.as_ref() else {
        active.remove(&form_id);
        return Vec::new();
    };

    let seed = appearance_selection_seed(source_fingerprint, reference_form_id, form_id);
    if leveled.chance_none != 0 && seed % 100 < u64::from(leveled.chance_none) {
        active.remove(&form_id);
        return Vec::new();
    }

    let mut entries = leveled.entries.iter().collect::<Vec<_>>();
    entries.sort_by_key(|entry| (entry.item_form_id, entry.level, entry.count));
    let selected_entries = if leveled.flags & LEVELED_USE_ALL != 0 {
        entries
    } else if entries.is_empty() {
        Vec::new()
    } else {
        vec![entries[(seed as usize) % entries.len()]]
    };

    let mut candidates = Vec::new();
    for entry in selected_entries {
        candidates.extend(resolve_actor_gear_candidates_inner(
            parsed,
            entry.item_form_id,
            reference_form_id,
            source_fingerprint,
            active,
            depth + 1,
        ));
    }
    active.remove(&form_id);
    candidates.sort_by_key(|candidate| candidate.form_id);
    candidates.dedup_by_key(|candidate| candidate.form_id);
    candidates
}

fn actor_weapon_model_candidates(
    parsed: &ParsedPlugin,
    form_id: u32,
    visited: &mut HashSet<u32>,
    depth: usize,
) -> Vec<ActorWeaponCandidate> {
    if depth >= 32 || !visited.insert(form_id) {
        return Vec::new();
    }
    let Some(base) = parsed.bases.get(&form_id) else {
        return Vec::new();
    };
    if base.kind == "WEAP" {
        let damage = match &base.item_stats {
            OpenMwItemStats::Weapon { damage, .. } => damage.unwrap_or_default(),
            _ => 0,
        };
        return vec![ActorWeaponCandidate {
            item_form_id: form_id,
            model_path: base.model.as_deref().and_then(normalized_actor_nif),
            damage,
            value: base.value.unwrap_or_default(),
            available: false,
        }];
    }
    let Some(leveled) = base.leveled.as_ref() else {
        return Vec::new();
    };
    let mut candidates = Vec::new();
    for entry in &leveled.entries {
        candidates.extend(actor_weapon_model_candidates(
            parsed,
            entry.item_form_id,
            visited,
            depth + 1,
        ));
    }
    candidates
}

fn appearance_selection_seed(
    source_fingerprint: &str,
    reference_form_id: u32,
    list_form_id: u32,
) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in format!("{source_fingerprint}:{reference_form_id:08x}:{list_form_id:08x}").bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
#[path = "tests/orchestrator.rs"]
mod tests;
