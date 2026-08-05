enable wgpu_ray_query;

const MAX_BOUNCE_COUNT: u32 = 4u;
const MAX_ALPHA_LAYERS: u32 = 8u;
const ENVIRONMENT_SAMPLE_COUNT: u32 = 16u;
const SOLARI_PI: f32 = 3.141592653589793;

#import bevy_solari::scene_bindings::{
    trace_ray,
    resolve_ray_hit_full,
    ResolvedMaterial,
    RAY_T_MIN,
    RAY_T_MAX,
}
#import bevy_solari::sampling::{power_heuristic}

// Group 0 is Solari's RaytracingSceneBindings. Group 1 is intentionally
// bevyout-owned so the offline texel/light contract stays independent of the
// realtime ReSTIR resources.
struct BakeTexel {
    position: vec4<f32>,
    normal: vec4<f32>,
}

struct BakeLight {
    position_and_range: vec4<f32>,
    color: vec4<f32>,
    direction_and_outer_cosine: vec4<f32>,
    inner_cosine_and_falloff: vec4<f32>,
}

struct BakeDirectionalLight {
    direction_and_illuminance: vec4<f32>,
    color: vec4<f32>,
}

struct BakeAlphaMaterial {
    data_offset_width_height_mode: vec4<u32>,
    base_alpha_cutoff_wrap: vec4<f32>,
    flags: vec4<u32>,
}

struct BakeVertexRecord {
    color_offset: u32,
    index_offset: u32,
    position_offset: u32,
    vertex_count: u32,
    index_count: u32,
}

struct BakeEmissiveTriangle {
    position0: vec4<f32>,
    position1: vec4<f32>,
    position2: vec4<f32>,
    emission_and_area: vec4<f32>,
    selection_cdf_probability_flags: vec4<f32>,
    identity: vec4<u32>,
}

