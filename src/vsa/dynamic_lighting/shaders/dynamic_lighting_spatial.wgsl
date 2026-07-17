#define_import_path bevyout_dynamic_lighting::spatial

#import bevyout_dynamic_lighting::{
    common::{snap_direction, to_light_space},
    types::{
        DynamicLight, LIGHT_TYPE_DISCO, LIGHT_TYPE_DISCOBALL, LIGHT_TYPE_INTERFERENCE,
        LIGHT_TYPE_POINT, LIGHT_TYPE_ROTOR, LIGHT_TYPE_SHOCK, LIGHT_TYPE_SPOT,
        LIGHT_TYPE_WAVE, light_type,
    },
}

const PI: f32 = 3.141592653589793;
const HALF_PI: f32 = 1.5707963267948966;
const TAU: f32 = 6.283185307179586;

fn spotlight(light: DynamicLight, light_direction: vec3<f32>) -> f32 {
    let theta = dot(light_direction, light.forward);
    let epsilon = light.parameter_a - light.parameter_b;
    return saturate((theta - light.parameter_b) / epsilon);
}

fn discoball(light: DynamicLight, light_direction: vec3<f32>) -> f32 {
    let rotated = to_light_space(light_direction, light.forward, light.up);
    let theta = dot(snap_direction(rotated), rotated);
    let epsilon = light.parameter_a - light.parameter_b;
    return saturate((theta - light.parameter_b) / epsilon);
}

fn wave(light: DynamicLight, world: vec3<f32>) -> f32 {
    return 0.7 + 0.3 * sin((distance(light.position, world) - light.parameter_a) * light.parameter_b);
}

fn interference(light: DynamicLight, light_minus_world: vec3<f32>) -> f32 {
    let world = to_light_space(light_minus_world, light.forward, light.up);
    let angle = atan2(sqrt(world.x * world.x + world.z * world.z), world.y) * light.parameter_b;
    return 0.5 + 0.5 * cos(angle - light.parameter_a);
}

fn rotor(light: DynamicLight, light_minus_world: vec3<f32>) -> f32 {
    let world = to_light_space(light_minus_world, light.forward, light.up);
    let angle = light.parameter_b * atan2(world.x, world.z);
    var scale = 0.5 + 0.5 * cos(angle + light.parameter_a);
    let absolute_center = light.radius_sqr * abs(light.parameter_c);
    var distance_sqr = dot(world.xz, world.xz);
    if light.parameter_c < 0.0 {
        if distance_sqr < absolute_center {
            scale *= pow(distance_sqr / absolute_center, PI);
        }
    } else {
        distance_sqr *= 1.0 / absolute_center;
        if distance_sqr < 1.0 {
            scale = 1.0 - distance_sqr + scale * distance_sqr;
        }
    }
    return pow(scale, HALF_PI);
}

fn shock(light: DynamicLight, world: vec3<f32>) -> f32 {
    let dist = light.parameter_b * distance(light.position, world);
    var brightness = 0.9 + 0.1 * sin((dist * 2.0 - light.parameter_a) * TAU);
    brightness *= 0.9 + 0.1 * cos((dist + light.parameter_a) * TAU);
    brightness *= 0.9 + 0.1 * sin((dist / 2.0 - light.parameter_a) * TAU);
    return brightness;
}

fn disco(light: DynamicLight, light_minus_world: vec3<f32>) -> f32 {
    let world = to_light_space(light_minus_world, light.forward, light.up);
    let horizontal = light.parameter_b * atan2(world.x, world.z);
    let vertical = light.parameter_b * atan2(sqrt(world.x * world.x + world.z * world.z), world.y);
    let scale_1 = 0.5 + 0.5 * cos(horizontal + light.parameter_a);
    let scale_2 = 0.5 + 0.5 * cos(vertical - light.parameter_c);
    var scale = scale_1 + scale_2 - scale_1 * scale_2;
    let dist = 0.5 * (world.x * world.x + world.z * world.z);
    if dist < 1.0 {
        scale *= dist;
    }
    return 1.0 - scale;
}

fn spatial_multiplier(
    light: DynamicLight,
    world: vec3<f32>,
    light_minus_world: vec3<f32>,
    light_direction: vec3<f32>,
) -> f32 {
    let kind = light_type(light);
    if kind == LIGHT_TYPE_POINT {
        return 1.0;
    }
    if kind == LIGHT_TYPE_SPOT {
        return spotlight(light, light_direction);
    }
    if kind == LIGHT_TYPE_DISCOBALL {
        return discoball(light, light_direction);
    }
    if kind == LIGHT_TYPE_WAVE {
        return wave(light, world);
    }
    if kind == LIGHT_TYPE_INTERFERENCE {
        return interference(light, light_minus_world);
    }
    if kind == LIGHT_TYPE_ROTOR {
        return rotor(light, light_minus_world);
    }
    if kind == LIGHT_TYPE_SHOCK {
        return shock(light, world);
    }
    if kind == LIGHT_TYPE_DISCO {
        return disco(light, light_minus_world);
    }
    return 0.0;
}
