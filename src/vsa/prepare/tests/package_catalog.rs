use super::*;

fn package(form_id: u32) -> PackageInput {
    PackageInput {
        form_id,
        ..PackageInput::default()
    }
}

#[test]
fn revision_is_pinned() {
    assert_eq!(PACKAGE_CATALOG_REVISION, "openmw-packages-v2");
}

#[test]
fn built_catalogs_carry_the_pinned_revision() {
    let catalog = build_package_catalog(&PackageCatalogInputs::default(), "fp");
    assert_eq!(catalog.revision, PACKAGE_CATALOG_REVISION);
    assert_eq!(catalog.source_fingerprint, "fp");
}

#[test]
fn catalog_is_sorted_by_form_id() {
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x30, package(0x30)), (0x10, package(0x10))]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(
        catalog
            .packages
            .iter()
            .map(|p| p.form_id)
            .collect::<Vec<_>>(),
        vec![0x10, 0x30]
    );
    assert_eq!(catalog.counters.total, 2);
}

#[test]
fn unsupported_package_type_is_diagnosed() {
    let mut input = package(0x10);
    input.package_type = 200;
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unsupported_type, 1);
    assert!(
        catalog.packages[0]
            .diagnostics
            .iter()
            .any(|message| message.contains("unsupported package type 200"))
    );
}

#[test]
fn known_package_types_are_not_flagged() {
    for package_type in 0..=MAX_KNOWN_PACKAGE_TYPE {
        let mut input = package(0x10);
        input.package_type = package_type;
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unsupported_type, 0, "type {package_type}");
    }
}

#[test]
fn unsupported_subrecord_is_diagnosed() {
    let mut input = package(0x10);
    input.unsupported_subrecords = vec!["XNAM".into()];
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unsupported_subrecord, 1);
    assert_eq!(catalog.counters.deferred_subrecord, 0);
    assert!(
        catalog.packages[0]
            .diagnostics
            .iter()
            .any(|message| message.contains("unsupported subrecord(s): XNAM"))
    );
}

/// M4 wave 11 follow-up regression: a known FO3 script/idle/topic
/// action-block subrecord (fopdoc-documented, just not decoded into
/// typed fields yet) must not produce a per-package diagnostic -- it is
/// deferred to #115/#15, not unsupported. Real-data acceptance found
/// this was 100% noise before the fix (3021/3021 packages on cell
/// 0001a273).
#[test]
fn a_known_deferred_subrecord_produces_no_diagnostic() {
    for subrecord in KNOWN_DEFERRED_PACKAGE_SUBRECORDS {
        let mut input = package(0x10);
        input.unsupported_subrecords = vec![(*subrecord).to_string()];
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unsupported_subrecord, 0, "{subrecord}");
        assert_eq!(catalog.counters.deferred_subrecord, 1, "{subrecord}");
        assert!(catalog.packages[0].diagnostics.is_empty(), "{subrecord}");
    }
}

/// A package mixing a known-deferred signature with a genuinely unknown
/// one still gets a real diagnostic -- deferred and unsupported are
/// counted (and reported) independently per package.
#[test]
fn mixed_deferred_and_unsupported_subrecords_are_split() {
    let mut input = package(0x10);
    input.unsupported_subrecords = vec!["SCHR".into(), "XNAM".into()];
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.deferred_subrecord, 1);
    assert_eq!(catalog.counters.unsupported_subrecord, 1);
    assert!(
        catalog.packages[0]
            .diagnostics
            .iter()
            .any(|message| message == "package 00000010 has unsupported subrecord(s): XNAM")
    );
}

#[test]
fn unresolved_location_formid_is_diagnosed() {
    let mut input = package(0x10);
    input.location = Some(PackageLocationInput {
        location_type: 4, // Object ID -- the only checkable location type
        form_id: Some(0xDEAD),
        raw_value: 0xDEAD,
        radius: 0,
    });
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved_location, 1);
    assert_eq!(catalog.counters.out_of_scope_location, 0);
    assert!(
        catalog.packages[0]
            .diagnostics
            .iter()
            .any(|message| message.contains("location references unresolved FormID 0000dead"))
    );
}