@group(1) @binding(0) var<storage, read> texels: array<BakeTexel>;
@group(1) @binding(1) var<storage, read> lights: array<BakeLight>;
@group(1) @binding(2) var<storage, read_write> output: array<vec4<f32>>;
@group(1) @binding(3) var<storage, read> params: array<u32>;
@group(1) @binding(4) var<storage, read> directional_lights: array<BakeDirectionalLight>;
@group(1) @binding(5) var<storage, read> ambient: array<vec4<f32>>;
@group(1) @binding(6) var<storage, read> alpha_materials: array<BakeAlphaMaterial>;
@group(1) @binding(7) var<storage, read> alpha_texels: array<f32>;
@group(1) @binding(8) var<storage, read> vertex_records: array<BakeVertexRecord>;
@group(1) @binding(9) var<storage, read> vertex_colors: array<vec4<f32>>;
@group(1) @binding(10) var<storage, read> vertex_indices: array<u32>;
@group(1) @binding(11) var<storage, read> vertex_positions: array<vec4<f32>>;
@group(1) @binding(12) var<storage, read> environment_texels: array<vec4<f32>>;
@group(1) @binding(13) var<storage, read> environment_cdf: array<f32>;
@group(1) @binding(14) var<storage, read> emissive_triangles: array<BakeEmissiveTriangle>;

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let texel_index = global_id.x;
    let texel_count = params[0];
    if texel_index >= texel_count {
        return;
    }

    let texel = texels[texel_index];
    let normal = normalize(texel.normal.xyz);
    let bounce_count = min(params[3], MAX_BOUNCE_COUNT);
    var irradiance = direct_irradiance(texel.position.xyz, normal, bounce_count > 0u);
    if bounce_count > 0u {
        let sample_count = max(params[2], 1u);
        let seed = params[4] ^ bitcast<u32>(texel.normal.w);
        for (var sample_index = 0u; sample_index < sample_count; sample_index += 1u) {
            var path_position = texel.position.xyz + normal * RAY_T_MIN;
            var path_normal = normal;
            var path_throughput = vec3<f32>(1.0, 1.0, 1.0);
            for (var bounce_index = 0u; bounce_index < bounce_count; bounce_index += 1u) {
                let direction = cosine_hemisphere_direction(
                    path_normal,
                    hash_u32(seed ^ (sample_index * 0x9e3779b9u) ^ bounce_index),
                    sample_index,
                    sample_count,
                );
                var ray_origin = path_position;
                var remaining = RAY_T_MAX;
                var found_hit = false;
                for (var layer = 0u; layer < MAX_ALPHA_LAYERS; layer += 1u) {
                    if remaining <= RAY_T_MIN {
                        break;
                    }
                    let ray = trace_ray(
                        ray_origin,
                        direction,
                        RAY_T_MIN,
                        remaining,
                        RAY_FLAG_NONE,
                    );
                    if ray.kind == RAY_QUERY_INTERSECTION_NONE {
                        // direct_irradiance already evaluates the authored
                        // environment at the current surface. Do not add an
                        // escaped environment sample a second time here.
                        break;
                    }
                    let hit = resolve_ray_hit_full(ray);
                    let side_table_id = side_table_index(hit.material);
                    if !hit_is_surface_usable(
                        side_table_id,
                        ray.primitive_index,
                        direction,
                    ) {
                        let advance = max(ray.t + RAY_T_MIN, RAY_T_MIN);
                        if advance >= remaining {
                            break;
                        }
                        ray_origin += direction * advance;
                        remaining -= advance;
                        continue;
                    }
                    let vertex_color = vertex_color_for_hit(
                        side_table_id,
                        ray.primitive_index,
                        ray.barycentrics,
                    );
                    let opacity = hit_opacity(side_table_id, hit.uv, vertex_color.w);
                    if opacity > 0.0 {
                        let hit_normal = hit_surface_normal(
                            side_table_id,
                            ray.primitive_index,
                            direction,
                            hit.world_normal,
                        );
                        let hit_diffuse = hit.material.base_color
                            * vertex_color.xyz
                            * (1.0 - clamp(hit.material.metallic, 0.0, 1.0));
                        let hit_direct = direct_irradiance(hit.world_position, hit_normal, true);
                        let emission_weight = hit_emission_weight(
                            side_table_id,
                            ray.primitive_index,
                            hit.world_position,
                            path_position,
                            direction,
                            max(dot(path_normal, direction), 0.0) / SOLARI_PI,
                            hit.material.emissive,
                        );
                        irradiance += hit_direct
                            * path_throughput
                            * hit_diffuse
                            * opacity
                            / f32(sample_count);
                        if max(
                            hit.material.emissive.x,
                            max(hit.material.emissive.y, hit.material.emissive.z),
                        ) > 0.0 {
                            irradiance += SOLARI_PI
                                * path_throughput
                                * hit.material.emissive
                                * opacity
                                * emission_weight
                                / f32(sample_count);
                        }
                        if hit_is_blended(side_table_id) {
                            path_throughput *= 1.0 - opacity;
                            let advance = max(ray.t + RAY_T_MIN, RAY_T_MIN);
                            if advance >= remaining {
                                break;
                            }
                            ray_origin += direction * advance;
                            remaining -= advance;
                            if max(
                                path_throughput.x,
                                max(path_throughput.y, path_throughput.z),
                            ) <= 0.001 {
                                break;
                            }
                            continue;
                        }
                        path_throughput *= hit_diffuse;
                        path_position = hit.world_position + hit_normal * RAY_T_MIN;
                        path_normal = hit_normal;
                        found_hit = true;
                        break;
                    }
                    let advance = max(ray.t + RAY_T_MIN, RAY_T_MIN);
                    if advance >= remaining {
                        break;
                    }
                    ray_origin += direction * advance;
                    remaining -= advance;
                }
                if !found_hit || max(path_throughput.x, max(path_throughput.y, path_throughput.z)) <= 0.0 {
                    break;
                }
            }
        }
    }
    output[texel_index] = vec4<f32>(irradiance, 1.0);
}

