use super::*;
use bevyout_core::actor::ActorKind;
use bevyout_core::actor_state::{ActorDefinition, ActorLifeState};
use bevyout_core::perception::PerceptionInputs;
use std::sync::Arc;

use super::{ActorAwareness, ActorRuntime, ActorStateRuntime, PlacementRoot};
use crate::vsa::PreparedPlacement;

fn witness_runtime(form_id: u32) -> ActorRuntime {
    ActorRuntime {
        base_form_id: form_id,
        reference_form_id: form_id,
        kind: ActorKind::Humanoid,
        assembly: None,
    }
}

fn seeing_awareness() -> ActorAwareness {
    ActorAwareness {
        last_player: Some(PerceptionInputs {
            target: TargetId::player(),
            position: [0.0; 3],
            distance: 2.0,
            angle_to_target: 0.0,
            has_line_of_sight: true,
            detectable: true,
        }),
        ..ActorAwareness::default()
    }
}

fn disabled_root() -> PlacementRoot {
    let ron = "(
            reference_form_id: 16,
            base_form_id: 1,
            asset_path: None,
            translation: (0.0, 0.0, 0.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 1.0,
            error: None,
            initially_enabled: false,
        )";
    let placement: PreparedPlacement = ron::de::from_str(ron).unwrap();
    PlacementRoot::new(placement)
}

fn dead_state() -> ActorStateRuntime {
    ActorStateRuntime {
        cell_form_id: 1,
        definition: Arc::new(ActorDefinition::default()),
        life_state: ActorLifeState::Dead,
    }
}

#[test]
fn live_witnesses_mark_dead_and_disabled_actors() {
    let runtime = witness_runtime(0x10);
    let awareness = seeing_awareness();
    let dead = dead_state();
    let disabled = disabled_root();
    let witnesses = live_witnesses(std::iter::once((
        &runtime,
        &awareness,
        Some(&dead),
        Some(&disabled),
    )));
    assert_eq!(witnesses.len(), 1);
    assert!(!witnesses[0].alive);
    assert!(!witnesses[0].enabled);
}
