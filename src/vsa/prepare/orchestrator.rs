use super::*;
use crate::vsa::catalog::{CellCatalog, build_cell_map};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Dispatches `prepare`: a single legacy selector goes straight through
/// `prepare_single` below; `--all`/`--all-interiors`/`--worldspace`,
/// `--list-only`, `--retry-failed`, or more than one positional selector
/// build a lightweight cell catalogue and resolve the batch through
/// `resolve_selection` (#46).
pub fn prepare(args: PrepareArgs) -> Result<()> {
    let mut explicit = args.selectors.clone();
    explicit.extend(args.cell.clone());

    let is_batch = args.list_only
        || args.all
        || args.all_interiors
        || args.worldspace.is_some()
        || args.retry_failed
        || explicit.len() > 1;

    if !is_batch {
        let selector_input = explicit
            .into_iter()
            .next()
            .context("provide a GECK EditorID/FormID selector or legacy --cell, or pass --all/--all-interiors/--worldspace/--retry-failed")?;
        return prepare_single(args, selector_input);
    }

    prepare_batch(args, explicit)
}

/// The single-cell CLI path. Builds a one-cell `BatchSession` (issue #47)
/// rather than duplicating the plugin-chain/BSA/audio/footstep loading
/// `prepare_cell` needs, so its output is identical to what the previous
/// `prepare_one` produced for a single cell (F47.1).
fn prepare_single(args: PrepareArgs, selector_input: String) -> Result<()> {
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
    let mut output = Vec::new();
    let result = prepare_cell(&session, args, selector_input, &mut output);
    for line in &output {
        println!("{line}");
    }
    result
}

