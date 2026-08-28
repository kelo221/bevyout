//! Player limb inspection and debug mutation (M9 wave 4).

use bevyout_core::combat::body::BodyPartId;
use bevyout_core::combat::limbs::{LimbImpact, ShotId, apply_limb_impact};
use bevyout_core::perception::TargetId;
use serde_json::json;

use super::stats::{PlayerLimbTarget, PlayerProgression};
use super::*;

pub(super) struct LimbCommandProvider;

impl ConsoleCommandProvider for LimbCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "showlimbs",
                "[player.]showlimbs",
                "Report player limb condition, cripple flags, and locomotion projection.",
                show_limbs,
            )
            .reference_callable(false),
            ConsoleCommand::new(
                "cripple",
                "[player.]cripple <part>",
                "Force-cripple one player body part for acceptance.",
                cripple_limb,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "selectlimb",
                "[player.]selectlimb <part>",
                "Select the limb a targeted Stimpak will restore.",
                select_limb,
            )
            .reference_callable(false)
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn parse_part(raw: &str) -> Result<BodyPartId, ConsoleError> {
    BodyPartId::parse(raw).ok_or_else(|| {
        ConsoleError::new(
            "unknown_body_part",
            format!("unknown body part {raw:?}; expected head, torso, left_arm, right_arm, left_leg, or right_leg"),
        )
    })
}

fn show_limbs(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let progression = world.get_resource::<PlayerProgression>().ok_or_else(|| {
        ConsoleError::new("player_unavailable", "player progression is unavailable")
    })?;
    let target = world
        .get_resource::<PlayerLimbTarget>()
        .map(|target| target.0)
        .unwrap_or(BodyPartId::Torso);
    let parts: serde_json::Map<String, serde_json::Value> = bevyout_core::combat::ALL_BODY_PARTS
        .into_iter()
        .map(|part| {
            let condition = progression.limbs.part(part);
            (
                part.label().to_string(),
                json!({
                    "current_milli": condition.current_milli,
                    "max_milli": condition.max_milli,
                    "crippled": condition.crippled,
                }),
            )
        })
        .collect();
    Ok(ConsoleCommandResult::new(
        json!({
            "parts": parts,
            "locomotion_bps": progression.limbs.locomotion_speed_bps(),
            "arm_reload_bps": progression.limbs.arm_reload_multiplier_bps(),
            "arm_spread_bps": progression.limbs.arm_spread_penalty_bps(),
            "head_perception_penalty": progression.limbs.head_perception_penalty(),
            "selected": target.label(),
        }),
        vec![format!(
            "limbs locomotion {} bps selected {}",
            progression.limbs.locomotion_speed_bps(),
            target.label()
        )],
    ))
}

fn cripple_limb(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "cripple requires a body part",
        ));
    };
    let part = parse_part(raw)?;
    let mut progression = world
        .get_resource_mut::<PlayerProgression>()
        .ok_or_else(|| {
            ConsoleError::new("player_unavailable", "player progression is unavailable")
        })?;
    let shot = ShotId::next_debug(&progression.limbs.applied_shots);
    let outcome = apply_limb_impact(
        &mut progression.limbs,
        LimbImpact {
            shot_id: shot,
            target: TargetId::player(),
            part,
            final_damage_milli: bevyout_core::combat::LIMB_MAX_MILLI,
        },
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "part": part.label(),
            "remaining_milli": outcome.remaining_milli,
            "crippled": true,
        }),
        vec![format!("{} crippled", part.label())],
    ))
}

fn select_limb(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "selectlimb requires a body part",
        ));
    };
    let part = parse_part(raw)?;
    world.insert_resource(PlayerLimbTarget(part));
    Ok(ConsoleCommandResult::new(
        json!({ "selected": part.label() }),
        vec![format!("selected limb {}", part.label())],
    ))
}

pub(super) fn restore_selected_player_limb(world: &mut World) {
    let part = world
        .get_resource::<PlayerLimbTarget>()
        .map(|target| target.0)
        .unwrap_or(BodyPartId::Torso);
    let Some(mut progression) = world.get_resource_mut::<PlayerProgression>() else {
        return;
    };
    super::stats::restore_targeted_stimpak(&mut progression, part);
}
