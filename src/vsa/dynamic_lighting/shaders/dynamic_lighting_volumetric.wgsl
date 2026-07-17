#import bevy_render::view::View
#import bevyout_dynamic_lighting::types::{DynamicLight, DynamicLightMeta}

@group(0) @binding(0) var source_hdr: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;
@group(0) @binding(2) var depth_texture: texture_depth_2d;
@group(0) @binding(3) var<uniform> view: View;
@group(0) @binding(4) var<storage, read> volumetric_lights: array<DynamicLight>;
@group(0) @binding(5) var<uniform> volumetric_meta: DynamicLightMeta;

const VOLUMETRIC_SPHERE: u32 = 1u;
const VOLUMETRIC_BOX: u32 = 2u;
const VOLUMETRIC_CONE_Z: u32 = 3u;
const VOLUMETRIC_CONE_Y: u32 = 4u;
const NO_HIT: f32 = -1.0;

fn screen_color(base: vec3<f32>, blend: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(1.0) - (vec3<f32>(1.0) - base) * (vec3<f32>(1.0) - blend);
}

fn nearest_point_on_finite_line(start: vec3<f32>, end: vec3<f32>, point: vec3<f32>) -> vec3<f32> {
    let line = end - start;
    let length_sqr = dot(line, line);
    if length_sqr <= 0.000001 {
        return start;
    }
    let distance = clamp(dot(point - start, line) / length_sqr, 0.0, 1.0);
    return start + line * distance;
}

fn ray_box_intersection(
    origin: vec3<f32>,
    direction: vec3<f32>,
    minimum: vec3<f32>,
    maximum: vec3<f32>,
    max_depth: f32,
) -> vec2<f32> {
    let safe_direction = select(
        direction,
        vec3<f32>(0.000001),
        abs(direction) < vec3<f32>(0.000001),
    );
    let inverse = vec3<f32>(1.0) / safe_direction;
    let first = (minimum - origin) * inverse;
    let second = (maximum - origin) * inverse;
    let near_values = min(first, second);
    let far_values = max(first, second);
    let near = max(0.0, max(near_values.x, max(near_values.y, near_values.z)));
    let far = min(max_depth, min(far_values.x, min(far_values.y, far_values.z)));
    if far < near {
        return vec2<f32>(NO_HIT);
    }
    return vec2<f32>(near, far);
}

fn ray_cone_intersection(
    origin: vec3<f32>,
    direction: vec3<f32>,
    tip: vec3<f32>,
    cone_direction: vec3<f32>,
    cone_angle: f32,
    cone_distance: f32,
    max_depth: f32,
) -> vec2<f32> {
    let w = origin - tip;
    let cosine = cos(cone_angle);
    let cosine_sqr = cosine * cosine;
    let sine_sqr = 1.0 - cosine_sqr;
    let direction_dot_axis = dot(direction, cone_direction);
    let w_dot_axis = dot(w, cone_direction);
    let direction_dot_w = dot(direction, w);
    let w_dot_w = dot(w, w);
    let a = direction_dot_axis * direction_dot_axis - cosine_sqr;
    let b = 2.0 * (direction_dot_axis * w_dot_axis - cosine_sqr * direction_dot_w);
    let c = w_dot_axis * w_dot_axis - cosine_sqr * w_dot_w;
    let discriminant = b * b - 4.0 * a * c;
    let cone_radius_sqr = w_dot_axis * w_dot_axis * sine_sqr / max(cosine_sqr, 0.000001);
    let radial_distance_sqr = w_dot_w - w_dot_axis * w_dot_axis;
    let inside = w_dot_axis >= 0.0 && w_dot_axis <= cone_distance
        && radial_distance_sqr <= cone_radius_sqr;

    var near = select(1e20, 0.0, inside);
    var far = select(-1e20, 0.0, inside);
    var hit = inside;
    if discriminant >= 0.0 && abs(a) > 0.000001 {
        let root = sqrt(discriminant);
        let t0 = (-b - root) / (2.0 * a);
        let t1 = (-b + root) / (2.0 * a);
        let height0 = w_dot_axis + t0 * direction_dot_axis;
        let height1 = w_dot_axis + t1 * direction_dot_axis;
        if t0 >= 0.0 && height0 >= 0.0 && height0 <= cone_distance {
            near = min(near, t0);
            far = max(far, t0);
            hit = true;
        }
        if t1 >= 0.0 && height1 >= 0.0 && height1 <= cone_distance {
            near = min(near, t1);
            far = max(far, t1);
            hit = true;
        }
    }
    let denominator = dot(direction, cone_direction);
    if abs(denominator) > 0.0001 {
        let cap_t = (cone_distance - w_dot_axis) / denominator;
        if cap_t >= 0.0 {
            let cap_point = w + cap_t * direction;
            let cap_height = w_dot_axis + cap_t * direction_dot_axis;
            let cap_radial_sqr = dot(cap_point, cap_point) - cap_height * cap_height;
            let cap_radius_sqr = cone_distance * cone_distance * sine_sqr
                / max(cosine_sqr, 0.000001);
            if cap_radial_sqr <= cap_radius_sqr {
                near = min(near, cap_t);
                far = max(far, cap_t);
                hit = true;
            }
        }
    }
    if !hit {
        return vec2<f32>(NO_HIT);
    }
    if inside {
        near = 0.0;
    }
    far = min(far, max_depth);
    if far < near {
        return vec2<f32>(NO_HIT);
    }
    return vec2<f32>(near, far);
}

