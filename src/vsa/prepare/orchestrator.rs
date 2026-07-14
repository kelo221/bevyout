use super::*;
use crate::vsa::catalog::CellCatalog;

/// Dispatches `prepare`: a single legacy selector goes straight through the
/// unchanged single-cell path below; `--all`/`--all-interiors`/`--worldspace`,
/// `--list-only`, or more than one positional selector build a lightweight
/// cell catalogue and resolve the batch through `resolve_selection` (#46).
pub fn prepare(args: PrepareArgs) -> Result<()> {
    let mut explicit = args.selectors.clone();
    explicit.extend(args.cell.clone());

    let is_batch = args.list_only
        || args.all
        || args.all_interiors
        || args.worldspace.is_some()
        || explicit.len() > 1;

    if !is_batch {
        let selector_input = explicit
            .into_iter()
            .next()
            .context("provide a GECK EditorID/FormID selector or legacy --cell, or pass --all/--all-interiors/--worldspace")?;
        return prepare_one(args, selector_input);
    }

    prepare_batch(args, explicit)
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

    let loaded_plugins = load_plugin_chain(&plugin_path, &data_root)?;
    let fingerprint = content_set_fingerprint(&loaded_plugins);
    let sources = loaded_plugins
        .iter()
        .map(|plugin| PluginSource {
            name: &plugin.name,
            bytes: &plugin.bytes,
        })
        .collect::<Vec<_>>();
    let catalog = CellCatalog::build(&sources, fingerprint)?;
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

    let resolved = resolve_selection(&cells, &worldspace_names, &spec)?;

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

    let total = resolved.len();
    let mut failed = Vec::new();
    for form_id in resolved {
        let selector_input = format!("{form_id:08x}");
        let mut cell_args = args.clone();
        cell_args.selectors = vec![selector_input.clone()];
        cell_args.cell = None;
        cell_args.all = false;
        cell_args.all_interiors = false;
        cell_args.worldspace = None;
        cell_args.list_only = false;
        if let Err(error) = prepare_one(cell_args, selector_input.clone()) {
            eprintln!("cell {selector_input} failed: {error:#}");
            failed.push(selector_input);
        }
    }

    if failed.is_empty() {
        println!("prepared {total} cells, 0 failed");
    } else {
        println!(
            "prepared {} cells, {} failed: {}",
            total - failed.len(),
            failed.len(),
            failed.join(", ")
        );
        bail!("{} of {total} cell(s) failed to prepare", failed.len());
    }
    Ok(())
}

fn prepare_one(args: PrepareArgs, selector_input: String) -> Result<()> {
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

    let loaded_plugins = load_plugin_chain(&plugin_path, &data_root)?;
    let source_fingerprint = content_set_fingerprint(&loaded_plugins);
    let source_plugins = loaded_plugins
        .iter()
        .map(|plugin| PreparedPluginSource {
            name: plugin.name.clone(),
            path: plugin.path.to_string_lossy().to_string(),
            fingerprint: fingerprint(&plugin.bytes),
        })
        .collect::<Vec<_>>();
    let mut diagnostics = Vec::new();
    let mut validator = esplugin::Plugin::new(esplugin::GameId::Fallout3, &plugin_path);
    if let Err(error) = validator.parse_file(esplugin::ParseOptions::header_only()) {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!("esplugin validation failed: {error}"),
        });
    }
    let plugin_sources = loaded_plugins
        .iter()
        .map(|plugin| PluginSource {
            name: &plugin.name,
            bytes: &plugin.bytes,
        })
        .collect::<Vec<_>>();
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

    let archives = load_archives(&data_root, &mut diagnostics)?;
    let audio_plugin_names = loaded_plugins
        .iter()
        .rev()
        .map(|plugin| plugin.name.clone())
        .collect::<Vec<_>>();
    let audio_archive_load = load_audio_archives(&data_root, &audio_plugin_names);
    diagnostics.extend(
        audio_archive_load
            .diagnostics
            .into_iter()
            .map(|message| Diagnostic {
                severity: "info".into(),
                message,
            }),
    );
    let (cell_audio, audio_clips) = stage_audio(
        &data_root,
        &audio_archive_load.archives,
        &parsed,
        &mut diagnostics,
        &cache_dir.join("audio"),
    )?;
    let (footstep_sets, hard_landing_clips) = stage_footsteps(
        &data_root,
        &audio_archive_load.archives,
        &mut diagnostics,
        &cache_dir.join("audio"),
    )?;
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
    let PlacementStage {
        jobs,
        mut placements,
        lights,
        cache_hits,
        cache_missing,
        cache_invalid,
        cache_explicit_rebuilds,
    } = stage_placements(
        std::mem::take(&mut parsed.references),
        &parsed.bases,
        &data_root,
        &archives,
        &staging_dir,
        &assets_dir,
        &mut diagnostics,
        args.rebuild_assets,
    )?;
    convert_staged_textures(&staging_dir, &mut diagnostics)?;
    let cache_summary = format!(
        "asset cache: reused {cache_hits}, missing {cache_missing}, invalid {cache_invalid}, explicitly rebuilt {cache_explicit_rebuilds}; scheduled {} NIF-to-GLB conversion(s)",
        jobs.len()
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: cache_summary.clone(),
    });
    println!("{cache_summary}");
    if !jobs.is_empty() {
        run_blender_batch(&blender, &jobs, &data_root, &staging_dir)
            .context("headless Blender conversion failed")?;
    }
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
            let path = cache_dir.join(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR));
            physics_assets.insert(relative_path.clone(), read_physics_asset(&path)?);
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
    println!("{mutability_log}");
    let failures = placements.iter().filter(|p| p.error.is_some()).count();
    if args.strict && failures > 0 {
        bail!("strict preparation failed with {failures} unresolved placements")
    }
    if placements.iter().all(|p| p.asset_path.is_none()) && lights.is_empty() {
        bail!(
            "no renderable assets were found in {}",
            super::super::manifest::cell_label(&cell)
        )
    }

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
        navmeshes,
        cell_audio,
        audio_clips,
        footstep_sets,
        hard_landing_clips,
        mutability_summary,
        bake: None,
    };
    let manifest_path = scene_dir.join("scene.ron");
    fs::write(
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    println!(
        "prepared {} ({} placements, {} unresolved) -> {}",
        super::super::manifest::cell_label(&manifest.cell),
        manifest.placements.len(),
        failures,
        manifest_path.display()
    );
    Ok(())
}
