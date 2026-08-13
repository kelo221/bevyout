use super::*;
use clap::Parser;

#[test]
fn fingerprint_check_always_uses_the_report_only_batch_path() {
    let args = PrepareArgs::try_parse_from(["prepare", "00000c49", "--check-fingerprints"])
        .expect("valid prepare arguments");

    assert!(prepare_requires_batch(&args, 1));
}

#[test]
fn exterior_radius_always_uses_the_batch_path() {
    let args = PrepareArgs::try_parse_from(["prepare", "00000c49", "--exterior-radius", "3"])
        .expect("valid prepare arguments");

    assert_eq!(args.exterior_radius, Some(3));
    assert!(prepare_requires_batch(&args, 1));
}

#[test]
fn exterior_radius_conflicts_with_whole_catalogue_selectors() {
    for conflicting in ["--all", "--all-interiors", "--worldspace"] {
        let mut argv = vec!["prepare", "00000c49", "--exterior-radius", "3", conflicting];
        if conflicting == "--worldspace" {
            argv.push("Wasteland");
        }

        assert!(PrepareArgs::try_parse_from(argv).is_err(), "{conflicting}");
    }
}

#[test]
fn all_exteriors_always_uses_the_batch_path() {
    let args = PrepareArgs::try_parse_from(["prepare", "--all-exteriors"])
        .expect("valid prepare arguments");

    assert!(args.all_exteriors);
    assert!(prepare_requires_batch(&args, 0));
}

#[test]
fn all_exteriors_conflicts_with_every_other_cell_selector() {
    for conflicting in [
        "--all",
        "--all-interiors",
        "--worldspace",
        "--exterior-radius",
        "positional",
    ] {
        let mut argv = vec!["prepare", "--all-exteriors"];
        match conflicting {
            "--worldspace" => argv.extend(["--worldspace", "Wasteland"]),
            "--exterior-radius" => argv.extend(["00000c49", "--exterior-radius", "3"]),
            "positional" => argv.push("00000c49"),
            flag => argv.push(flag),
        }

        assert!(PrepareArgs::try_parse_from(argv).is_err(), "{conflicting}");
    }
}

#[test]
fn single_exterior_selection_identifies_the_worldspace_index_to_publish() {
    let exterior = CellInfo {
        form_id: 0x0000_0aaa,
        editor_id: Some("WastelandSynthetic".into()),
        interior: false,
        worldspace_form_id: Some(0x0000_003c),
        ..synthetic_cell_info()
    };
    let interior = CellInfo {
        form_id: 0x0001_7f37,
        editor_id: Some("SuperDuperMart".into()),
        interior: true,
        worldspace_form_id: None,
        ..synthetic_cell_info()
    };
    let cells = [&exterior, &interior];

    assert_eq!(
        selected_exterior_worldspace(cells, &CellSelector::FormId(0x0000_0aaa)),
        Some(0x0000_003c)
    );
    assert_eq!(
        selected_exterior_worldspace(cells, &CellSelector::EditorId("wastelandsynthetic".into())),
        Some(0x0000_003c)
    );
    assert_eq!(
        selected_exterior_worldspace(cells, &CellSelector::EditorId("SuperDuperMart".into())),
        None
    );
}

fn synthetic_cell_info() -> CellInfo {
    CellInfo {
        form_id: 0,
        editor_id: None,
        name: None,
        interior: false,
        behave_like_exterior: false,
        ambient_rgba: [0.0; 4],
        directional_rgba: [0.0; 4],
        image_space_form_id: None,
        image_space: None,
        lighting_template_form_id: None,
        lighting_template_flags: 0,
        lighting_template: None,
        raw_lighting: None,
        effective_lighting: None,
        water_form_id: None,
        water_height: None,
        grid: None,
        worldspace_form_id: None,
        day_night_profile: None,
        day_night_preview_profile: None,
    }
}

