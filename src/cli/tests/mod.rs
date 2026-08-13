use super::*;
use std::path::{Path, PathBuf};

#[test]
fn view_and_render_validate_day_night_cycle_duration() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "view",
        "--manifest",
        "scene.ron",
        "--day-night-cycle-seconds",
        "60",
    ])
    .unwrap();
    let CommandLine::View(args) = cli.command else {
        panic!("expected view command");
    };
    assert_eq!(args.day_night_cycle_seconds, Some(60.0));

    let cli = Cli::try_parse_from([
        "bevyout",
        "render",
        "SuperDuperMart",
        "--day-night-cycle-seconds",
        "86400",
    ])
    .unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(args.day_night_cycle_seconds, Some(86_400.0));

    assert!(
        Cli::try_parse_from([
            "bevyout",
            "view",
            "--manifest",
            "scene.ron",
            "--day-night-cycle-seconds",
            "0",
        ])
        .is_err()
    );
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "render",
            "SuperDuperMart",
            "--day-night-cycle-seconds",
            "86401",
        ])
        .is_err()
    );
}

#[test]
fn animation_zoo_requires_an_actor_and_validates_bridge_options() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "animation-zoo",
        "SuperDuperMart",
        "--actor",
        "00041606",
        "--start-clip",
        "idle",
    ])
    .unwrap();
    let CommandLine::AnimationZoo(args) = cli.command else {
        panic!("expected animation-zoo command");
    };
    assert_eq!(args.selector, "SuperDuperMart");
    assert_eq!(args.actor, "00041606");
    assert_eq!(args.start_clip.as_deref(), Some("idle"));
    assert_eq!(args.agent_port, 15_702);
    assert!(!args.agent_bridge);
    assert!(Cli::try_parse_from(["bevyout", "animation-zoo", "SuperDuperMart"]).is_err());
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "animation-zoo",
            "SuperDuperMart",
            "--actor",
            "00041606",
            "--agent-port",
            "16000",
        ])
        .is_err()
    );
}

#[test]
fn ragdoll_lab_defaults_to_avian_and_accepts_boxddd_comparison() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "ragdoll-lab",
        "SuperDuperMart",
        "--actor",
        "00041606",
    ])
    .unwrap();
    let CommandLine::RagdollLab(args) = cli.command else {
        panic!("expected ragdoll-lab command");
    };
    assert_eq!(args.selector, "SuperDuperMart");
    assert_eq!(args.actor, "00041606");
    assert_eq!(args.backend, RagdollLabBackend::Avian);
    assert_eq!(args.agent_port, 15_702);

    let cli = Cli::try_parse_from([
        "bevyout",
        "ragdoll-lab",
        "SuperDuperMart",
        "--actor",
        "00041606",
        "--backend",
        "boxddd",
        "--agent-bridge",
        "--agent-port",
        "16000",
    ])
    .unwrap();
    let CommandLine::RagdollLab(args) = cli.command else {
        panic!("expected ragdoll-lab command");
    };
    assert_eq!(args.backend, RagdollLabBackend::Boxddd);
    assert!(args.agent_bridge);
    assert_eq!(args.agent_port, 16_000);

    assert!(Cli::try_parse_from(["bevyout", "ragdoll-lab", "SuperDuperMart"]).is_err());
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "ragdoll-lab",
            "SuperDuperMart",
            "--actor",
            "00041606",
            "--agent-port",
            "16000",
        ])
        .is_err()
    );
}

#[test]
fn nif_convert_requires_one_source_and_parses_conversion_options() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "nif-convert",
        "--input",
        "mesh.nif",
        "--output",
        "out.glb",
        "--conversion",
        "quick-ao",
        "--allow-lossy",
        "--force",
    ])
    .unwrap();
    let CommandLine::NifConvert(args) = cli.command else {
        panic!("expected nif-convert command");
    };
    assert_eq!(args.input.as_deref(), Some(Path::new("mesh.nif")));
    assert_eq!(args.output, PathBuf::from("out.glb"));
    assert_eq!(args.conversion, NifConversionMode::QuickAo);
    assert!(args.allow_lossy);
    assert!(args.force);

    assert!(Cli::try_parse_from(["bevyout", "nif-convert", "--output", "out.glb"]).is_err());
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "nif-convert",
            "--input",
            "mesh.nif",
            "--asset",
            "meshes/mesh.nif",
            "--output",
            "out.glb",
        ])
        .is_err()
    );
}