/// M4 wave 11 follow-up regression: "Near Reference" (0) and "In Cell"
/// (1) location types carry a real FormID that this single-cell
/// prepare pass's decoded scope can never fully cover -- they must be
/// counted separately from genuinely unresolved links, not flagged.
/// Real-data measurement: 2216 + 140 = 2356/2356 of the old
/// "unresolved location" count on cell 0001a273 was exactly this case.
#[test]
fn out_of_scope_location_types_are_not_flagged_unresolved() {
    for location_type in [0u32, 1] {
        let mut input = package(0x10);
        input.location = Some(PackageLocationInput {
            location_type,
            form_id: Some(0xDEAD),
            raw_value: 0xDEAD,
            radius: 0,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(
            catalog.counters.unresolved_location, 0,
            "type {location_type}"
        );
        assert_eq!(
            catalog.counters.out_of_scope_location, 1,
            "type {location_type}"
        );
        assert!(
            catalog.packages[0].diagnostics.is_empty(),
            "type {location_type}"
        );
    }
}

/// A location type that never carries a FormID at all ("Near Current
/// Location" 2, "Near Editor Location" 3, "Near Linked Reference" 6, "At
/// Package Location" 7) is neither `OUT_OF_SCOPE_LOCATION_TYPES` nor
/// `CHECKABLE_LOCATION_TYPES` -- defensively inert even if a `Some`
/// form_id somehow reached it (the real decoder never produces one).
#[test]
fn no_reference_location_types_are_never_counted() {
    for location_type in [2u32, 3, 6, 7] {
        let mut input = package(0x10);
        input.location = Some(PackageLocationInput {
            location_type,
            form_id: Some(0xDEAD),
            raw_value: 0xDEAD,
            radius: 0,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(
            catalog.counters.unresolved_location, 0,
            "type {location_type}"
        );
        assert_eq!(
            catalog.counters.out_of_scope_location, 0,
            "type {location_type}"
        );
        assert!(
            catalog.packages[0].diagnostics.is_empty(),
            "type {location_type}"
        );
    }
}

#[test]
fn unresolved_target_formid_is_diagnosed() {
    let mut input = package(0x10);
    input.target = Some(PackageTargetInput {
        target_type: 1, // Object ID -- the only checkable target type
        form_id: Some(0xBEEF),
        raw_value: 0xBEEF,
        count_or_distance: 1,
    });
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved_target, 1);
    assert_eq!(catalog.counters.out_of_scope_target, 0);
    assert!(
        catalog.packages[0]
            .diagnostics
            .iter()
            .any(|message| message.contains("target references unresolved FormID 0000beef"))
    );
}

/// M4 wave 11 follow-up regression: "Specific Reference" (0) target
/// types carry a real FormID out of this prepare pass's verifiable
/// scope. Real-data measurement: 715/718 of the old "unresolved target"
/// count on cell 0001a273 was exactly this case (the remaining 3 were
/// genuine Object-ID misses, still caught by
/// `unresolved_target_formid_is_diagnosed`).
#[test]
fn out_of_scope_target_type_is_not_flagged_unresolved() {
    let mut input = package(0x10);
    input.target = Some(PackageTargetInput {
        target_type: 0,
        form_id: Some(0xBEEF),
        raw_value: 0xBEEF,
        count_or_distance: 1,
    });
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved_target, 0);
    assert_eq!(catalog.counters.out_of_scope_target, 1);
    assert!(catalog.packages[0].diagnostics.is_empty());
}

/// A target type that never carries a FormID at all ("Object Type" 2,
/// "Linked Reference" 3) is neither `OUT_OF_SCOPE_TARGET_TYPES` nor
/// `CHECKABLE_TARGET_TYPES` -- defensively inert.
#[test]
fn no_reference_target_types_are_never_counted() {
    for target_type in [2i32, 3] {
        let mut input = package(0x10);
        input.target = Some(PackageTargetInput {
            target_type,
            form_id: Some(0xBEEF),
            raw_value: 0xBEEF,
            count_or_distance: 1,
        });
        let inputs = PackageCatalogInputs {
            packages: HashMap::from([(0x10, input)]),
            ..PackageCatalogInputs::default()
        };
        let catalog = build_package_catalog(&inputs, "fp");
        assert_eq!(catalog.counters.unresolved_target, 0, "type {target_type}");
        assert_eq!(
            catalog.counters.out_of_scope_target, 0,
            "type {target_type}"
        );
        assert!(
            catalog.packages[0].diagnostics.is_empty(),
            "type {target_type}"
        );
    }
}

#[test]
fn resolved_location_and_target_are_not_flagged() {
    let mut input = package(0x10);
    input.location = Some(PackageLocationInput {
        location_type: 4,
        form_id: Some(0x20),
        raw_value: 0x20,
        radius: 0,
    });
    input.target = Some(PackageTargetInput {
        target_type: 1,
        form_id: Some(0x20),
        raw_value: 0x20,
        count_or_distance: 1,
    });
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        known_form_ids: HashSet::from([0x20]),
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved_location, 0);
    assert_eq!(catalog.counters.unresolved_target, 0);
    assert_eq!(catalog.counters.out_of_scope_location, 0);
    assert_eq!(catalog.counters.out_of_scope_target, 0);
    assert!(catalog.packages[0].diagnostics.is_empty());
}

/// The "Object Type" sentinel (`location_type == 5` / `target_type ==
/// 2`) always decodes to `form_id: None` upstream in
/// `actor_support::parse_package` -- proves the catalog never invents an
/// unresolved-link diagnostic for a `None` form_id regardless of cause.
#[test]
fn a_none_form_id_is_never_flagged_as_unresolved() {
    let mut input = package(0x10);
    input.location = Some(PackageLocationInput {
        location_type: 5,
        form_id: None,
        raw_value: 3,
        radius: 0,
    });
    input.target = Some(PackageTargetInput {
        target_type: 2,
        form_id: None,
        raw_value: 3,
        count_or_distance: 1,
    });
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, input)]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved_location, 0);
    assert_eq!(catalog.counters.unresolved_target, 0);
    assert_eq!(catalog.counters.out_of_scope_location, 0);
    assert_eq!(catalog.counters.out_of_scope_target, 0);
}