#[test]
fn dialogue_voice_discovery_is_automatic_without_the_legacy_flag() {
    assert!(should_discover_dialogue_voice(false));
    assert!(should_discover_dialogue_voice(true));
}

#[test]
fn normal_reprepare_recovers_existing_authored_dialogue_and_voice() {
    let root = std::env::temp_dir().join(format!("bevyout-reprepare-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let dialogue_root = root.join("scenes/00003a2a/dialogue");
    fs::create_dir_all(dialogue_root.join("authored")).unwrap();
    fs::create_dir_all(root.join("audio")).unwrap();
    fs::write(
        dialogue_root.join("authored/moira_brown.yarn"),
        "title: MoiraBrown\n---\nMoira: hello\n===\n",
    )
    .unwrap();
    let catalog =
        crate::vsa::dialogue::prepare_catalog(vec![crate::vsa::dialogue::DialogueSource {
            relative_path: "authored/moira_brown.yarn".into(),
            kind: crate::vsa::dialogue::DialogueSourceKind::Authored,
            content: "title: MoiraBrown\n---\nMoira: hello\n===\n".into(),
        }]);
    fs::write(
        dialogue_root.join("catalog.ron"),
        ron::ser::to_string(&catalog).unwrap(),
    )
    .unwrap();
    fs::write(root.join("audio/moira.ogg"), b"preserved-voice").unwrap();
    let index = bevyout_core::dialogue::PreparedDialogueVoiceIndex {
        revision: bevyout_core::dialogue::DIALOGUE_VOICE_INDEX_REVISION.into(),
        entries: vec![bevyout_core::dialogue::DialogueVoiceAsset {
            line_key: bevyout_core::dialogue::DialogueLineKey::new("MoiraBrown:0"),
            asset_path: "audio/moira.ogg".into(),
            source_path: Some("dialogue/voice/moira.ogg".into()),
            ..Default::default()
        }],
        ..Default::default()
    };
    fs::write(
        dialogue_root.join("voice_index.ron"),
        ron::ser::to_string(&index).unwrap(),
    )
    .unwrap();

    let sources = read_existing_authored_dialogue_sources(&root, 0x0000_3a2a).unwrap();
    let voice = read_existing_authored_voice_input(&root, 0x0000_3a2a)
        .unwrap()
        .unwrap();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].relative_path, "authored/moira_brown.yarn");
    assert_eq!(voice.entries.len(), 1);
    assert_eq!(voice.entries[0].bytes, b"preserved-voice");
    let _ = fs::remove_dir_all(root);
}

#[cfg(test)]
mod day_night_profile_tests {
    use super::*;
    use crate::vsa::openmw_esm4::CellMetadata;
    use bevyout_core::time_of_day::{ColorKeyframes, DayNightTimings};

    fn colors(value: f32) -> ColorKeyframes {
        ColorKeyframes {
            sunrise: [value; 4],
            day: [value; 4],
            sunset: [value; 4],
            night: [value; 4],
        }
    }

    fn weather(form_id: u32, editor_id: &str) -> WeatherRecord {
        WeatherRecord {
            form_id,
            editor_id: Some(editor_id.into()),
            sky_upper: colors(0.1),
            sky_lower: colors(0.2),
            ambient: colors(0.3),
            sunlight: colors(0.4),
        }
    }

    fn climate(form_id: u32, weather_form_id: u32) -> ClimateRecord {
        ClimateRecord {
            form_id,
            editor_id: Some(format!("Climate{form_id:08x}")),
            weather_entries: vec![
                super::super::super::super::openmw_esm4::ClimateWeatherEntry {
                    weather_form_id,
                    chance: 100,
                },
            ],
            timings: DayNightTimings::default(),
        }
    }

    fn subrecord(signature: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut result = signature.to_vec();
        result.extend_from_slice(&(data.len() as u16).to_le_bytes());
        result.extend_from_slice(data);
        result
    }