#[test]
fn native_conversion_is_authoritative_and_blender_flags_are_rejected() {
    let cli = Cli::try_parse_from(["bevyout", "prepare", "SuperDuperMart"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(
        args.actor_animation_converter,
        ActorAnimationConverter::Native
    );

    let cli = Cli::try_parse_from([
        "bevyout",
        "prepare",
        "SuperDuperMart",
        "--actor-animation-converter",
        "disabled",
    ])
    .unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(
        args.actor_animation_converter,
        ActorAnimationConverter::Disabled
    );

    assert!(
        Cli::try_parse_from([
            "bevyout",
            "prepare",
            "SuperDuperMart",
            "--converter",
            "unknown",
        ])
        .is_err()
    );
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "prepare",
            "SuperDuperMart",
            "--actor-animation-converter",
            "blender",
        ])
        .is_err()
    );
}

#[test]
fn render_actor_animation_converter_defaults_to_native_and_accepts_disabled() {
    let cli = Cli::try_parse_from(["bevyout", "render", "SuperDuperMart"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(
        args.actor_animation_converter,
        ActorAnimationConverter::Native
    );

    let cli = Cli::try_parse_from([
        "bevyout",
        "render",
        "SuperDuperMart",
        "--actor-animation-converter",
        "disabled",
    ])
    .unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(
        args.actor_animation_converter,
        ActorAnimationConverter::Disabled
    );
}

#[test]
fn static_batch_chunk_size_defaults_to_64_metres_and_enforces_bounds() {
    let cli = Cli::try_parse_from(["bevyout", "bake", "--manifest", "scene.ron"]).unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert_eq!(args.static_batch_chunk_meters, None);
    assert_eq!(args.lightmap_backend, LightmapBackendPreference::Auto);
    assert!(args.lightmap_environment_map.is_none());
    assert_eq!(args.irradiance_spacing_meters, 8.0);
    assert_eq!(args.irradiance_samples, 64);
    assert_eq!(args.lightmap_min_samples, 8);
    assert_eq!(args.lightmap_max_samples, 8);
    assert_eq!(args.lightmap_variance_threshold, 0.0);
    assert_eq!(args.lightmap_bounces, 1);
    assert_eq!(args.lightmap_texels_per_meter, None);
    assert!(args.lightmap_density_overrides.is_empty());
    assert!(!args.lightmap_debug_uv);
    assert!(!args.lightmap_debug_samples);
    assert!(!args.lightmap_debug_variance);
    assert_eq!(args.lightmap_tile_size, None);
    assert_eq!(args.lightmap_denoise_iterations, 1);
    assert!(!args.lightmap_force_retrace);
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "bake",
            "--manifest",
            "scene.ron",
            "--lightmap-bounces",
            "8",
        ])
        .is_ok()
    );
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "bake",
            "--manifest",
            "scene.ron",
            "--lightmap-bounces",
            "9",
        ])
        .is_err()
    );
    let cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--lightmap-density",
        "000151e3=32",
    ])
    .unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert_eq!(args.lightmap_density_overrides.len(), 1);
    assert_eq!(
        args.lightmap_density_overrides[0].reference_form_id,
        0x0001_51e3
    );
    assert_eq!(args.lightmap_density_overrides[0].texels_per_meter, 32.0);
    let cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--lightmap-debug-uv",
        "--lightmap-debug-samples",
        "--lightmap-debug-variance",
    ])
    .unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert!(args.lightmap_debug_uv);
    assert!(args.lightmap_debug_samples);
    assert!(args.lightmap_debug_variance);
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "bake",
            "--manifest",
            "scene.ron",
            "--quality",
            "preview",
        ])
        .is_err()
    );
    assert!(
        Cli::try_parse_from([
            "bevyout",
            "bake",
            "--manifest",
            "scene.ron",
            "--blender",
            "blender.exe",
        ])
        .is_err()
    );

    for value in ["7.99", "256.01", "NaN", "inf"] {
        assert!(
            Cli::try_parse_from([
                "bevyout",
                "bake",
                "--manifest",
                "scene.ron",
                "--static-batch-chunk-meters",
                value,
            ])
            .is_err()
        );
    }

    for value in ["1.99", "32.01", "NaN", "inf"] {
        assert!(
            Cli::try_parse_from([
                "bevyout",
                "bake",
                "--manifest",
                "scene.ron",
                "--irradiance-spacing-meters",
                value,
            ])
            .is_err()
        );
    }

    for value in ["0", "513"] {
        assert!(
            Cli::try_parse_from([
                "bevyout",
                "bake",
                "--manifest",
                "scene.ron",
                "--irradiance-samples",
                value,
            ])
            .is_err()
        );
    }

    for value in ["15", "24", "1024"] {
        assert!(
            Cli::try_parse_from([
                "bevyout",
                "bake",
                "--manifest",
                "scene.ron",
                "--lightmap-tile-size",
                value,
            ])
            .is_err()
        );
    }
    let cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--lightmap-force-retrace",
    ])
    .unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert!(args.lightmap_force_retrace);
}

