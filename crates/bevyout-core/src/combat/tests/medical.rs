use super::*;
use crate::combat::BodyPartId;
use crate::combat::limbs::{LIMB_MAX_MILLI, LimbImpact, LimbState, ShotId, apply_limb_impact};
use crate::perception::{TargetClass, TargetId};
use crate::time::GameTime;

fn cripple(state: &mut LimbState, part: BodyPartId, shot: u64) {
    apply_limb_impact(
        state,
        LimbImpact {
            shot_id: ShotId::from_sequence(shot),
            target: TargetId {
                class: TargetClass::Player,
                form_id: 0,
            },
            part,
            final_damage_milli: LIMB_MAX_MILLI,
        },
    );
}

#[test]
fn targeted_stimpak_restores_one_limb() {
    let mut state = LimbState::healthy();
    apply_limb_impact(
        &mut state,
        LimbImpact {
            shot_id: ShotId::from_sequence(1),
            target: TargetId::player(),
            part: BodyPartId::LeftArm,
            final_damage_milli: 80_000,
        },
    );
    let outcome = restore_limbs(
        &mut state,
        MedicalSource::TargetedStimpak,
        Some(BodyPartId::LeftArm),
        GameTime::from_ms(0),
    );
    assert_eq!(outcome.restored_milli, STIMPAK_RESTORE_MILLI);
    assert_eq!(state.part(BodyPartId::LeftArm).current_milli, 50_000);
    assert!(!state.part(BodyPartId::LeftArm).crippled);
}

#[test]
fn doctor_and_owned_bed_fully_restore() {
    let mut state = LimbState::healthy();
    cripple(&mut state, BodyPartId::Head, 1);
    let doctor = restore_limbs(
        &mut state,
        MedicalSource::Doctor,
        None,
        GameTime::from_ms(0),
    );
    assert_eq!(state.part(BodyPartId::Head).current_milli, LIMB_MAX_MILLI);
    assert!(!state.part(BodyPartId::Head).crippled);
    assert_eq!(doctor.source, MedicalSource::Doctor);

    cripple(&mut state, BodyPartId::RightLeg, 2);
    let bed = restore_limbs(
        &mut state,
        MedicalSource::OwnedBed,
        None,
        GameTime::from_ms(3_600_000),
    );
    assert_eq!(bed.source, MedicalSource::OwnedBed);
    assert_eq!(bed.at.as_ms(), 3_600_000);
    assert_eq!(
        state.part(BodyPartId::RightLeg).current_milli,
        LIMB_MAX_MILLI
    );
}
