//! Pure converter-selection policy shared by CLI adapters and feature tests.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ConverterBackend {
    #[default]
    Native,
    Blender,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ActorAnimationBackend {
    #[default]
    Disabled,
    Blender,
}

impl ActorAnimationBackend {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Blender => "blender",
        }
    }
}

impl ConverterBackend {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Blender => "blender",
        }
    }
}

pub(crate) const fn resolve_converter_backend(
    requested: Option<ConverterBackend>,
) -> ConverterBackend {
    match requested {
        Some(backend) => backend,
        None => ConverterBackend::Native,
    }
}

pub(crate) const fn resolve_actor_animation_backend(
    requested: Option<ActorAnimationBackend>,
) -> ActorAnimationBackend {
    match requested {
        Some(backend) => backend,
        None => ActorAnimationBackend::Disabled,
    }
}

pub(crate) const fn actor_animation_backend_requires_blender(
    backend: ActorAnimationBackend,
) -> bool {
    matches!(backend, ActorAnimationBackend::Blender)
}

pub(crate) fn prepare_converter_identity(
    scene_converter_revision: &str,
    actor_animation_backend: ActorAnimationBackend,
    actor_animation_catalog_revision: &str,
    actor_animation_converter_revision: &str,
) -> String {
    match actor_animation_backend {
        ActorAnimationBackend::Disabled => format!(
            "{scene_converter_revision}+actor-animation=disabled@{actor_animation_catalog_revision}"
        ),
        ActorAnimationBackend::Blender => format!(
            "{scene_converter_revision}+actor-animation=blender@{actor_animation_catalog_revision}+{actor_animation_converter_revision}"
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_animation_backend_participates_in_prepare_identity() {
        let disabled = prepare_converter_identity(
            "native-scene-v1",
            ActorAnimationBackend::Disabled,
            "actor-catalog-v1",
            "actor-kf-v1",
        );
        let blender = prepare_converter_identity(
            "native-scene-v1",
            ActorAnimationBackend::Blender,
            "actor-catalog-v1",
            "actor-kf-v1",
        );
        assert_ne!(disabled, blender);
        assert!(disabled.contains("actor-animation=disabled@actor-catalog-v1"));
        assert!(blender.contains("actor-animation=blender@actor-catalog-v1+actor-kf-v1"));
        assert_eq!(
            disabled,
            prepare_converter_identity(
                "native-scene-v1",
                ActorAnimationBackend::Disabled,
                "actor-catalog-v1",
                "actor-kf-v2",
            ),
            "a Blender-only converter change must not invalidate disabled preparation"
        );
    }
}