fn direct_irradiance(
    position: vec3<f32>,
    normal: vec3<f32>,
    include_emissive: bool,
) -> vec3<f32> {
    var irradiance = ambient[0].xyz + environment_irradiance(position, normal);
    let light_count = params[1];
    for (var light_index = 0u; light_index < light_count; light_index += 1u) {
        let light = lights[light_index];
        let to_light = light.position_and_range.xyz - position;
        let distance_squared = dot(to_light, to_light);
        let range = light.position_and_range.w;
        if (distance_squared <= 0.000001 || distance_squared >= range * range) {
            continue;
        }
        let distance = sqrt(distance_squared);
        let direction = to_light / distance;
        let cosine = max(dot(normal, direction), 0.0);
        if (cosine <= 0.0) {
            continue;
        }

        var angular_factor = 1.0;
        let outer_cosine = light.direction_and_outer_cosine.w;
        if (outer_cosine >= 0.0) {
            let cone_cosine = dot(
                normalize(light.direction_and_outer_cosine.xyz),
                -direction,
            );
            if (cone_cosine <= outer_cosine) {
                continue;
            }
            let inner_cosine = light.inner_cosine_and_falloff.x;
            let blend = clamp((cone_cosine - outer_cosine) / (inner_cosine - outer_cosine), 0.0, 1.0);
            let exponent = light.inner_cosine_and_falloff.y;
            angular_factor = select(1.0, pow(blend, exponent), exponent > 0.0);
        }

        let visibility = ray_visibility(
            position + normal * RAY_T_MIN,
            direction,
            min(distance - RAY_T_MIN, RAY_T_MAX),
        );
        if visibility <= 0.0 {
            continue;
        }

        let range_factor = distance_squared / max(range * range, 0.0001);
        let range_smooth = pow(max(1.0 - range_factor * range_factor, 0.0), 2.0);
        irradiance += light.color.xyz * (range_smooth / max(distance_squared, 0.0001)) * cosine
            * angular_factor
            * visibility;
    }
    let directional = directional_lights[0];
    let directional_direction = normalize(directional.direction_and_illuminance.xyz);
    let directional_cosine = max(dot(normal, directional_direction), 0.0);
    if directional_cosine > 0.0 && directional.direction_and_illuminance.w > 0.0 {
        let visibility = ray_visibility(
            position + normal * RAY_T_MIN,
            directional_direction,
            RAY_T_MAX,
        );
        if visibility > 0.0 {
            irradiance += directional.color.xyz
                * directional.direction_and_illuminance.w
                * directional_cosine
                * visibility;
        }
    }
    if include_emissive && params[11] > 0u {
        irradiance += emissive_irradiance(position, normal);
    }
    return irradiance;
}

