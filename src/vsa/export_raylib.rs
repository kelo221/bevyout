use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use glam::{EulerRot, Mat4, Quat, Vec3};
use serde::Serialize;
use serde_json::Value;

use bevyout_core::lighting::{
    DEFAULT_AMBIENT_SCALE, DEFAULT_LIGHTING_SCALE, ambient_irradiance, point_light_intensity,
    srgb_to_linear_rgb,
};

use crate::cli::ExportRaylibArgs;

use super::assets::find_texture_ktx_tool;
use super::manifest::{
    CellInfo, PreparedCellLighting, PreparedLight, PreparedSceneManifest, PreparedSemantic,
};
use super::prepare::{STATIC_POINT_SHADOW_NEAR_Z, is_static_shadow_caster};
use super::scenes::resolve_cached_manifest;

const SCHEMA: u32 = 1;
const EYE_HEIGHT: f32 = 1.6;
const GLB_MAGIC: u32 = 0x4654_6C67;
const GLB_JSON_CHUNK: u32 = 0x4E4F_534A;

#[derive(Clone, Debug, Serialize)]
struct RaylibScene {
    schema: u32,
    shadow_resolution: u32,
    cell: RaylibCell,
    camera_spawn: RaylibCameraSpawn,
    models: Vec<RaylibModel>,
    instances: Vec<RaylibInstance>,
    materials: Vec<RaylibMaterial>,
    lights: Vec<RaylibLight>,
}

#[derive(Clone, Debug, Serialize)]
struct RaylibCell {
    editor_id: String,
    form_id: String,
    bounds_min: [f32; 3],
    bounds_max: [f32; 3],
    ambient_linear: [f32; 3],
    ambient_intensity: f32,
}

#[derive(Clone, Debug, Serialize)]
struct RaylibCameraSpawn {
    position: [f32; 3],
    yaw: f32,
    pitch: f32,
}

#[derive(Clone, Debug, Serialize)]
struct RaylibModel {
    id: u32,
    path: String,
}

#[derive(Clone, Debug, Serialize)]
struct RaylibInstance {
    model_id: u32,
    reference_form_id: String,
    semantic: String,
    world_from_model: [f32; 16],
    translation: [f32; 3],
    rotation_xyzw: [f32; 4],
    scale: f32,
    casts_shadow: bool,
    receives_shadow: bool,
    initially_enabled: bool,
}

#[derive(Clone, Debug, Serialize)]
struct RaylibMaterial {
    model_id: u32,
    material_index: u32,
    alpha_mode: String,
    alpha_cutoff: f32,
    unlit: bool,
    emissive_strength: f32,
    double_sided: bool,
    albedo: String,
    normal: String,
}

#[derive(Clone, Debug, Serialize)]
struct RaylibLight {
    reference_form_id: String,
    kind: String,
    enabled: bool,
    position: [f32; 3],
    direction: [f32; 3],
    color_linear: [f32; 3],
    intensity: f32,
    range: f32,
    inner_cone: f32,
    outer_cone: f32,
    casts_shadow: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ExportReport {
    placements_seen: usize,
    placements_exported: usize,
    actors_excluded: usize,
    unique_models: usize,
    lights_exported: usize,
    missing_models: Vec<String>,
    unsupported_materials: Vec<String>,
}

#[derive(Clone, Copy)]
struct BuildOptions<'a> {
    no_actors: bool,
    shadow_resolution: u32,
    available_models: Option<&'a BTreeSet<String>>,
}

