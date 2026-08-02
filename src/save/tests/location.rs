use super::*;

use bevyout_core::manifest::exterior::{
    WorldLocation, WorldLocationExterior, WorldLocationInterior,
};

fn save_with_location(location: Option<WorldLocation>) -> SaveGame {
    let mut save = SaveGame::default();
    save.header.current_cell = 0x0001_51e3;
    save.location = location;
    save
}

fn append_record_and_checksum(mut bytes: Vec<u8>, record_tag: [u8; 4], payload: &[u8]) -> Vec<u8> {
    let checksum_start = bytes.len() - (8 + 32);
    bytes.truncate(checksum_start);
    write_record(&mut bytes, record_tag, payload).unwrap();
    let checksum: [u8; 32] = Sha256::digest(&bytes).into();
    write_record(&mut bytes, tag("CHKS"), &checksum).unwrap();
    bytes
}

#[test]
fn v7_round_trip_preserves_both_exact_location_variants() {
    let locations = [
        WorldLocation::Exterior(WorldLocationExterior {
            worldspace_form_id: 0x0001_51e3,
            position: [12.345678, 3.500001, -8.125004],
            rotation_xyzw: [0.1234567, -0.7070001, 0.2222222, 0.6543211],
        }),
        WorldLocation::Interior(WorldLocationInterior {
            cell_form_id: 0x0002_0001,
            position: [-4.125003, 7.750002, 0.000007],
            rotation_xyzw: [-0.3333333, 0.4444444, -0.5555555, 0.6666666],
        }),
    ];

    for location in locations {
        let save = save_with_location(Some(location.clone()));
        let encoded = encode_save(&save).unwrap();
        let decoded = decode_save(&encoded).unwrap();

        assert_eq!(decoded.location, Some(location));
        assert_eq!(decoded.legacy_location_fallback_cell(), None);
        assert_eq!(encode_save(&decoded).unwrap(), encoded);
    }
}

#[test]
fn malformed_wloc_is_rejected() {
    let encoded = encode_save(&save_with_location(None)).unwrap();
    let bytes = append_record_and_checksum(encoded, tag("WLOC"), b"not a WorldLocation");

    let error = decode_save(&bytes).unwrap_err().to_string();
    assert!(error.contains("decoding player world location"), "{error}");
}

#[test]
fn duplicate_wloc_is_rejected() {
    let location = WorldLocation::Interior(WorldLocationInterior {
        cell_form_id: 0x0002_0001,
        position: [1.0, 2.0, 3.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
    });
    let save = save_with_location(Some(location.clone()));
    let encoded = encode_save(&save).unwrap();
    let payload = ron::ser::to_string(&location).unwrap();
    let bytes = append_record_and_checksum(encoded, tag("WLOC"), payload.as_bytes());

    let error = decode_save(&bytes).unwrap_err().to_string();
    assert!(error.contains("duplicate WLOC"), "{error}");
}

#[test]
fn legacy_save_has_an_identity_only_fallback_without_an_exact_transform() {
    let mut save = save_with_location(None);
    save.header.format_version = 6;

    let encoded = encode_save(&save).unwrap();
    let decoded = decode_save(&encoded).unwrap();

    assert!(decoded.location.is_none());
    assert_eq!(decoded.legacy_location_fallback_cell(), Some(0x0001_51e3));
    assert_eq!(encode_save(&decoded).unwrap(), encoded);
}

#[test]
fn invalid_wloc_identity_or_transform_is_rejected() {
    let mut save = save_with_location(Some(WorldLocation::Exterior(WorldLocationExterior {
        worldspace_form_id: 0,
        position: [f32::NAN, 0.0, 0.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 0.0],
    })));

    let error = encode_save(&save).unwrap_err().to_string();
    assert!(error.contains("invalid player world location"), "{error}");

    save.location = Some(WorldLocation::Exterior(WorldLocationExterior {
        worldspace_form_id: 0x0001_51e3,
        position: [1.0, 2.0, 3.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 0.0],
    }));
    assert!(encode_save(&save).is_err());
}
