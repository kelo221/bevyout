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
    let navmeshes = stage_navmeshes(&scene_dir, &mut diagnostics, &parsed.navmeshes)?;
    // Parses the cell's references and stages their NIF/texture files under
    // the shared `staging_dir`/`assets_dir` (F48.4 parallel phase). Every
    // path written here is content-addressed (the NIF by its
    // normalized model path, textures by their own asset path, the
    // eventual GLB/physics pair by a content hash of the NIF bytes), so two
    // cells racing to stage the *same* missing asset write the same bytes
    // to the same path -- redundant in the worst case, not corrupting. That
    // is what makes it safe to run for several cells concurrently, unlike
    // the Blender/texture-conversion step immediately below.
    let stage = stage_placements(
        std::mem::take(&mut parsed.references),
        &parsed.bases,
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
        visual_assets,
        mut placements,
        lights,
        cache_hits,
        cache_missing,
        cache_invalid,
        cache_explicit_rebuilds,
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
    {
        let _blender_guard = session.blender_lock.lock().unwrap();
        convert_staged_textures(&staging_dir, &mut diagnostics)?;
        if !jobs.is_empty() {
            run_blender_batch(&blender, &jobs, &data_root, &staging_dir)
                .context("headless Blender conversion failed")?;
        }
    }
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
    for placement in &mut placements {
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
        if let Some(reason) = dynamic_rejection_reason(&placement.semantic, asset) {
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
        converter_revision: Some(NIF_CONVERTER_REVISION.into()),
        physics_schema_version: Some(PHYSICS_ASSET_SCHEMA_VERSION),
        asset_root: cache_dir.to_string_lossy().to_string(),
        source_plugin: plugin_path.to_string_lossy().to_string(),
        source_fingerprint,
        source_plugins,
        cell,
        placements,
        lights,
        diagnostics,
        visual_issues,
        navmeshes,
        cell_audio,
        audio_clips,
        footstep_sets,
        hard_landing_clips,
        mutability_summary,
        bake: None,
        static_point_shadows,
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
