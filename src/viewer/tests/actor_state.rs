use super::*;
use crate::vsa::ActorBlueprint;

#[test]
fn loaded_state_wins_over_revisit_seed() {
    let mut app = App::new();
    app.init_resource::<ActiveSaveState>()
        .init_resource::<ActorDefinitionCatalogs>()
        .add_systems(Update, (seed_actor_states, sync_actor_lifecycle).chain());
    app.world_mut()
        .resource_mut::<ActorDefinitionCatalogs>()
        .insert(
            0x10,
            PreparedActorCatalog {
                revision: ACTOR_CATALOG_REVISION.into(),
                source_fingerprint: "test".into(),
                entries: vec![ActorCatalogEntry::Prepared(Box::new(ActorBlueprint {
                    base_form_id: 0x20,
                    resolved_base_form_id: Some(0x20),
                    reference_form_id: 0x30,
                    record_kind: "NPC_".into(),
                    health: Some(100),
                    ..Default::default()
                }))],
                ..Default::default()
            },
        );
    app.world_mut()
        .resource_mut::<ActiveSaveState>()
        .0
        .cells
        .entry(0x10)
        .or_default()
        .actors
        .insert(0x30, ActorInstanceState::new(0x30, ActorLifeState::Dead));
    let entity = app
        .world_mut()
        .spawn(ActorRuntime {
            base_form_id: 0x20,
            reference_form_id: 0x30,
            kind: bevyout_core::actor::ActorKind::Humanoid,
            assembly: None,
        })
        .id();

    app.update();

    assert_eq!(
        app.world()
            .get::<ActorStateRuntime>(entity)
            .unwrap()
            .life_state,
        ActorLifeState::Dead
    );
    assert_eq!(
        app.world().resource::<ActiveSaveState>().0.cells[&0x10]
            .actors
            .len(),
        1
    );
}