pub fn export_raylib(args: ExportRaylibArgs) -> Result<()> {
    let cache_dir = args
        .cache_dir
        .unwrap_or_else(|| PathBuf::from(".bevyout/cache"));
    let manifest_path = resolve_cached_manifest(&cache_dir, &args.selector)?;
    let text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("could not read {}", manifest_path.display()))?;
    let manifest: PreparedSceneManifest =
        ron::de::from_str(&text).context("could not parse prepared scene manifest")?;

    let output = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!(".bevyout/raylib/{:08x}", manifest.cell.form_id)));
    fs::create_dir_all(output.join("models"))
        .with_context(|| format!("could not create {}", output.display()))?;
    fs::create_dir_all(output.join("textures"))?;

    let asset_root = fs::canonicalize(&manifest.asset_root)
        .with_context(|| format!("asset root does not exist: {}", manifest.asset_root))?;

    let mut missing_models = Vec::new();
    let mut unsupported_materials = Vec::new();
    let mut available = BTreeSet::new();
    let mut materials = Vec::new();
    let unique_paths = unique_asset_paths(&manifest, args.no_actors);
    for source_path in &unique_paths {
        let source = resolve_asset_path(&asset_root, source_path);
        if !source.is_file() {
            missing_models.push(source_path.clone());
            continue;
        }
        let dest_name = sanitize_model_filename(source_path);
        let dest = output.join("models").join(&dest_name);
        link_or_copy(&source, &dest)?;
        available.insert(source_path.clone());
    }
    for (id, source_path) in available.iter().enumerate() {
        let source = resolve_asset_path(&asset_root, source_path);
        match inspect_glb_materials(&source, id as u32, &mut unsupported_materials) {
            Ok(mut inspected) => materials.append(&mut inspected),
            Err(error) => unsupported_materials.push(format!("{source_path}: {error}")),
        }
    }
    missing_models.sort();
    missing_models.dedup();
    unsupported_materials.sort();
    unsupported_materials.dedup();
    materials.sort_by_key(|material| (material.model_id, material.material_index));
    rewrite_albedo_paths(
        &asset_root,
        &output,
        &mut materials,
        &mut unsupported_materials,
    )?;
    unsupported_materials.sort();
    unsupported_materials.dedup();

    let (mut scene, mut report) = build_export(
        &manifest,
        BuildOptions {
            no_actors: args.no_actors,
            shadow_resolution: args.shadow_resolution,
            available_models: Some(&available),
        },
    );
    scene.materials = materials;
    report.missing_models = missing_models;
    report.unsupported_materials = unsupported_materials;

    let scene_bytes = serde_json::to_vec_pretty(&scene)?;
    fs::write(output.join("scene.json"), &scene_bytes)?;
    fs::write(
        output.join("export-report.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;

    println!(
        "export-raylib {} -> {}",
        cell_name(&manifest.cell),
        output.display()
    );
    println!(
        "  placements {}/{}  models {}  lights {}  actors excluded {}  missing {}",
        report.placements_exported,
        report.placements_seen,
        report.unique_models,
        report.lights_exported,
        report.actors_excluded,
        report.missing_models.len()
    );
    Ok(())
}

fn build_export(
    manifest: &PreparedSceneManifest,
    options: BuildOptions<'_>,
) -> (RaylibScene, ExportReport) {
    let mut report = ExportReport {
        placements_seen: manifest.placements.len(),
        ..ExportReport::default()
    };

    let mut source_paths = BTreeSet::new();
    let mut kept = Vec::new();
    for placement in &manifest.placements {
        if options.no_actors && is_actor_semantic(&placement.semantic) {
            report.actors_excluded += 1;
            continue;
        }
        let Some(asset_path) = placement.asset_path.as_deref() else {
            continue;
        };
        if let Some(available) = options.available_models {
            if !available.contains(asset_path) {
                continue;
            }
        }
        let Some(semantic) = semantic_name(&placement.semantic) else {
            continue;
        };
        source_paths.insert(asset_path.to_string());
        kept.push((placement, semantic));
    }

    let models: Vec<RaylibModel> = source_paths
        .iter()
        .enumerate()
        .map(|(id, path)| RaylibModel {
            id: id as u32,
            path: format!("models/{}", sanitize_model_filename(path)),
        })
        .collect();
    let model_id_by_source: BTreeMap<&str, u32> = source_paths
        .iter()
        .enumerate()
        .map(|(id, path)| (path.as_str(), id as u32))
        .collect();

    let mut instances: Vec<RaylibInstance> = kept
        .into_iter()
        .map(|(placement, semantic)| {
            let asset_path = placement
                .asset_path
                .as_deref()
                .expect("kept placement has asset");
            let model_id = *model_id_by_source
                .get(asset_path)
                .expect("model id assigned");
            let world_from_model = Mat4::from_scale_rotation_translation(
                Vec3::splat(placement.scale),
                Quat::from_array(placement.rotation_xyzw),
                Vec3::from_array(placement.translation),
            )
            .to_cols_array();
            RaylibInstance {
                model_id,
                reference_form_id: form_id_hex(placement.reference_form_id),
                semantic: semantic.to_string(),
                world_from_model,
                translation: placement.translation,
                rotation_xyzw: placement.rotation_xyzw,
                scale: placement.scale,
                casts_shadow: is_static_shadow_caster(placement),
                receives_shadow: true,
                initially_enabled: placement.initially_enabled,
            }
        })
        .collect();
    instances.sort_by(|a, b| a.reference_form_id.cmp(&b.reference_form_id));
    report.placements_exported = instances.len();
    report.unique_models = models.len();

    let mut lights: Vec<RaylibLight> = manifest
        .lights
        .iter()
        .filter(|light| light.initially_enabled)
        .map(export_light)
        .collect();
    lights.sort_by(|a, b| a.reference_form_id.cmp(&b.reference_form_id));
    report.lights_exported = lights.len();

    let lighting = effective_lighting(&manifest.cell);
    let ambient_linear = ambient_irradiance(
        lighting.ambient_rgba,
        DEFAULT_LIGHTING_SCALE,
        DEFAULT_AMBIENT_SCALE,
    );
    let (bounds_min, bounds_max) = instance_bounds(&instances);
    let camera_spawn = camera_spawn(manifest, &instances);

    let scene = RaylibScene {
        schema: SCHEMA,
        shadow_resolution: options.shadow_resolution,
        cell: RaylibCell {
            editor_id: manifest
                .cell
                .editor_id
                .clone()
                .unwrap_or_else(|| form_id_hex(manifest.cell.form_id)),
            form_id: form_id_hex(manifest.cell.form_id),
            bounds_min,
            bounds_max,
            ambient_linear,
            ambient_intensity: 1.0,
        },
        camera_spawn,
        models,
        instances,
        materials: Vec::new(),
        lights,
    };
    (scene, report)
}

fn export_light(light: &PreparedLight) -> RaylibLight {
    let is_spot = prepared_light_is_spot(light);
    let (inner_cone, outer_cone) = if is_spot {
        prepared_spot_angles(light)
    } else {
        (0.0, 0.0)
    };
    let direction = Quat::from_array(light.rotation_xyzw) * -Vec3::Z;
    let casts_shadow = !is_spot
        && light.radius.is_finite()
        && light.radius > STATIC_POINT_SHADOW_NEAR_Z
        && (light.kind.is_empty() || light.kind.eq_ignore_ascii_case("point"))
        && light.flags & 0x200 == 0;
    RaylibLight {
        reference_form_id: form_id_hex(light.reference_form_id),
        kind: if is_spot { "spot" } else { "point" }.into(),
        enabled: light.initially_enabled,
        position: light.translation,
        direction: direction.to_array(),
        color_linear: srgb_to_linear_rgb([
            light.color_rgba[0],
            light.color_rgba[1],
            light.color_rgba[2],
        ]),
        intensity: point_light_intensity(
            light.radius,
            light.intensity_lumens,
            DEFAULT_LIGHTING_SCALE,
        ),
        range: light.radius,
        inner_cone,
        outer_cone,
        casts_shadow,
    }
}

fn prepared_light_is_spot(light: &PreparedLight) -> bool {
    let authored_spot = light.kind.eq_ignore_ascii_case("spot") || light.flags & 0x200 != 0;
    authored_spot && light.spot_fov_radians.is_finite() && light.spot_fov_radians > f32::EPSILON
}

fn prepared_spot_angles(light: &PreparedLight) -> (f32, f32) {
    let outer =
        (light.spot_fov_radians * 0.5).clamp(f32::EPSILON, std::f32::consts::FRAC_PI_2 - 0.0001);
    ((outer * 0.8).min(outer), outer)
}

fn effective_lighting(cell: &CellInfo) -> PreparedCellLighting {
    cell.effective_lighting
        .clone()
        .unwrap_or(PreparedCellLighting {
            ambient_rgba: cell.ambient_rgba,
            directional_rgba: cell.directional_rgba,
            ..PreparedCellLighting::default()
        })
}

fn scene_focus(manifest: &PreparedSceneManifest) -> Vec3 {
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut found = false;
    for placement in &manifest.placements {
        if placement.asset_path.is_none() {
            continue;
        }
        let position = Vec3::from_array(placement.translation);
        minimum = minimum.min(position);
        maximum = maximum.max(position);
        found = true;
    }
    if found {
        (minimum + maximum) * 0.5
    } else {
        Vec3::ZERO
    }
}

fn transition_camera_position(manifest: &PreparedSceneManifest) -> Option<Vec3> {
    manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
        .find_map(|placement| {
            matches!(
                &placement.semantic,
                PreparedSemantic::Door(door) if door.destination.is_some()
            )
            .then_some(Vec3::from_array(placement.translation) + Vec3::Y * EYE_HEIGHT)
        })
}

fn looking_at(eye: Vec3, focus: Vec3) -> Quat {
    if (focus - eye).length_squared() < 1e-10 {
        return Quat::IDENTITY;
    }
    Mat4::look_at_rh(eye, focus, Vec3::Y)
        .inverse()
        .to_scale_rotation_translation()
        .1
}

fn camera_spawn(
    manifest: &PreparedSceneManifest,
    instances: &[RaylibInstance],
) -> RaylibCameraSpawn {
    let focus = if instances.is_empty() {
        scene_focus(manifest)
    } else {
        let (min, max) = instance_bounds(instances);
        (Vec3::from_array(min) + Vec3::from_array(max)) * 0.5
    };
    let position =
        transition_camera_position(manifest).unwrap_or(focus + Vec3::new(0.0, 4.0, 12.0));
    let (yaw, pitch, _) = looking_at(position, focus).to_euler(EulerRot::YXZ);
    RaylibCameraSpawn {
        position: position.to_array(),
        yaw,
        pitch,
    }
}

fn instance_bounds(instances: &[RaylibInstance]) -> ([f32; 3], [f32; 3]) {
    if instances.is_empty() {
        return ([0.0; 3], [0.0; 3]);
    }
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for instance in instances {
        let position = Vec3::from_array(instance.translation);
        min = min.min(position);
        max = max.max(position);
    }
    (min.to_array(), max.to_array())
}

fn is_actor_semantic(semantic: &PreparedSemantic) -> bool {
    matches!(
        semantic,
        PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_) | PreparedSemantic::Corpse
    )
}

