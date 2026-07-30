use super::*;

#[cfg(test)]
mod actor_cache_tests {
    use super::*;
    use bevyout_core::actor::{ActorFallbackDecision, ActorFallbackLevel};

    fn appearance(level: ActorFallbackLevel) -> PreparedActorAppearance {
        PreparedActorAppearance {
            descriptor: Some(ActorAssemblyDescriptor {
                skeleton: "characters/skeleton.nif".into(),
                visual_inputs: vec!["characters/skeleton.nif".into()],
                body_parts: Vec::new(),
                apparel: Vec::new(),
                head_parts: Vec::new(),
                head_anim_parts: Vec::new(),
                eye_geometry: Vec::new(),
                eye_texture: None,
            }),
            blueprint: bevyout_core::actor::ActorAssemblyBlueprint {
                skeleton_path: Some("characters/skeleton.nif".into()),
                fallback: ActorFallbackDecision {
                    level,
                    ..Default::default()
                },
                ..Default::default()
            },
            inventory: Vec::new(),
        }
    }

    #[test]
    fn actor_cache_input_includes_the_fallback_selection_metadata() {
        let bytes = vec![b"same-nif".to_vec()];
        let exact =
            actor_cache_input_bytes(&bytes, Some(&appearance(ActorFallbackLevel::AuthoredExact)))
                .unwrap();
        let fallback = actor_cache_input_bytes(
            &bytes,
            Some(&appearance(ActorFallbackLevel::RaceSexSpecific)),
        )
        .unwrap();

        assert_ne!(fingerprint(&exact), fingerprint(&fallback));
    }

    #[test]
    fn actor_cache_input_includes_the_selected_eye_texture() {
        let bytes = vec![b"same-nif".to_vec()];
        let mut brown = appearance(ActorFallbackLevel::RaceSexSpecific);
        brown.descriptor.as_mut().unwrap().eye_texture =
            Some("textures/characters/eyes/brown.dds".into());
        let mut blue = brown.clone();
        blue.descriptor.as_mut().unwrap().eye_texture =
            Some("textures/characters/eyes/blue.dds".into());

        let brown = actor_cache_input_bytes(&bytes, Some(&brown)).unwrap();
        let blue = actor_cache_input_bytes(&bytes, Some(&blue)).unwrap();

        assert_ne!(fingerprint(&brown), fingerprint(&blue));
    }
}
