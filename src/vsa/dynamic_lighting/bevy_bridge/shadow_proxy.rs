//! Explicit Bevy shadow-capture boundary for custom DynamicLights.

use bevy::{light::PointLight, prelude::*, render::extract_component::ExtractComponent};

use super::{DynamicLight, DynamicLightingSettings};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum DynamicShadowMode {
    #[default]
    Realtime,
}

/// Separately identified, black `PointLight` used only to make Bevy render a
/// shadow cubemap. The custom WGSL pass owns visible light contribution.
#[derive(Clone, Component, Copy, Debug, ExtractComponent)]
pub(crate) struct DynamicLightShadowProxy {
    pub(crate) dynamic_light: Entity,
    pub(crate) mode: DynamicShadowMode,
}

impl DynamicLightShadowProxy {
    pub(crate) fn realtime(dynamic_light: Entity) -> Self {
        Self {
            dynamic_light,
            mode: DynamicShadowMode::Realtime,
        }
    }

    /// A nonzero intensity keeps the shadow capture eligible; black color
    /// guarantees that Bevy's built-in direct-light contribution is zero.
    pub(crate) fn shadow_only_point_light(range: f32) -> PointLight {
        PointLight {
            color: Color::BLACK,
            intensity: 1.0,
            range,
            shadow_maps_enabled: true,
            affects_lightmapped_mesh_diffuse: false,
            ..default()
        }
    }
}

pub(super) fn sync_shadow_proxies(
    settings: Res<DynamicLightingSettings>,
    dynamic_lights: Query<(&DynamicLight, &GlobalTransform)>,
    mut proxies: Query<(
        &DynamicLightShadowProxy,
        &mut PointLight,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    for (proxy, mut point_light, mut transform, mut visibility) in &mut proxies {
        let Ok((dynamic_light, dynamic_transform)) = dynamic_lights.get(proxy.dynamic_light) else {
            if *visibility != Visibility::Hidden {
                *visibility = Visibility::Hidden;
            }
            continue;
        };

        let desired_visibility = if settings.shadow_proxies_enabled {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if *visibility != desired_visibility {
            *visibility = desired_visibility;
        }
        if point_light.range != dynamic_light.config.radius {
            point_light.range = dynamic_light.config.radius;
        }
        let realtime = matches!(proxy.mode, DynamicShadowMode::Realtime);
        if point_light.shadow_maps_enabled != realtime {
            point_light.shadow_maps_enabled = realtime;
        }
        if point_light.affects_lightmapped_mesh_diffuse {
            point_light.affects_lightmapped_mesh_diffuse = false;
        }
        // Keep these invariant even if an inspector or scene loader changed the
        // proxy: temporal effects never drive a Bevy light.
        if point_light.color != Color::BLACK {
            point_light.color = Color::BLACK;
        }
        if point_light.intensity != 1.0 {
            point_light.intensity = 1.0;
        }

        let (_, rotation, translation) = dynamic_transform.to_scale_rotation_translation();
        if transform.translation != translation {
            transform.translation = translation;
        }
        if transform.rotation != rotation {
            transform.rotation = rotation;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_proxy_has_zero_builtin_direct_contribution() {
        let proxy = DynamicLightShadowProxy::shadow_only_point_light(7.5);
        assert_eq!(proxy.color.to_linear(), LinearRgba::BLACK);
        assert_eq!(proxy.intensity, 1.0);
        assert_eq!(proxy.range, 7.5);
        assert!(proxy.shadow_maps_enabled);
        assert!(!proxy.affects_lightmapped_mesh_diffuse);
    }
}
