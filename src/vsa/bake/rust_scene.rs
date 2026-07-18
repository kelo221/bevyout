use super::{
    JobPlacement,
    gltf_extension_policy::{material_extensions_used, unsupported_required_extensions},
};
use anyhow::{Context, Result, bail};
use bevy::math::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use gltf::mesh::util::{ReadColors, ReadTexCoords};
use image::RgbaImage;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const GLB_MAGIC: u32 = 0x4654_6c67;
const GLB_JSON_CHUNK: u32 = 0x4e4f_534a;
const GLB_BIN_CHUNK: u32 = 0x004e_4942;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AlphaMode {
    Opaque,
    Mask,
    Blend,
}

#[derive(Clone, Copy, Debug)]
enum WrapMode {
    Clamp,
    Mirror,
    Repeat,
}

#[derive(Clone, Debug)]
pub(crate) struct SampledTexture {
    image: Arc<RgbaImage>,
    wrap_s: WrapMode,
    wrap_t: WrapMode,
}

impl SampledTexture {
    pub(crate) fn sample(&self, uv: Vec2) -> Vec4 {
        let wrap = |value: f32, mode: WrapMode| match mode {
            WrapMode::Clamp => value.clamp(0.0, 1.0),
            WrapMode::Repeat => value.rem_euclid(1.0),
            WrapMode::Mirror => {
                let value = value.rem_euclid(2.0);
                if value > 1.0 { 2.0 - value } else { value }
            }
        };
        let u = wrap(uv.x, self.wrap_s);
        let v = wrap(uv.y, self.wrap_t);
        let width = self.image.width().max(1);
        let height = self.image.height().max(1);
        let x = u * (width.saturating_sub(1)) as f32;
        // glTF (0, 0) and decoded image row zero are both upper-left.
        let y = v * (height.saturating_sub(1)) as f32;
        let x0 = x.floor() as u32;
        let y0 = y.floor() as u32;
        let x1 = (x0 + 1).min(width - 1);
        let y1 = (y0 + 1).min(height - 1);
        let tx = x - x0 as f32;
        let ty = y - y0 as f32;
        let pixel = |px, py| {
            let value = self.image.get_pixel(px, py).0;
            Vec4::new(
                value[0] as f32 / 255.0,
                value[1] as f32 / 255.0,
                value[2] as f32 / 255.0,
                value[3] as f32 / 255.0,
            )
        };
        let top = pixel(x0, y0).lerp(pixel(x1, y0), tx);
        let bottom = pixel(x0, y1).lerp(pixel(x1, y1), tx);
        top.lerp(bottom, ty)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TransportMaterial {
    pub(crate) base_color_factor: Vec4,
    pub(crate) metallic_factor: f32,
    pub(crate) emissive_factor: Vec3,
    pub(crate) base_color_texture: Option<SampledTexture>,
    pub(crate) emissive_texture: Option<SampledTexture>,
    pub(crate) alpha_mode: AlphaMode,
    pub(crate) alpha_cutoff: f32,
    pub(crate) double_sided: bool,
}

impl Default for TransportMaterial {
    fn default() -> Self {
        Self {
            base_color_factor: Vec4::ONE,
            metallic_factor: 1.0,
            emissive_factor: Vec3::ZERO,
            base_color_texture: None,
            emissive_texture: None,
            alpha_mode: AlphaMode::Opaque,
            alpha_cutoff: 0.5,
            double_sided: false,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ComposedPrimitive {
    pub(crate) name: String,
    pub(crate) reference_form_ids: Vec<u32>,
    pub(crate) material: usize,
    pub(crate) positions: Vec<Vec3>,
    pub(crate) normals: Vec<Vec3>,
    pub(crate) uvs: Vec<Vec2>,
    pub(crate) colors: Vec<Vec4>,
    pub(crate) transport_colors: Vec<Vec4>,
    pub(crate) indices: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SceneBounds {
    pub(crate) minimum: Vec3,
    pub(crate) maximum: Vec3,
}

impl SceneBounds {
    pub(crate) fn center(self) -> Vec3 {
        (self.minimum + self.maximum) * 0.5
    }

    pub(crate) fn extent(self) -> Vec3 {
        self.maximum - self.minimum
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct BatchingStats {
    pub(crate) chunk_size_meters: f32,
    pub(crate) visual_objects_before: usize,
    pub(crate) visual_objects_after: usize,
    pub(crate) render_primitives_before: usize,
    pub(crate) render_primitives_after: usize,
    pub(crate) materials_before: usize,
    pub(crate) materials_after: usize,
    pub(crate) batches_created: usize,
    pub(crate) largest_batch: usize,
    pub(crate) excluded_large: usize,
    pub(crate) seam_edges_matched: usize,
    pub(crate) seam_vertices_adjusted: usize,
    pub(crate) seam_max_correction_meters: f32,
}

pub(crate) struct RustBakeScene {
    pub(crate) primitives: Vec<ComposedPrimitive>,
    pub(crate) materials: Vec<TransportMaterial>,
    pub(crate) bounds: SceneBounds,
    pub(crate) batching: BatchingStats,
    resources: OutputResources,
}

#[derive(Clone)]
struct SourcePrimitive {
    name: String,
    material: usize,
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Vec2>,
    colors: Vec<Vec4>,
    indices: Vec<u32>,
}

struct LoadedAsset {
    primitives: Vec<SourcePrimitive>,
}

struct AssetDocument {
    gltf: gltf::Gltf,
    json: Value,
    buffers: Vec<Vec<u8>>,
    image_bytes: Vec<Vec<u8>>,
}

#[derive(Default)]
struct OutputResources {
    binary: Vec<u8>,
    buffer_views: Vec<Value>,
    images: Vec<Value>,
    samplers: Vec<Value>,
    textures: Vec<Value>,
    materials: Vec<Value>,
    image_by_hash: HashMap<String, usize>,
    sampler_by_json: HashMap<String, usize>,
    texture_by_json: HashMap<String, usize>,
    material_by_json: HashMap<String, usize>,
    decoded_images: HashMap<String, Arc<RgbaImage>>,
    transport_materials: Vec<TransportMaterial>,
}

pub(crate) fn compose_scene(
    asset_root: &Path,
    placements: &[JobPlacement],
    chunk_size: f32,
) -> Result<RustBakeScene> {
    let mut resources = OutputResources::default();
    let mut asset_cache = HashMap::<String, LoadedAsset>::new();
    let mut fragments = Vec::new();
    let mut contribution = BTreeMap::<u32, usize>::new();
    let mut source_material_count = 0;

    let mut ordered = placements.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|placement| placement.reference_form_id);
    for placement in ordered {
        if !asset_cache.contains_key(&placement.asset_path) {
            let path = resolve_asset_path(asset_root, &placement.asset_path);
            let loaded = load_asset(&path, &mut resources)?;
            source_material_count += loaded
                .primitives
                .iter()
                .map(|primitive| primitive.material)
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            asset_cache.insert(placement.asset_path.clone(), loaded);
        }
        let asset = asset_cache
            .get(&placement.asset_path)
            .expect("asset was inserted");
        let transform = Mat4::from_scale_rotation_translation(
            Vec3::splat(placement.scale),
            Quat::from_array(placement.rotation_xyzw),
            Vec3::from_array(placement.translation),
        );
        let normal_transform = Mat3::from_mat4(transform).inverse().transpose();
        for primitive in &asset.primitives {
            let positions = primitive
                .positions
                .iter()
                .map(|position| transform.transform_point3(*position))
                .collect::<Vec<_>>();
            let normals = primitive
                .normals
                .iter()
                .map(|normal| (normal_transform * *normal).normalize_or_zero())
                .collect::<Vec<_>>();
            if positions.is_empty() || primitive.indices.len() < 3 {
                continue;
            }
            *contribution.entry(placement.reference_form_id).or_default() += 1;
            fragments.push(ComposedPrimitive {
                name: format!("{}_{:08x}", primitive.name, placement.reference_form_id),
                reference_form_ids: vec![placement.reference_form_id],
                material: primitive.material,
                positions,
                normals,
                uvs: primitive.uvs.clone(),
                colors: primitive.colors.clone(),
                transport_colors: if placement.ao_mode == "ao-quick-v1" {
                    vec![Vec4::ONE; primitive.colors.len()]
                } else {
                    primitive.colors.clone()
                },
                indices: primitive.indices.clone(),
            });
        }
    }

    let missing = placements
        .iter()
        .filter(|placement| !contribution.contains_key(&placement.reference_form_id))
        .map(|placement| format!("{:08x}", placement.reference_form_id))
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        bail!(
            "Rust scene composition produced no geometry for placements: {}",
            missing.join(", ")
        );
    }
    let seam_stitch = stitch_static_seams(&mut fragments);
    let visual_objects_before = fragments.len();
    let render_primitives_before = fragments.len();
    let materials_before = source_material_count;
    let (primitives, mut batching) = batch_fragments(fragments, chunk_size);
    batching.visual_objects_before = visual_objects_before;
    batching.render_primitives_before = render_primitives_before;
    batching.materials_before = materials_before;
    batching.materials_after = resources.materials.len();
    batching.seam_edges_matched = seam_stitch.edges_matched;
    batching.seam_vertices_adjusted = seam_stitch.vertices_adjusted;
    batching.seam_max_correction_meters = seam_stitch.max_correction_meters;
    let bounds = scene_bounds(&primitives)?;
    Ok(RustBakeScene {
        primitives,
        materials: resources.transport_materials.clone(),
        bounds,
        batching,
        resources,
    })
}

impl RustBakeScene {
    pub(crate) fn write_glb(&mut self, output: &Path) -> Result<()> {
        let mut accessors = Vec::new();
        let mut meshes = Vec::new();
        let mut nodes = Vec::new();
        for primitive in &self.primitives {
            let position_accessor = push_vec3_accessor(
                &mut self.resources,
                &mut accessors,
                &primitive.positions,
                Some(34962),
                true,
            );
            let normal_accessor = push_vec3_accessor(
                &mut self.resources,
                &mut accessors,
                &primitive.normals,
                Some(34962),
                false,
            );
            let uv_accessor = push_vec2_accessor(
                &mut self.resources,
                &mut accessors,
                &primitive.uvs,
                Some(34962),
            );
            let color_accessor = push_vec4_accessor(
                &mut self.resources,
                &mut accessors,
                &primitive.colors,
                Some(34962),
            );
            let index_accessor =
                push_index_accessor(&mut self.resources, &mut accessors, &primitive.indices);
            let mesh_index = meshes.len();
            meshes.push(json!({
                "name": primitive.name,
                "primitives": [{
                    "attributes": {
                        "POSITION": position_accessor,
                        "NORMAL": normal_accessor,
                        "TEXCOORD_0": uv_accessor,
                        "COLOR_0": color_accessor
                    },
                    "indices": index_accessor,
                    "material": primitive.material,
                    "mode": 4
                }]
            }));
            nodes.push(json!({
                "name": primitive.name,
                "mesh": mesh_index,
                "extras": {
                    "bevyout_reference_form_ids": primitive.reference_form_ids,
                    "bevyout_batch_size": primitive.reference_form_ids.len()
                }
            }));
        }
        let mut root = Map::new();
        root.insert(
            "asset".into(),
            json!({"version":"2.0", "generator":"bevyout Rust bake"}),
        );
        root.insert("scene".into(), json!(0));
        root.insert(
            "scenes".into(),
            json!([{"name":"BakedCell", "nodes": (0..nodes.len()).collect::<Vec<_>>() }]),
        );
        root.insert("nodes".into(), Value::Array(nodes));
        root.insert("meshes".into(), Value::Array(meshes));
        root.insert("accessors".into(), Value::Array(accessors));
        root.insert(
            "bufferViews".into(),
            Value::Array(self.resources.buffer_views.clone()),
        );
        root.insert(
            "buffers".into(),
            json!([{"byteLength": self.resources.binary.len()}]),
        );
        if !self.resources.materials.is_empty() {
            root.insert(
                "materials".into(),
                Value::Array(self.resources.materials.clone()),
            );
        }
        if !self.resources.textures.is_empty() {
            root.insert(
                "textures".into(),
                Value::Array(self.resources.textures.clone()),
            );
        }
        if !self.resources.images.is_empty() {
            root.insert("images".into(), Value::Array(self.resources.images.clone()));
        }
        if !self.resources.samplers.is_empty() {
            root.insert(
                "samplers".into(),
                Value::Array(self.resources.samplers.clone()),
            );
        }
        let extensions_used = material_extensions_used(&self.resources.materials);
        if !extensions_used.is_empty() {
            root.insert("extensionsUsed".into(), json!(extensions_used));
        }
        write_glb(output, &Value::Object(root), &self.resources.binary)?;
        gltf::Gltf::open(output).with_context(|| {
            format!("Rust-composed GLB failed validation: {}", output.display())
        })?;
        Ok(())
    }
}

fn load_asset(path: &Path, resources: &mut OutputResources) -> Result<LoadedAsset> {
    let document = load_document(path)?;
    let required = document
        .json
        .get("extensionsRequired")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str);
    let required = unsupported_required_extensions(required);
    if !required.is_empty() {
        bail!(
            "{} requires unsupported glTF extensions: {}",
            path.display(),
            required.join(", ")
        );
    }
    if document.gltf.document.animations().next().is_some() {
        bail!("static bake asset contains animation: {}", path.display());
    }
    let material_map = document
        .gltf
        .document
        .materials()
        .map(|material| resources.import_material(&document, material))
        .collect::<Result<Vec<_>>>()?;
    let default_material = if document.gltf.document.materials().next().is_none() {
        Some(resources.import_default_material()?)
    } else {
        None
    };
    let mut primitives = Vec::new();
    let scene = document
        .gltf
        .document
        .default_scene()
        .or_else(|| document.gltf.document.scenes().next())
        .with_context(|| format!("GLB has no scene: {}", path.display()))?;
    let mut global_transforms = HashMap::new();
    for node in scene.nodes() {
        collect_node_transforms(node, Mat4::IDENTITY, &mut global_transforms);
    }
    for node in scene.nodes() {
        collect_node_primitives(
            node,
            Mat4::IDENTITY,
            &document,
            &material_map,
            default_material,
            &global_transforms,
            &mut primitives,
        )?;
    }
    if primitives.is_empty() {
        bail!(
            "GLB contains no supported render primitives: {}",
            path.display()
        );
    }
    Ok(LoadedAsset { primitives })
}

/// Returns whether a cached GLB contains animation channels. Irradiance
/// volumes are baked from a fixed pose, so animated assets must be left out of
/// the static bake while remaining available to the runtime viewer.
pub(crate) fn asset_contains_animation(asset_root: &Path, relative: &str) -> Result<bool> {
    let path = resolve_asset_path(asset_root, relative);
    let bytes =
        fs::read(&path).with_context(|| format!("could not read GLB {}", path.display()))?;
    let json = glb_json_bytes(&bytes)
        .with_context(|| format!("could not parse GLB {}", path.display()))?;
    // This is deliberately a bounded byte scan rather than deserializing the
    // full document: some source exports contain very deeply nested node
    // arrays, and the bake preflight must not recurse through those graphs.
    Ok(json
        .windows(b"\"animations\"".len())
        .any(|window| window == b"\"animations\""))
}

fn load_document(path: &Path) -> Result<AssetDocument> {
    let bytes = fs::read(path).with_context(|| format!("could not read GLB {}", path.display()))?;
    let json = parse_glb_json(&bytes)?;
    let gltf = gltf::Gltf::from_slice(&bytes)
        .with_context(|| format!("could not parse GLB {}", path.display()))?;
    let mut buffers = Vec::new();
    for buffer in gltf.document.buffers() {
        let bytes = match buffer.source() {
            gltf::buffer::Source::Bin => gltf
                .blob
                .as_ref()
                .context("GLB references a BIN buffer but has no binary blob")?
                .clone(),
            gltf::buffer::Source::Uri(uri) if !uri.starts_with("data:") => {
                fs::read(path.parent().unwrap_or_else(|| Path::new(".")).join(uri))?
            }
            gltf::buffer::Source::Uri(_) => {
                bail!("data-URI GLB buffers are unsupported: {}", path.display())
            }
        };
        buffers.push(bytes);
    }
    let mut image_bytes = Vec::new();
    for image in gltf.document.images() {
        let bytes = match image.source() {
            gltf::image::Source::View { view, .. } => {
                let data = buffers
                    .get(view.buffer().index())
                    .context("image buffer is missing")?;
                let start = view.offset();
                let end = start + view.length();
                data.get(start..end)
                    .context("image buffer view exceeds its buffer")?
                    .to_vec()
            }
            gltf::image::Source::Uri { uri, .. } if !uri.starts_with("data:") => {
                fs::read(path.parent().unwrap_or_else(|| Path::new(".")).join(uri))?
            }
            gltf::image::Source::Uri { .. } => {
                bail!("data-URI GLB images are unsupported: {}", path.display())
            }
        };
        image_bytes.push(bytes);
    }
    Ok(AssetDocument {
        gltf,
        json,
        buffers,
        image_bytes,
    })
}

fn collect_node_primitives(
    node: gltf::Node<'_>,
    parent_transform: Mat4,
    document: &AssetDocument,
    material_map: &[usize],
    default_material: Option<usize>,
    global_transforms: &HashMap<usize, Mat4>,
    output: &mut Vec<SourcePrimitive>,
) -> Result<()> {
    let transform = parent_transform * Mat4::from_cols_array_2d(&node.transform().matrix());
    let skin_matrices = node
        .skin()
        .map(|skin| skin_matrices(skin, document, global_transforms))
        .transpose()?;
    if let Some(mesh) = node.mesh()
        && !is_non_rendering_name(node.name().unwrap_or_default())
        && !is_non_rendering_name(mesh.name().unwrap_or_default())
    {
        for (primitive_index, primitive) in mesh.primitives().enumerate() {
            if primitive.mode() != gltf::mesh::Mode::Triangles {
                bail!(
                    "static bake only supports triangle primitives in {}",
                    mesh.name().unwrap_or("unnamed mesh")
                );
            }
            if primitive.morph_targets().next().is_some() {
                bail!(
                    "static bake does not support morph targets in {}",
                    mesh.name().unwrap_or("unnamed mesh")
                );
            }
            let reader =
                primitive.reader(|buffer| document.buffers.get(buffer.index()).map(Vec::as_slice));
            let local_positions = reader
                .read_positions()
                .context("GLB primitive has no POSITION attribute")?
                .map(Vec3::from_array)
                .collect::<Vec<_>>();
            let local_normals = reader
                .read_normals()
                .map(|values| values.map(Vec3::from_array).collect::<Vec<_>>())
                .unwrap_or_else(|| generated_normals(&local_positions, reader.read_indices()));
            let (positions, normals) = if let Some(matrices) = skin_matrices.as_deref() {
                if reader.read_joints(1).is_some() || reader.read_weights(1).is_some() {
                    bail!(
                        "static bake does not support a second skin influence set in {}",
                        mesh.name().unwrap_or("unnamed mesh")
                    );
                }
                let joints = reader
                    .read_joints(0)
                    .context("skinned GLB primitive has no JOINTS_0 attribute")?
                    .into_u16()
                    .collect::<Vec<_>>();
                let weights = reader
                    .read_weights(0)
                    .context("skinned GLB primitive has no WEIGHTS_0 attribute")?
                    .into_f32()
                    .collect::<Vec<_>>();
                flatten_skin(
                    &local_positions,
                    &local_normals,
                    &joints,
                    &weights,
                    matrices,
                )?
            } else {
                if reader.read_joints(0).is_some() || reader.read_weights(0).is_some() {
                    bail!(
                        "GLB primitive has skin attributes but its node has no skin in {}",
                        mesh.name().unwrap_or("unnamed mesh")
                    );
                }
                let normal_transform = Mat3::from_mat4(transform).inverse().transpose();
                (
                    local_positions
                        .iter()
                        .map(|position| transform.transform_point3(*position))
                        .collect(),
                    local_normals
                        .iter()
                        .map(|normal| (normal_transform * *normal).normalize_or_zero())
                        .collect(),
                )
            };
            let uvs = reader
                .read_tex_coords(0)
                .map(tex_coords_f32)
                .unwrap_or_else(|| vec![Vec2::ZERO; positions.len()]);
            let colors = reader
                .read_colors(0)
                .map(colors_f32)
                .unwrap_or_else(|| vec![Vec4::ONE; positions.len()]);
            let indices: Vec<u32> = reader.read_indices().map_or_else(
                || (0..positions.len() as u32).collect(),
                |values| values.into_u32().collect(),
            );
            if !indices.len().is_multiple_of(3)
                || indices
                    .iter()
                    .any(|index| *index as usize >= positions.len())
            {
                bail!(
                    "GLB primitive has invalid triangle indices in {}",
                    mesh.name().unwrap_or("unnamed mesh")
                );
            }
            let material = primitive
                .material()
                .index()
                .and_then(|index| material_map.get(index).copied())
                .or(default_material)
                .context("GLB primitive material could not be resolved")?;
            output.push(SourcePrimitive {
                name: format!("{}_{}", mesh.name().unwrap_or("mesh"), primitive_index),
                material,
                positions,
                normals,
                uvs,
                colors,
                indices,
            });
        }
    }
    for child in node.children() {
        collect_node_primitives(
            child,
            transform,
            document,
            material_map,
            default_material,
            global_transforms,
            output,
        )?;
    }
    Ok(())
}

fn collect_node_transforms(
    node: gltf::Node<'_>,
    parent_transform: Mat4,
    output: &mut HashMap<usize, Mat4>,
) {
    let transform = parent_transform * Mat4::from_cols_array_2d(&node.transform().matrix());
    output.insert(node.index(), transform);
    for child in node.children() {
        collect_node_transforms(child, transform, output);
    }
}

fn skin_matrices(
    skin: gltf::Skin<'_>,
    document: &AssetDocument,
    global_transforms: &HashMap<usize, Mat4>,
) -> Result<Vec<Mat4>> {
    let joints = skin.joints().collect::<Vec<_>>();
    let inverse_bind = skin
        .reader(|buffer| document.buffers.get(buffer.index()).map(Vec::as_slice))
        .read_inverse_bind_matrices()
        .map(|matrices| {
            matrices
                .map(|matrix| Mat4::from_cols_array_2d(&matrix))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![Mat4::IDENTITY; joints.len()]);
    if inverse_bind.len() != joints.len() {
        bail!(
            "skin {} has {} joints but {} inverse bind matrices",
            skin.index(),
            joints.len(),
            inverse_bind.len()
        );
    }
    joints
        .into_iter()
        .zip(inverse_bind)
        .map(|(joint, inverse_bind)| {
            let joint_global = global_transforms.get(&joint.index()).with_context(|| {
                format!(
                    "skin {} joint {} is outside the selected GLB scene",
                    skin.index(),
                    joint.index()
                )
            })?;
            Ok(*joint_global * inverse_bind)
        })
        .collect()
}

type SkinJoints = [u16; 4];
type SkinWeights = [f32; 4];

fn flatten_skin(
    positions: &[Vec3],
    normals: &[Vec3],
    joints: &[SkinJoints],
    weights: &[SkinWeights],
    matrices: &[Mat4],
) -> Result<(Vec<Vec3>, Vec<Vec3>)> {
    if positions.len() != normals.len()
        || positions.len() != joints.len()
        || positions.len() != weights.len()
    {
        bail!("skinned GLB vertex attribute counts do not match");
    }
    let normal_matrices = matrices
        .iter()
        .map(|matrix| Mat3::from_mat4(*matrix).inverse().transpose())
        .collect::<Vec<_>>();
    let mut flattened_positions = Vec::with_capacity(positions.len());
    let mut flattened_normals = Vec::with_capacity(normals.len());
    for (((position, normal), joints), weights) in
        positions.iter().zip(normals).zip(joints).zip(weights)
    {
        let total = weights.iter().copied().sum::<f32>();
        if !total.is_finite() || total <= f32::EPSILON {
            bail!("skinned GLB vertex has no finite nonzero influence weight");
        }
        let mut flattened_position = Vec3::ZERO;
        let mut flattened_normal = Vec3::ZERO;
        for influence in 0..4 {
            let weight = weights[influence] / total;
            if weight <= 0.0 {
                continue;
            }
            let joint = joints[influence] as usize;
            let matrix = matrices
                .get(joint)
                .context("skinned GLB vertex references an invalid joint")?;
            flattened_position += matrix.transform_point3(*position) * weight;
            flattened_normal += normal_matrices[joint] * *normal * weight;
        }
        flattened_positions.push(flattened_position);
        flattened_normals.push(flattened_normal.normalize_or_zero());
    }
    Ok((flattened_positions, flattened_normals))
}

fn generated_normals(
    positions: &[Vec3],
    indices: Option<gltf::mesh::util::ReadIndices<'_>>,
) -> Vec<Vec3> {
    let indices = indices.map_or_else(
        || (0..positions.len() as u32).collect::<Vec<_>>(),
        |values| values.into_u32().collect(),
    );
    let mut normals = vec![Vec3::ZERO; positions.len()];
    for triangle in indices.chunks_exact(3) {
        let [a, b, c] = [
            triangle[0] as usize,
            triangle[1] as usize,
            triangle[2] as usize,
        ];
        if let (Some(pa), Some(pb), Some(pc)) =
            (positions.get(a), positions.get(b), positions.get(c))
        {
            let normal = (*pb - *pa).cross(*pc - *pa);
            normals[a] += normal;
            normals[b] += normal;
            normals[c] += normal;
        }
    }
    normals.into_iter().map(Vec3::normalize_or_zero).collect()
}

fn tex_coords_f32(values: ReadTexCoords<'_>) -> Vec<Vec2> {
    values.into_f32().map(Vec2::from_array).collect()
}

fn colors_f32(values: ReadColors<'_>) -> Vec<Vec4> {
    values.into_rgba_f32().map(Vec4::from_array).collect()
}

impl OutputResources {
    fn import_default_material(&mut self) -> Result<usize> {
        self.insert_material(
            json!({"name":"DefaultMaterial"}),
            TransportMaterial::default(),
        )
    }

    fn import_material(
        &mut self,
        document: &AssetDocument,
        material: gltf::Material<'_>,
    ) -> Result<usize> {
        let local_index = material.index().context("source material has no index")?;
        let mut value = document
            .json
            .get("materials")
            .and_then(Value::as_array)
            .and_then(|materials| materials.get(local_index))
            .cloned()
            .context("source material JSON is missing")?;
        remap_material_textures(&mut value, |texture| self.import_texture(document, texture))?;
        let pbr = material.pbr_metallic_roughness();
        let base_color_texture = pbr
            .base_color_texture()
            .map(|info| self.sampled_texture(document, info.texture()))
            .transpose()?;
        let emissive_texture = material
            .emissive_texture()
            .map(|info| self.sampled_texture(document, info.texture()))
            .transpose()?;
        let alpha_mode = match material.alpha_mode() {
            gltf::material::AlphaMode::Opaque => AlphaMode::Opaque,
            gltf::material::AlphaMode::Mask => AlphaMode::Mask,
            gltf::material::AlphaMode::Blend => AlphaMode::Blend,
        };
        let transport = TransportMaterial {
            base_color_factor: Vec4::from_array(pbr.base_color_factor()),
            metallic_factor: pbr.metallic_factor(),
            emissive_factor: Vec3::from_array(material.emissive_factor()),
            base_color_texture,
            emissive_texture,
            alpha_mode,
            alpha_cutoff: material.alpha_cutoff().unwrap_or(0.5),
            double_sided: material.double_sided(),
        };
        self.insert_material(value, transport)
    }

    fn insert_material(&mut self, value: Value, transport: TransportMaterial) -> Result<usize> {
        let mut signature = value.clone();
        if let Some(object) = signature.as_object_mut() {
            object.remove("name");
            object.remove("extras");
        }
        let key = serde_json::to_string(&signature)?;
        if let Some(index) = self.material_by_json.get(&key) {
            return Ok(*index);
        }
        let index = self.materials.len();
        self.materials.push(value);
        self.transport_materials.push(transport);
        self.material_by_json.insert(key, index);
        Ok(index)
    }

    fn import_texture(&mut self, document: &AssetDocument, local_index: usize) -> Result<usize> {
        let source = document
            .json
            .get("textures")
            .and_then(Value::as_array)
            .and_then(|textures| textures.get(local_index))
            .cloned()
            .context("source texture JSON is missing")?;
        let mut texture = source;
        let object = texture
            .as_object_mut()
            .context("source texture is not an object")?;
        let image = object
            .get("source")
            .and_then(Value::as_u64)
            .context("source texture has no image")? as usize;
        object.insert("source".into(), json!(self.import_image(document, image)?));
        if let Some(local_sampler) = object.get("sampler").and_then(Value::as_u64) {
            object.insert(
                "sampler".into(),
                json!(self.import_sampler(document, local_sampler as usize)?),
            );
        }
        let key = serde_json::to_string(&texture)?;
        if let Some(index) = self.texture_by_json.get(&key) {
            return Ok(*index);
        }
        let index = self.textures.len();
        self.textures.push(texture);
        self.texture_by_json.insert(key, index);
        Ok(index)
    }

    fn import_sampler(&mut self, document: &AssetDocument, local_index: usize) -> Result<usize> {
        let sampler = document
            .json
            .get("samplers")
            .and_then(Value::as_array)
            .and_then(|samplers| samplers.get(local_index))
            .cloned()
            .context("source sampler JSON is missing")?;
        let key = serde_json::to_string(&sampler)?;
        if let Some(index) = self.sampler_by_json.get(&key) {
            return Ok(*index);
        }
        let index = self.samplers.len();
        self.samplers.push(sampler);
        self.sampler_by_json.insert(key, index);
        Ok(index)
    }

    fn import_image(&mut self, document: &AssetDocument, local_index: usize) -> Result<usize> {
        let bytes = document
            .image_bytes
            .get(local_index)
            .context("source image bytes are missing")?;
        let hash = format!("{:x}", Sha256::digest(bytes));
        if let Some(index) = self.image_by_hash.get(&hash) {
            return Ok(*index);
        }
        align_binary(&mut self.binary, 4);
        let offset = self.binary.len();
        self.binary.extend_from_slice(bytes);
        let view = self.buffer_views.len();
        self.buffer_views
            .push(json!({"buffer":0, "byteOffset":offset, "byteLength":bytes.len()}));
        let source = document
            .json
            .get("images")
            .and_then(Value::as_array)
            .and_then(|images| images.get(local_index));
        let mime = source
            .and_then(|value| value.get("mimeType"))
            .and_then(Value::as_str)
            .unwrap_or("image/png");
        let mut image = Map::new();
        image.insert("bufferView".into(), json!(view));
        image.insert("mimeType".into(), json!(mime));
        if let Some(name) = source.and_then(|value| value.get("name")).cloned() {
            image.insert("name".into(), name);
        }
        let index = self.images.len();
        self.images.push(Value::Object(image));
        self.image_by_hash.insert(hash, index);
        Ok(index)
    }

    fn sampled_texture(
        &mut self,
        document: &AssetDocument,
        texture: gltf::Texture<'_>,
    ) -> Result<SampledTexture> {
        let local_image = texture.source().index();
        let bytes = document
            .image_bytes
            .get(local_image)
            .context("texture image bytes are missing")?;
        let hash = format!("{:x}", Sha256::digest(bytes));
        let image = if let Some(image) = self.decoded_images.get(&hash) {
            Arc::clone(image)
        } else {
            let decoded = Arc::new(
                image::load_from_memory(bytes)
                    .context("could not decode bake material texture")?
                    .to_rgba8(),
            );
            self.decoded_images.insert(hash, Arc::clone(&decoded));
            decoded
        };
        let sampler = texture.sampler();
        Ok(SampledTexture {
            image,
            wrap_s: wrap_mode(sampler.wrap_s()),
            wrap_t: wrap_mode(sampler.wrap_t()),
        })
    }
}

fn remap_material_textures(
    value: &mut Value,
    mut import: impl FnMut(usize) -> Result<usize>,
) -> Result<()> {
    fn visit(
        value: &mut Value,
        key: Option<&str>,
        import: &mut impl FnMut(usize) -> Result<usize>,
    ) -> Result<()> {
        match value {
            Value::Object(object) => {
                if key.is_some_and(|key| key.ends_with("Texture"))
                    && let Some(index) = object.get("index").and_then(Value::as_u64)
                {
                    object.insert("index".into(), json!(import(index as usize)?));
                }
                for (child_key, child) in object {
                    visit(child, Some(child_key), import)?;
                }
            }
            Value::Array(values) => {
                for child in values {
                    visit(child, key, import)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    visit(value, None, &mut import)
}

const SEAM_STITCH_TOLERANCE_METERS: f32 = 0.0001;
const SEAM_EDGE_DOT_MIN: f32 = 0.9999;
const SEAM_NORMAL_DOT_MIN: f32 = 0.9999;
const SEAM_T_JUNCTION_GRID_METERS: f32 = 0.1;

#[derive(Clone, Copy, Debug)]
struct BoundaryEdge {
    fragment_index: usize,
    placement: u32,
    start: usize,
    end: usize,
    midpoint: Vec3,
    direction: Vec3,
    normal: Vec3,
}

#[derive(Clone, Copy, Debug)]
struct BoundaryPoint {
    fragment_index: usize,
    placement: u32,
    global_index: usize,
    normal: Vec3,
}

#[derive(Clone, Copy, Debug, Default)]
struct SeamStitchStats {
    edges_matched: usize,
    vertices_adjusted: usize,
    max_correction_meters: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PositionKey([u32; 3]);

impl PositionKey {
    fn from_position(position: Vec3) -> Self {
        Self(position.to_array().map(f32::to_bits))
    }
}

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        if self.parent[value] != value {
            let root = self.find(self.parent[value]);
            self.parent[value] = root;
        }
        self.parent[value]
    }

    fn union(&mut self, left: usize, right: usize) {
        let mut left = self.find(left);
        let mut right = self.find(right);
        if left == right {
            return;
        }
        if self.rank[left] < self.rank[right] {
            std::mem::swap(&mut left, &mut right);
        }
        self.parent[right] = left;
        if self.rank[left] == self.rank[right] {
            self.rank[left] += 1;
        }
    }
}

fn collect_boundary_edges(
    fragment_index: usize,
    fragment: &ComposedPrimitive,
    vertex_offset: usize,
    boundaries: &mut Vec<BoundaryEdge>,
) {
    let Some(placement) = fragment.reference_form_ids.first().copied() else {
        return;
    };
    let mut edge_occurrences = HashMap::<(u32, u32), Option<(u32, u32, Vec3)>>::new();
    for triangle in fragment.indices.chunks_exact(3) {
        let [a, b, c] = [triangle[0], triangle[1], triangle[2]];
        let [a_index, b_index, c_index] = [a as usize, b as usize, c as usize];
        if a_index >= fragment.positions.len()
            || b_index >= fragment.positions.len()
            || c_index >= fragment.positions.len()
        {
            continue;
        }
        let normal = (fragment.positions[b_index] - fragment.positions[a_index])
            .cross(fragment.positions[c_index] - fragment.positions[a_index])
            .normalize_or_zero();
        if normal.length_squared() <= f32::EPSILON {
            continue;
        }
        for (start, end) in [(a, b), (b, c), (c, a)] {
            if start == end {
                continue;
            }
            let key = if start < end {
                (start, end)
            } else {
                (end, start)
            };
            if let Some(occurrence) = edge_occurrences.get_mut(&key) {
                *occurrence = None;
            } else {
                edge_occurrences.insert(key, Some((start, end, normal)));
            }
        }
    }

    for (start, end, normal) in edge_occurrences.into_values().flatten() {
        let start_position = fragment.positions[start as usize];
        let end_position = fragment.positions[end as usize];
        let edge = end_position - start_position;
        let length = edge.length();
        if length <= SEAM_STITCH_TOLERANCE_METERS {
            continue;
        }
        boundaries.push(BoundaryEdge {
            fragment_index,
            placement,
            start: vertex_offset + start as usize,
            end: vertex_offset + end as usize,
            midpoint: (start_position + end_position) * 0.5,
            direction: edge / length,
            normal,
        });
    }
}

fn seam_grid_key(value: Vec3) -> (i64, i64, i64) {
    let scale = 1.0 / SEAM_STITCH_TOLERANCE_METERS;
    (
        (value.x * scale).floor() as i64,
        (value.y * scale).floor() as i64,
        (value.z * scale).floor() as i64,
    )
}

fn stitch_static_seams(fragments: &mut [ComposedPrimitive]) -> SeamStitchStats {
    let mut old_positions = Vec::new();
    let mut vertex_offsets = Vec::with_capacity(fragments.len());
    for fragment in fragments.iter() {
        vertex_offsets.push(old_positions.len());
        old_positions.extend_from_slice(&fragment.positions);
    }
    if old_positions.is_empty() {
        return SeamStitchStats::default();
    }

    let mut disjoint = DisjointSet::new(old_positions.len());
    for (fragment_index, fragment) in fragments.iter().enumerate() {
        let mut first_by_position = HashMap::<PositionKey, usize>::new();
        for (local_index, position) in fragment.positions.iter().copied().enumerate() {
            let global_index = vertex_offsets[fragment_index] + local_index;
            if let Some(previous) =
                first_by_position.insert(PositionKey::from_position(position), global_index)
            {
                disjoint.union(previous, global_index);
            }
        }
    }

    let mut boundaries = Vec::new();
    for (fragment_index, fragment) in fragments.iter().enumerate() {
        collect_boundary_edges(
            fragment_index,
            fragment,
            vertex_offsets[fragment_index],
            &mut boundaries,
        );
    }
    boundaries.sort_by_key(|edge| (edge.placement, edge.fragment_index, edge.start, edge.end));
    if boundaries.len() < 2 {
        return stitch_t_junctions(fragments);
    }

    let mut spatial = HashMap::<(i64, i64, i64), Vec<usize>>::new();
    let mut matched = vec![false; boundaries.len()];
    let mut matched_pairs = Vec::new();
    let tolerance = SEAM_STITCH_TOLERANCE_METERS;

    for current_index in 0..boundaries.len() {
        let current = boundaries[current_index];
        let current_key = seam_grid_key(current.midpoint);
        let mut best = None::<(f32, usize, bool)>;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let key = (current_key.0 + dx, current_key.1 + dy, current_key.2 + dz);
                    let Some(candidates) = spatial.get(&key) else {
                        continue;
                    };
                    for &other_index in candidates {
                        if matched[other_index]
                            || boundaries[other_index].placement == current.placement
                        {
                            continue;
                        }
                        let other = boundaries[other_index];
                        if current.direction.dot(other.direction).abs() < SEAM_EDGE_DOT_MIN
                            || current.normal.dot(other.normal) < SEAM_NORMAL_DOT_MIN
                        {
                            continue;
                        }
                        let same_score = old_positions[current.start]
                            .distance(old_positions[other.start])
                            + old_positions[current.end].distance(old_positions[other.end]);
                        let reverse_score = old_positions[current.start]
                            .distance(old_positions[other.end])
                            + old_positions[current.end].distance(old_positions[other.start]);
                        let (score, reverse) = if same_score <= reverse_score {
                            (same_score, false)
                        } else {
                            (reverse_score, true)
                        };
                        if score > 2.0 * tolerance {
                            continue;
                        }
                        if best.is_none_or(|(best_score, best_index, _)| {
                            score < best_score || (score == best_score && other_index < best_index)
                        }) {
                            best = Some((score, other_index, reverse));
                        }
                    }
                }
            }
        }
        if let Some((_, other_index, reverse)) = best {
            matched[current_index] = true;
            matched[other_index] = true;
            let other = boundaries[other_index];
            let (other_start, other_end) = if reverse {
                (other.end, other.start)
            } else {
                (other.start, other.end)
            };
            matched_pairs.push(((current.start, other_start), (current.end, other_end)));
        }
        spatial.entry(current_key).or_default().push(current_index);
    }

    if matched_pairs.is_empty() {
        return stitch_t_junctions(fragments);
    }
    for &((left_start, right_start), (left_end, right_end)) in &matched_pairs {
        disjoint.union(left_start, right_start);
        disjoint.union(left_end, right_end);
    }

    let mut matched_roots = HashSet::new();
    for &((left_start, _), (left_end, _)) in &matched_pairs {
        matched_roots.insert(disjoint.find(left_start));
        matched_roots.insert(disjoint.find(left_end));
    }

    let mut sums = HashMap::<usize, ([f64; 3], usize)>::new();
    for (global_index, position) in old_positions.iter().copied().enumerate() {
        let root = disjoint.find(global_index);
        if !matched_roots.contains(&root) {
            continue;
        }
        let entry = sums.entry(root).or_insert(([0.0; 3], 0));
        entry.0[0] += f64::from(position.x);
        entry.0[1] += f64::from(position.y);
        entry.0[2] += f64::from(position.z);
        entry.1 += 1;
    }
    let canonical = sums
        .into_iter()
        .map(|(root, (sum, count))| {
            (
                root,
                Vec3::new(
                    (sum[0] / count as f64) as f32,
                    (sum[1] / count as f64) as f32,
                    (sum[2] / count as f64) as f32,
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut stats = SeamStitchStats {
        edges_matched: matched_pairs.len(),
        ..Default::default()
    };
    for (fragment_index, fragment) in fragments.iter_mut().enumerate() {
        for (local_index, position) in fragment.positions.iter_mut().enumerate() {
            let root = disjoint.find(vertex_offsets[fragment_index] + local_index);
            let Some(&replacement) = canonical.get(&root) else {
                continue;
            };
            let correction = position.distance(replacement);
            if correction > 0.0 {
                *position = replacement;
                stats.vertices_adjusted += 1;
                stats.max_correction_meters = stats.max_correction_meters.max(correction);
            }
        }
    }
    let t_junction_stats = stitch_t_junctions(fragments);
    stats.vertices_adjusted += t_junction_stats.vertices_adjusted;
    stats.max_correction_meters = stats
        .max_correction_meters
        .max(t_junction_stats.max_correction_meters);
    stats
}

fn t_junction_grid_key(value: Vec3) -> (i64, i64, i64) {
    let scale = 1.0 / SEAM_T_JUNCTION_GRID_METERS;
    (
        (value.x * scale).floor() as i64,
        (value.y * scale).floor() as i64,
        (value.z * scale).floor() as i64,
    )
}

fn stitch_t_junctions(fragments: &mut [ComposedPrimitive]) -> SeamStitchStats {
    let mut old_positions = Vec::new();
    let mut vertex_offsets = Vec::with_capacity(fragments.len());
    for fragment in fragments.iter() {
        vertex_offsets.push(old_positions.len());
        old_positions.extend_from_slice(&fragment.positions);
    }
    if old_positions.is_empty() {
        return SeamStitchStats::default();
    }

    let mut boundaries = Vec::new();
    for (fragment_index, fragment) in fragments.iter().enumerate() {
        collect_boundary_edges(
            fragment_index,
            fragment,
            vertex_offsets[fragment_index],
            &mut boundaries,
        );
    }
    if boundaries.is_empty() {
        return SeamStitchStats::default();
    }

    let mut points = Vec::new();
    let mut seen_points = HashSet::new();
    for edge in &boundaries {
        for global_index in [edge.start, edge.end] {
            let point_key = (
                global_index,
                edge.placement,
                PositionKey::from_position(edge.normal),
            );
            if seen_points.insert(point_key) {
                points.push(BoundaryPoint {
                    fragment_index: edge.fragment_index,
                    placement: edge.placement,
                    global_index,
                    normal: edge.normal,
                });
            }
        }
    }

    let mut edge_grid = HashMap::<(i64, i64, i64), Vec<usize>>::new();
    for (edge_index, edge) in boundaries.iter().enumerate() {
        let start = old_positions[edge.start];
        let end = old_positions[edge.end];
        let minimum = start.min(end);
        let maximum = start.max(end);
        let minimum_key = t_junction_grid_key(minimum);
        let maximum_key = t_junction_grid_key(maximum);
        let cell_count = (maximum_key.0 - minimum_key.0 + 3)
            .saturating_mul(maximum_key.1 - minimum_key.1 + 3)
            .saturating_mul(maximum_key.2 - minimum_key.2 + 3);
        if cell_count <= 4096 {
            for x in (minimum_key.0 - 1)..=(maximum_key.0 + 1) {
                for y in (minimum_key.1 - 1)..=(maximum_key.1 + 1) {
                    for z in (minimum_key.2 - 1)..=(maximum_key.2 + 1) {
                        edge_grid.entry((x, y, z)).or_default().push(edge_index);
                    }
                }
            }
        }
    }

    let mut replacements = HashMap::<(usize, PositionKey), (f32, Vec3)>::new();
    for point in points {
        let position = old_positions[point.global_index];
        let point_key = (point.fragment_index, PositionKey::from_position(position));
        let grid_key = t_junction_grid_key(position);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let key = (grid_key.0 + dx, grid_key.1 + dy, grid_key.2 + dz);
                    let Some(edge_indices) = edge_grid.get(&key) else {
                        continue;
                    };
                    for &edge_index in edge_indices {
                        let edge = boundaries[edge_index];
                        if edge.placement == point.placement
                            || point.normal.dot(edge.normal) < SEAM_NORMAL_DOT_MIN
                        {
                            continue;
                        }
                        let start = old_positions[edge.start];
                        let segment = old_positions[edge.end] - start;
                        let start64 = [f64::from(start.x), f64::from(start.y), f64::from(start.z)];
                        let segment64 = [
                            f64::from(segment.x),
                            f64::from(segment.y),
                            f64::from(segment.z),
                        ];
                        let point64 = [
                            f64::from(position.x),
                            f64::from(position.y),
                            f64::from(position.z),
                        ];
                        let segment_length_squared = segment64
                            .iter()
                            .map(|component| component * component)
                            .sum::<f64>();
                        if segment_length_squared <= f64::EPSILON {
                            continue;
                        }
                        let projection_factor = point64
                            .iter()
                            .zip(start64.iter())
                            .zip(segment64.iter())
                            .map(|((point, start), segment)| (point - start) * segment)
                            .sum::<f64>()
                            / segment_length_squared;
                        if !(0.05..=0.95).contains(&projection_factor)
                            || projection_factor == 0.05
                            || projection_factor == 0.95
                        {
                            continue;
                        }
                        let projection = Vec3::new(
                            (start64[0] + segment64[0] * projection_factor) as f32,
                            (start64[1] + segment64[1] * projection_factor) as f32,
                            (start64[2] + segment64[2] * projection_factor) as f32,
                        );
                        let correction = position.distance(projection);
                        if correction > SEAM_STITCH_TOLERANCE_METERS {
                            continue;
                        }
                        let replace = replacements
                            .get(&point_key)
                            .is_none_or(|(best_correction, _)| correction < *best_correction);
                        if replace {
                            replacements.insert(point_key, (correction, projection));
                        }
                    }
                }
            }
        }
    }

    let mut stats = SeamStitchStats::default();
    for (fragment_index, fragment) in fragments.iter_mut().enumerate() {
        for (local_index, position) in fragment.positions.iter_mut().enumerate() {
            let original = old_positions[vertex_offsets[fragment_index] + local_index];
            let key = (fragment_index, PositionKey::from_position(original));
            let Some((correction, replacement)) = replacements.get(&key).copied() else {
                continue;
            };
            if correction > 0.0 {
                *position = replacement;
                stats.vertices_adjusted += 1;
                stats.max_correction_meters = stats.max_correction_meters.max(correction);
            }
        }
    }
    stats
}

fn batch_fragments(
    fragments: Vec<ComposedPrimitive>,
    chunk_size: f32,
) -> (Vec<ComposedPrimitive>, BatchingStats) {
    let mut groups = BTreeMap::<(i32, i32, i32, usize), Vec<ComposedPrimitive>>::new();
    let mut passthrough = Vec::new();
    let mut excluded_large = 0;
    for fragment in fragments {
        let bounds = primitive_bounds(&fragment);
        let extent = bounds.extent();
        if extent.max_element() > chunk_size + 1e-6 {
            excluded_large += 1;
            passthrough.push(fragment);
            continue;
        }
        let center = bounds.center();
        let chunk = (center / chunk_size).floor().as_ivec3();
        groups
            .entry((chunk.x, chunk.y, chunk.z, fragment.material))
            .or_default()
            .push(fragment);
    }
    let mut batches_created = 0;
    let mut largest_batch = 0;
    for ((x, y, z, material), mut group) in groups {
        group.sort_by(|left, right| left.name.cmp(&right.name));
        if group.len() == 1 {
            passthrough.push(group.pop().expect("single item"));
            continue;
        }
        largest_batch = largest_batch.max(group.len());
        batches_created += 1;
        let mut output = ComposedPrimitive {
            name: format!("batch_{x}_{y}_{z}_{material}"),
            reference_form_ids: Vec::new(),
            material,
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            colors: Vec::new(),
            transport_colors: Vec::new(),
            indices: Vec::new(),
        };
        for fragment in group {
            let base = output.positions.len() as u32;
            output
                .reference_form_ids
                .extend(fragment.reference_form_ids);
            output.positions.extend(fragment.positions);
            output.normals.extend(fragment.normals);
            output.uvs.extend(fragment.uvs);
            output.colors.extend(fragment.colors);
            output.transport_colors.extend(fragment.transport_colors);
            output
                .indices
                .extend(fragment.indices.into_iter().map(|index| index + base));
        }
        output.reference_form_ids.sort_unstable();
        output.reference_form_ids.dedup();
        passthrough.push(output);
    }
    passthrough.sort_by(|left, right| left.name.cmp(&right.name));
    let stats = BatchingStats {
        chunk_size_meters: chunk_size,
        visual_objects_after: passthrough.len(),
        render_primitives_after: passthrough.len(),
        batches_created,
        largest_batch,
        excluded_large,
        ..Default::default()
    };
    (passthrough, stats)
}

fn scene_bounds(primitives: &[ComposedPrimitive]) -> Result<SceneBounds> {
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    for position in primitives.iter().flat_map(|primitive| &primitive.positions) {
        minimum = minimum.min(*position);
        maximum = maximum.max(*position);
    }
    if !minimum.is_finite() || !maximum.is_finite() {
        bail!("Rust-composed scene has no finite geometry bounds");
    }
    Ok(SceneBounds { minimum, maximum })
}

fn primitive_bounds(primitive: &ComposedPrimitive) -> SceneBounds {
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    for position in &primitive.positions {
        minimum = minimum.min(*position);
        maximum = maximum.max(*position);
    }
    SceneBounds { minimum, maximum }
}

fn push_vec3_accessor(
    resources: &mut OutputResources,
    accessors: &mut Vec<Value>,
    values: &[Vec3],
    target: Option<u32>,
    bounds: bool,
) -> usize {
    align_binary(&mut resources.binary, 4);
    let offset = resources.binary.len();
    for value in values {
        for component in value.to_array() {
            resources.binary.extend_from_slice(&component.to_le_bytes());
        }
    }
    let view = resources.buffer_views.len();
    let mut buffer_view = json!({"buffer":0, "byteOffset":offset, "byteLength":values.len()*12});
    if let Some(target) = target {
        buffer_view["target"] = json!(target);
    }
    resources.buffer_views.push(buffer_view);
    let mut accessor =
        json!({"bufferView":view, "componentType":5126, "count":values.len(), "type":"VEC3"});
    if bounds && !values.is_empty() {
        let mut minimum = Vec3::splat(f32::INFINITY);
        let mut maximum = Vec3::splat(f32::NEG_INFINITY);
        for value in values {
            minimum = minimum.min(*value);
            maximum = maximum.max(*value);
        }
        accessor["min"] = json!(minimum.to_array());
        accessor["max"] = json!(maximum.to_array());
    }
    let index = accessors.len();
    accessors.push(accessor);
    index
}

fn push_vec2_accessor(
    resources: &mut OutputResources,
    accessors: &mut Vec<Value>,
    values: &[Vec2],
    target: Option<u32>,
) -> usize {
    align_binary(&mut resources.binary, 4);
    let offset = resources.binary.len();
    for value in values {
        for component in value.to_array() {
            resources.binary.extend_from_slice(&component.to_le_bytes());
        }
    }
    let view = resources.buffer_views.len();
    let mut buffer_view = json!({"buffer":0, "byteOffset":offset, "byteLength":values.len()*8});
    if let Some(target) = target {
        buffer_view["target"] = json!(target);
    }
    resources.buffer_views.push(buffer_view);
    let index = accessors.len();
    accessors.push(
        json!({"bufferView":view, "componentType":5126, "count":values.len(), "type":"VEC2"}),
    );
    index
}

fn push_vec4_accessor(
    resources: &mut OutputResources,
    accessors: &mut Vec<Value>,
    values: &[Vec4],
    target: Option<u32>,
) -> usize {
    align_binary(&mut resources.binary, 4);
    let offset = resources.binary.len();
    for value in values {
        for component in value.to_array() {
            resources.binary.extend_from_slice(&component.to_le_bytes());
        }
    }
    let view = resources.buffer_views.len();
    let mut buffer_view = json!({"buffer":0, "byteOffset":offset, "byteLength":values.len()*16});
    if let Some(target) = target {
        buffer_view["target"] = json!(target);
    }
    resources.buffer_views.push(buffer_view);
    let index = accessors.len();
    accessors.push(
        json!({"bufferView":view, "componentType":5126, "count":values.len(), "type":"VEC4"}),
    );
    index
}

fn push_index_accessor(
    resources: &mut OutputResources,
    accessors: &mut Vec<Value>,
    values: &[u32],
) -> usize {
    align_binary(&mut resources.binary, 4);
    let offset = resources.binary.len();
    for value in values {
        resources.binary.extend_from_slice(&value.to_le_bytes());
    }
    let view = resources.buffer_views.len();
    resources.buffer_views.push(
        json!({"buffer":0, "byteOffset":offset, "byteLength":values.len()*4, "target":34963}),
    );
    let index = accessors.len();
    accessors.push(
        json!({"bufferView":view, "componentType":5125, "count":values.len(), "type":"SCALAR"}),
    );
    index
}

fn write_glb(path: &Path, root: &Value, binary: &[u8]) -> Result<()> {
    let mut json_bytes = serde_json::to_vec(root)?;
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let mut bin = binary.to_vec();
    align_binary(&mut bin, 4);
    let total = 12 + 8 + json_bytes.len() + 8 + bin.len();
    let mut output = Vec::with_capacity(total);
    output.extend_from_slice(&GLB_MAGIC.to_le_bytes());
    output.extend_from_slice(&2_u32.to_le_bytes());
    output.extend_from_slice(&(total as u32).to_le_bytes());
    output.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    output.extend_from_slice(&GLB_JSON_CHUNK.to_le_bytes());
    output.extend_from_slice(&json_bytes);
    output.extend_from_slice(&(bin.len() as u32).to_le_bytes());
    output.extend_from_slice(&GLB_BIN_CHUNK.to_le_bytes());
    output.extend_from_slice(&bin);
    fs::write(path, output).with_context(|| format!("could not write {}", path.display()))
}

fn parse_glb_json(bytes: &[u8]) -> Result<Value> {
    Ok(serde_json::from_slice(glb_json_bytes(bytes)?)?)
}

fn glb_json_bytes(bytes: &[u8]) -> Result<&[u8]> {
    if bytes.len() < 20 || u32::from_le_bytes(bytes[0..4].try_into()?) != GLB_MAGIC {
        bail!("invalid GLB header");
    }
    let length = u32::from_le_bytes(bytes[12..16].try_into()?) as usize;
    let kind = u32::from_le_bytes(bytes[16..20].try_into()?);
    if kind != GLB_JSON_CHUNK || 20 + length > bytes.len() {
        bail!("GLB has no valid JSON chunk");
    }
    Ok(&bytes[20..20 + length])
}

fn align_binary(bytes: &mut Vec<u8>, alignment: usize) {
    while !bytes.len().is_multiple_of(alignment) {
        bytes.push(0);
    }
}

fn resolve_asset_path(asset_root: &Path, relative: &str) -> PathBuf {
    asset_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))
}

fn is_non_rendering_name(name: &str) -> bool {
    let name = name.to_ascii_lowercase().replace(['_', ' '], "");
    ["shadefade", "fx", "editormarker", "marker", "collision"]
        .iter()
        .any(|prefix| name.starts_with(prefix))
}

fn wrap_mode(mode: gltf::texture::WrappingMode) -> WrapMode {
    match mode {
        gltf::texture::WrappingMode::ClampToEdge => WrapMode::Clamp,
        gltf::texture::WrappingMode::MirroredRepeat => WrapMode::Mirror,
        gltf::texture::WrappingMode::Repeat => WrapMode::Repeat,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_sampling_wraps_with_gltf_upper_left_origin() {
        let image = RgbaImage::from_raw(1, 2, vec![255, 0, 0, 255, 0, 255, 0, 128]).unwrap();
        let texture = SampledTexture {
            image: Arc::new(image),
            wrap_s: WrapMode::Repeat,
            wrap_t: WrapMode::Clamp,
        };
        assert_eq!(texture.sample(Vec2::ZERO), Vec4::new(1.0, 0.0, 0.0, 1.0));
        assert!((texture.sample(Vec2::new(0.0, 1.0)).w - 128.0 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn batching_offsets_indices_and_preserves_reference_ids() {
        let fragment = |name: &str, reference| ComposedPrimitive {
            name: name.into(),
            reference_form_ids: vec![reference],
            material: 0,
            positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
            normals: vec![Vec3::Z; 3],
            uvs: vec![Vec2::ZERO; 3],
            colors: vec![Vec4::ONE; 3],
            transport_colors: vec![Vec4::ONE; 3],
            indices: vec![0, 1, 2],
        };
        let (batched, stats) = batch_fragments(vec![fragment("a", 1), fragment("b", 2)], 64.0);
        assert_eq!(stats.batches_created, 1);
        assert_eq!(batched[0].indices, vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(batched[0].reference_form_ids, vec![1, 2]);
    }

    #[test]
    fn static_skin_is_flattened_from_joint_bind_matrices() {
        let positions = [Vec3::new(1.0, 0.0, 0.0)];
        let normals = [Vec3::Y];
        let joints = [[0, 1, 0, 0]];
        let weights = [[0.25, 0.75, 0.0, 0.0]];
        let matrices = [
            Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0)),
            Mat4::from_translation(Vec3::new(0.0, 4.0, 0.0)),
        ];

        let (flattened_positions, flattened_normals) =
            flatten_skin(&positions, &normals, &joints, &weights, &matrices).unwrap();

        assert_eq!(flattened_positions, vec![Vec3::new(1.0, 3.5, 0.0)]);
        assert_eq!(flattened_normals, vec![Vec3::Y]);
    }

    fn floor_quad(
        name: &str,
        reference_form_id: u32,
        minimum_x: f32,
        maximum_x: f32,
        reverse_winding: bool,
    ) -> ComposedPrimitive {
        let positions = vec![
            Vec3::new(minimum_x, 0.0, 0.0),
            Vec3::new(maximum_x, 0.0, 0.0),
            Vec3::new(maximum_x, 0.0, 1.0),
            Vec3::new(minimum_x, 0.0, 1.0),
        ];
        let indices = if reverse_winding {
            vec![0, 1, 2, 0, 2, 3]
        } else {
            vec![0, 2, 1, 0, 3, 2]
        };
        ComposedPrimitive {
            name: name.into(),
            reference_form_ids: vec![reference_form_id],
            material: 0,
            positions,
            normals: vec![Vec3::Y; 4],
            uvs: vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y],
            colors: vec![Vec4::new(1.0, 0.5, 0.25, 1.0); 4],
            transport_colors: vec![Vec4::ONE; 4],
            indices,
        }
    }

    fn triangle_fragment(
        name: &str,
        reference_form_id: u32,
        positions: [Vec3; 3],
    ) -> ComposedPrimitive {
        ComposedPrimitive {
            name: name.into(),
            reference_form_ids: vec![reference_form_id],
            material: 0,
            positions: positions.into(),
            normals: vec![Vec3::Y; 3],
            uvs: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
            colors: vec![Vec4::ONE; 3],
            transport_colors: vec![Vec4::ONE; 3],
            indices: vec![0, 1, 2],
        }
    }

    #[test]
    fn seam_stitch_moves_only_positions_across_coplanar_placement_boundary() {
        let delta = 0.00003;
        let left = floor_quad("left", 1, 0.0, 1.0, false);
        let right = floor_quad("right", 2, 1.0 + delta, 2.0, false);
        let original_left = left.clone();
        let original_right = right.clone();
        let mut fragments = vec![left, right];

        let stats = stitch_static_seams(&mut fragments);

        assert_eq!(stats.edges_matched, 1);
        assert_eq!(stats.vertices_adjusted, 4);
        assert!(stats.max_correction_meters > 0.0);
        assert_eq!(fragments[0].positions[1], fragments[1].positions[0]);
        assert_eq!(fragments[0].positions[2], fragments[1].positions[3]);
        assert_eq!(fragments[0].positions[0], original_left.positions[0]);
        assert_eq!(fragments[1].positions[1], original_right.positions[1]);
        assert_eq!(fragments[0].normals, original_left.normals);
        assert_eq!(fragments[1].normals, original_right.normals);
        assert_eq!(fragments[0].uvs, original_left.uvs);
        assert_eq!(fragments[1].uvs, original_right.uvs);
        assert_eq!(fragments[0].colors, original_left.colors);
        assert_eq!(fragments[1].colors, original_right.colors);
        assert_eq!(fragments[0].indices, original_left.indices);
        assert_eq!(fragments[1].indices, original_right.indices);
    }

    #[test]
    fn seam_stitch_rejects_opposite_winding_far_and_same_placement_edges() {
        let delta = 0.00003;
        let mut opposite = vec![
            floor_quad("left", 1, 0.0, 1.0, false),
            floor_quad("right", 2, 1.0 + delta, 2.0, true),
        ];
        let opposite_original = opposite.clone();
        assert_eq!(stitch_static_seams(&mut opposite).edges_matched, 0);
        assert_eq!(opposite[0].positions, opposite_original[0].positions);
        assert_eq!(opposite[1].positions, opposite_original[1].positions);

        let mut far = vec![
            floor_quad("left", 1, 0.0, 1.0, false),
            floor_quad(
                "right",
                2,
                1.0 + SEAM_STITCH_TOLERANCE_METERS * 2.0,
                2.0,
                false,
            ),
        ];
        assert_eq!(stitch_static_seams(&mut far).edges_matched, 0);

        let mut same_placement = vec![
            floor_quad("left", 1, 0.0, 1.0, false),
            floor_quad("right", 1, 1.0 + delta, 2.0, false),
        ];
        assert_eq!(stitch_static_seams(&mut same_placement).edges_matched, 0);
    }

    #[test]
    fn seam_stitch_closes_a_coplanar_t_junction_without_merging_vertices() {
        let delta = 0.00003;
        let point_fragment = triangle_fragment(
            "point",
            1,
            [
                Vec3::new(1.0, 0.0, delta),
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(2.0, 0.0, 1.0),
            ],
        );
        let edge_fragment = triangle_fragment(
            "edge",
            2,
            [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(2.0, 0.0, 0.0),
            ],
        );
        let mut fragments = vec![point_fragment, edge_fragment];

        let stats = stitch_static_seams(&mut fragments);

        assert_eq!(stats.edges_matched, 0);
        assert_eq!(stats.vertices_adjusted, 1);
        assert_eq!(fragments[0].positions[0], Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(fragments[1].positions[0], Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(fragments[1].positions[2], Vec3::new(2.0, 0.0, 0.0));
    }
}
