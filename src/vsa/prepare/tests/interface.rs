use super::*;

#[test]
fn pipboy_sprite_allowlist_uses_direct_interface_dds_paths() {
    assert!(PIPBOY_SPRITES.iter().all(|path| {
        path.starts_with("interface/") && path.ends_with(".dds") && !path.contains("..")
    }));
    assert!(PIPBOY_SPRITES.contains(&"interface/shared/background/pipboy.dds"));
    assert!(PIPBOY_SPRITES.contains(&"interface/stats/head.dds"));
}

#[test]
fn hud_sprite_allowlist_uses_direct_interface_dds_paths() {
    assert!(HUD_SPRITES.iter().all(|path| {
        path.starts_with("interface/hud/") && path.ends_with(".dds") && !path.contains("..")
    }));
    assert!(HUD_SPRITES.contains(&"interface/hud/crosshair.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/hud_comp_direction_strip.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/hud_tick_mark.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/hud_left_seperator.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/hud_right_seperator.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/glow_hud_compass_objective_marker.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/hitgradientleft.dds"));
    assert!(HUD_SPRITES.contains(&"interface/hud/hitgradientright.dds"));
}
