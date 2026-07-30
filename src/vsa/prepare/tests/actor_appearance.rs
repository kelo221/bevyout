use super::*;

fn apparel(form_id: u32, mask: u32, rating: f32, value: i32) -> ApparelCandidate {
    ApparelCandidate {
        form_id,
        male_worn: Some(format!("m/{form_id:08x}.nif")),
        female_worn: Some(format!("f/{form_id:08x}.nif")),
        biped_slot_mask: mask,
        base_armor_rating: rating,
        max_condition: Some(100),
        value,
        ..ApparelCandidate::default()
    }
}

#[test]
fn damaged_condition_reduces_effective_rating() {
    let mut candidate = apparel(1, FO3_SLOT_UPPER_BODY, 20.0, 0);
    candidate.current_condition = Some(25);
    assert_eq!(effective_armor_rating(&candidate), 5.0);
}

#[test]
fn value_then_form_id_break_equal_rating_ties() {
    let selected = select_spawn_outfit(
        &[
            apparel(3, FO3_SLOT_UPPER_BODY, 10.0, 20),
            apparel(2, FO3_SLOT_UPPER_BODY, 10.0, 20),
            apparel(1, FO3_SLOT_UPPER_BODY, 10.0, 10),
        ],
        false,
        |_| true,
    );
    assert_eq!(selected.worn[0].form_id, 2);
}

#[test]
fn unavailable_apparel_does_not_hide_underwear() {
    let selected = select_spawn_outfit(&[apparel(1, FO3_SLOT_UPPER_BODY, 10.0, 0)], false, |_| {
        false
    });
    assert!(selected.worn.is_empty());
    assert!(race_body_part_visible(0, selected.occupied_slots));
}
