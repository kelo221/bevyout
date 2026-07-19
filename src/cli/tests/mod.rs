use super::*;
use std::path::Path;

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
fn native_converter_is_default_and_blender_remains_explicit() {
    let cli = Cli::try_parse_from(["bevyout", "prepare", "SuperDuperMart"]).unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.converter, PrepareConverter::Native);

    let cli = Cli::try_parse_from([
        "bevyout",
        "prepare",
        "SuperDuperMart",
        "--converter",
        "blender",
        "--jobs",
        "8",
    ])
    .unwrap();
    let CommandLine::Prepare(args) = cli.command else {
        panic!("expected prepare command");
    };
    assert_eq!(args.converter, PrepareConverter::Blender);
    assert_eq!(args.jobs, Some(8));

    let cli = Cli::try_parse_from(["bevyout", "render", "SuperDuperMart"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(args.converter, PrepareConverter::Native);

    let cli = Cli::try_parse_from([
        "bevyout",
        "render",
        "SuperDuperMart",
        "--converter",
        "blender",
    ])
    .unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert_eq!(args.converter, PrepareConverter::Blender);

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
}

#[test]
fn static_batch_chunk_size_defaults_to_64_metres_and_enforces_bounds() {
    let cli = Cli::try_parse_from([
        "bevyout",
        "bake",
        "--manifest",
        "scene.ron",
        "--quality",
        "irradiance",
    ])
    .unwrap();
    let CommandLine::Bake(args) = cli.command else {
        panic!("expected bake command");
    };
    assert_eq!(args.static_batch_chunk_meters, 64.0);
    assert_eq!(args.irradiance_spacing_meters, 8.0);
    assert_eq!(args.irradiance_samples, 64);
    assert!(matches!(args.quality, BakeQuality::Irradiance));

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
    assert!(matches!(args.quality, BakeQuality::Irradiance));

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

    let cli =
        Cli::try_parse_from(["bevyout", "render", "SuperDuperMart", "--realtime-shadows"]).unwrap();
    let CommandLine::Render(args) = cli.command else {
        panic!("expected render command");
    };
    assert!(args.realtime_shadows);

    let cli = Cli::try_parse_from(["bevyout", "view", "--manifest", "scene.ron"]).unwrap();
    let CommandLine::View(args) = cli.command else {
        panic!("expected view command");
    };
    assert!(!args.realtime_shadows);

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
