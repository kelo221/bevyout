use super::*;

#[test]
fn actor_animation_backend_participates_in_prepare_identity() {
    let disabled = prepare_converter_identity(
        "native-scene-v1",
        ActorAnimationBackend::Disabled,
        "actor-catalog-v1",
        "actor-kf-v1",
    );
    assert!(disabled.contains("actor-animation=disabled@actor-catalog-v1"));
    assert_eq!(
        disabled,
        prepare_converter_identity(
            "native-scene-v1",
            ActorAnimationBackend::Disabled,
            "actor-catalog-v1",
            "actor-kf-v2",
        ),
        "a native actor converter change must not invalidate disabled preparation"
    );
    let native = prepare_converter_identity(
        "native-scene-v1",
        ActorAnimationBackend::Native,
        "actor-catalog-v1",
        "actor-kf-native-v1",
    );
    assert_ne!(native, disabled);
    assert!(native.contains("actor-animation=native@actor-catalog-v1+actor-kf-native-v1"));
}