fn emissive_irradiance(position: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    let count = params[11];
    let total_weight = bitcast<f32>(params[12]);
    if count == 0u || total_weight <= 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let seed = params[4]
        ^ bitcast<u32>(position.x)
        ^ bitcast<u32>(position.y)
        ^ bitcast<u32>(position.z);
    let cdf_target = unit_float(hash_u32(seed ^ 0x63d835f1u)) * total_weight;
    var low = 0u;
    var high = count;
    for (var iteration = 0u; iteration < 32u; iteration += 1u) {
        if low >= high {
            break;
        }
        let middle = low + (high - low) / 2u;
        if emissive_triangles[middle].selection_cdf_probability_flags.x > cdf_target {
            high = middle;
        } else {
            low = middle + 1u;
        }
    }
    let triangle = emissive_triangles[min(low, count - 1u)];
    let square_root = sqrt(unit_float(hash_u32(seed ^ 0x9e3779b9u)));
    let secondary = unit_float(hash_u32(seed ^ 0xd1b54a32u));
    let barycentric = vec3<f32>(
        1.0 - square_root,
        square_root * (1.0 - secondary),
        square_root * secondary,
    );
    let emitter_position = triangle.position0.xyz * barycentric.x
        + triangle.position1.xyz * barycentric.y
        + triangle.position2.xyz * barycentric.z;
    let emitter_normal = normalize(cross(
        triangle.position1.xyz - triangle.position0.xyz,
        triangle.position2.xyz - triangle.position0.xyz,
    ));
    let to_emitter = emitter_position - position;
    let distance_squared = dot(to_emitter, to_emitter);
    if distance_squared <= 1.0e-6 || dot(emitter_normal, emitter_normal) <= 1.0e-8 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let distance = sqrt(distance_squared);
    let direction = to_emitter / distance;
    let receiver_cosine = max(dot(normal, direction), 0.0);
    var emitter_cosine = max(dot(emitter_normal, -direction), 0.0);
    if triangle.selection_cdf_probability_flags.z > 0.5 {
        emitter_cosine = abs(dot(emitter_normal, -direction));
    }
    let area = triangle.emission_and_area.w;
    let selection_probability = triangle.selection_cdf_probability_flags.y;
    let pdf = selection_probability * distance_squared / max(area * emitter_cosine, 1.0e-20);
    if receiver_cosine <= 0.0 || emitter_cosine <= 0.0 || pdf <= 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let visibility = ray_visibility(
        position + normal * RAY_T_MIN,
        direction,
        min(distance - RAY_T_MIN, RAY_T_MAX),
    );
    if visibility <= 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let bsdf_pdf = receiver_cosine / SOLARI_PI;
    return triangle.emission_and_area.xyz
        * receiver_cosine
        * visibility
        / pdf
        * power_heuristic(pdf, bsdf_pdf);
}

fn hit_emission_weight(
    side_table_id: u32,
    primitive_index: u32,
    hit_position: vec3<f32>,
    scattering_origin: vec3<f32>,
    ray_direction: vec3<f32>,
    bsdf_pdf: f32,
    emission: vec3<f32>,
) -> f32 {
    if max(emission.x, max(emission.y, emission.z)) <= 0.0 {
        return 1.0;
    }
    let count = params[11];
    for (var index = 0u; index < count; index += 1u) {
        let triangle = emissive_triangles[index];
        if triangle.identity.x != side_table_id || triangle.identity.y != primitive_index {
            continue;
        }
        let emitter_normal = normalize(cross(
            triangle.position1.xyz - triangle.position0.xyz,
            triangle.position2.xyz - triangle.position0.xyz,
        ));
        var emitter_cosine = max(dot(emitter_normal, -ray_direction), 0.0);
        if triangle.selection_cdf_probability_flags.z > 0.5 {
            emitter_cosine = abs(dot(emitter_normal, -ray_direction));
        }
        let distance_squared = dot(
            hit_position - scattering_origin,
            hit_position - scattering_origin,
        );
        let area = triangle.emission_and_area.w;
        let pdf = triangle.selection_cdf_probability_flags.y
            * distance_squared
            / max(area * emitter_cosine, 1.0e-20);
        if pdf > 0.0 {
            return power_heuristic(bsdf_pdf, pdf);
        }
        break;
    }
    return 1.0;
}

fn environment_texel(x: i32, y: i32) -> vec3<f32> {
    let width = i32(params[5]);
    let height = i32(params[6]);
    let wrapped_x = ((x % width) + width) % width;
    let clamped_y = clamp(y, 0, height - 1);
    return environment_texels[u32(clamped_y * width + wrapped_x)].xyz;
}

struct EnvironmentImportanceSample {
    direction: vec3<f32>,
    radiance: vec3<f32>,
    pdf: f32,
}

fn environment_importance_total() -> f32 {
    return environment_cdf[params[8] - 1u];
}

fn environment_pixel_solid_angle(y: u32) -> f32 {
    let height = f32(params[6]);
    let theta0 = SOLARI_PI * f32(y) / height;
    let theta1 = SOLARI_PI * f32(y + 1u) / height;
    return (2.0 * SOLARI_PI / f32(params[5])) * (cos(theta0) - cos(theta1));
}

