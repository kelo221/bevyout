//! Thin main-world ECS bridge for the isolated DynamicLighting core.

mod shadow_proxy;

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::core::{
    DynamicLightConfig, DynamicLightEffect, DynamicLightType, DynamicLightVolumetricParameters,
    LightEffectRuntime, UnityRandom, advance_effect,
};
use super::render::DynamicLightingRenderPlugin;
pub(crate) use shadow_proxy::DynamicLightShadowProxy;

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct DynamicLight {
    pub(crate) config: DynamicLightConfig,
}

impl DynamicLight {
    pub(crate) fn with_effect(intensity: f32, effect: DynamicLightEffect) -> Self {
        Self {
            config: DynamicLightConfig {
                intensity,
                effect,
                ..Default::default()
            },
        }
    }

    pub(crate) fn with_color(mut self, color: Color) -> Self {
        let linear = color.to_linear();
        self.config.color = [linear.red, linear.green, linear.blue];
        self
    }

    pub(crate) fn with_radius(mut self, radius: f32) -> Self {
        self.config.radius = radius;
        self
    }

    pub(crate) fn with_type(mut self, light_type: DynamicLightType) -> Self {
        self.config.light_type = light_type;
        self
    }

    pub(crate) fn with_shadows(mut self, enabled: bool) -> Self {
        self.config.shadow_enabled = enabled;
        self
    }

    pub(crate) fn with_volumetric(mut self, volumetric: DynamicLightVolumetricParameters) -> Self {
        self.config.volumetric = volumetric;
        self
    }

    pub(crate) fn strobe(intensity: f32) -> Self {
        Self::with_effect(intensity, DynamicLightEffect::Strobe)
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub(crate) struct DynamicLightRuntime {
    pub(crate) state: LightEffectRuntime,
}

#[derive(Component, Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct DynamicLightOrdinal(pub(crate) u64);

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct DynamicLightViewMask(pub(crate) u32);

impl Default for DynamicLightViewMask {
    fn default() -> Self {
        Self(u32::MAX)
    }
}

#[derive(Resource, Clone, Copy, Debug, Reflect)]
#[reflect(Resource)]
pub(crate) struct DynamicLightingSettings {
    pub(crate) enabled: bool,
    pub(crate) volumetric_enabled: bool,
    pub(crate) freeze_effect_time: bool,
    pub(crate) shadow_proxies_enabled: bool,
    pub(crate) random_seed: i32,
}

#[derive(Default)]
struct DynamicLightingDiagnosticCounters {
    extracted_lights: AtomicUsize,
    extracted_volumetric_lights: AtomicUsize,
}

#[derive(Resource, Clone, Default)]
pub(crate) struct DynamicLightingDiagnostics(Arc<DynamicLightingDiagnosticCounters>);

impl DynamicLightingDiagnostics {
    pub(crate) fn extracted_light_count(&self) -> usize {
        self.0.extracted_lights.load(Ordering::Relaxed)
    }

    pub(crate) fn extracted_volumetric_light_count(&self) -> usize {
        self.0.extracted_volumetric_lights.load(Ordering::Relaxed)
    }

    pub(super) fn set_extracted_light_count(&self, count: usize) {
        self.0.extracted_lights.store(count, Ordering::Relaxed);
    }

    pub(super) fn set_extracted_volumetric_light_count(&self, count: usize) {
        self.0
            .extracted_volumetric_lights
            .store(count, Ordering::Relaxed);
    }
}

impl Default for DynamicLightingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            volumetric_enabled: true,
            freeze_effect_time: false,
            shadow_proxies_enabled: true,
            random_seed: 12345,
        }
    }
}

#[derive(Resource)]
struct DynamicLightingRandom {
    seed: i32,
    stream: UnityRandom,
}

impl FromWorld for DynamicLightingRandom {
    fn from_world(world: &mut World) -> Self {
        let seed = world.resource::<DynamicLightingSettings>().random_seed;
        Self {
            seed,
            stream: UnityRandom::from_seed(seed),
        }
    }
}

#[derive(Resource, Default)]
struct NextDynamicLightOrdinal(u64);