    fn record(signature: &[u8; 4], form_id: u32, data: &[u8]) -> Vec<u8> {
        let mut result = signature.to_vec();
        result.extend_from_slice(&(data.len() as u32).to_le_bytes());
        result.extend_from_slice(&0_u32.to_le_bytes());
        result.extend_from_slice(&form_id.to_le_bytes());
        result.extend_from_slice(&[0; 8]);
        result.extend_from_slice(data);
        result
    }

    fn cell_metadata(climate_form_id: Option<u32>) -> Option<CellMetadata> {
        let mut bytes = record(b"TES4", 0, &[]);
        let data = climate_form_id
            .map(|form_id| subrecord(b"XCCM", &form_id.to_le_bytes()))
            .unwrap_or_default();
        bytes.extend(record(b"CELL", 1, &data));
        parse_content_set(
            &[PluginSource {
                name: "EnvironmentTests.esm",
                bytes: &bytes,
            }],
            &parse_cell_selector("1").unwrap(),
        )
        .expect("synthetic CELL climate metadata must parse")
        .cell_metadata
    }

    fn cell(worldspace_form_id: Option<u32>) -> super::super::super::super::manifest::CellInfo {
        super::super::super::super::manifest::CellInfo {
            form_id: 1,
            editor_id: None,
            name: None,
            interior: true,
            behave_like_exterior: true,
            ambient_rgba: [0.0; 4],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: None,
            worldspace_form_id,
            day_night_profile: None,
            day_night_preview_profile: None,
        }
    }

    #[test]
    fn authoritative_weather_uses_first_resolved_positive_weight_entry() {
        let mut parsed = ParsedPlugin::default();
        parsed.weathers.insert(0x20, weather(0x20, "FirstUsable"));
        parsed.weathers.insert(0x30, weather(0x30, "SecondUsable"));
        parsed.climates.insert(
            0x10,
            ClimateRecord {
                form_id: 0x10,
                editor_id: Some("Climate".into()),
                weather_entries: vec![
                    super::super::super::super::openmw_esm4::ClimateWeatherEntry {
                        weather_form_id: 0x99,
                        chance: 50,
                    },
                    super::super::super::super::openmw_esm4::ClimateWeatherEntry {
                        weather_form_id: 0x30,
                        chance: 0,
                    },
                    super::super::super::super::openmw_esm4::ClimateWeatherEntry {
                        weather_form_id: 0x20,
                        chance: 10,
                    },
                    super::super::super::super::openmw_esm4::ClimateWeatherEntry {
                        weather_form_id: 0x30,
                        chance: 90,
                    },
                ],
                timings: DayNightTimings::default(),
            },
        );
        parsed.worldspaces.insert(
            0x01,
            WorldspaceRecord {
                form_id: 0x01,
                editor_id: None,
                name: None,
                parent_form_id: None,
                parent_flags: 0,
                climate_form_id: Some(0x10),
            },
        );

        let profile = resolve_authoritative_day_night_profile(&cell(Some(0x01)), &parsed).unwrap();
        assert_eq!(profile.weather_form_id, 0x20);
    }

