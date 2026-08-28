use super::*;

pub(crate) struct RunViewOptions {
    pub(crate) disable_physics: bool,
    pub(crate) realtime_shadows: bool,
    pub(crate) worldspace_lod: bool,
    pub(crate) trace_seconds: Option<f32>,
    pub(crate) day_night_cycle_seconds: Option<f32>,
    pub(crate) agent_port: Option<u16>,
    pub(crate) unfocused: bool,
    pub(crate) save_slot: Option<String>,
    pub(crate) wgpu_validation: bool,
}

pub(crate) fn run_view(manifest_path: PathBuf, options: RunViewOptions) -> Result<()> {
    let RunViewOptions {
        disable_physics,
        realtime_shadows,
        worldspace_lod,
        trace_seconds,
        day_night_cycle_seconds,
        agent_port,
        unfocused,
        save_slot,
        wgpu_validation,
    } = options;
    let manifest_path = fs::canonicalize(&manifest_path).context("manifest does not exist")?;
    let text = fs::read_to_string(&manifest_path)?;
    let mut manifest: PreparedSceneManifest = from_str(&text).context("invalid scene manifest")?;
    crate::vsa::hydrate_exterior_package(&mut manifest)?;
    ensure_prepared_manifest_compatible_any(
        &manifest,
        SUPPORTED_PREPARED_CONVERTER_REVISIONS,
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
            // A manifest/catalog pair from an older build cross-matches
            // above even though its shape is stale (serde-defaulted fields
            // silently degrade equip rules); pin the pair to this build's
            // catalog revision instead of loading it degraded.
            if catalog.revision != ITEM_CATALOG_REVISION {
                anyhow::bail!(
                    "item catalog revision {} is stale, expected {ITEM_CATALOG_REVISION}; run `prepare` again",
                    catalog.revision,
                );
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
    let asset_root = PathBuf::from(&manifest.asset_root);
    let actor_definition_catalog = actor_state::load_catalog_for_manifest(&manifest, &asset_root)?;
    let actor_animation_catalog =
        actor_animation::load_catalog_for_manifest(&manifest, &asset_root)?;
    let screen_fx_catalog = screen_fx::load_catalog_for_manifest(&manifest, &asset_root)?;
    // M9 wave 1 (#310): GMST settings for the stat kernels; unlike the
    // catalogs above, a missing or stale gmst catalog falls back to GOTY
    // defaults instead of failing startup.
    let stats_settings = stats::load_settings_for_manifest(&manifest, &asset_root);
    // M9 wave 2 (#314): perk definitions for the perk console commands;
    // also degrades to an empty catalog instead of failing startup.
    let perk_catalog = stats::load_perk_catalog_for_manifest(&manifest, &asset_root);
    // M9 wave 3 (#318): ingestible definitions for the chem/aid runtime and
    // console commands; also degrades to empty instead of failing startup.
    let effect_catalog = effects::load_effect_catalog_for_manifest(&manifest, &asset_root);
    let recipe_catalog = recipes::load_recipe_catalog_for_manifest(&manifest, &asset_root);
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
                warn!("{warning}");
            }
            Ok(outcome.save)
        })
        .transpose()?;
    let physics_assets = player::load_prepared_physics_assets(&manifest, &asset_root)?;
    let report_path = render_report_path(&manifest_path);
    // Issue #180: an agent-bridge launch must never steal focus/raise the
    // window over whatever the human is doing, so it always starts
    // unfocused even without the explicit `--unfocused` flag.
    let start_unfocused = should_start_unfocused(unfocused, agent_port);
    let mut app = App::new();
    app.add_plugins((
        task_pools::ViewerIoTaskPoolPlugin,
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(default_primary_window(start_unfocused)),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_root.to_string_lossy().to_string(),
                ..default()
            })
            .set(super::gpu_validation::viewer_render_plugin(
                super::gpu_validation::gpu_validation_enabled_from_env(wgpu_validation),
            )),
        FrameTimeDiagnosticsPlugin::new(RENDER_REPORT_HISTORY),
        RenderDiagnosticsPlugin,
        AutoExposurePlugin,
    ));
    // Issue #180: with no `WinitSettings` configured, Bevy's default
    // `game()` update mode drops to a reactive/low-power tick once the
    // window loses focus or is occluded (routine on macOS -- Cmd+Tab,
    // another window overlapping it, or the `--unfocused` launch this issue
    // adds). The agent bridge's `bevyout.*` methods run via
    // `with_method_main`, which only executes on an app schedule tick, so a
    // starved tick rate reads as the bridge hanging (measured: 2+ minutes
    // unresponsive with no recovery). `WinitSettings::continuous()` keeps
    // both the focused and unfocused update modes at `UpdateMode::Continuous`
    // unconditionally. Trade-off: the viewer now burns CPU/GPU continuously
    // even when unfocused/occluded, which a power-conscious desktop app
    // would normally avoid -- accepted because a responsive agent bridge and
    // an always-ticking manual-acceptance window matter more here than idle
    // power draw.
    app.insert_resource(bevy::winit::WinitSettings::continuous());
    // Realtime point shadows are a short-range quality/performance aid for
    // the strongest light. Keep the prepared cubemap artifacts at their
    // independent resolution; this only lowers the runtime shadow target.
    app.insert_resource(PointLightShadowMap {
        size: REALTIME_POINT_SHADOW_MAP_SIZE,
    });
    // Issue #55 (A15): `RenderAssetBytesPerFrame` was tried here and
    // REVERTED — a 16 MB/frame upload throttle made the first hop's reveal
    // measure 119-126 ms (vs 25-35 unthrottled) because the freshly
    // revealed cell's meshes/images were still queued behind the budget.
    // Don't reintroduce it without re-measuring the full chain.
    app.insert_resource(physics_assets);
    app.insert_resource(item_catalog.clone());
    let mut actor_definition_catalogs = actor_state::ActorDefinitionCatalogs::default();
    if let Some(catalog) = actor_definition_catalog {
        actor_definition_catalogs.insert(manifest.cell.form_id, catalog);
    }
    app.insert_resource(actor_definition_catalogs);
    let mut actor_animation_catalogs = actor_animation::ActorAnimationCatalogs::default();
    if let Some(catalog) = actor_animation_catalog {
        actor_animation_catalogs.insert(manifest.cell.form_id, catalog);
    }
    app.insert_resource(actor_animation_catalogs);
    app.insert_resource(screen_fx_catalog);
    app.insert_resource(stats_settings);
    app.insert_resource(perk_catalog);
    app.insert_resource(effect_catalog);
    app.insert_resource(recipe_catalog);
    // F51.4: `[world] resident_cell_limit` in `.bevyout/config.toml` (or the
    // user config) remains the conservative interior graph-preload budget.
    // `[world]` limits are intentionally separate: interior preloading keeps
    // its conservative graph budget while exterior streaming defaults to the
    // authored 5x5 uGrids-style window.
    app.add_plugins(plugins::ViewerPlugins {
        disable_physics,
        resident_cell_limit: crate::config::resident_cell_limit(),
        exterior_resident_cell_limit: crate::config::exterior_resident_cell_limit(),
        agent_port,
        day_night_cycle_seconds,
        worldspace_lod,
    });
    app.insert_resource(LoadingTarget::NewGame {
        manifest: manifest_path.clone(),
    });
    if let Some(save) = loaded_save {
        info!(
            "save slot loaded: cell {:08x}, {} cell states",
            save.header.current_cell,
            save.world.cells.len()
        );
        let canonical = save
            .canonical
            .clone()
            .map(interaction::CanonicalItemLedger::from_snapshot)
            .transpose()
            .context("loading canonical item ledger")?
            .unwrap_or_default();
        if let Some(player_state) = &save.player {
            let canonical_player = canonical
                .ledger
                .holders()
                .get(&crate::item_transaction::HolderId::Player);
            let stacks: Vec<inventory::InventoryStack> = canonical_player
                .map(|state| {
                    state.items.iter().map(|item| inventory::InventoryStack {
                        base_form_id: item.base_form_id,
                        count: i32::try_from(item.count).unwrap_or(i32::MAX),
                        condition: item.state.condition,
                    })
                })
                .map(|items| items.collect())
                .unwrap_or_else(|| {
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
                        })
                        .collect()
                });
            app.insert_resource(interaction::PlayerInventory::from_stack_states(stacks));
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
        app.insert_resource(canonical);
        app.insert_resource(world_items::NextRuntimeItemId(save.next_runtime_item_id));
        app.insert_resource(world::ActiveSaveState(save.world));
        app.insert_resource(world::CurrentWorldLocation(save.location.clone()));
        app.insert_resource(world::PlaythroughSeed(save.rng_state));
        app.insert_resource(weapon::CombatRngRuntime(save.combat_rng));
        app.insert_resource(stats::PlayerProgression {
            stats: save.rpg.stats.clone(),
            perks: save.rpg.perks.clone(),
            unspent_skill_points: save.rpg.unspent_skill_points,
            total_skill_points: save.rpg.total_skill_points,
            radiation: save.rpg.radiation,
            effects: save.rpg.effects.clone(),
            chem_doses_ms: save.rpg.chem_doses_ms.clone(),
            addictions: save.rpg.addictions.clone(),
            current_health: save.rpg.current_health,
            limbs: save.rpg.limbs.clone(),
            crime: save.rpg.crime.clone(),
        });
        app.insert_resource(effects::RngResource(save.rpg.rng));
        {
            let mut runtime = crate::viewer::game_time::GameTimeRuntime::default();
            runtime.world.restore_snapshot(save.rpg.lifecycle.clone());
            runtime.world.clock = save.rpg.clock;
            runtime.world.effects = save.rpg.effects.clone();
            runtime.world.chem_doses_ms = save.rpg.chem_doses_ms.clone();
            runtime.world.addictions = save.rpg.addictions.clone();
            runtime.world.radiation = save.rpg.radiation;
            runtime.world.current_health = save.rpg.current_health;
            runtime.world.limbs = save.rpg.limbs.clone();
            app.insert_resource(runtime);
        }
        app.world_mut()
            .resource_mut::<dialogue::DialogueRuntime>()
            .restore_snapshot(save.dialogue);
    }
    let image_space_emission = image_space_emission_multiplier(manifest.cell.image_space.as_ref());
    app.insert_resource(crate::viewer::LoadedSceneManifest(manifest))
        .insert_resource(UnlitMode(false))
        .insert_resource(LightingScale(DEFAULT_LIGHTING_SCALE))
        .insert_resource(IrradianceIntensity(1.0))
        .insert_resource(AmbientScale(0.05))
        .insert_resource(FogStrength(DEFAULT_FOG_STRENGTH))
        .insert_resource(VolumetricFogMultiplier(DEFAULT_VOLUMETRIC_FOG_MULTIPLIER))
        .insert_resource(AoStrength(1.0))
        .insert_resource(EmissionScale(DEFAULT_EMISSION_SCALE))
        .init_resource::<MaterialClampSettings>()
        .init_resource::<MaterialClampBaselines>()
        .init_resource::<LegacyChanSettings>()
        .init_resource::<OverlayLightingSettings>()
        .init_resource::<LegacyWorldMaterials>()
        .insert_resource(ReflectionProbeSettings::default())
        .insert_resource(image_space_emission)
        .insert_resource(ImageSpaceBloomOverrides::default())
        .insert_resource(AuthorizedEmissionMaterials::default())
        .insert_resource(AoMeshBases::default())
        .insert_resource(AoEligibility::default())
        .insert_resource(RenderReportPath(report_path))
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(LightsDisabled(false))
        .insert_resource(PreparedPointShadowRuntime::default())
        .insert_resource(RealtimeShadowLight::default())
        .insert_resource(RealtimeShadowSettings {
            enabled: realtime_shadows,
        })
        .insert_resource(PointLightShadowSamples::default())
        // F35.6: the CLI's view/render flow auto-advances Boot -> Loading ->
        // InGame with no menu stop; MainMenu remains reachable in the state
        // graph but the CLI never observes it (LoadingTarget is always set).
        // Issue #131: release both the logical `ButtonInput<Key>` and
        // physical `ButtonInput<KeyCode>` resources on any window focus
        // change, not just `KeyboardFocusLost` -- a same-frame focus
        // false/true bounce (e.g. macOS's Cmd+Shift+5 overlay) suppresses
        // both Bevy's own key release and its `KeyboardFocusLost` message.
        // See the doc comment on `release_stuck_keys_on_focus_change` for
        // why.
        .add_systems(
            PreUpdate,
            release_stuck_keys_on_focus_change.after(bevy::input::InputSystems),
        )
        // Issue #131 follow-up: on macOS, a Cmd+Shift+5 screen recording can
        // leave the window unfocused forever -- mouse events keep reaching
        // it but no `WindowFocused { focused: true }` ever arrives, so the
        // console stays dead. `Update` (rather than `PreUpdate`) is fine
        // here: this only needs to observe the click before the frame's
        // `ButtonInput::clear()`, same timing as `capture_cursor_input`
        // below, and unlike `release_stuck_keys_on_focus_change` it doesn't
        // need to run relative to `InputSystems`. See the doc comment on
        // `request_focus_on_click_while_unfocused` for the component-write
        // vs. direct-winit-call trade-off.
        .add_systems(Update, request_focus_on_click_while_unfocused)
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
            apply_volumetric_fog.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                apply_realtime_shadow_light,
                mark_prepared_shadow_meshes,
                attach_prepared_lightmaps,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                apply_fog_strength,
                (track_ao_mesh_eligibility, apply_ao_strength).chain(),
                apply_irradiance_intensity,
                apply_horizontal_fov,
                update_fps_text,
                apply_unlit_mode,
                apply_material_clamps,
                apply_reflection_probe_settings,
                configure_glow_cards,
                configure_fallout_surface_materials,
                apply_overlay_lighting_settings,
                apply_legacy_chan_strength,
                configure_fallout_translucency,
            ),
        )
        .add_systems(
            Update,
            (configure_fallout_emission, apply_emission_scale).chain(),
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

/// Issue #180: an explicit `--unfocused` flag or an active agent bridge
/// (`agent_port.is_some()`) both mean the launch must not steal OS input
/// focus on startup.
fn should_start_unfocused(unfocused: bool, agent_port: Option<u16>) -> bool {
    unfocused || agent_port.is_some()
}

fn default_primary_window(unfocused: bool) -> Window {
    Window {
        resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
        focused: !unfocused,
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
#[path = "tests/app.rs"]
mod tests;