#[derive(SystemParam)]
struct DynamicLightUpdateQueries<'w, 's> {
    order: Query<'w, 's, (Entity, &'static DynamicLightOrdinal), With<DynamicLight>>,
    lights: Query<
        'w,
        's,
        (
            &'static DynamicLight,
            &'static mut DynamicLightRuntime,
            &'static GlobalTransform,
        ),
    >,
}

pub(crate) struct DynamicLightingPlugin;

impl Plugin for DynamicLightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DynamicLightingRenderPlugin)
            .register_type::<DynamicLightingSettings>()
            .init_resource::<DynamicLightingSettings>()
            .init_resource::<DynamicLightingDiagnostics>()
            .init_resource::<DynamicLightingRandom>()
            .init_resource::<NextDynamicLightOrdinal>()
            .add_systems(
                Update,
                (
                    ensure_dynamic_light_runtime,
                    reset_changed_dynamic_lights,
                    update_dynamic_lights,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                shadow_proxy::sync_shadow_proxies.before(TransformSystems::Propagate),
            );
    }
}

fn ensure_dynamic_light_runtime(
    mut commands: Commands,
    mut next: ResMut<NextDynamicLightOrdinal>,
    lights: Query<Entity, (With<DynamicLight>, Without<DynamicLightRuntime>)>,
) {
    for entity in &lights {
        let ordinal = next.0;
        next.0 = next.0.wrapping_add(1);
        commands.entity(entity).insert((
            DynamicLightRuntime::default(),
            DynamicLightOrdinal(ordinal),
            DynamicLightViewMask::default(),
        ));
    }
}

fn reset_changed_dynamic_lights(
    mut lights: Query<&mut DynamicLightRuntime, Changed<DynamicLight>>,
) {
    for mut runtime in &mut lights {
        runtime.state = LightEffectRuntime::default();
    }
}

/// Advances only custom DynamicLighting state. It never queries or mutates a
/// Bevy PointLight; pixel lighting is produced by the render-world WGSL pass.
fn update_dynamic_lights(
    time: Res<Time>,
    settings: Res<DynamicLightingSettings>,
    mut random: ResMut<DynamicLightingRandom>,
    mut queries: DynamicLightUpdateQueries,
) {
    if random.seed != settings.random_seed {
        random.seed = settings.random_seed;
        random.stream = UnityRandom::from_seed(settings.random_seed);
    }
    if !settings.enabled || settings.freeze_effect_time {
        return;
    }

    let mut order = queries
        .order
        .iter()
        .map(|(entity, ordinal)| (ordinal.0, entity))
        .collect::<Vec<_>>();
    order.sort_unstable();

    let delta = time.delta_secs();
    for (_, entity) in order {
        let Ok((light, mut runtime, _transform)) = queries.lights.get_mut(entity) else {
            continue;
        };
        advance_effect(&light.config, &mut runtime.state, &mut random.stream, delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standalone_dynamic_light_requires_no_point_light() {
        let mut app = App::new();
        app.init_resource::<Time>();
        app.add_plugins(DynamicLightingPlugin);
        let entity = app
            .world_mut()
            .spawn((DynamicLight::strobe(2.0), GlobalTransform::IDENTITY))
            .id();
        app.update();
        let world = app.world();
        assert!(world.entity(entity).contains::<DynamicLightRuntime>());
        assert!(!world.entity(entity).contains::<bevy::light::PointLight>());
    }

    #[test]
    fn changing_authored_effect_resets_the_source_cache_before_reuse() {
        let mut app = App::new();
        app.init_resource::<Time>();
        app.add_plugins(DynamicLightingPlugin);
        let entity = app
            .world_mut()
            .spawn((
                DynamicLight::with_effect(2.0, DynamicLightEffect::Steady),
                GlobalTransform::IDENTITY,
            ))
            .id();
        app.update();
        {
            let mut entity_mut = app.world_mut().entity_mut(entity);
            let mut runtime = entity_mut.get_mut::<DynamicLightRuntime>().unwrap();
            runtime.state.elapsed_seconds = 123.0;
            runtime.state.intensity = 0.33;
            runtime.state.initialized = true;
        }
        app.world_mut()
            .entity_mut(entity)
            .get_mut::<DynamicLight>()
            .unwrap()
            .config
            .effect = DynamicLightEffect::Strobe;

        app.update();
        let runtime = app
            .world()
            .entity(entity)
            .get::<DynamicLightRuntime>()
            .unwrap();
        assert_eq!(runtime.state.elapsed_seconds, 0.0);
        assert_eq!(runtime.state.intensity, 1.0);
        assert!(runtime.state.strobe_active);
    }
}