    #[test]
    fn authoritative_climate_precedence_is_cell_then_worldspace_then_parent() {
        let mut parsed = ParsedPlugin::default();
        for (weather_form_id, editor_id) in [
            (0x20, "CellWeather"),
            (0x30, "WorldWeather"),
            (0x40, "ParentWeather"),
        ] {
            parsed
                .weathers
                .insert(weather_form_id, weather(weather_form_id, editor_id));
        }
        parsed.climates.insert(0x10, climate(0x10, 0x20));
        parsed.climates.insert(0x11, climate(0x11, 0x30));
        parsed.climates.insert(0x12, climate(0x12, 0x40));
        parsed.worldspaces.insert(
            0x01,
            WorldspaceRecord {
                form_id: 0x01,
                editor_id: None,
                name: None,
                parent_form_id: Some(0x02),
                parent_flags: WorldspaceRecord::USE_PARENT_CLIMATE,
                climate_form_id: Some(0x11),
            },
        );
        parsed.worldspaces.insert(
            0x02,
            WorldspaceRecord {
                form_id: 0x02,
                editor_id: None,
                name: None,
                parent_form_id: None,
                parent_flags: 0,
                climate_form_id: Some(0x12),
            },
        );

        parsed.cell_metadata = cell_metadata(Some(0x10));
        let profile = resolve_authoritative_day_night_profile(&cell(Some(0x01)), &parsed).unwrap();
        assert_eq!(profile.climate_form_id, Some(0x10));
        assert_eq!(profile.weather_form_id, 0x20);

        parsed.cell_metadata = cell_metadata(None);
        let profile = resolve_authoritative_day_night_profile(&cell(Some(0x01)), &parsed).unwrap();
        assert_eq!(profile.climate_form_id, Some(0x12));
        assert_eq!(profile.weather_form_id, 0x40);

        parsed.worldspaces.get_mut(&0x01).unwrap().parent_flags = 0;
        let profile = resolve_authoritative_day_night_profile(&cell(Some(0x01)), &parsed).unwrap();
        assert_eq!(profile.climate_form_id, Some(0x11));
        assert_eq!(profile.weather_form_id, 0x30);
    }

    #[test]
    fn authoritative_profile_is_absent_for_missing_climate_or_weather_records() {
        let mut parsed = ParsedPlugin {
            cell_metadata: cell_metadata(Some(0x404)),
            ..ParsedPlugin::default()
        };
        assert!(resolve_authoritative_day_night_profile(&cell(None), &parsed).is_none());

        parsed.cell_metadata = cell_metadata(Some(0x10));
        parsed.climates.insert(0x10, climate(0x10, 0x505));
        assert!(resolve_authoritative_day_night_profile(&cell(None), &parsed).is_none());

        assert!(resolve_preview_day_night_profile(&ParsedPlugin::default()).is_none());
    }

    #[test]
    fn preview_prefers_wasteland_clear_then_lowest_usable_form_id() {
        let mut parsed = ParsedPlugin::default();
        parsed.weathers.insert(0x01, weather(0x01, "Cloudy"));
        parsed
            .weathers
            .insert(0x50, weather(0x50, "WastelandClear"));
        let profile = resolve_preview_day_night_profile(&parsed).unwrap();
        assert_eq!(profile.weather_form_id, 0x50);
        assert_eq!(
            profile.source,
            PreparedDayNightProfileSource::PreviewFallback
        );

        parsed.weathers.remove(&0x50);
        let profile = resolve_preview_day_night_profile(&parsed).unwrap();
        assert_eq!(profile.weather_form_id, 0x01);
    }
}

#[cfg(test)]
mod actor_assembly_policy_tests {
    use super::*;
    use crate::vsa::openmw_esm4::{
        ApparelModelSet, BaseRecord, LeveledListData, LeveledListEntry, RacePartEntry,
    };

    fn race_part(index: u32, model_path: Option<&str>) -> RacePartEntry {
        RacePartEntry {
            index,
            model_path: model_path.map(str::to_owned),
        }
    }

    fn apparel(form_id: u32) -> (u32, BaseRecord) {
        let mut record = BaseRecord::default();
        record.kind = "ARMO".into();
        record.apparel_models = Some(ApparelModelSet {
            male_worn: Some(format!("armor/m/{form_id:08x}.nif")),
            female_worn: Some(format!("armor/f/{form_id:08x}.nif")),
            ..ApparelModelSet::default()
        });
        record.item_stats = OpenMwItemStats::Apparel {
            armor_rating: Some(10.0),
            max_condition: Some(100),
            biped_slot_mask: Some(4),
        };
        (form_id, record)
    }

    fn leveled(form_id: u32, flags: u8, entries: &[u32]) -> (u32, BaseRecord) {
        let mut record = BaseRecord::default();
        record.kind = "LVLI".into();
        record.leveled = Some(LeveledListData {
            flags,
            entries: entries
                .iter()
                .copied()
                .map(|item_form_id| LeveledListEntry {
                    level: 1,
                    item_form_id,
                    count: 1,
                })
                .collect(),
            ..LeveledListData::default()
        });
        (form_id, record)
    }

