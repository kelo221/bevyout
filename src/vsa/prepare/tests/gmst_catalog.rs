use super::*;

fn setting(name: &str, value: GmstValue) -> (String, GmstValue) {
    (name.to_string(), value)
}

#[test]
fn revision_is_pinned() {
    assert_eq!(GMST_CATALOG_REVISION, "openmw-gmst-v1");
}

#[test]
fn builds_settings_from_pairs_with_defaults_and_counters() {
    let inputs = GmstCatalogInputs {
        settings_pairs: vec![
            setting("fAVDHealthEnduranceMult", GmstValue::Float(25.0)),
            setting("iMaxPlayerLevel", GmstValue::Int(20)),
            setting("fUnknownSetting", GmstValue::Float(1.5)),
        ],
        actor_values: vec![PreparedActorValueInfo {
            form_id: 0x40,
            editor_id: "Lockpick".into(),
            name: Some("Lockpick".into()),
            description: Some("Picking locks.".into()),
        }],
        undecoded: 2,
    };
    let catalog = build_gmst_catalog(&inputs, "fp");
    assert_eq!(catalog.revision, GMST_CATALOG_REVISION);
    assert_eq!(catalog.source_fingerprint, "fp");
    // Overridden settings apply; everything else keeps GOTY defaults.
    assert_eq!(catalog.settings.health_endurance_mult, 25.0);
    assert_eq!(catalog.settings.max_player_level, 20);
    assert_eq!(catalog.settings.health_base, 100.0);
    assert_eq!(catalog.settings.xp_base, 200);
    // Unknown settings are kept in the input but not consumed.
    assert_eq!(catalog.counters.total, 5);
    assert_eq!(catalog.counters.consumed, 2);
    assert_eq!(catalog.counters.undecoded, 2);
    assert_eq!(catalog.counters.actor_values, 1);
    assert_eq!(catalog.actor_values.len(), 1);
}

#[test]
fn empty_inputs_keep_full_defaults() {
    let catalog = build_gmst_catalog(&GmstCatalogInputs::default(), "fp");
    assert_eq!(catalog.settings, GmstSettings::default());
    assert_eq!(catalog.counters.total, 0);
}

#[test]
fn catalog_round_trips_through_ron() {
    let inputs = GmstCatalogInputs {
        settings_pairs: vec![
            setting("fAVDActionPointsBase", GmstValue::Int(70)),
            setting("bSomeFlag", GmstValue::Bool(true)),
            setting("sSomeText", GmstValue::Str("hello".into())),
        ],
        actor_values: vec![PreparedActorValueInfo {
            form_id: 0x41,
            editor_id: "Science".into(),
            name: None,
            description: None,
        }],
        undecoded: 0,
    };
    let catalog = build_gmst_catalog(&inputs, "fp");
    let text = ron::ser::to_string_pretty(&catalog, ron::ser::PrettyConfig::default())
        .expect("serialize gmst catalog");
    let back: PreparedGmstCatalog = ron::from_str(&text).expect("deserialize gmst catalog");
    assert_eq!(back, catalog);
}

#[test]
fn relative_path_is_deterministic_from_fingerprint() {
    let path = PreparedGmstCatalog::relative_path("abc123");
    assert_eq!(
        path.to_string_lossy().replace('\\', "/"),
        "catalogs/abc123/gmst.ron"
    );
}
