//! Pure policy for authored container animation sounds.

pub(crate) use super::super::assets::AnimationSoundCue;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SelectedContainerAudio {
    pub(crate) open_editor_id: Option<String>,
    pub(crate) close_editor_id: Option<String>,
    pub(crate) open_candidates: usize,
    pub(crate) close_candidates: usize,
}

pub(crate) fn select_container_audio(cues: &[AnimationSoundCue]) -> SelectedContainerAudio {
    fn select(cues: &[AnimationSoundCue], sequence: &str) -> (Option<String>, usize) {
        let mut candidates = cues
            .iter()
            .filter(|cue| {
                cue.sequence.eq_ignore_ascii_case(sequence)
                    && cue.time.is_finite()
                    && !cue.editor_id.trim().is_empty()
            })
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| {
            left.time
                .total_cmp(&right.time)
                .then_with(|| {
                    left.editor_id
                        .to_ascii_lowercase()
                        .cmp(&right.editor_id.to_ascii_lowercase())
                })
                .then_with(|| left.editor_id.cmp(&right.editor_id))
        });
        (
            candidates
                .first()
                .map(|cue| cue.editor_id.trim().to_owned()),
            candidates.len(),
        )
    }

    let (open_editor_id, open_candidates) = select(cues, "Open");
    let (close_editor_id, close_candidates) = select(cues, "Close");
    SelectedContainerAudio {
        open_editor_id,
        close_editor_id,
        open_candidates,
        close_candidates,
    }
}

pub(crate) fn apply_container_audio_fallback(
    is_container: bool,
    record_open: Option<u32>,
    record_close: Option<u32>,
    cue_open: Option<u32>,
    cue_close: Option<u32>,
) -> (Option<u32>, Option<u32>) {
    if !is_container {
        return (record_open, record_close);
    }
    (record_open.or(cue_open), record_close.or(cue_close))
}