fn semantic_name(semantic: &PreparedSemantic) -> Option<&'static str> {
    Some(match semantic {
        PreparedSemantic::Static => "static",
        PreparedSemantic::Pickup(_) => "pickup",
        PreparedSemantic::Container => "container",
        PreparedSemantic::Door(_) => "door",
        PreparedSemantic::Activator => "activator",
        PreparedSemantic::Furniture => "furniture",
        PreparedSemantic::Npc(_) => "npc",
        PreparedSemantic::Creature(_) => "creature",
        PreparedSemantic::Corpse => "corpse",
        PreparedSemantic::Unsupported => return None,
    })
}

fn unique_asset_paths(manifest: &PreparedSceneManifest, no_actors: bool) -> Vec<String> {
    let paths: BTreeSet<String> = manifest
        .placements
        .iter()
        .filter(|placement| !(no_actors && is_actor_semantic(&placement.semantic)))
        .filter(|placement| semantic_name(&placement.semantic).is_some())
        .filter_map(|placement| placement.asset_path.clone())
        .collect();
    paths.into_iter().collect()
}

fn sanitize_model_filename(path: &str) -> String {
    path.replace('\\', "/")
        .to_ascii_lowercase()
        .replace('/', "_")
}

fn resolve_asset_path(asset_root: &Path, relative: &str) -> PathBuf {
    asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))
}