#[test]
fn progress_mode_parses_all_values_and_defaults_to_auto() {
    let cli = Cli::try_parse_from(["bevyout", "prepare", "SuperDuperMart"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.progress.mode, ProgressMode::Auto);

    for (value, expected) in [
        ("auto", ProgressMode::Auto),
        ("tty", ProgressMode::Tty),
        ("plain", ProgressMode::Plain),
        ("off", ProgressMode::Off),
    ] {
        let cli = Cli::try_parse_from([
            "bevyout",
            "bake",
            "--manifest",
            "scene.ron",
            "--progress",
            value,
        ])
        .unwrap();
        let CommandLine::Bake(args) = cli.command else {
            panic!("expected bake command");
        };
        assert_eq!(args.progress.mode, expected);
    }

    assert!(
        Cli::try_parse_from([
            "bevyout",
            "prepare",
            "SuperDuperMart",
            "--progress",
            "invalid",
        ])
        .is_err()
    );
}

#[test]
fn bake_backend_accepts_explicit_solari_prototype_request() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--bake-backend",
        "solari",
    ])
    .unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert_eq!(args.lightmap_backend, LightmapBackendPreference::Solari);
}

#[test]
fn bake_accepts_an_authored_hdr_environment_map() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--lightmap-environment-map",
        "lighting/interior.hdr",
    ])
    .unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert_eq!(
        args.lightmap_environment_map,
        Some(PathBuf::from("lighting/interior.hdr"))
    );
}

