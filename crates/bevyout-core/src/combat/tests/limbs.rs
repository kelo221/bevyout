use super::*;
use crate::combat::body::BodyPartId;
use crate::perception::{TargetClass, TargetId};

fn impact(shot: u64, part: BodyPartId, milli: u32) -> LimbImpact {
    LimbImpact {
        shot_id: ShotId::from_sequence(shot),
        target: TargetId {
            class: TargetClass::Actor,
            form_id: 0x20,
        },
        part,
        final_damage_milli: milli,
    }
}

#[test]
fn healthy_state_has_six_full_parts() {
    let state = LimbState::healthy();
    assert_eq!(state.parts.len(), 6);
    assert_eq!(state.part(BodyPartId::Head).current_milli, LIMB_MAX_MILLI);
    assert!(!state.part(BodyPartId::Head).crippled);
    assert_eq!(state.locomotion_speed_bps(), LOCOMOTION_FULL_BPS);
}

#[test]
fn unknown_nodes_fall_back_to_torso() {
    assert_eq!(BodyPartId::from_node_name(""), BodyPartId::Torso);
    assert_eq!(
        BodyPartId::from_node_name("widget_mesh_02"),
        BodyPartId::Torso
    );
    assert_eq!(
        BodyPartId::from_node_name("Bip01 Spine2"),
        BodyPartId::Torso
    );
}

#[test]
fn named_nodes_map_onto_stable_parts() {
    assert_eq!(BodyPartId::from_node_name("Bip01 Head"), BodyPartId::Head);
    assert_eq!(
        BodyPartId::from_node_name("Bip01 L UpperArm"),
        BodyPartId::LeftArm
    );
    assert_eq!(
        BodyPartId::from_node_name("Bip01 R Forearm"),
        BodyPartId::RightArm
    );
    assert_eq!(
        BodyPartId::from_node_name("Bip01 L Calf"),
        BodyPartId::LeftLeg
    );
    assert_eq!(
        BodyPartId::from_node_name("Bip01 R Foot"),
        BodyPartId::RightLeg
    );
}

#[test]
fn cripple_emits_one_transition() {
    let mut state = LimbState::healthy();
    let first = apply_limb_impact(&mut state, impact(1, BodyPartId::LeftLeg, LIMB_MAX_MILLI));
    assert!(first.newly_crippled);
    assert_eq!(first.remaining_milli, 0);
    let second = apply_limb_impact(&mut state, impact(2, BodyPartId::LeftLeg, 50_000));
    assert!(!second.newly_crippled);
    assert!(second.already_crippled);
    assert_eq!(second.remaining_milli, 0);
}

#[test]
fn duplicate_shot_id_is_rejected() {
    let mut state = LimbState::healthy();
    let first = apply_limb_impact(&mut state, impact(7, BodyPartId::Torso, 10_000));
    assert!(!first.duplicate);
    assert_eq!(state.part(BodyPartId::Torso).current_milli, 90_000);
    let second = apply_limb_impact(&mut state, impact(7, BodyPartId::Torso, 10_000));
    assert!(second.duplicate);
    assert_eq!(state.part(BodyPartId::Torso).current_milli, 90_000);
}

#[test]
fn leg_and_arm_penalties_are_pinned() {
    let mut state = LimbState::healthy();
    apply_limb_impact(&mut state, impact(1, BodyPartId::LeftLeg, LIMB_MAX_MILLI));
    assert_eq!(state.locomotion_speed_bps(), LOCOMOTION_ONE_LEG_BPS);
    apply_limb_impact(&mut state, impact(2, BodyPartId::RightLeg, LIMB_MAX_MILLI));
    assert_eq!(state.locomotion_speed_bps(), LOCOMOTION_TWO_LEG_BPS);
    apply_limb_impact(&mut state, impact(3, BodyPartId::RightArm, LIMB_MAX_MILLI));
    assert_eq!(state.arm_reload_multiplier_bps(), 15_000);
    assert_eq!(state.arm_spread_penalty_bps(), 2_500);
    apply_limb_impact(&mut state, impact(4, BodyPartId::LeftArm, LIMB_MAX_MILLI));
    assert_eq!(state.arm_reload_multiplier_bps(), 20_000);
    assert_eq!(state.arm_spread_penalty_bps(), 5_000);
}

#[test]
fn reequip_weapon_instances_do_not_collide() {
    let mut state = LimbState::healthy();
    let first = apply_limb_impact(
        &mut state,
        LimbImpact {
            shot_id: ShotId::from_weapon_shot(11, 1),
            target: TargetId {
                class: TargetClass::Actor,
                form_id: 0x20,
            },
            part: BodyPartId::Torso,
            final_damage_milli: 10_000,
        },
    );
    let second = apply_limb_impact(
        &mut state,
        LimbImpact {
            shot_id: ShotId::from_weapon_shot(12, 1),
            target: TargetId {
                class: TargetClass::Actor,
                form_id: 0x20,
            },
            part: BodyPartId::Torso,
            final_damage_milli: 10_000,
        },
    );
    assert!(!first.duplicate);
    assert!(!second.duplicate);
    assert_eq!(state.part(BodyPartId::Torso).current_milli, 80_000);
}

#[test]
fn crippled_head_requests_blur_and_perception_penalty() {
    let mut state = LimbState::healthy();
    assert_eq!(state.head_perception_penalty(), 0);
    let outcome = apply_limb_impact(&mut state, impact(1, BodyPartId::Head, LIMB_MAX_MILLI));
    assert!(outcome.head_blur);
    assert_eq!(state.head_perception_penalty(), HEAD_PERCEPTION_PENALTY);
    assert!(state.requests_head_blur());
}