fn form_id_hex(form_id: u32) -> String {
    format!("{form_id:08x}")
}

fn cell_name(cell: &CellInfo) -> String {
    match cell.editor_id.as_deref() {
        Some(editor_id) => format!("{editor_id} ({:08x})", cell.form_id),
        None => form_id_hex(cell.form_id),
    }
}

fn link_or_copy(src: &Path, dst: &Path) -> Result<()> {
    if dst.exists() {
        return Ok(());
    }
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    match fs::hard_link(src, dst) {
        Ok(()) => Ok(()),
        Err(_) => {
            fs::copy(src, dst).with_context(|| {
                format!("could not copy {} -> {}", src.display(), dst.display())
            })?;
            Ok(())
        }
    }
}

fn inspect_glb_materials(
    path: &Path,
    model_id: u32,
    unsupported: &mut Vec<String>,
) -> Result<Vec<RaylibMaterial>> {
    let gltf = gltf::Gltf::open(path)
        .with_context(|| format!("could not parse GLB {}", path.display()))?;
    let json = parse_glb_json(path)?;
    let json_materials = json.get("materials").and_then(Value::as_array);
    let mut materials = Vec::new();
    for material in gltf.document.materials() {
        let Some(index) = material.index() else {
            continue;
        };
        let alpha_mode = match material.alpha_mode() {
            gltf::material::AlphaMode::Opaque => "opaque",
            gltf::material::AlphaMode::Mask => "mask",
            gltf::material::AlphaMode::Blend => "blend",
        };
        let mut unlit = false;
        let mut emissive_strength = 1.0;
        if let Some(json_material) = json_materials.and_then(|materials| materials.get(index)) {
            if let Some(extensions) = json_material.get("extensions").and_then(Value::as_object) {
                unlit = extensions.contains_key("KHR_materials_unlit");
                if let Some(strength) = extensions
                    .get("KHR_materials_emissive_strength")
                    .and_then(|value| value.get("emissiveStrength"))
                    .and_then(Value::as_f64)
                {
                    emissive_strength = strength as f32;
                }
                for name in ["KHR_materials_volume", "KHR_materials_specular"] {
                    if extensions.contains_key(name) {
                        unsupported.push(format!(
                            "{name} {} material {index}",
                            path.file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or("model")
                        ));
                    }
                }
            }
        }
        materials.push(RaylibMaterial {
            model_id,
            material_index: index as u32,
            alpha_mode: alpha_mode.into(),
            alpha_cutoff: material.alpha_cutoff().unwrap_or(0.5),
            unlit,
            emissive_strength,
            double_sided: material.double_sided(),
            albedo: material_texture_uri(&json, index, TextureSlot::Albedo),
            normal: material_texture_uri(&json, index, TextureSlot::Normal),
        });
    }
    Ok(materials)
}