fn prepare_batch(args: PrepareArgs, explicit: Vec<String>) -> Result<()> {
    let spec = SelectionSpec {
        all: args.all,
        all_interiors: args.all_interiors,
        worldspace: args.worldspace.clone(),
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
    if args.check_fingerprints {
        return report_fingerprints(&manifest, &resolved, &fingerprint);
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
    let current_fingerprints = CellFingerprints::current(fingerprint.clone());
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

    let session = BatchSession::new(
        &plugin_path,
        &data_root,
        &cache_dir,
        loaded_plugins,
        fingerprint,
    )?;

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
                    cell_args.worldspace = None;
                    cell_args.list_only = false;
                    cell_args.retry_failed = false;
                    cell_args.check_fingerprints = false;

                    let mut output = Vec::new();
                    let result =
                        prepare_cell(&session, cell_args, selector_input.clone(), &mut output);

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
        bail!(
            "{} of {} cell(s) failed to prepare",
            failed_entries.len(),
            to_run.len()
        );
    }
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
) -> Result<()> {
    let current = CellFingerprints::current(plugin_content_set_fingerprint);
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
) -> Result<()> {
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
    let mut parsed = parse_content_set(&plugin_sources, &selector)
        .context("failed to parse Fallout content set")?;
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
    if !cell.interior {
        bail!(
            "{} is not an interior cell; LAND support is not part of this slice",
            super::super::manifest::cell_label(&cell)
        )
    }

    // BSA archive indexes: indexed once in `BatchSession::new`, not per cell
    // (F47.2).
    diagnostics.extend(session.archive_diagnostics.iter().cloned());
    // Audio archive indexes: same -- indexed once in `BatchSession::new`.
    diagnostics.extend(session.audio_diagnostics.iter().cloned());
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
        cell.water_height = metadata.water_height;
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
    let blender = find_blender(args.blender)?;
    let (navmeshes, nav_graph, nav_graph_summary) = stage_navmeshes(
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
    // the Blender/texture-conversion step immediately below.
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
    let mut references = std::mem::take(&mut parsed.references);
    let actor_models = build_actor_appearance_models(
        &parsed,
        &actor_references,
        &source_fingerprint,
        &data_root,
        &session.archives,
        &mut diagnostics,
    )?;
    let (catalog_references, catalog_reference_ids) =
        catalog_item_references(&parsed.bases, &references);
    references.extend(catalog_references);
    let stage = stage_placements(
        references,
        &parsed.bases,
        &actor_models,
        &data_root,
        &session.archives,
        &staging_dir,
        &assets_dir,
        &mut diagnostics,
        args.rebuild_assets,
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
    // F48.4 serialization point: `convert_staged_textures` walks every
    // `.dds` under the *whole* `staging_dir` (not just this cell's), and
    // `run_blender_batch` writes the job list to a fixed filename
    // (`staging_dir/blender_jobs.ron`) before invoking Blender. Two cells
    // running this concurrently could convert/miss each other's textures,
    // or overwrite each other's job file mid-Blender-run. Neither is
    // content-addressed the way the staging writes above are, so this
    // block holds `session.blender_lock` for its duration: only one cell's
    // Blender/texture-conversion step runs at a time, while every other
    // cell's parse/stage phase keeps running in parallel around it.
    let item_icons;
    {
        let _blender_guard = session.blender_lock.lock().unwrap();
        item_icons = stage_item_icons(
            &parsed.bases,
            &data_root,
            &session.archives,
            &staging_dir,
            &source_fingerprint,
            &mut diagnostics,
        )?;
        convert_staged_textures(&staging_dir, &mut diagnostics)?;
        if !jobs.is_empty() {
            run_blender_batch(&blender, &jobs, &data_root, &staging_dir)
                .context("headless Blender conversion failed")?;
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
    let item_catalog = build_item_catalog(
        &parsed.bases,
        &item_icons,
        &catalog_placements,
        &physics_assets,
        &source_fingerprint,
    );
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
    let actor_catalog_inputs = build_actor_catalog_inputs(&parsed, &actor_references);
    let actor_catalog = build_actor_catalog(&actor_catalog_inputs, &source_fingerprint);
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
    if placements.iter().all(|p| p.asset_path.is_none()) && lights.is_empty() {
        bail!(
            "no renderable assets were found in {}",
            super::super::manifest::cell_label(&cell)
        )
    }

    let static_point_shadows = prepare_static_point_shadows(
        StaticShadowPrepareOptions {
            asset_root: &cache_dir,
            scene_dir: &scene_dir,
            resolution: args.shadow_resolution,
            rebuild: args.rebuild_shadows,
            ktx: args.toktx,
        },
        &placements,
        &lights,
        &mut diagnostics,
    )?;

    let manifest = PreparedSceneManifest {
        schema_version: CURRENT_MANIFEST_SCHEMA_VERSION,
        prepare_revision: Some(CURRENT_PREPARE_REVISION.into()),
        converter_revision: Some(PREPARED_CONVERTER_REVISION.into()),
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
        leveled_lists,
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
        creature_animation_files: actor
            .creature
            .as_ref()
            .map(|creature| creature.animation_files.clone())
            .unwrap_or_default(),
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
/// `known_bases` is every decoded base FormID (used for hair/eyes/
/// head-part existence checks -- see `ActorCatalogInputs::known_bases`'s
/// doc comment for why those links are honestly diagnosed unresolved until
/// `HAIR`/`EYES`/`HDPT` get their own decode task).
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
            (form_id, FactionInput { form_id, ranks })
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
        classes,
        factions,
        packages,
        known_bases,
        placements,
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
    source_fingerprint: &str,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<HashMap<u32, ActorAssemblyDescriptor>> {
    let mut result = HashMap::new();
    for reference in references {
        let Some(base) = resolve_actor_appearance_base(
            parsed,
            reference.base_form_id,
            reference.kind,
            reference.form_id,
            source_fingerprint,
        ) else {
            continue;
        };
        let Some(actor) = base.actor.as_ref() else {
            continue;
        };
        let skeleton = base.model.clone();
        let mut visual_inputs = Vec::new();
        let mut body_parts = Vec::new();
        let mut apparel_inputs = Vec::new();
        match reference.kind {
            ReferenceKind::Creature => {
                // The CREA model list contains visual attachments relative to
                // the skeleton path. The canonical descriptor below retains
                // the skeleton as both the explicit reference and the first
                // visual input because creature skeleton NIFs may render too.
                if let Some(creature) = actor.creature.as_ref() {
                    let model_directory = base.model.as_ref().and_then(|model| {
                        model
                            .rfind('\\')
                            .or_else(|| model.rfind('/'))
                            .map(|index| model[..index].to_owned())
                    });
                    visual_inputs.extend(creature.model_list.iter().map(|model| {
                        if model.contains('\\') || model.contains('/') {
                            model.clone()
                        } else if let Some(directory) = model_directory.as_ref() {
                            format!("{directory}\\{model}")
                        } else {
                            model.clone()
                        }
                    }));
                }
            }
            ReferenceKind::Npc => {
                let female = actor
                    .base_config
                    .is_some_and(|config| config.flags & 1 != 0);
                // Body-part NIFs carry only partial armatures. Include the
                // actor skeleton as the shared bone/physics source so the
                // assembled GLB can drive every part and the ragdoll sidecar
                // contains the authored articulated bodies.
                if let Some(race) = actor.race_form_id.and_then(|id| parsed.races.get(&id)) {
                    let body = if female {
                        &race.body_parts_female
                    } else {
                        &race.body_parts_male
                    };
                    let head = if female {
                        &race.head_parts_female
                    } else {
                        &race.head_parts_male
                    };
                    body_parts.extend(body.iter().filter_map(|part| {
                        part.model_path.clone().map(|path| ActorBodyPartInput {
                            path,
                            index: part.index,
                        })
                    }));
                    visual_inputs.extend(body_parts.iter().map(|part| part.path.clone()));
                    visual_inputs.extend(head.iter().filter_map(|part| part.model_path.clone()));
                }
                // Worn apparel is appearance-only in this slice. Weapons are
                // runtime attachments; baking inventory weapons into the actor
                // body makes every carried weapon visible and corrupts ragdolls.
                let gear = base
                    .inventory
                    .iter()
                    .flat_map(|entry| {
                        resolve_actor_gear_candidates(
                            parsed,
                            entry.item_form_id,
                            reference.form_id,
                            source_fingerprint,
                        )
                    })
                    .collect::<Vec<_>>();
                let mut available_models = HashSet::new();
                for candidate in &gear {
                    let Some(model) = candidate.worn_model(female) else {
                        continue;
                    };
                    if resolve_asset(data_root, archives, model)?.is_some() {
                        available_models.insert(normalize_asset_path(model));
                    }
                }
                let outfit = select_spawn_outfit(&gear, female, |model| {
                    available_models.contains(&normalize_asset_path(model))
                });
                diagnostics.extend(outfit.diagnostics.into_iter().map(|message| Diagnostic {
                    severity: "warning".into(),
                    message: format!("actor {:08x}: {message}", reference.form_id),
                }));
                apparel_inputs.extend(outfit.worn.into_iter().map(|item| ActorApparelInput {
                    path: item.model_path,
                    form_id: item.form_id,
                    biped_slot_mask: item.biped_slot_mask,
                }));
                visual_inputs.extend(apparel_inputs.iter().map(|item| item.path.clone()));
            }
            ReferenceKind::Object => continue,
        }
        visual_inputs = visual_inputs
            .into_iter()
            .map(|model| normalize_asset_path(&model))
            .filter(|model| {
                model.to_ascii_lowercase().ends_with(".nif")
                    && !is_editor_marker(model)
                    && !is_non_rendering_effect(model)
            })
            .collect();
        let skeleton = skeleton
            .map(|model| normalize_asset_path(&model))
            .filter(|model| {
                model.to_ascii_lowercase().ends_with(".nif")
                    && !is_editor_marker(model)
                    && !is_non_rendering_effect(model)
            });
        if let Some(mut assembly) = canonical_actor_assembly(skeleton, visual_inputs) {
            body_parts.retain(|part| part.path.to_ascii_lowercase().ends_with(".nif"));
            for part in &mut body_parts {
                part.path = normalize_asset_path(&part.path);
            }
            body_parts.sort_by(|left, right| {
                left.index
                    .cmp(&right.index)
                    .then_with(|| left.path.cmp(&right.path))
            });
            body_parts.dedup();
            for item in &mut apparel_inputs {
                item.path = normalize_asset_path(&item.path);
            }
            apparel_inputs.sort_by(|left, right| {
                left.path
                    .cmp(&right.path)
                    .then_with(|| left.form_id.cmp(&right.form_id))
            });
            apparel_inputs.dedup();
            assembly.body_parts = body_parts;
            assembly.apparel = apparel_inputs;
            result.insert(reference.form_id, assembly);
        }
    }
    Ok(result)
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
    let mut candidates = actor_gear_model_candidates(parsed, root_form_id, &mut HashSet::new(), 0);
    candidates.sort_by_key(|candidate| candidate.0);
    candidates.dedup_by_key(|candidate| candidate.0);
    if candidates.is_empty() {
        return Vec::new();
    }
    let index = (appearance_selection_seed(source_fingerprint, reference_form_id, root_form_id)
        as usize)
        % candidates.len();
    vec![candidates[index].1.clone()]
}

fn actor_gear_model_candidates(
    parsed: &ParsedPlugin,
    form_id: u32,
    visited: &mut HashSet<u32>,
    depth: usize,
) -> Vec<(u32, ApparelCandidate)> {
    if depth >= 32 || !visited.insert(form_id) {
        return Vec::new();
    }
    let Some(base) = parsed.bases.get(&form_id) else {
        return Vec::new();
    };
    if crate::vsa::assets::actor_visual_gear_kind(&base.kind) {
        let Some(models) = base.apparel_models.as_ref() else {
            return Vec::new();
        };
        let OpenMwItemStats::Apparel {
            armor_rating,
            max_condition,
            biped_slot_mask,
        } = &base.item_stats
        else {
            return Vec::new();
        };
        return vec![(
            form_id,
            ApparelCandidate {
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
            },
        )];
    }
    let Some(leveled) = base.leveled.as_ref() else {
        return Vec::new();
    };
    let mut candidates = Vec::new();
    for entry in &leveled.entries {
        candidates.extend(actor_gear_model_candidates(
            parsed,
            entry.item_form_id,
            visited,
            depth + 1,
        ));
    }
    candidates
}

fn actor_kind_matches(base: &BaseRecord, kind: ReferenceKind) -> bool {
    base.actor.as_ref().is_some_and(|actor| {
        matches!(
            (kind, actor.creature.is_some()),
            (ReferenceKind::Creature, true) | (ReferenceKind::Npc, false)
        )
    })
}

/// Flattens an LVLN/LVLC target into concrete actors while retaining stable
/// FormID ordering. Template shells such as `LvlRaiderGun` point at one of
/// these lists through `TPLT`; their own inventory is intentionally empty.
fn actor_candidate_ids(
    parsed: &ParsedPlugin,
    target_form_id: u32,
    kind: ReferenceKind,
    visited: &mut HashSet<u32>,
    depth: usize,
) -> Vec<u32> {
    if depth >= 32 || !visited.insert(target_form_id) {
        return Vec::new();
    }
    let Some(base) = parsed.bases.get(&target_form_id) else {
        return Vec::new();
    };
    if actor_kind_matches(base, kind) {
        return vec![target_form_id];
    }
    let Some(leveled) = base.leveled.as_ref() else {
        return Vec::new();
    };
    let mut candidates = Vec::new();
    for entry in &leveled.entries {
        candidates.extend(actor_candidate_ids(
            parsed,
            entry.item_form_id,
            kind,
            visited,
            depth + 1,
        ));
    }
    candidates.sort_unstable();
    candidates.dedup();
    candidates
}

/// Resolves both direct leveled placements and NPC/CREA template shells to a
/// concrete actor for appearance work. The actor catalog keeps the source
/// FormID for gameplay bookkeeping; this helper only chooses the deterministic
/// visual/inventory source used by the assembled GLB.
fn resolve_actor_appearance_base<'a>(
    parsed: &'a ParsedPlugin,
    start_form_id: u32,
    kind: ReferenceKind,
    reference_form_id: u32,
    source_fingerprint: &str,
) -> Option<&'a BaseRecord> {
    let mut current_form_id = start_form_id;
    let mut visited = HashSet::new();
    for _ in 0..32 {
        if !visited.insert(current_form_id) {
            return None;
        }
        let base = parsed.bases.get(&current_form_id)?;
        if !actor_kind_matches(base, kind) {
            let candidates =
                actor_candidate_ids(parsed, current_form_id, kind, &mut HashSet::new(), 0);
            let selected = candidates.get(
                (appearance_selection_seed(source_fingerprint, reference_form_id, current_form_id)
                    as usize)
                    % candidates.len().max(1),
            )?;
            current_form_id = *selected;
            continue;
        }
        if let Some(template_form_id) = base.base_template_form_id {
            let candidates =
                actor_candidate_ids(parsed, template_form_id, kind, &mut HashSet::new(), 0);
            if let Some(selected) = candidates.get(
                (appearance_selection_seed(source_fingerprint, reference_form_id, template_form_id)
                    as usize)
                    % candidates.len().max(1),
            ) {
                current_form_id = *selected;
                continue;
            }
        }
        return Some(base);
    }
    None
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
