use super::*;

#[test]
fn glow_card_names_match_case_insensitively() {
    assert!(is_glow_card_mesh_name("lightglow01"));
    assert!(is_glow_card_mesh_name("LightGlow01:0.001"));
    assert!(is_glow_card_mesh_name("LIGHTGLOW2"));
    assert!(is_glow_card_mesh_name("LightGlow"));
}

#[test]
fn ordinary_mesh_names_do_not_match() {
    assert!(!is_glow_card_mesh_name("ShackHangingLight02:51"));
    assert!(!is_glow_card_mesh_name("GlowLight01"));
    assert!(!is_glow_card_mesh_name(""));
}

#[test]
fn short_or_multibyte_prefixes_do_not_match_or_panic() {
    assert!(!is_glow_card_mesh_name("lightglo"));
    assert!(!is_glow_card_mesh_name("lightgl"));
    // A name that straddles the 9-byte prefix window mid-character must not
    // panic on `str` slicing (`get` returns `None` on a non-UTF8 boundary).
    assert!(!is_glow_card_mesh_name("lightgl\u{e9}ow"));
}
