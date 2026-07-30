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
    Native,
    Blender,
}

impl ActorAnimationBackend {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Native => "native",
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
        ActorAnimationBackend::Native => format!(
            "{scene_converter_revision}+actor-animation=native@{actor_animation_catalog_revision}+{actor_animation_converter_revision}"
        ),
        ActorAnimationBackend::Blender => format!(
            "{scene_converter_revision}+actor-animation=blender@{actor_animation_catalog_revision}+{actor_animation_converter_revision}"
        ),
    }
}

#[cfg(test)]
#[path = "converter_policy/tests.rs"]
mod tests;
