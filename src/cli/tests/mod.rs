use super::*;

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
    assert_eq!(args.selector.as_deref(), Some("SuperDuperMart"));

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
