use super::*;

pub(crate) fn run_view(
    manifest_path: PathBuf,
    disable_physics: bool,
    trace_seconds: Option<f32>,
    agent_port: Option<u16>,
    save_slot: Option<String>,
) -> Result<()> {
    let manifest_path = fs::canonicalize(&manifest_path).context("manifest does not exist")?;
    let text = fs::read_to_string(&manifest_path)?;
    let manifest: PreparedSceneManifest = from_str(&text).context("invalid scene manifest")?;
    ensure_prepared_manifest_compatible(
        &manifest,
        NIF_CONVERTER_REVISION,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )?;
    ensure_baked_scene_compatible(&manifest)?;
    let item_catalog = manifest
        .item_catalog_path
        .as_deref()
        .map(|relative| -> Result<PreparedItemCatalog> {
            let path = PathBuf::from(&manifest.asset_root)
                .join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
            let text = fs::read_to_string(&path)
                .with_context(|| format!("reading item catalog {}", path.display()))?;
            let hash = fingerprint(text.as_bytes());
            if manifest.item_catalog_hash.as_deref() != Some(hash.as_str()) {
                anyhow::bail!("item catalog hash does not match scene manifest");
            }
            let catalog: PreparedItemCatalog =
                from_str(&text).context("invalid prepared item catalog")?;
            if manifest.item_catalog_revision.as_deref() != Some(catalog.revision.as_str()) {
                anyhow::bail!("item catalog revision does not match scene manifest");
            }
            if catalog.source_fingerprint != manifest.source_fingerprint {
                anyhow::bail!(
                    "item catalog fingerprint {} does not match scene {}",
                    catalog.source_fingerprint,
                    manifest.source_fingerprint
                );
            }
            Ok(catalog)
        })
        .transpose()?
        .unwrap_or_default();
    // Issue #60 (F60.3): load and compatibility-check the save slot before
    // any window exists, so a mismatched save fails fast with a plain error.
    let loaded_save = save_slot
        .map(|slot| -> Result<crate::save::SaveGame> {
            let store = crate::save::SaveStore::new(".");
            let outcome = store
                .read_slot(&slot)
                .with_context(|| format!("loading save slot '{slot}'"))?;
            let plugins: Vec<crate::save::SavePlugin> = manifest
                .source_plugins
                .iter()
                .map(|plugin| crate::save::SavePlugin {
                    name: plugin.name.clone(),
                    fingerprint: plugin.fingerprint.clone(),
                })
                .collect();
            outcome
                .save
                .ensure_compatible(&manifest.source_fingerprint, &plugins)
                .with_context(|| {
                    format!("save slot '{slot}' is incompatible with the loaded content")
                })?;
            if let Some(warning) = outcome.warning {
                eprintln!("warning: {warning}");
            }
            Ok(outcome.save)
        })
        .transpose()?;
    let asset_root = PathBuf::from(&manifest.asset_root);
    let physics_assets = player::load_prepared_physics_assets(&manifest, &asset_root)?;
    let report_path = render_report_path(&manifest_path);
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(default_primary_window()),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_root.to_string_lossy().to_string(),
                ..default()
            }),
        FrameTimeDiagnosticsPlugin::new(RENDER_REPORT_HISTORY),
        RenderDiagnosticsPlugin,
        AutoExposurePlugin,
    ));
    app.add_plugins(crate::console::ConsolePlugin);
    // Issue #55 (A15): `RenderAssetBytesPerFrame` was tried here and
    // REVERTED — a 16 MB/frame upload throttle made the first hop's reveal
    // measure 119-126 ms (vs 25-35 unthrottled) because the freshly
    // revealed cell's meshes/images were still queued behind the budget.
    // Don't reintroduce it without re-measuring the full chain.
    app.insert_resource(physics_assets);
    app.insert_resource(item_catalog.clone());
    if let Some(port) = agent_port {
        agent_bridge::install(&mut app, port);
    }
    app.add_plugins(AppStatePlugin);
    app.insert_resource(LoadingTarget::NewGame {
        manifest: manifest_path.clone(),
    });

    player::install(&mut app, disable_physics);
    bindings::install(&mut app);
    audio::install(&mut app);
    interaction::install(&mut app);
    pipboy::install(&mut app);
    pipboy_reader::install(&mut app);
    animation::install(&mut app);
    console::install(&mut app);
    console_ui::install(&mut app);
    // F51.4: `[world] resident_cell_limit` in `.bevyout/config.toml` (or the
    // user config); `view`'s CLI args have no `--config` override plumbed
    // through yet (see src/config.rs's `resident_cell_limit` doc comment),
    // so this always uses the same project/user discovery `render`/`prepare`
    // use, defaulting to 4 when no config file is found.
    world::install(&mut app, crate::config::resident_cell_limit());
    world_items::install(&mut app);
    if let Some(save) = loaded_save {
        info!(
            "save slot loaded: cell {:08x}, {} cell states",
            save.header.current_cell,
            save.world.cells.len()
        );
        if let Some(player_state) = &save.player {
            app.insert_resource(interaction::PlayerInventory::from_stack_states(
                player_state
                    .inventory
                    .iter()
                    .map(|stack| inventory::InventoryStack {
                        base_form_id: stack.base_form_id,
                        count: stack.count,
                        condition: stack.condition.or_else(|| {
                            item_catalog
                                .items
                                .iter()
                                .find(|item| item.base_form_id == stack.base_form_id)
                                .and_then(|item| match &item.stats {
                                    PreparedItemStats::Weapon { max_condition, .. }
                                    | PreparedItemStats::Apparel { max_condition, .. } => {
                                        *max_condition
                                    }
                                    _ => None,
                                })
                        }),
                    }),
            ));
            // Issue #98 (F98.4): rebuild the equipped set and hotkey
            // bindings directly from persisted entries -- see
            // `player::equipment::EquipmentState::restore`'s doc comment
            // for why this trusts the save rather than replaying `equip`.
            app.insert_resource(interaction::PlayerEquipment::restore(
                player_state
                    .equipped
                    .iter()
                    .filter(|item| item.kind == crate::save::EquippedKind::Apparel)
                    .flat_map(|item| {
                        let key = inventory::StackKey {
                            base_form_id: item.base_form_id,
                            condition: item.condition,
                        };
                        let mask = item_catalog
                            .items
                            .iter()
                            .find(|catalog_item| catalog_item.base_form_id == item.base_form_id)
                            .and_then(|catalog_item| match &catalog_item.stats {
                                PreparedItemStats::Apparel {
                                    biped_slot_mask, ..
                                } => *biped_slot_mask,
                                _ => None,
                            })
                            .unwrap_or(0);
                        player::equipment::slots_from_mask(mask)
                            .into_iter()
                            .map(move |slot| (slot, key))
                            .collect::<Vec<_>>()
                    }),
                player_state
                    .equipped
                    .iter()
                    .find(|item| item.kind == crate::save::EquippedKind::Weapon)
                    .map(|item| {
                        let ammo_form_id = item_catalog
                            .items
                            .iter()
                            .find(|catalog_item| catalog_item.base_form_id == item.base_form_id)
                            .and_then(|catalog_item| match &catalog_item.stats {
                                PreparedItemStats::Weapon { ammo_form_id, .. } => *ammo_form_id,
                                _ => None,
                            });
                        (
                            inventory::StackKey {
                                base_form_id: item.base_form_id,
                                condition: item.condition,
                            },
                            ammo_form_id,
                        )
                    }),
                player_state
                    .equipped
                    .iter()
                    .find(|item| item.kind == crate::save::EquippedKind::Ammo)
                    .map(|item| inventory::StackKey {
                        base_form_id: item.base_form_id,
                        condition: item.condition,
                    }),
            ));
            let mut hotkeys = bindings::HotkeyBindings::default();
            for (index, binding) in player_state.hotkeys.iter().enumerate() {
                if let Some(binding) = binding {
                    hotkeys.assign(
                        (index + 1) as u8,
                        inventory::StackKey {
                            base_form_id: binding.base_form_id,
                            condition: binding.condition,
                        },
                    );
                }
            }
            app.insert_resource(hotkeys);
        }
        app.insert_resource(world_items::NextRuntimeItemId(save.next_runtime_item_id));
        app.insert_resource(world::ActiveSaveState(save.world));
        app.insert_resource(world::PlaythroughSeed(save.rng_state));
    }
    app.insert_resource(manifest)
        .insert_resource(UnlitMode(false))
        .insert_resource(LightingScale(DEFAULT_LIGHTING_SCALE))
        .insert_resource(IrradianceIntensity(1.0))
        .insert_resource(AmbientScale(0.05))
        .insert_resource(FogStrength(DEFAULT_FOG_STRENGTH))
        .insert_resource(AoStrength(1.0))
        .insert_resource(AoMeshBases::default())
        .insert_resource(RenderReportPath(report_path))
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(LightsDisabled(false))
        .insert_resource(PreparedPointShadowRuntime::default())
        .insert_resource(PointLightShadowSamples::default())
        // F35.6: the CLI's view/render flow auto-advances Boot -> Loading ->
        // InGame with no menu stop; MainMenu remains reachable in the state
        // graph but the CLI never observes it (LoadingTarget is always set).
        .add_systems(Update, (auto_advance_from_boot, auto_advance_from_loading))
        .add_systems(
            OnEnter(AppState::InGame),
            (
                capture_cursor,
                spawn_prepared_scene,
                // Issue #60 (F60.2): apply save state to the launch cell
                // before its colliders exist, so deleted references are
                // suppressed and dynamic restores ride the collider build.
                world::apply_save_state_at_startup,
                player::build_prepared_colliders,
                spawn_reticle,
            )
                .chain(),
        )
        .add_systems(Update, apply_lighting_scale)
        .add_systems(
            Update,
            (
                apply_fog_strength,
                apply_ao_strength,
                apply_irradiance_intensity,
                apply_horizontal_fov,
                update_fps_text,
                apply_unlit_mode,
                configure_glow_cards,
            ),
        )
        .add_systems(Update, record_render_sample)
        .add_systems(
            Update,
            (
                capture_cursor_input,
                free_fly_camera,
                player::fps_mouse_look,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    if let Some(seconds) = trace_seconds {
        if !seconds.is_finite() || seconds <= 0.0 {
            anyhow::bail!("--trace-seconds must be finite and greater than zero");
        }
        app.insert_resource(TraceCaptureLimit { remaining: seconds })
            .add_systems(Update, stop_trace_capture);
    }
    app.run();
    Ok(())
}

fn default_primary_window() -> Window {
    Window {
        resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
        focused: true,
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    }
}

#[derive(Resource)]
struct TraceCaptureLimit {
    remaining: f32,
}

fn stop_trace_capture(
    time: Res<Time>,
    mut limit: ResMut<TraceCaptureLimit>,
    mut app_exit: MessageWriter<AppExit>,
) {
    limit.remaining -= time.delta_secs();
    if limit.remaining <= 0.0 {
        app_exit.write(AppExit::Success);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_window_defaults_to_1080p() {
        let window = default_primary_window();
        assert_eq!(window.resolution.width(), 1920.0);
        assert_eq!(window.resolution.height(), 1080.0);
    }
}
