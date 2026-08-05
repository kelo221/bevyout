//! Pure Fallout flat-overlay classification shared by preparation, baking,
//! prepared shadows, reflection captures, and the viewer.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FalloutOverlayKind {
    #[default]
    None,
    Decal,
    Debris,
}

pub(crate) fn classify_fallout_overlay(
    editor_id: Option<&str>,
    model: Option<&str>,
) -> FalloutOverlayKind {
    let editor_id = editor_id.unwrap_or_default().to_ascii_lowercase();
    let model = model
        .unwrap_or_default()
        .replace('\\', "/")
        .to_ascii_lowercase();
    let filename = model.rsplit('/').next().unwrap_or(model.as_str());

    // Reviewed FO3 presentation surfaces. Keep this deliberately narrower
    // than alpha blending/masking: fences, foliage, grates, and other alpha
    // geometry remain ordinary static lighting participants.
    if editor_id.starts_with("stain")
        || editor_id.contains("graffiti")
        || filename.starts_with("stain")
        || filename.contains("graffiti")
    {
        return FalloutOverlayKind::Decal;
    }
    if editor_id.starts_with("assortedpapers")
        || editor_id.starts_with("shackpaperdebris")
        || filename.starts_with("assortedpapers")
        || filename.starts_with("shackpaperdebris")
    {
        return FalloutOverlayKind::Debris;
    }

    FalloutOverlayKind::None
}
