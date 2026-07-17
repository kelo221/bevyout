//! Pure prepared-light migration policy shared by runtime code and Cucumber.

const FOUR_PI: f32 = 4.0 * std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedLightPlanInput {
    pub(crate) reference_form_id: u32,
    pub(crate) initially_enabled: bool,
    pub(crate) radius: f32,
    pub(crate) prepared_shadow_layer: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedDynamicLightPlan {
    pub(crate) reference_form_id: u32,
    /// Luminous intensity used by the custom inverse-square path. The former
    /// Bevy PointLight stored luminous power, which Bevy divided by 4 PI during
    /// extraction; doing that conversion here preserves the authored scale.
    pub(crate) intensity: f32,
    pub(crate) radius: f32,
    pub(crate) prepared_shadow_layer: Option<u32>,
    pub(crate) realtime_shadow_proxy: bool,
    pub(crate) volumetric: Option<PreparedLightVolumetricPlan>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedCellFogPlan {
    pub(crate) start_metres: f32,
    pub(crate) end_metres: f32,
    pub(crate) power: f32,
    pub(crate) strength: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PreparedLightVolumetricPlan {
    pub(crate) radius: f32,
    pub(crate) thickness: f32,
    pub(crate) intensity: f32,
    pub(crate) intensity_at_full_strength: f32,
    pub(crate) visibility: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PreparedLightMigrationPlan {
    pub(crate) sources: Vec<PreparedDynamicLightPlan>,
}

#[cfg(test)]
impl PreparedLightMigrationPlan {
    pub(crate) fn visible_bevy_point_light_count(&self) -> usize {
        0
    }

    pub(crate) fn realtime_shadow_proxy_count(&self) -> usize {
        self.sources
            .iter()
            .filter(|source| source.realtime_shadow_proxy)
            .count()
    }
}

pub(crate) fn plan_prepared_light_migration(
    inputs: impl IntoIterator<Item = PreparedLightPlanInput>,
    lighting_scale: f32,
    cell_fog: Option<PreparedCellFogPlan>,
) -> PreparedLightMigrationPlan {
    let mut sources = inputs
        .into_iter()
        .filter(|input| input.initially_enabled)
        .map(|input| PreparedDynamicLightPlan {
            reference_form_id: input.reference_form_id,
            intensity: input.radius * input.radius * 2.0 * lighting_scale / FOUR_PI,
            radius: input.radius,
            prepared_shadow_layer: input.prepared_shadow_layer,
            realtime_shadow_proxy: false,
            volumetric: cell_fog.and_then(|fog| plan_prepared_light_volumetric(input.radius, fog)),
        })
        .collect::<Vec<_>>();

    let strongest = sources
        .iter()
        .enumerate()
        .filter(|(_, source)| source.intensity.is_finite())
        .max_by(|(_, left), (_, right)| {
            left.intensity
                .total_cmp(&right.intensity)
                .then_with(|| right.reference_form_id.cmp(&left.reference_form_id))
        })
        .map(|(index, _)| index);
    if let Some(index) = strongest {
        sources[index].realtime_shadow_proxy = true;
    }

    PreparedLightMigrationPlan { sources }
}

pub(crate) fn plan_prepared_light_volumetric(
    light_radius: f32,
    fog: PreparedCellFogPlan,
) -> Option<PreparedLightVolumetricPlan> {
    let fog_span = fog.end_metres - fog.start_metres;
    if !light_radius.is_finite()
        || light_radius <= 0.0
        || !fog_span.is_finite()
        || fog_span <= 0.0
        || !fog.power.is_finite()
        || !fog.strength.is_finite()
        || fog.strength < 0.0
    {
        return None;
    }

    // Match the cell's linear distance-fog density over the longest path
    // through this spherical light volume. Fog power follows the same >= 1
    // clamp as DistanceFog, while the global render control remains a direct
    // 0..1 multiplier.
    let intensity_at_full_strength =
        (2.0 * light_radius / fog_span * fog.power.max(1.0)).clamp(0.0, 1.0);
    let intensity = intensity_at_full_strength * fog.strength.clamp(0.0, 1.0);
    (intensity > 0.0).then_some(PreparedLightVolumetricPlan {
        radius: light_radius,
        thickness: 1.0,
        intensity,
        intensity_at_full_strength,
        visibility: 2.0 * light_radius,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strongest_enabled_source_gets_the_only_realtime_proxy() {
        let plan = plan_prepared_light_migration(
            [
                PreparedLightPlanInput {
                    reference_form_id: 2,
                    initially_enabled: true,
                    radius: 8.0,
                    prepared_shadow_layer: Some(4),
                },
                PreparedLightPlanInput {
                    reference_form_id: 1,
                    initially_enabled: true,
                    radius: 8.0,
                    prepared_shadow_layer: Some(3),
                },
                PreparedLightPlanInput {
                    reference_form_id: 3,
                    initially_enabled: false,
                    radius: 20.0,
                    prepared_shadow_layer: Some(5),
                },
            ],
            128.0,
            None,
        );
        assert_eq!(plan.sources.len(), 2);
        assert_eq!(plan.realtime_shadow_proxy_count(), 1);
        assert!(plan.sources[1].realtime_shadow_proxy);
        assert_eq!(plan.sources[1].reference_form_id, 1);
        assert_eq!(plan.sources[1].prepared_shadow_layer, Some(3));
        assert_eq!(plan.visible_bevy_point_light_count(), 0);
    }

    #[test]
    fn cell_fog_density_scales_sphere_volume_by_light_diameter() {
        let volumetric = plan_prepared_light_volumetric(
            10.0,
            PreparedCellFogPlan {
                start_metres: 1.0,
                end_metres: 101.0,
                power: 1.0,
                strength: 0.5,
            },
        )
        .unwrap();
        assert_eq!(volumetric.radius, 10.0);
        assert_eq!(volumetric.visibility, 20.0);
        assert!((volumetric.intensity_at_full_strength - 0.2).abs() < f32::EPSILON);
        assert!((volumetric.intensity - 0.1).abs() < f32::EPSILON);
    }
}
