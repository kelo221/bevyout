#define_import_path bevyout_dynamic_lighting::common

fn to_light_space(value: vec3<f32>, forward: vec3<f32>, up: vec3<f32>) -> vec3<f32> {
    let right = cross(forward, up);
    return transpose(mat3x3<f32>(right, up, forward)) * value;
}

fn max3(value: vec3<f32>) -> f32 {
    return max(max(value.x, value.y), value.z);
}

fn snap_direction_round(value: f32) -> f32 {
    if abs(value) < tan(3.141592653589793 / 8.0) {
        return 0.0;
    }
    return sign(value);
}

fn snap_direction(input_value: vec3<f32>) -> vec3<f32> {
    let input = input_value / max3(abs(input_value));
    let rounded = vec3<f32>(
        snap_direction_round(input.x),
        snap_direction_round(input.y),
        snap_direction_round(input.z),
    );
    return normalize(rounded);
}
