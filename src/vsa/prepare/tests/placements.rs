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
                facegen: None,
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

    #[test]
    fn actor_cache_input_includes_facegen_sources_and_coefficients() {
        let bytes = vec![b"same-nif".to_vec()];
        let mut authored = appearance(ActorFallbackLevel::AuthoredExact);
        authored.descriptor.as_mut().unwrap().facegen = Some(ActorFaceGenDescriptor {
            base_form_id: 0x10,
            race_form_id: Some(0x20),
            head_path: "characters/head/headfemale.nif".into(),
            tri_path: "characters/head/headfemale.tri".into(),
            geometry_morph_path: "characters/head/headfemale.egm".into(),
            texture_morph_path: "characters/head/headfemale.egt".into(),
            tri_hash: "tri-a".into(),
            geometry_morph_hash: "egm-a".into(),
            texture_morph_hash: "egt-a".into(),
            base_vertex_count: 3,
            texture_coordinate_count: 3,
            resolved: bevyout_core::facegen::FaceGenResolved {
                actor: bevyout_core::facegen::FaceGenRaw::default(),
                race: bevyout_core::facegen::FaceGenRaw::default(),
                coefficients: bevyout_core::facegen::FaceGenCoefficients::zero(),
            },
        });
        let mut changed = authored.clone();
        changed
            .descriptor
            .as_mut()
            .unwrap()
            .facegen
            .as_mut()
            .unwrap()
            .texture_morph_hash = "egt-b".into();

        let first = actor_cache_input_bytes(&bytes, Some(&authored)).unwrap();
        let second = actor_cache_input_bytes(&bytes, Some(&changed)).unwrap();
        assert_ne!(fingerprint(&first), fingerprint(&second));
        assert!(NATIVE_ACTOR_CONVERTER_REVISION.contains("facegen"));
        assert!(!NATIVE_NIF_CONVERTER_REVISION.contains("facegen"));
    }
}