#[test]
fn serialization_is_deterministic_across_runs() {
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x30, package(0x30)), (0x10, package(0x10))]),
        ..PackageCatalogInputs::default()
    };
    let a = build_package_catalog(&inputs, "fp");
    let b = build_package_catalog(&inputs, "fp");
    let ron_a = ron::ser::to_string_pretty(&a, ron::ser::PrettyConfig::default()).unwrap();
    let ron_b = ron::ser::to_string_pretty(&b, ron::ser::PrettyConfig::default()).unwrap();
    assert_eq!(ron_a, ron_b);
}

#[test]
fn writes_content_set_wide_artifact_under_catalogs() {
    let cache_dir = std::env::temp_dir().join(format!(
        "bevyout-package-catalog-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
    ));
    let inputs = PackageCatalogInputs {
        packages: HashMap::from([(0x10, package(0x10))]),
        ..PackageCatalogInputs::default()
    };
    let catalog = build_package_catalog(&inputs, "shared-fp");
    let (relative, hash) = write_package_catalog(&cache_dir, &catalog).unwrap();
    assert_eq!(relative, "catalogs/shared-fp/packages.ron");
    let bytes = std::fs::read(cache_dir.join(&relative)).unwrap();
    assert_eq!(fingerprint(&bytes), hash);
    std::fs::remove_dir_all(&cache_dir).unwrap();
}