fn environment_importance_weight(x: u32, y: u32) -> f32 {
    let pixel = environment_texels[y * params[5] + x].xyz;
    let luminance = dot(pixel, vec3<f32>(0.2126, 0.7152, 0.0722));
    return max(luminance, 0.0) * environment_pixel_solid_angle(y);
}

fn environment_importance_pdf(incoming_direction: vec3<f32>) -> f32 {
    let total = environment_importance_total();
    if total <= 0.0 {
        return 1.0 / (4.0 * SOLARI_PI);
    }
    let direction = normalize(incoming_direction);
    let raw_u = 0.5 + atan2(direction.z, direction.x) / (2.0 * SOLARI_PI);
    let u = raw_u - floor(raw_u);
    let v = acos(clamp(direction.y, -1.0, 1.0)) / SOLARI_PI;
    let x = min(u32(floor(u * f32(params[5]))), params[5] - 1u);
    let y = min(u32(floor(v * f32(params[6]))), params[6] - 1u);
    let solid_angle = environment_pixel_solid_angle(y);
    let probability = environment_importance_weight(x, y) / total;
    if probability <= 0.0 || solid_angle <= 0.0 {
        return 0.0;
    }
    return probability / solid_angle;
}

fn unit_float(value: u32) -> f32 {
    return f32(value) * 2.3283064365386963e-10;
}

fn sample_environment_importance(u0: f32, u1: f32) -> EnvironmentImportanceSample {
    let total = environment_importance_total();
    if total <= 0.0 {
        let y = 1.0 - 2.0 * clamp(u0, 0.0, 1.0);
        let phi = 2.0 * SOLARI_PI * (u1 - floor(u1));
        let radius = sqrt(max(1.0 - y * y, 0.0));
        let direction = vec3<f32>(radius * cos(phi), y, radius * sin(phi));
        return EnvironmentImportanceSample(
            direction,
            environment_radiance(direction),
            1.0 / (4.0 * SOLARI_PI),
        );
    }
    let cdf_target = clamp(u0, 0.0, 1.0 - 1.0e-7) * total;
    let count = params[8];
    var low = 0u;
    var high = count;
    for (var iteration = 0u; iteration < 32u; iteration += 1u) {
        if low >= high {
            break;
        }
        let middle = low + (high - low) / 2u;
        if environment_cdf[middle] > cdf_target {
            high = middle;
        } else {
            low = middle + 1u;
        }
    }
    let index = min(low, count - 1u);
    var previous_cdf = 0.0;
    if index > 0u {
        previous_cdf = environment_cdf[index - 1u];
    }
    let interval = max(environment_cdf[index] - previous_cdf, 1.0e-20);
    let cdf_residual = clamp((cdf_target - previous_cdf) / interval, 0.0, 1.0 - 1.0e-7);
    let x = index % params[5];
    let y = index / params[5];
    let u = (f32(x) + (u1 - floor(u1))) / f32(params[5]);
    let theta0 = SOLARI_PI * f32(y) / f32(params[6]);
    let theta1 = SOLARI_PI * f32(y + 1u) / f32(params[6]);
    let cos_theta = mix(cos(theta0), cos(theta1), cdf_residual);
    let theta = acos(clamp(cos_theta, -1.0, 1.0));
    let phi = (u - 0.5) * 2.0 * SOLARI_PI;
    let radius = sin(theta);
    let direction = vec3<f32>(radius * cos(phi), cos(theta), radius * sin(phi));
    return EnvironmentImportanceSample(
        direction,
        environment_radiance(direction),
        environment_importance_weight(x, y) / total / environment_pixel_solid_angle(y),
    );
}

