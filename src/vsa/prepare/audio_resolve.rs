//! Audio descriptor resolution.

use super::*;

pub(crate) fn resolve_audio_descriptor(
    parsed: &ParsedPlugin,
    form_id: u32,
) -> Option<AudioDescriptor> {
    fn visit(
        parsed: &ParsedPlugin,
        form_id: u32,
        visiting: &mut HashSet<u32>,
    ) -> Option<AudioDescriptor> {
        if !visiting.insert(form_id) {
            return None;
        }
        let result = if let Some(sound) = parsed.sounds.get(&form_id) {
            sound_descriptor(sound)
        } else if let Some(reference) = parsed.sound_references.get(&form_id) {
            sound_reference_descriptor(parsed, reference, visiting)
        } else {
            None
        };
        visiting.remove(&form_id);
        result
    }
    visit(parsed, form_id, &mut HashSet::new())
}

pub(crate) fn sound_descriptor(sound: &SoundRecord) -> Option<AudioDescriptor> {
    let params = sound.parameters.unwrap_or_default();
    Some(AudioDescriptor {
        source_path: sound.file.clone()?,
        editor_id: sound.editor_id.clone(),
        flags: params.flags,
        min_attenuation: params.min_attenuation,
        max_attenuation: params.max_attenuation,
        frequency_adjustment: params.frequency_adjustment,
        static_attenuation_hundredths_db: params.static_attenuation.unwrap_or_default(),
        looping: params.is_looping(),
        is_2d: params.is_two_dimensional(),
    })
}

pub(crate) fn sound_reference_descriptor(
    parsed: &ParsedPlugin,
    reference: &SoundReferenceRecord,
    visiting: &mut HashSet<u32>,
) -> Option<AudioDescriptor> {
    if let Some(source_path) = reference.file.clone() {
        let sound_info = reference
            .sound_info
            .unwrap_or(crate::vsa::openmw_esm4::SoundInfo {
                frequency_adjustment: 0,
                frequency_variance: 0,
                priority: 0,
                decibel_variance: 0,
                static_attenuation: 0,
            });
        return Some(AudioDescriptor {
            source_path,
            editor_id: reference.editor_id.clone(),
            flags: reference.loop_info.map_or(0, |loop_info| loop_info.flags),
            min_attenuation: 0,
            max_attenuation: 0,
            frequency_adjustment: sound_info.frequency_adjustment,
            static_attenuation_hundredths_db: sound_info.static_attenuation,
            looping: reference
                .loop_info
                .is_some_and(|loop_info| loop_info.flags & 1 != 0),
            is_2d: false,
        });
    }
    reference.sound_reference_form_id.and_then(|form_id| {
        if !visiting.insert(form_id) {
            return None;
        }
        let resolved = parsed
            .sounds
            .get(&form_id)
            .and_then(sound_descriptor)
            .or_else(|| {
                parsed
                    .sound_references
                    .get(&form_id)
                    .and_then(|reference| sound_reference_descriptor(parsed, reference, visiting))
            });
        visiting.remove(&form_id);
        resolved
    })
}
