//! Prepare-time fallback from authored NIF animation sound cues.

use super::*;

fn resolve_editor_id(
    parsed: &ParsedPlugin,
    editor_id: &str,
    asset_path: &str,
    sequence: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<u32> {
    let candidates = sound_form_ids_by_editor_id(parsed, editor_id);
    let resolved = candidates
        .iter()
        .copied()
        .find(|form_id| resolve_audio_descriptor(parsed, *form_id).is_some());
    if candidates.len() > 1 {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!(
                "container animation audio {asset_path} {sequence} cue {editor_id} matched {} sound records; using {}",
                candidates.len(),
                resolved.map_or_else(|| "none".into(), |form_id| format!("{form_id:08x}")),
            ),
        });
    }
    if resolved.is_none() {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!(
                "container animation audio {asset_path} {sequence} cue {editor_id} could not be resolved"
            ),
        });
    }
    resolved
}

pub(crate) fn apply_container_animation_audio(
    cache_dir: &Path,
    parsed: &ParsedPlugin,
    placements: &mut [PreparedPlacement],
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<u32> {
    let mut needs_by_asset = HashMap::<String, (bool, bool)>::new();
    let mut selected_by_asset = HashMap::new();
    let mut additional_form_ids = HashSet::new();

    for placement in placements.iter() {
        if !matches!(placement.semantic, PreparedSemantic::Container) {
            continue;
        }
        let Some(asset_path) = placement.asset_path.as_deref() else {
            continue;
        };
        let needs = needs_by_asset.entry(asset_path.to_owned()).or_default();
        needs.0 |= placement.audio.open_sound_form_id.is_none();
        needs.1 |= placement.audio.close_sound_form_id.is_none();
    }

    for (asset_path, &(needs_open, needs_close)) in &needs_by_asset {
        if !needs_open && !needs_close {
            continue;
        }
        let glb_path = cache_dir.join(asset_path);
        match read_glb_animation_sound_cues(&glb_path) {
            Ok(cues) => {
                let selected = select_container_audio(&cues);
                if needs_open && selected.open_candidates > 1 {
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "container animation audio {asset_path} has {} Open sound cues; using the earliest deterministic cue",
                            selected.open_candidates
                        ),
                    });
                }
                if needs_close && selected.close_candidates > 1 {
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "container animation audio {asset_path} has {} Close sound cues; using the earliest deterministic cue",
                            selected.close_candidates
                        ),
                    });
                }
                selected_by_asset.insert(asset_path.to_owned(), Some(selected));
            }
            Err(error) => {
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "container animation audio metadata could not be read from {asset_path}: {error}"
                    ),
                });
                selected_by_asset.insert(asset_path.to_owned(), None);
            }
        }
    }

    let mut resolved_by_asset = HashMap::new();
    for (asset_path, selected) in &selected_by_asset {
        let Some(selected) = selected else {
            continue;
        };
        let (needs_open, needs_close) = needs_by_asset[asset_path];
        let open = needs_open
            .then_some(selected.open_editor_id.as_deref())
            .flatten()
            .and_then(|editor_id| {
                resolve_editor_id(parsed, editor_id, asset_path, "Open", diagnostics)
            });
        let close = needs_close
            .then_some(selected.close_editor_id.as_deref())
            .flatten()
            .and_then(|editor_id| {
                resolve_editor_id(parsed, editor_id, asset_path, "Close", diagnostics)
            });
        resolved_by_asset.insert(asset_path.clone(), (open, close));
    }

    for placement in placements {
        let Some(asset_path) = placement.asset_path.as_deref() else {
            continue;
        };
        let Some(&(cue_open, cue_close)) = resolved_by_asset.get(asset_path) else {
            continue;
        };
        let record_open = placement.audio.open_sound_form_id;
        let record_close = placement.audio.close_sound_form_id;
        let (open, close) = apply_container_audio_fallback(
            matches!(placement.semantic, PreparedSemantic::Container),
            record_open,
            record_close,
            cue_open,
            cue_close,
        );
        placement.audio.open_sound_form_id = open;
        placement.audio.close_sound_form_id = close;
        if record_open.is_none()
            && let Some(form_id) = open
        {
            additional_form_ids.insert(form_id);
        }
        if record_close.is_none()
            && let Some(form_id) = close
        {
            additional_form_ids.insert(form_id);
        }
    }

    let mut additional_form_ids = additional_form_ids.into_iter().collect::<Vec<_>>();
    additional_form_ids.sort_unstable();
    additional_form_ids
}
