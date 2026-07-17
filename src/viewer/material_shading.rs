use bevy::asset::{AssetEvent, Assets};
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;

use super::material_shading_policy::{
    SPECULAR_ALPHA_ROUGHNESS_DEFAULTS, specular_alpha_roughness_eligible,
};

pub(crate) fn install(app: &mut App) {
    app.add_systems(Update, apply_specular_alpha_roughness_policy);
}

fn apply_specular_alpha_roughness_policy(
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.read() {
        let id = match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => *id,
            AssetEvent::LoadedWithDependencies { id } => *id,
            AssetEvent::Unused { .. } | AssetEvent::Removed { .. } => continue,
        };

        let Some(mut material) = materials.get_mut(id) else {
            continue;
        };

        if material.specular_alpha_roughness {
            continue;
        }

        let normal_and_specular_share_image = match (
            material.normal_map_texture.as_ref(),
            material.specular_texture.as_ref(),
        ) {
            (Some(normal), Some(specular)) => normal == specular,
            _ => false,
        };

        if !specular_alpha_roughness_eligible(
            material.normal_map_texture.is_some(),
            material.specular_texture.is_some(),
            normal_and_specular_share_image,
            material.metallic_roughness_texture.is_some(),
            !matches!(material.alpha_mode, AlphaMode::Opaque),
        ) {
            continue;
        }

        material.specular_alpha_roughness = true;
        material.specular_alpha_roughness_min = SPECULAR_ALPHA_ROUGHNESS_DEFAULTS.min;
        material.specular_alpha_roughness_max = SPECULAR_ALPHA_ROUGHNESS_DEFAULTS.max;
        material.specular_alpha_roughness_curve = SPECULAR_ALPHA_ROUGHNESS_DEFAULTS.curve;
    }
}
