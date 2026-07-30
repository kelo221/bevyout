//! Fallout interface sprite discovery and staging.

use super::*;

/// Direct, non-atlas sprites used by the current Pip-Boy status surface.
const PIPBOY_SPRITES: [&str; 8] = [
    "interface/shared/background/pipboy.dds",
    "interface/stats/face_00.dds",
    "interface/stats/head.dds",
    "interface/stats/left_arm.dds",
    "interface/stats/left_leg.dds",
    "interface/stats/right_arm.dds",
    "interface/stats/right_leg.dds",
    "interface/stats/torso.dds",
];

/// Direct, non-atlas sprites used by the first-person HUD
/// (`viewer::hud`). The game's HUD textures are white shapes baked into
/// the alpha channel and tinted with the HUD colour at draw time; the
/// viewer reproduces that with `ImageNode::color` over the same sprites.
const HUD_SPRITES: [&str; 12] = [
    // Crosshair core plus its soft phosphor glow.
    "interface/hud/crosshair.dds",
    "interface/hud/glow_crosshair.dds",
    // Compass: scrolling N/E/S/W direction strip, tick marks, and the
    // landmark / objective markers that ride on it.
    "interface/hud/hud_comp_direction_strip.dds",
    "interface/hud/hud_tick_mark.dds",
    "interface/hud/glow_hud_tick_mark.dds",
    "interface/hud/hud_compass_mark.dds",
    "interface/hud/glow_hud_compass_objective_marker.dds",
    // Bracket caps framing the AP (left) and HP (right) meters.
    "interface/hud/hud_left_seperator.dds",
    "interface/hud/hud_right_seperator.dds",
    // Thin rule under the ammo counter.
    "interface/hud/hud_bottom_info_seperator.dds",
    // Full-screen red edge gradients flashed on incoming damage.
    "interface/hud/hitgradientleft.dds",
    "interface/hud/hitgradientright.dds",
];

fn stage_sprites(
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    sprites: &[&str],
) -> Result<()> {
    for source_path in sprites {
        let Some(bytes) = resolve_asset(data_root, archives, source_path)
            .with_context(|| format!("reading interface sprite {source_path}"))?
        else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("missing interface sprite {source_path}"),
            });
            continue;
        };
        let destination = staging_dir.join(source_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        if !destination.is_file() {
            fs::write(destination, bytes)?;
        }
    }
    Ok(())
}

pub(crate) fn stage_pipboy_sprites(
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    stage_sprites(
        data_root,
        archives,
        staging_dir,
        diagnostics,
        &PIPBOY_SPRITES,
    )
}

pub(crate) fn stage_hud_sprites(
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    stage_sprites(data_root, archives, staging_dir, diagnostics, &HUD_SPRITES)
}

#[cfg(test)]
#[path = "tests/interface.rs"]
mod tests;