fn environment_radiance(incoming_direction: vec3<f32>) -> vec3<f32> {
    if params[7] == 0u {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let direction = normalize(incoming_direction);
    let raw_u = 0.5 + atan2(direction.z, direction.x) / (2.0 * SOLARI_PI);
    let u = raw_u - floor(raw_u);
    let v = acos(clamp(direction.y, -1.0, 1.0)) / SOLARI_PI;
    let x = u * f32(params[5]) - 0.5;
    let y = v * f32(params[6]) - 0.5;
    let x0 = i32(floor(x));
    let y0 = i32(floor(y));
    let x1 = x0 + 1;
    let y1 = y0 + 1;
    let tx = x - f32(x0);
    let ty = y - f32(y0);
    let top = mix(environment_texel(x0, y0), environment_texel(x1, y0), tx);
    let bottom = mix(environment_texel(x0, y1), environment_texel(x1, y1), tx);
    return mix(top, bottom, ty);
}

fn environment_irradiance(position: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    if params[7] == 0u {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let seed = params[4]
        ^ bitcast<u32>(position.x)
        ^ bitcast<u32>(position.y)
        ^ bitcast<u32>(position.z);
    var cosine_sum = vec3<f32>(0.0, 0.0, 0.0);
    var environment_sum = vec3<f32>(0.0, 0.0, 0.0);
    for (var sample_index = 0u; sample_index < ENVIRONMENT_SAMPLE_COUNT; sample_index += 1u) {
        let random_u = unit_float(hash_u32(seed ^ 0x4f1b2d39u ^ sample_index));
        let random_v = unit_float(hash_u32(seed ^ 0xb7e15a94u ^ sample_index));
        let direction = cosine_hemisphere_direction(
            normal,
            hash_u32(seed ^ sample_index),
            sample_index,
            ENVIRONMENT_SAMPLE_COUNT,
        );
        let cosine = max(dot(normal, direction), 0.0);
        let pdf = cosine / SOLARI_PI;
        let cosine_visibility = ray_visibility(position + normal * RAY_T_MIN, direction, RAY_T_MAX);
        if pdf > 0.0 && cosine_visibility > 0.0 {
            if params[9] != 0u {
                cosine_sum += environment_radiance(direction) * (cosine * cosine_visibility / pdf);
            } else {
                let environment_pdf = environment_importance_pdf(direction);
                let weight = power_heuristic(pdf, environment_pdf);
                cosine_sum += environment_radiance(direction) * (cosine * cosine_visibility * weight / pdf);
            }
        }
        if params[9] != 0u {
            continue;
        }
        let environment_sample = sample_environment_importance(random_u, random_v);
        let environment_cosine = max(dot(normal, environment_sample.direction), 0.0);
        let environment_visibility = ray_visibility(
            position + normal * RAY_T_MIN,
            environment_sample.direction,
            RAY_T_MAX,
        );
        if environment_cosine > 0.0 && environment_sample.pdf > 0.0 && environment_visibility > 0.0
        {
            let cosine_pdf = environment_cosine / SOLARI_PI;
            let weight = power_heuristic(environment_sample.pdf, cosine_pdf);
            environment_sum += environment_sample.radiance
                * (environment_cosine * environment_visibility * weight / environment_sample.pdf);
        }
    }
    return (cosine_sum + environment_sum) / f32(ENVIRONMENT_SAMPLE_COUNT);
}

fn wrap_alpha_coordinate(value: f32, mode: u32) -> f32 {
    if mode == 0u {
        return clamp(value, 0.0, 1.0);
    }
    if mode == 1u {
        return value - floor(value);
    }
    let repeated = value - floor(value / 2.0) * 2.0;
    return select(repeated, 2.0 - repeated, repeated > 1.0);
}

fn sample_alpha(material: BakeAlphaMaterial, uv: vec2<f32>) -> f32 {
    let data = material.data_offset_width_height_mode;
    if data.y == 0u || data.z == 0u {
        return material.base_alpha_cutoff_wrap.x;
    }
    let u = wrap_alpha_coordinate(uv.x, u32(material.base_alpha_cutoff_wrap.z));
    let v = wrap_alpha_coordinate(uv.y, u32(material.base_alpha_cutoff_wrap.w));
    let x = u * f32(data.y - 1u);
    let y = v * f32(data.z - 1u);
    let x0 = u32(floor(x));
    let y0 = u32(floor(y));
    let x1 = min(x0 + 1u, data.y - 1u);
    let y1 = min(y0 + 1u, data.z - 1u);
    let tx = x - f32(x0);
    let ty = y - f32(y0);
    let top = mix(
        alpha_texel(data, x0, y0),
        alpha_texel(data, x1, y0),
        tx,
    );
    let bottom = mix(
        alpha_texel(data, x0, y1),
        alpha_texel(data, x1, y1),
        tx,
    );
    return mix(top, bottom, ty) * material.base_alpha_cutoff_wrap.x;
}

fn alpha_texel(data: vec4<u32>, x: u32, y: u32) -> f32 {
    return alpha_texels[data.x + y * data.y + x];
}

fn side_table_index(material: ResolvedMaterial) -> u32 {
    return u32(max(material.reflectance, 0.0) + 0.5);
}

fn geometric_normal_for_hit(side_table_id: u32, primitive_index: u32) -> vec3<f32> {
    if side_table_id >= arrayLength(&vertex_records) {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let record = vertex_records[side_table_id];
    let triangle_offset = primitive_index * 3u;
    if triangle_offset + 2u >= record.index_count {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let index_base = record.index_offset + triangle_offset;
    let i0 = vertex_indices[index_base];
    let i1 = vertex_indices[index_base + 1u];
    let i2 = vertex_indices[index_base + 2u];
    if i0 >= record.vertex_count || i1 >= record.vertex_count || i2 >= record.vertex_count {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let p0 = vertex_positions[record.position_offset + i0].xyz;
    let p1 = vertex_positions[record.position_offset + i1].xyz;
    let p2 = vertex_positions[record.position_offset + i2].xyz;
    return normalize(cross(p1 - p0, p2 - p0));
}

fn hit_is_surface_usable(
    side_table_id: u32,
    primitive_index: u32,
    ray_direction: vec3<f32>,
) -> bool {
    let geometric_normal = geometric_normal_for_hit(side_table_id, primitive_index);
    if dot(geometric_normal, geometric_normal) <= 1.0e-8 {
        return true;
    }
    if side_table_id >= arrayLength(&alpha_materials) {
        return true;
    }
    let double_sided = alpha_materials[side_table_id].flags.x != 0u;
    return double_sided || dot(geometric_normal, ray_direction) < 0.0;
}

fn hit_surface_normal(
    side_table_id: u32,
    primitive_index: u32,
    ray_direction: vec3<f32>,
    fallback: vec3<f32>,
) -> vec3<f32> {
    let geometric_normal = geometric_normal_for_hit(side_table_id, primitive_index);
    if dot(geometric_normal, geometric_normal) <= 1.0e-8 {
        return normalize(fallback);
    }
    if dot(geometric_normal, -ray_direction) < 0.0 {
        return -geometric_normal;
    }
    return geometric_normal;
}

fn hit_opacity(side_table_id: u32, uv: vec2<f32>, vertex_alpha: f32) -> f32 {
    if side_table_id >= arrayLength(&alpha_materials) {
        // A missing side-table entry is conservative: Solari's native opaque
        // material remains a blocker rather than becoming accidentally
        // transparent because the auxiliary table was not populated.
        return 1.0;
    }
    let material = alpha_materials[side_table_id];
    let mode = material.data_offset_width_height_mode.w;
    if mode == 0u {
        return 1.0;
    }
    let alpha = clamp(
        sample_alpha(material, uv) * vertex_alpha,
        0.0,
        1.0,
    );
    if mode == 1u {
        return select(0.0, 1.0, alpha >= material.base_alpha_cutoff_wrap.y);
    }
    return alpha;
}

fn hit_is_blended(side_table_id: u32) -> bool {
    return side_table_id < arrayLength(&alpha_materials)
        && alpha_materials[side_table_id].data_offset_width_height_mode.w == 2u;
}

fn ray_visibility(origin: vec3<f32>, direction: vec3<f32>, max_distance: f32) -> f32 {
    if max_distance <= RAY_T_MIN {
        return 1.0;
    }
    var ray_origin = origin;
    var remaining = max_distance;
    var visibility = 1.0;
    for (var layer = 0u; layer < MAX_ALPHA_LAYERS; layer += 1u) {
        if remaining <= RAY_T_MIN {
            return visibility;
        }
        let ray = trace_ray(
            ray_origin,
            direction,
            RAY_T_MIN,
            min(remaining, RAY_T_MAX),
            RAY_FLAG_NONE,
        );
        if ray.kind == RAY_QUERY_INTERSECTION_NONE {
            return visibility;
        }
        let hit = resolve_ray_hit_full(ray);
        let side_table_id = side_table_index(hit.material);
        if !hit_is_surface_usable(side_table_id, ray.primitive_index, direction) {
            let advance = max(ray.t + RAY_T_MIN, RAY_T_MIN);
            if advance >= remaining {
                return visibility;
            }
            ray_origin += direction * advance;
            remaining -= advance;
            continue;
        }
        let vertex_color = vertex_color_for_hit(
            side_table_id,
            ray.primitive_index,
            ray.barycentrics,
        );
        let opacity = hit_opacity(side_table_id, hit.uv, vertex_color.w);
        if opacity > 0.0 {
            visibility *= 1.0 - opacity;
            if visibility <= 0.001 {
                return 0.0;
            }
        }
        let advance = max(ray.t + RAY_T_MIN, RAY_T_MIN);
        if advance >= remaining {
            return visibility;
        }
        ray_origin += direction * advance;
        remaining -= advance;
    }
    return visibility;
}

fn vertex_color_for_hit(
    side_table_id: u32,
    primitive_index: u32,
    barycentrics: vec2<f32>,
) -> vec4<f32> {
    if side_table_id >= params[10] {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    let record = vertex_records[side_table_id];
    let triangle_offset = primitive_index * 3u;
    if triangle_offset + 2u >= record.index_count {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    let index_base = record.index_offset + triangle_offset;
    let i0 = vertex_indices[index_base];
    let i1 = vertex_indices[index_base + 1u];
    let i2 = vertex_indices[index_base + 2u];
    if i0 >= record.vertex_count || i1 >= record.vertex_count || i2 >= record.vertex_count {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    let c0 = vertex_colors[record.color_offset + i0];
    let c1 = vertex_colors[record.color_offset + i1];
    let c2 = vertex_colors[record.color_offset + i2];
    let weights = vec3<f32>(
        1.0 - barycentrics.x - barycentrics.y,
        barycentrics.x,
        barycentrics.y,
    );
    return c0 * weights.x + c1 * weights.y + c2 * weights.z;
}

fn cosine_hemisphere_direction(
    normal: vec3<f32>,
    seed: u32,
    sample_index: u32,
    sample_count: u32,
) -> vec3<f32> {
    let radial_jitter = unit_float(hash_u32(seed ^ 0x4f1b2d39u ^ sample_index));
    let u = (f32(sample_index) + radial_jitter) / f32(max(sample_count, 1u));
    let scramble = hash_u32(seed ^ 0xb7e15a94u);
    let v = f32(reverseBits(sample_index ^ scramble)) * 2.3283064e-10;
    let radius = sqrt(u);
    let angle = 6.283185307179586 * v;
    let local = vec3<f32>(radius * cos(angle), radius * sin(angle), sqrt(1.0 - u));
    let tangent = select(
        normalize(cross(normal, vec3<f32>(0.0, 0.0, 1.0))),
        normalize(cross(normal, vec3<f32>(0.0, 1.0, 0.0))),
        abs(normal.z) >= 0.999,
    );
    let bitangent = cross(normal, tangent);
    return normalize(tangent * local.x + bitangent * local.y + normal * local.z);
}

fn hash_u32(value: u32) -> u32 {
    var result = value;
    result ^= result >> 16u;
    result *= 0x7feb352du;
    result ^= result >> 15u;
    result *= 0x846ca68bu;
    return result ^ (result >> 16u);
}