fn shape_opacity(
    light: DynamicLight,
    camera_position: vec3<f32>,
    world_position: vec3<f32>,
) -> f32 {
    let radius = light.radius_sqr;
    if radius <= 0.0 {
        return 0.0;
    }
    var opacity = 0.0;
    if light.channel == VOLUMETRIC_SPHERE {
        let closest = nearest_point_on_finite_line(camera_position, world_position, light.position);
        let center_distance = distance(closest, light.position);
        if center_distance < radius + 0.00001 {
            opacity = (radius - center_distance) / radius;
        }
    } else if light.channel == VOLUMETRIC_BOX {
        let extent = radius * vec3<f32>(light.parameter_b, light.parameter_c, light.shimmer_scale);
        let ray = world_position - camera_position;
        let max_depth = length(ray);
        if max_depth > 0.000001 {
            let hit = ray_box_intersection(
                camera_position,
                ray / max_depth,
                light.position - extent,
                light.position + extent,
                max_depth,
            );
            if hit.x != NO_HIT {
                opacity = (hit.y - hit.x) / length(2.0 * extent);
            }
        }
    } else if light.channel == VOLUMETRIC_CONE_Z || light.channel == VOLUMETRIC_CONE_Y {
        let ray = world_position - camera_position;
        let max_depth = length(ray);
        if max_depth > 0.000001 {
            let hit = ray_cone_intersection(
                camera_position,
                ray / max_depth,
                light.position,
                normalize(light.forward),
                light.parameter_b,
                radius,
                max_depth,
            );
            if hit.x != NO_HIT {
                opacity = (hit.y - hit.x) / radius;
            }
        }
    }
    opacity = smoothstep(0.0, 1.0, opacity);
    opacity = clamp(opacity * light.parameter_a, 0.0, 1.0);
    opacity = min(opacity, distance(camera_position, world_position) * light.volumetric_visibility);
    return opacity * light.volumetric_intensity;
}

@fragment
fn fragment(@builtin(position) frag_position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(frag_position.xy);
    let source = textureLoad(source_hdr, pixel, 0);
    if volumetric_meta.enabled == 0u || volumetric_meta.count == 0u {
        return source;
    }
    let depth = textureLoad(depth_texture, pixel, 0);
    if depth <= 0.0 {
        return source;
    }
    let uv = (frag_position.xy - view.viewport.xy) / view.viewport.zw;
    let ndc = vec3<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), depth);
    let homogeneous_world = view.world_from_clip * vec4<f32>(ndc, 1.0);
    let world_position = homogeneous_world.xyz / homogeneous_world.w;

    var fog_color = vec3<f32>(0.0);
    var maximum_opacity = 0.0;
    for (var index = 0u; index < volumetric_meta.count; index += 1u) {
        let light = volumetric_lights[index];
        let opacity = shape_opacity(light, view.world_position, world_position);
        maximum_opacity = max(maximum_opacity, opacity);
        fog_color = screen_color(fog_color, light.color * opacity);
    }
    let screened = screen_color(fog_color, source.rgb);
    return vec4<f32>(mix(screened, fog_color, clamp(maximum_opacity, 0.0, 1.0)), source.a);
}