enum TextureSlot {
    Albedo,
    Normal,
}

fn material_texture_uri(json: &Value, material_index: usize, slot: TextureSlot) -> String {
    let Some(materials) = json.get("materials").and_then(Value::as_array) else {
        return String::new();
    };
    let Some(material) = materials.get(material_index) else {
        return String::new();
    };
    let texture = match slot {
        TextureSlot::Albedo => material
            .get("pbrMetallicRoughness")
            .and_then(|pbr| pbr.get("baseColorTexture")),
        TextureSlot::Normal => material.get("normalTexture"),
    };
    let Some(texture_index) = texture
        .and_then(|texture| texture.get("index"))
        .and_then(Value::as_u64)
    else {
        return String::new();
    };
    let Some(textures) = json.get("textures").and_then(Value::as_array) else {
        return String::new();
    };
    let Some(source) = textures
        .get(texture_index as usize)
        .and_then(|texture| texture.get("source"))
        .and_then(Value::as_u64)
    else {
        return String::new();
    };
    json.get("images")
        .and_then(Value::as_array)
        .and_then(|images| images.get(source as usize))
        .and_then(|image| image.get("uri"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn rewrite_albedo_paths(
    asset_root: &Path,
    output: &Path,
    materials: &mut [RaylibMaterial],
    unsupported: &mut Vec<String>,
) -> Result<()> {
    let mut uris = BTreeSet::new();
    for material in materials.iter() {
        if !material.albedo.is_empty() {
            uris.insert(material.albedo.clone());
        }
        if !material.normal.is_empty() {
            uris.insert(material.normal.clone());
        }
    }
    let mut exported = BTreeMap::new();
    let tool = find_texture_ktx_tool()?;
    for uri in uris {
        match export_albedo_png(asset_root, output, &uri, &tool) {
            Ok(relative) => {
                exported.insert(uri, relative);
            }
            Err(error) => unsupported.push(format!("{uri}: {error}")),
        }
    }
    for material in materials {
        material.albedo = exported.get(&material.albedo).cloned().unwrap_or_default();
        material.normal = exported.get(&material.normal).cloned().unwrap_or_default();
    }
    Ok(())
}

fn export_albedo_png(asset_root: &Path, output: &Path, uri: &str, tool: &Path) -> Result<String> {
    let relative = sanitize_texture_filename(uri);
    let dest = output.join("textures").join(&relative);
    if !dest.exists() {
        let source = resolve_asset_path(asset_root, uri.trim_start_matches('/'));
        if !source.is_file() {
            bail!("missing {}", source.display());
        }
        extract_ktx2_png(tool, &source, &dest)?;
    }
    Ok(format!("textures/{relative}"))
}

fn extract_ktx2_png(tool: &Path, source: &Path, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    let command = Command::new(tool)
        .arg("extract")
        .arg("--transcode")
        .arg("rgba8")
        .arg("--level")
        .arg("0")
        .arg(source)
        .arg(dest)
        .output()
        .context("failed to start KTX-Software")?;
    if !command.status.success() {
        bail!(
            "ktx extract failed with {}:\n{}\n{}",
            command.status,
            String::from_utf8_lossy(&command.stdout).trim(),
            String::from_utf8_lossy(&command.stderr).trim()
        );
    }
    if !dest.is_file() {
        bail!("ktx extract did not write {}", dest.display());
    }
    Ok(())
}

fn sanitize_texture_filename(uri: &str) -> String {
    let stem = uri
        .trim_start_matches('/')
        .replace('\\', "/")
        .to_ascii_lowercase()
        .replace('/', "_");
    Path::new(&stem)
        .with_extension("png")
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("albedo.png")
        .to_string()
}

fn parse_glb_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path)?;
    if bytes.len() < 20 || u32::from_le_bytes(bytes[0..4].try_into()?) != GLB_MAGIC {
        bail!("invalid GLB header");
    }
    let length = u32::from_le_bytes(bytes[12..16].try_into()?) as usize;
    let kind = u32::from_le_bytes(bytes[16..20].try_into()?);
    if kind != GLB_JSON_CHUNK || 20 + length > bytes.len() {
        bail!("GLB has no valid JSON chunk");
    }
    Ok(serde_json::from_slice(&bytes[20..20 + length])?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vsa::manifest::{
        PreparedActor, PreparedDoor, PreparedDoorDestination, PreparedPlacement,
        PreparedRuntimeMutability,
    };

    fn placement(reference_form_id: u32, semantic: PreparedSemantic) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id,
            base_form_id: 0x0001_2345,
            asset_path: Some(format!("meshes/{reference_form_id:08x}.glb")),
            translation: [reference_form_id as f32, 0.0, 2.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: Default::default(),
            step_support: false,
            mutability: PreparedRuntimeMutability::Immutable,
            mutability_root_form_id: None,
            reference_kind: "REFR".into(),
            base_kind: "STAT".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic,
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            linked_reference_form_id: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    fn empty_cell(form_id: u32) -> CellInfo {
        CellInfo {
            form_id,
            editor_id: Some("SuperDuperMart".into()),
            name: None,
            interior: true,
            behave_like_exterior: false,
            ambient_rgba: [0.1, 0.2, 0.3, 1.0],
            directional_rgba: [0.0; 4],
            image_space_form_id: None,
            image_space: None,
            lighting_template_form_id: None,
            lighting_template_flags: 0,
            lighting_template: None,
            raw_lighting: None,
            effective_lighting: None,
            water_form_id: None,
            water_height: None,
            grid: None,
            worldspace_form_id: None,
            day_night_profile: None,
            day_night_preview_profile: None,
        }
    }

    fn minimal_manifest(
        placements: Vec<PreparedPlacement>,
        lights: Vec<PreparedLight>,
    ) -> PreparedSceneManifest {
        PreparedSceneManifest {
            schema_version: 13,
            prepare_revision: None,
            converter_revision: None,
            physics_schema_version: None,
            asset_root: ".".into(),
            source_plugin: "Fallout3.esm".into(),
            source_fingerprint: "content-hash".into(),
            item_catalog_path: None,
            item_catalog_revision: None,
            item_catalog_hash: None,
            recipe_catalog_path: None,
            recipe_catalog_revision: None,
            recipe_catalog_hash: None,
            actor_catalog_path: None,
            actor_catalog_revision: None,
            actor_catalog_hash: None,
            actor_animation_catalog_path: None,
            actor_animation_catalog_revision: None,
            actor_animation_catalog_hash: None,
            image_space_modifier_catalog_path: None,
            image_space_modifier_catalog_revision: None,
            image_space_modifier_catalog_hash: None,
            source_plugins: Vec::new(),
            visual_issues: Vec::new(),
            cell: empty_cell(0x0001_7f37),
            placements,
            lights,
            diagnostics: Vec::new(),
            navmeshes: Vec::new(),
            nav_graph: None,
            cell_audio: Default::default(),
            audio_clips: Vec::new(),
            footstep_sets: Vec::new(),
            hard_landing_clips: Vec::new(),
            bake: None,
            static_point_shadows: None,
            reflection_probes: None,
            mutability_summary: Default::default(),
            leveled_lists: Default::default(),
            dialogue: None,
            exterior: None,
        }
    }

    fn options(no_actors: bool) -> BuildOptions<'static> {
        BuildOptions {
            no_actors,
            shadow_resolution: 512,
            available_models: None,
        }
    }

    #[test]
    fn no_actors_drops_npc_creature_corpse_and_keeps_door() {
        let actor = PreparedActor {
            base_template_form_id: None,
            assembly: None,
        };
        let manifest = minimal_manifest(
            vec![
                placement(1, PreparedSemantic::Static),
                placement(2, PreparedSemantic::Npc(actor.clone())),
                placement(3, PreparedSemantic::Creature(actor)),
                placement(4, PreparedSemantic::Corpse),
                placement(
                    5,
                    PreparedSemantic::Door(PreparedDoor {
                        lock_level: None,
                        key_form_id: None,
                        trapped: false,
                        destination: Some(PreparedDoorDestination {
                            door_reference_form_id: 9,
                            cell_form_id: 0x0001_7f37,
                            translation: [0.0; 3],
                            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                        }),
                    }),
                ),
            ],
            Vec::new(),
        );
        let (scene, report) = build_export(&manifest, options(true));
        assert_eq!(report.actors_excluded, 3);
        assert_eq!(report.placements_exported, 2);
        let semantics: Vec<_> = scene
            .instances
            .iter()
            .map(|instance| instance.semantic.as_str())
            .collect();
        assert_eq!(semantics, ["static", "door"]);
        assert!(
            !scene.instances.iter().any(|instance| matches!(
                instance.semantic.as_str(),
                "npc" | "creature" | "corpse"
            ))
        );
    }

    #[test]
    fn lighting_matches_core_helpers() {
        let light = PreparedLight {
            reference_form_id: 0xabc,
            base_form_id: 1,
            translation: [1.0, 2.0, 3.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            color_rgba: [0.5, 0.25, 0.125, 1.0],
            radius: 8.0,
            intensity_lumens: 1_000.0,
            kind: "point".into(),
            flags: 0,
            spot_fov_radians: 0.0,
            spot_falloff_exponent: 0.0,
            initially_enabled: true,
        };
        let manifest = minimal_manifest(Vec::new(), vec![light.clone()]);
        let (scene, report) = build_export(&manifest, options(true));
        assert_eq!(report.lights_exported, 1);
        assert_eq!(
            scene.lights[0].intensity,
            point_light_intensity(8.0, 1_000.0, DEFAULT_LIGHTING_SCALE)
        );
        assert_eq!(
            scene.lights[0].color_linear,
            srgb_to_linear_rgb([0.5, 0.25, 0.125])
        );
        assert_eq!(
            scene.cell.ambient_linear,
            ambient_irradiance(
                [0.1, 0.2, 0.3, 1.0],
                DEFAULT_LIGHTING_SCALE,
                DEFAULT_AMBIENT_SCALE
            )
        );
        assert!(scene.lights[0].casts_shadow);
        assert_eq!(scene.shadow_resolution, 512);
    }

    #[test]
    fn reexport_is_byte_identical() {
        let manifest = minimal_manifest(
            vec![
                placement(10, PreparedSemantic::Static),
                placement(4, PreparedSemantic::Furniture),
            ],
            vec![PreparedLight {
                reference_form_id: 7,
                base_form_id: 8,
                translation: [0.0, 1.0, 0.0],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                color_rgba: [1.0, 1.0, 1.0, 1.0],
                radius: 4.0,
                intensity_lumens: 0.0,
                kind: String::new(),
                flags: 0,
                spot_fov_radians: 0.0,
                spot_falloff_exponent: 0.0,
                initially_enabled: true,
            }],
        );
        let (a, _) = build_export(&manifest, options(true));
        let (b, _) = build_export(&manifest, options(true));
        assert_eq!(
            serde_json::to_vec_pretty(&a).unwrap(),
            serde_json::to_vec_pretty(&b).unwrap()
        );
    }
}