    #[test]
    fn upper_body_texture_slot_is_not_required_geometry() {
        let parts = [
            race_part(0, Some("upper.nif")),
            race_part(1, Some("left.nif")),
            race_part(2, Some("right.nif")),
            race_part(3, None),
        ];
        assert!(required_race_body_parts_present(&parts));
        assert!(!race_body_part_requires_model(3));
    }

    #[test]
    fn base_head_is_required_but_supplemental_head_slots_are_optional() {
        let parts = [race_part(0, Some("head.nif")), race_part(3, None)];
        assert!(required_race_head_parts_present(&parts));
        assert!(race_head_part_requires_model(0));
        assert!(!race_head_part_requires_model(3));
        assert_eq!(race_head_mesh_role(6), ActorMeshRole::Eyes);
        assert_eq!(race_head_mesh_role(7), ActorMeshRole::Eyes);
    }

    #[test]
    fn creature_main_model_matches_its_resolved_directory() {
        assert!(creature_path_is_main_model(
            "creatures/protectron/protectron.nif"
        ));
        assert!(!creature_path_is_main_model(
            "creatures/protectron/blowawaydome.nif"
        ));
    }

    #[test]
    fn creature_primary_falls_back_to_largest_visual_without_using_source_order() {
        let parts = vec![
            (0, "creatures/gutsy/buzzsaw.nif".into(), 200, false),
            (1, "creatures/gutsy/misterhandy.nif".into(), 2_000, false),
            (2, "creatures/gutsy/flamer.nif".into(), 300, false),
        ];
        assert_eq!(select_creature_main_model(&parts), Some(1));

        let reversed = parts.into_iter().rev().collect::<Vec<_>>();
        assert_eq!(
            reversed[select_creature_main_model(&reversed).unwrap()].1,
            "creatures/gutsy/misterhandy.nif"
        );
    }

    #[test]
    fn actor_outfit_use_all_keeps_one_candidate_from_each_direct_entry() {
        let parsed = ParsedPlugin {
            bases: HashMap::from([
                leveled(0x10, 0x04, &[0x20, 0x30]),
                leveled(0x20, 0, &[0x100, 0x101]),
                leveled(0x30, 0, &[0x200, 0x201]),
                apparel(0x100),
                apparel(0x101),
                apparel(0x200),
                apparel(0x201),
            ])
            .into(),
            ..ParsedPlugin::default()
        };

        let selected = resolve_actor_gear_candidates(&parsed, 0x10, 1, "fp");
        assert_eq!(selected.len(), 2);
        assert!(
            selected
                .iter()
                .any(|item| matches!(item.form_id, 0x100 | 0x101))
        );
        assert!(
            selected
                .iter()
                .any(|item| matches!(item.form_id, 0x200 | 0x201))
        );
    }

    #[test]
    fn actor_outfit_without_use_all_selects_only_one_candidate() {
        let parsed = ParsedPlugin {
            bases: HashMap::from([
                leveled(0x10, 0, &[0x100, 0x200]),
                apparel(0x100),
                apparel(0x200),
            ])
            .into(),
            ..ParsedPlugin::default()
        };

        assert_eq!(
            resolve_actor_gear_candidates(&parsed, 0x10, 1, "fp").len(),
            1
        );
    }

    #[test]
    fn actor_outfit_honors_guaranteed_chance_none() {
        let mut root = leveled(0x10, 0, &[0x100]).1;
        root.leveled.as_mut().expect("leveled fixture").chance_none = 100;
        let parsed = ParsedPlugin {
            bases: HashMap::from([(0x10, root), apparel(0x100)]).into(),
            ..ParsedPlugin::default()
        };

        assert!(resolve_actor_gear_candidates(&parsed, 0x10, 1, "fp").is_empty());
    }
}
