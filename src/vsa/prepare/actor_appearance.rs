//! Pure spawn-time NPC outfit selection.
//!
//! Fallout actor visuals are prepared once. Runtime inventory/equipment may
//! later grow its own presentation adapter, but preparation needs a stable,
//! testable answer for which mutually-compatible ARMO records are initially
//! worn and which race body parts remain as underwear/skin fallbacks.

#[cfg(test)]
pub(crate) const FO3_SLOT_UPPER_BODY: u32 = 0x0000_0004;
#[cfg(test)]
pub(crate) const FO3_SLOT_LEFT_HAND: u32 = 0x0000_0008;
#[cfg(test)]
pub(crate) const FO3_SLOT_RIGHT_HAND: u32 = 0x0000_0010;
#[cfg(test)]
#[allow(dead_code)]
pub(crate) const PF_EDITOR_VISIBLE: u16 = 0x0001;

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ApparelCandidate {
    pub(crate) form_id: u32,
    pub(crate) male_worn: Option<String>,
    pub(crate) female_worn: Option<String>,
    pub(crate) male_world: Option<String>,
    pub(crate) female_world: Option<String>,
    pub(crate) biped_slot_mask: u32,
    pub(crate) base_armor_rating: f32,
    pub(crate) max_condition: Option<u32>,
    pub(crate) current_condition: Option<u32>,
    pub(crate) value: i32,
}

impl ApparelCandidate {
    pub(crate) fn worn_model(&self, female: bool) -> Option<&str> {
        if female {
            self.female_worn.as_deref()
        } else {
            self.male_worn.as_deref()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SelectedApparel {
    pub(crate) form_id: u32,
    pub(crate) model_path: String,
    pub(crate) biped_slot_mask: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SpawnOutfit {
    pub(crate) worn: Vec<SelectedApparel>,
    pub(crate) occupied_slots: u32,
    pub(crate) diagnostics: Vec<String>,
}

pub(crate) fn effective_armor_rating(candidate: &ApparelCandidate) -> f32 {
    let base = if candidate.base_armor_rating.is_finite() {
        candidate.base_armor_rating.max(0.0)
    } else {
        0.0
    };
    let Some(max_condition) = candidate.max_condition.filter(|value| *value != 0) else {
        return base;
    };
    let current = candidate.current_condition.unwrap_or(max_condition);
    base * (current.min(max_condition) as f32 / max_condition as f32)
}

pub(crate) fn select_spawn_outfit(
    candidates: &[ApparelCandidate],
    female: bool,
    mut model_available: impl FnMut(&str) -> bool,
) -> SpawnOutfit {
    let mut ranked = candidates.iter().collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        effective_armor_rating(right)
            .total_cmp(&effective_armor_rating(left))
            .then_with(|| right.value.cmp(&left.value))
            .then_with(|| left.form_id.cmp(&right.form_id))
    });
    ranked.dedup_by_key(|candidate| candidate.form_id);

    let mut outfit = SpawnOutfit::default();
    for candidate in ranked {
        if candidate.biped_slot_mask == 0 {
            outfit.diagnostics.push(format!(
                "apparel {:08x} has no biped slots and was not equipped",
                candidate.form_id
            ));
            continue;
        }
        let Some(model_path) = candidate.worn_model(female) else {
            outfit.diagnostics.push(format!(
                "apparel {:08x} has no {} worn model; retaining race body fallback",
                candidate.form_id,
                if female { "female" } else { "male" }
            ));
            continue;
        };
        if !model_available(model_path) {
            outfit.diagnostics.push(format!(
                "apparel {:08x} worn model {model_path} is unavailable; retaining race body fallback",
                candidate.form_id
            ));
            continue;
        }
        if outfit.occupied_slots & candidate.biped_slot_mask != 0 {
            continue;
        }
        outfit.occupied_slots |= candidate.biped_slot_mask;
        outfit.worn.push(SelectedApparel {
            form_id: candidate.form_id,
            model_path: model_path.to_owned(),
            biped_slot_mask: candidate.biped_slot_mask,
        });
    }
    outfit.worn.sort_by(|left, right| {
        left.model_path
            .to_ascii_lowercase()
            .cmp(&right.model_path.to_ascii_lowercase())
            .then_with(|| left.form_id.cmp(&right.form_id))
    });
    outfit.diagnostics.sort();
    outfit
}

#[cfg(test)]
pub(crate) fn race_body_part_visible(index: u32, occupied_slots: u32) -> bool {
    let covered_slot = match index {
        0 => FO3_SLOT_UPPER_BODY,
        1 => FO3_SLOT_LEFT_HAND,
        2 => FO3_SLOT_RIGHT_HAND,
        _ => 0,
    };
    covered_slot == 0 || occupied_slots & covered_slot == 0
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn partition_is_editor_visible(flags: u16) -> bool {
    flags & PF_EDITOR_VISIBLE != 0
}

#[cfg(test)]
#[path = "tests/actor_appearance.rs"]
mod tests;
