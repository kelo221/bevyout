enable wgpu_ray_query;

const MAX_BOUNCE_COUNT: u32 = 4u;
const ENVIRONMENT_SAMPLE_COUNT: u32 = 16u;
const SOLARI_PI: f32 = 3.141592653589793;

#import bevy_solari::scene_bindings::{
    trace_ray,
    resolve_ray_hit_full,
    light_sources,
    RAY_T_MIN,
    RAY_T_MAX,
}
#import bevy_solari::sampling::{
    sample_random_light,
    random_emissive_light_pdf,
    power_heuristic,
}

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
}

struct BakeVertexRecord {
    color_offset: u32,
    index_offset: u32,
    vertex_count: u32,
    index_count: u32,
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
@group(1) @binding(11) var<storage, read> environment_texels: array<vec4<f32>>;
@group(1) @binding(12) var<storage, read> environment_cdf: array<f32>;

@compute @workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let texel_index = global_id.x;
    let texel_count = params[0];
    if texel_index >= texel_count {
        return;
    }

    let texel = texels[texel_index];
    let normal = normalize(texel.normal.xyz);
    var irradiance = direct_irradiance(texel.position.xyz, normal);
    let bounce_count = min(params[3], MAX_BOUNCE_COUNT);
    if bounce_count > 0u {
        let sample_count = max(params[2], 1u);
        let seed = params[4] ^ texel_index;
        irradiance += emissive_irradiance(
            texel.position.xyz + normal * RAY_T_MIN,
            normal,
            seed,
        );
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
                let bsdf_pdf = max(dot(path_normal, direction), 0.0) / SOLARI_PI;
                var ray_origin = path_position;
                var remaining = RAY_T_MAX;
                var found_hit = false;
                for (var layer = 0u; layer < 8u; layer += 1u) {
                    if remaining <= RAY_T_MIN {
                        break;
                    }
                    let ray = trace_ray(
                        ray_origin,
                        direction,
                        RAY_T_MIN,
                        remaining,
                        RAY_FLAG_TERMINATE_ON_FIRST_HIT,
                    );
                    if ray.kind == RAY_QUERY_INTERSECTION_NONE {
                        irradiance += SOLARI_PI * path_throughput
                            * environment_radiance(direction)
                            / f32(sample_count);
                        break;
                    }
                    let hit = resolve_ray_hit_full(ray);
                    let vertex_color = vertex_color_for_hit(
                        ray.instance_index,
                        ray.primitive_index,
                        ray.barycentrics,
                    );
                    if hit_blocks_ray(ray.instance_index, hit.uv, vertex_color.w) {
                        let hit_normal = normalize(hit.world_normal);
                        let hit_diffuse = hit.material.base_color
                            * srgb_to_linear_rgb(vertex_color.xyz)
                            * (1.0 - clamp(hit.material.metallic, 0.0, 1.0));
                        let hit_direct =
                            direct_irradiance(hit.world_position, hit_normal)
                                + emissive_irradiance(
                                    hit.world_position + hit_normal * RAY_T_MIN,
                                    hit_normal,
                                    seed ^ sample_index ^ bounce_index,
                                );
                        irradiance +=
                            hit_direct * path_throughput * hit_diffuse
                                / f32(sample_count);
                        if max(
                            hit.material.emissive.x,
                            max(hit.material.emissive.y, hit.material.emissive.z),
                        ) > 0.0 {
                            let light_pdf = random_emissive_light_pdf(hit);
                            irradiance += SOLARI_PI
                                * path_throughput
                                * hit.material.emissive
                                * power_heuristic(bsdf_pdf, light_pdf)
                                / f32(sample_count);
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

fn direct_irradiance(position: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
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

        if !ray_visible(
            position + normal * RAY_T_MIN,
            direction,
            min(distance - RAY_T_MIN, RAY_T_MAX),
        ) {
            continue;
        }

        let range_factor = distance_squared / max(range * range, 0.0001);
        let range_smooth = pow(max(1.0 - range_factor * range_factor, 0.0), 2.0);
        irradiance += light.color.xyz * (range_smooth / max(distance_squared, 0.0001)) * cosine
            * angular_factor;
    }
    let directional = directional_lights[0];
    let directional_direction = normalize(directional.direction_and_illuminance.xyz);
    let directional_cosine = max(dot(normal, directional_direction), 0.0);
    if directional_cosine > 0.0 && directional.direction_and_illuminance.w > 0.0 {
        if ray_visible(
            position + normal * RAY_T_MIN,
            directional_direction,
            RAY_T_MAX,
        ) {
            irradiance += directional.color.xyz
                * directional.direction_and_illuminance.w
                * directional_cosine;
        }
    }
    return irradiance;
}

fn emissive_irradiance(position: vec3<f32>, normal: vec3<f32>, seed: u32) -> vec3<f32> {
    if arrayLength(&light_sources) == 0u {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    var rng = seed;
    let sample = sample_random_light(position, normal, &rng);
    let cosine = max(dot(normal, sample.wi), 0.0);
    if cosine <= 0.0 || sample.inverse_pdf <= 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let light_pdf = 1.0 / sample.inverse_pdf;
    let bsdf_pdf = cosine / SOLARI_PI;
    return sample.radiance
        * sample.inverse_pdf
        * cosine
        * power_heuristic(light_pdf, bsdf_pdf);
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
    let theta = SOLARI_PI * (f32(y) + 0.5) / f32(params[6]);
    return max(luminance, 0.0) * max(sin(theta), 0.0);
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
    let x = index % params[5];
    let y = index / params[5];
    let u = (f32(x) + (u1 - floor(u1))) / f32(params[5]);
    let v = (f32(y) + (u0 - floor(u0))) / f32(params[6]);
    let theta = v * SOLARI_PI;
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
    if params[9] != 0u {
        return environment_texels[0].xyz * SOLARI_PI;
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
        if pdf > 0.0 && ray_visible(position + normal * RAY_T_MIN, direction, RAY_T_MAX) {
            let environment_pdf = environment_importance_pdf(direction);
            let weight = power_heuristic(pdf, environment_pdf);
            cosine_sum += environment_radiance(direction) * (cosine * weight / pdf);
        }
        let environment_sample = sample_environment_importance(random_u, random_v);
        let environment_cosine = max(dot(normal, environment_sample.direction), 0.0);
        if environment_cosine > 0.0
            && environment_sample.pdf > 0.0
            && ray_visible(
                position + normal * RAY_T_MIN,
                environment_sample.direction,
                RAY_T_MAX,
            )
        {
            let cosine_pdf = environment_cosine / SOLARI_PI;
            let weight = power_heuristic(environment_sample.pdf, cosine_pdf);
            environment_sum += environment_sample.radiance
                * (environment_cosine * weight / environment_sample.pdf);
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
    return mix(top, bottom, ty);
}

fn alpha_texel(data: vec4<u32>, x: u32, y: u32) -> f32 {
    return alpha_texels[data.x + y * data.y + x];
}

fn hit_blocks_ray(instance_index: u32, uv: vec2<f32>, vertex_alpha: f32) -> bool {
    if instance_index >= arrayLength(&alpha_materials) {
        // A missing side-table entry is conservative: Solari's native opaque
        // material remains a blocker rather than becoming accidentally
        // transparent because the auxiliary table was not populated.
        return true;
    }
    let material = alpha_materials[instance_index];
    let mode = material.data_offset_width_height_mode.w;
    if mode == 0u {
        return true;
    }
    return clamp(sample_alpha(material, uv) * vertex_alpha, 0.0, 1.0)
        >= material.base_alpha_cutoff_wrap.y;
}

fn ray_visible(origin: vec3<f32>, direction: vec3<f32>, max_distance: f32) -> bool {
    if max_distance <= RAY_T_MIN {
        return true;
    }
    var ray_origin = origin;
    var remaining = max_distance;
    for (var layer = 0u; layer < 8u; layer += 1u) {
        if remaining <= RAY_T_MIN {
            return true;
        }
        let ray = trace_ray(
            ray_origin,
            direction,
            RAY_T_MIN,
            min(remaining, RAY_T_MAX),
            RAY_FLAG_TERMINATE_ON_FIRST_HIT,
        );
        if ray.kind == RAY_QUERY_INTERSECTION_NONE {
            return true;
        }
        let hit = resolve_ray_hit_full(ray);
        let vertex_color = vertex_color_for_hit(
            ray.instance_index,
            ray.primitive_index,
            ray.barycentrics,
        );
        if hit_blocks_ray(ray.instance_index, hit.uv, vertex_color.w) {
            return false;
        }
        let advance = max(ray.t + RAY_T_MIN, RAY_T_MIN);
        if advance >= remaining {
            return true;
        }
        ray_origin += direction * advance;
        remaining -= advance;
    }
    return false;
}

fn srgb_channel_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        return value / 12.92;
    }
    return pow((value + 0.055) / 1.055, 2.4);
}

fn srgb_to_linear_rgb(value: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        srgb_channel_to_linear(value.x),
        srgb_channel_to_linear(value.y),
        srgb_channel_to_linear(value.z),
    );
}

fn vertex_color_for_hit(
    instance_index: u32,
    primitive_index: u32,
    barycentrics: vec2<f32>,
) -> vec4<f32> {
    if instance_index >= params[10] {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
    let record = vertex_records[instance_index];
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
    let u = (f32(sample_index) + 0.5) / f32(max(sample_count, 1u));
    let scramble = hash_u32(seed);
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