#[test]
fn accepts_editor_id_selectors_and_legacy_paths() {
    let cli = Cli::try_parse_from(["bevyout", "prepare", "SuperDuperMart"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.selectors, vec!["SuperDuperMart".to_string()]);

    let cli = Cli::try_parse_from(["bevyout", "bake", "SuperDuperMart"]).unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert_eq!(args.selector.as_deref(), Some("SuperDuperMart"));
    assert!(args.manifest.is_none());

    let cli = Cli::try_parse_from(["bevyout", "bake", "SuperDuperMart", "--force"]).unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert!(args.force);

    let cli = Cli::try_parse_from(["bevyout", "render", "SuperDuperMart"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(args.selector, "SuperDuperMart");
    assert!(!args.realtime_shadows);
    assert!(!args.worldspace_lod);

    let cli =
        Cli::try_parse_from(["bevyout", "render", "SuperDuperMart", "--realtime-shadows"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert!(args.realtime_shadows);

    let cli =
        Cli::try_parse_from(["bevyout", "render", "SuperDuperMart", "--worldspace-lod"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert!(args.worldspace_lod);

    let cli = Cli::try_parse_from(["bevyout", "view", "--manifest", "scene.ron"]).unwrap();
    let CommandLine::View(args) = cli.command else {
        panic!("expected view command");
    };
    assert!(!args.realtime_shadows);
    assert!(!args.worldspace_lod);

    let cli = Cli::try_parse_from([
        "bevyout",
        "view",
        "--manifest",
        "scene.ron",
        "--realtime-shadows",
    ])
    .unwrap();
    let CommandLine::View(args) = cli.command else {
        panic!("expected view command");
    };
    assert!(args.realtime_shadows);

    let cli = Cli::try_parse_from([
        "bevyout",
        "view",
        "--manifest",
        "scene.ron",
        "--worldspace-lod",
    ])
    .unwrap();
    let CommandLine::View(args) = cli.command else {
        panic!("expected view command");
    };
    assert!(args.worldspace_lod);

    let cli = Cli::try_parse_from(["bevyout", "prepare", "--cell", "00017f37"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.cell.as_deref(), Some("00017f37"));

    assert!(
        Cli::try_parse_from([
            "bevyout",
            "bake",
            "SuperDuperMart",
            "--manifest",
            "scene.ron",
        ])
        .is_err()
    );
}

#[test]
fn prepare_batch_selectors_parse_and_combine() {
    let cli =
        Cli::try_parse_from(["bevyout", "prepare", "--all-interiors", "--list-only"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert!(args.all_interiors);
    assert!(args.list_only);
    assert!(!args.all);

    let cli = Cli::try_parse_from([
        "bevyout",
        "prepare",
        "--worldspace",
        "WastelandNV",
        "--all-interiors",
        "SuperDuperMart",
        "MegatonExt",
    ])
    .unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.worldspace.as_deref(), Some("WastelandNV"));
    assert!(args.all_interiors);
    assert_eq!(
        args.selectors,
        vec!["SuperDuperMart".to_string(), "MegatonExt".to_string()]
    );

    let cli = Cli::try_parse_from(["bevyout", "prepare", "--all"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert!(args.all);
}

#[test]
fn prepare_all_conflicts_with_other_selection_flags() {
    for arguments in [
        vec!["bevyout", "prepare", "--all", "--all-interiors"],
        vec!["bevyout", "prepare", "--all", "--worldspace", "Wasteland"],
        vec!["bevyout", "prepare", "--all", "SuperDuperMart"],
        vec!["bevyout", "prepare", "--all", "--cell", "00017f37"],
    ] {
        assert!(
            Cli::try_parse_from(arguments.clone()).is_err(),
            "expected {arguments:?} to be rejected"
        );
    }
}

#[test]
fn cell_catalogue_contract_parses() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "cells",
        "--game-root",
        "game",
        "--plugin",
        "Patch.esp",
        "--interiors-only",
    ])
    .unwrap();
    let CommandLine::Cells(args) = cli.command else {
        panic!("expected cells command");
    };
    assert_eq!(
        args.game_root.as_deref(),
        Some(std::path::Path::new("game"))
    );
    assert_eq!(
        args.plugin.as_deref(),
        Some(std::path::Path::new("Patch.esp"))
    );
    assert!(args.interiors_only);
}

#[test]
fn script_run_contract_parses() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "script",
        "run",
        "tests/example.bscript",
        "--headless",
        "--transcript",
        "out.jsonl",
        "--keep-going",
    ])
    .unwrap();
    let CommandLine::Script(args) = cli.command else {
        panic!("expected script command");
    };
    let ScriptCommand::Run(args) = args.command;
    assert!(args.headless);
    assert!(args.keep_going);
    assert_eq!(
        args.transcript.as_deref(),
        Some(std::path::Path::new("out.jsonl"))
    );
}

#[test]
fn prepared_shadow_resolution_and_rebuild_contract_parse() {
    let cli = Cli::try_parse_from(["bevyout", "prepare", "MegatonPlayerHouse"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.shadow_resolution, 512);

    let cli = Cli::try_parse_from(["bevyout", "render", "MegatonPlayerHouse"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(args.shadow_resolution, 512);

    let cli = Cli::try_parse_from([
        "bevyout",
        "prepare",
        "MegatonPlayerHouse",
        "--shadow-resolution",
        "128",
        "--rebuild-shadows",
        "--toktx",
        "ktx.exe",
    ])
    .unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.shadow_resolution, 128);
    assert!(args.rebuild_shadows);
    assert_eq!(args.toktx.as_deref(), Some(std::path::Path::new("ktx.exe")));

    for invalid in ["64", "129", "1024"] {
        assert!(
            Cli::try_parse_from([
                "bevyout",
                "render",
                "MegatonPlayerHouse",
                "--shadow-resolution",
                invalid,
            ])
            .is_err()
        );
    }
}

#[test]
fn cache_stats_contract_parses() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "cache",
        "stats",
        "--cache",
        "prepared-cache",
        "--manifest-set",
        "reports/sample-cells.ron",
        "--json",
        "reports/cache.json",
        "--csv",
        "reports/cache.csv",
    ])
    .unwrap();
    let CommandLine::Cache(args) = cli.command else {
        panic!("expected cache command");
    };
    let CacheCommand::Stats(args) = args.command;
    assert_eq!(args.cache, PathBuf::from("prepared-cache"));
    assert_eq!(
        args.manifest_set.as_deref(),
        Some(Path::new("reports/sample-cells.ron"))
    );
    assert_eq!(args.json.as_deref(), Some(Path::new("reports/cache.json")));
    assert_eq!(args.csv.as_deref(), Some(Path::new("reports/cache.csv")));
}
