use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::bsa::BsaArchive;
use super::manifest::Diagnostic;
use super::paths::{fingerprint, normalize_asset_path};

/// Bump this whenever the embedded NIFTools conversion/filtering changes.
/// It is part of the content-addressed GLB name so stale conversions cannot
/// silently survive a converter fix.
pub(crate) const NIF_CONVERTER_REVISION: &str =
    "niftools-blender52-textures-v16-breakable-constraint-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssetConversion {
    Preserve,
    QuickAo,
}

impl AssetConversion {
    pub(crate) fn profile_tag(self) -> &'static str {
        match self {
            Self::Preserve => "ao-none",
            Self::QuickAo => "ao-quick-v1",
        }
    }
}

pub(crate) fn asset_conversion(static_asset: bool) -> AssetConversion {
    if static_asset {
        AssetConversion::QuickAo
    } else {
        AssetConversion::Preserve
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BlenderAssetJob {
    pub(crate) input: PathBuf,
    pub(crate) output: PathBuf,
    pub(crate) model: String,
    pub(crate) conversion: AssetConversion,
}

pub(crate) fn content_addressed_glb_name(converter_revision: &str, nif_bytes: &[u8]) -> String {
    let mut cache_key = converter_revision.as_bytes().to_vec();
    cache_key.push(0);
    cache_key.extend_from_slice(nif_bytes);
    format!("{}.glb", fingerprint(&cache_key))
}

pub(crate) fn find_blender(explicit: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        if path.exists() {
            return Ok(path);
        }
        bail!("Blender executable does not exist: {}", path.display());
    }
    let candidates = [
        PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 5.2\blender.exe"),
        PathBuf::from(r"C:\Program Files\Blender Foundation\Blender 4.5\blender.exe"),
    ];
    candidates
        .into_iter()
        .find(|p| p.exists())
        .context("Blender was not found; pass --blender explicitly")
}

pub(crate) fn stage_textures(
    nif_bytes: &[u8],
    data_root: &Path,
    archives: &[BsaArchive],
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    for texture in texture_references(nif_bytes) {
        let Some(bytes) = resolve_asset(data_root, archives, &texture)
            .with_context(|| format!("reading texture {texture}"))?
        else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("missing texture {texture}"),
            });
            continue;
        };
        let destination = staging_dir.join(texture.replace('/', std::path::MAIN_SEPARATOR_STR));
        if destination.exists() {
            continue;
        }
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(destination, bytes)?;
    }
    Ok(())
}

pub(crate) fn convert_staged_textures(
    staging_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<()> {
    let converter = {
        let installed = PathBuf::from(r"C:\Program Files\ImageMagick-7.1.2-Q16-HDRI\magick.exe");
        if installed.exists() {
            Some(installed)
        } else if let Some(name) = ["magick.exe", "magick"].into_iter().find(|name| {
            Command::new(name)
                .arg("-version")
                .output()
                .is_ok_and(|output| output.status.success())
        }) {
            Some(PathBuf::from(name))
        } else {
            None
        }
    };
    let Some(converter) = converter else {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: "ImageMagick was not found; DDS textures will remain unconverted".into(),
        });
        return Ok(());
    };
    let mut dds_files = Vec::new();
    collect_files_with_extension(staging_dir, "dds", &mut dds_files)?;
    for dds in dds_files {
        let png = dds.with_extension("png");
        if png.exists() {
            fs::remove_file(&dds)?;
            continue;
        }
        let result = Command::new(&converter)
            .arg(&dds)
            .arg("-strip")
            .arg(&png)
            .output();
        match result {
            Ok(output) if output.status.success() => {
                fs::remove_file(&dds)?;
            }
            Ok(output) => diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "could not convert {} to PNG: {}",
                    dds.display(),
                    String::from_utf8_lossy(&output.stderr).trim()
                ),
            }),
            Err(error) => diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!("could not run ImageMagick for {}: {error}", dds.display()),
            }),
        }
    }
    Ok(())
}

fn collect_files_with_extension(
    root: &Path,
    extension: &str,
    output: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files_with_extension(&path, extension, output)?;
        } else if path
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case(extension))
        {
            output.push(path);
        }
    }
    Ok(())
}

fn texture_references(bytes: &[u8]) -> Vec<String> {
    const EXTENSIONS: [&str; 5] = [".dds", ".tga", ".bmp", ".png", ".jpg"];
    let mut found = HashSet::new();
    let mut start = None;
    let mut inspect = |run: &[u8]| {
        if run.len() < 5 {
            return;
        }
        let text = String::from_utf8_lossy(run);
        let lower = text.to_ascii_lowercase();
        for extension in EXTENSIONS {
            let mut search_from = 0;
            while let Some(relative) = lower[search_from..].find(extension) {
                let end = search_from + relative + extension.len();
                let prefix = &lower[..search_from + relative];
                let separator = prefix
                    .rfind('\\')
                    .max(prefix.rfind('/'))
                    .map(|index| index + 1);
                let candidate_start = prefix.rfind("textures").or(separator).unwrap_or(0);
                let candidate = normalize_asset_path(&text[candidate_start..end]);
                // Fallout asset folders legitimately contain spaces (for
                // example `textures/dungeons/wasteland homes`). Keep the
                // complete normalized path instead of dropping those valid
                // texture references.
                if candidate.ends_with(extension) && candidate.starts_with("textures/") {
                    found.insert(candidate);
                }
                search_from = end;
                if search_from >= lower.len() {
                    break;
                }
            }
        }
    };
    for (index, byte) in bytes.iter().copied().enumerate() {
        if (0x20..=0x7e).contains(&byte) {
            if start.is_none() {
                start = Some(index);
            }
        } else if let Some(begin) = start.take() {
            inspect(&bytes[begin..index]);
        }
    }
    if let Some(begin) = start {
        inspect(&bytes[begin..]);
    }
    found.into_iter().collect()
}

pub(crate) fn run_blender_batch(
    blender: &Path,
    jobs: &[BlenderAssetJob],
    data_root: &Path,
    staging_dir: &Path,
) -> Result<()> {
    let job_file = staging_dir.join("blender_jobs.ron");
    let job_text = blender_jobs_json(jobs);
    fs::write(&job_file, job_text)?;
    let script = r#"import bpy, json, os, sys
from mathutils import Vector
def patch_niftools_blender52():
    from io_scene_niftools.modules.nif_import.geometry.vertex import Vertex
    from io_scene_niftools.modules.nif_import.property.nodes_wrapper import NodesWrapper
    from io_scene_niftools.modules.nif_import.collision import Collision, get_material
    from io_scene_niftools.modules.nif_import.collision.bound import Bound
    from io_scene_niftools.modules.nif_import.property.material import Material
    from io_scene_niftools.modules.nif_import.geometry.vertex.groups import VertexGroup
    from io_scene_niftools.modules.nif_import.constraint import Constraint
    from nifgen.formats.nif.bshavok.niobjects.BhkConstraint import BhkConstraint
    def map_normals_compat(b_mesh, normals):
        if len(b_mesh.vertices) != len(normals): raise RuntimeError('normal/vertex count mismatch')
        no_array = Vertex.normalize(normals)
        if hasattr(b_mesh, 'normals_split_custom_set_from_vertices'):
            b_mesh.normals_split_custom_set_from_vertices(no_array)
    def link_normal_node_compat(self, b_texture_node):
        # Blender 5.x replaced ShaderNodeTree.inputs/outputs with the
        # interface API. Niftools' normal-map group is optional for this
        # static export, so leave the image node unlinked rather than fail.
        b_texture_node.label = 'Normal'
    Vertex.map_normals = staticmethod(map_normals_compat)
    NodesWrapper.link_normal_node = link_normal_node_compat
    # Keep authored collision geometry. The addon uses a Blender 4-era context
    # override which Blender 5 rejects; temp_override is the supported form.
    def set_b_collider_compat(b_obj, radius, n_obj=None, bounds_type='BOX', display_type='BOX'):
        b_obj.show_bounds = True
        b_obj.display_type = 'BOUNDS'
        b_obj.display_bounds_type = display_type
        bpy.context.view_layer.objects.active = b_obj
        bpy.ops.object.select_all(action='DESELECT')
        b_obj.select_set(True)
        with bpy.context.temp_override(
            active_object=b_obj,
            object=b_obj,
            selected_objects=[b_obj],
            selected_editable_objects=[b_obj],
        ):
            bpy.ops.rigidbody.object_add()
        b_r_body = b_obj.rigid_body
        b_r_body.enabled = True
        b_r_body.use_margin = True
        b_r_body.collision_margin = radius
        b_r_body.collision_shape = bounds_type
        b_r_body.type = 'PASSIVE'
        b_obj['bevyout_collision'] = True
        if n_obj is not None:
            havok_material = getattr(n_obj, 'material', None)
            material_enum = getattr(havok_material, 'material', None)
            if material_enum is not None:
                material_id = getattr(material_enum, 'value', None)
                material_name = getattr(material_enum, 'name', str(material_enum))
                if material_id is not None:
                    b_obj['bevyout_havok_material'] = int(material_id)
                b_obj['bevyout_havok_material_name'] = material_name
                b_obj.data.materials.append(get_material(material_name))
    Collision.set_b_collider = staticmethod(set_b_collider_compat)
    Bound.import_bounding_box = lambda self, n_block: []
    def set_alpha_compat(b_mat, n_alpha_prop):
        # Blender 4.2+ removed Material.blend_method/shadow_method. Keep the
        # alpha metadata and only assign properties that still exist.
        if hasattr(b_mat, 'alpha_threshold'):
            b_mat.alpha_threshold = n_alpha_prop.threshold / 255
        if hasattr(b_mat, 'niftools_alpha'):
            b_mat.niftools_alpha.alphaflag = n_alpha_prop.flags
    Material.set_alpha = staticmethod(set_alpha_compat)
    VertexGroup.set_face_maps = classmethod(lambda cls, face_maps, b_obj: None)
    def apply_constraint_scale_compat(self, scale):
        # NIFTools 5.2 calls self.constraint here, but bhkBreakableConstraint
        # stores constraint_data instead. Constraint records contain no render
        # geometry, so skip this broken physics-only traversal during import.
        pass
    BhkConstraint.apply_scale = apply_constraint_scale_compat
    # Constraint import is likewise physics-only and its 5.2 importer assumes
    # every Havok record has the obsolete `constraint` field. Collision meshes
    # are exported separately, so skip Blender rigid-body joint construction.
    Constraint.import_bhk_constraints = lambda self: None
def bake_quick_ao():
    objects = [obj for obj in bpy.context.scene.objects
        if obj.type == 'MESH' and len(obj.data.polygons)
               and not obj.get('bevyout_collision', False)]
    if not objects:
        return
    scene = bpy.context.scene
    previous_engine = scene.render.engine
    scene.render.engine = 'CYCLES'
    scene.cycles.samples = 4
    scene.cycles.max_bounces = 1
    scene.cycles.use_denoising = False
    for obj in objects:
        mesh = obj.data
        while mesh.color_attributes:
            mesh.color_attributes.remove(mesh.color_attributes[-1])
        mesh.color_attributes.new(name='BevyOutQuickAO', type='FLOAT_COLOR', domain='CORNER')
        mesh.color_attributes.active_color_index = len(mesh.color_attributes) - 1
    bpy.ops.object.select_all(action='DESELECT')
    for obj in objects: obj.select_set(True)
    bpy.context.view_layer.objects.active = objects[0]
    result = bpy.ops.object.bake(type='AO', target='VERTEX_COLORS', width=8, height=8,
                                 max_ray_distance=1.0, margin=2)
    if 'FINISHED' not in result:
        scene.render.engine = previous_engine
        raise RuntimeError('quick AO vertex bake failed')
    for obj in objects:
        attribute = obj.data.color_attributes.active_color
        for item in attribute.data:
            values = tuple(item.color)
            raw = max(0.0, min(1.0, values[0]))
            ao = 0.72 + 0.28 * raw
            item.color = (ao, ao, ao, values[3] if len(values) > 3 else 1.0)
    scene.render.engine = previous_engine

bpy.ops.preferences.addon_enable(module='io_scene_niftools')
patch_niftools_blender52()
bpy.context.preferences.filepaths.texture_directory = os.path.abspath(sys.argv[-1])
with open(sys.argv[-2], 'r', encoding='utf8') as f: jobs=json.load(f)
for job in jobs:
    nif_path = job['input']
    output_path = job['output']
    conversion = job.get('conversion', 'ao-none')
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    for datablocks in (bpy.data.meshes, bpy.data.curves, bpy.data.materials, bpy.data.cameras, bpy.data.lights):
        for datablock in list(datablocks):
            if datablock.users == 0: datablocks.remove(datablock)
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    result=bpy.ops.import_scene.nif(filepath=nif_path, process='EVERYTHING', animation=False, scale_correction=1.0/70.0, use_custom_normals=False, use_embedded_texture=False)
    if 'FINISHED' not in result: raise RuntimeError('NIF import failed: '+nif_path)
    non_rendering_prefixes=('shadefade','fx','editormarker','marker','collision')
    def is_non_rendering_object(obj):
        name=obj.name.casefold().replace('_','').replace(' ','')
        return name.startswith(non_rendering_prefixes)
    authored_collision = any(obj.get('bevyout_collision', False)
                             for obj in bpy.context.scene.objects)
    for obj in list(bpy.context.scene.objects):
        if obj.get('bevyout_collision', False):
            obj.hide_render = False
            obj.hide_viewport = False
        elif obj.display_type == 'BOUNDS' or is_non_rendering_object(obj): bpy.data.objects.remove(obj, do_unlink=True)
        elif obj.type == 'MESH' and not any(
            material and any(node.bl_idname == 'ShaderNodeTexImage' and node.image for node in material.node_tree.nodes)
            for material in obj.data.materials
        ):
            # NIF collision/helper meshes have no texture and should not be rendered.
            bpy.data.objects.remove(obj, do_unlink=True)
    if authored_collision:
        for obj in bpy.context.scene.objects:
            if obj.type == 'MESH' and not obj.get('bevyout_collision', False):
                obj['bevyout_collision_source'] = 'authored'
    def object_world_center(obj):
        points = [obj.matrix_world @ Vector(corner) for corner in obj.bound_box]
        return sum(points, Vector()) / len(points) if points else obj.matrix_world.translation.copy()
    def object_root(obj):
        seen = set()
        while obj.parent is not None and obj.name not in seen:
            seen.add(obj.name)
            obj = obj.parent
        return obj
    render_meshes = [obj for obj in bpy.context.scene.objects
                     if obj.type == 'MESH' and not obj.get('bevyout_collision', False)]
    glow_cards = [obj for obj in render_meshes
                  if obj.name.casefold().startswith('lightglow')]
    for glow_card in glow_cards:
        root = object_root(glow_card)
        candidates = [obj for obj in render_meshes
                      if obj is not glow_card and object_root(obj) is root]
        if not candidates:
            continue
        glow_center = object_world_center(glow_card)
        bulb = min(candidates,
                   key=lambda obj: (object_world_center(obj) - glow_center).length)
        for slot, source_material in enumerate(list(bulb.data.materials)):
            if source_material is None:
                continue
            material = source_material.copy()
            material['bevyout_emissive_bulb'] = True
            bulb.data.materials[slot] = material
        # The card is only an authored halo hint. The physical bulb below is
        # rendered with its own diffuse texture and emission instead.
        bpy.data.objects.remove(glow_card, do_unlink=True)
    if conversion == 'ao-quick-v1':
        bake_quick_ao()
    for material in bpy.data.materials:
        if not material.use_nodes: continue
        tree = material.node_tree
        output = next((node for node in tree.nodes if node.bl_idname == 'ShaderNodeOutputMaterial'), None)
        if output is None: continue
        alpha_flags = getattr(getattr(material, 'niftools_alpha', None), 'alphaflag', 0)
        alpha_threshold = getattr(material, 'alpha_threshold', 0.5)
        alpha_blend = bool(alpha_flags & 1)
        alpha_test = bool(alpha_flags & (1 << 9))
        images = [node for node in tree.nodes if node.bl_idname == 'ShaderNodeTexImage' and node.image]
        if not images: continue
        def image_name(node):
            return node.image.name.lower() if node.image else ''
        def is_glow_image(node):
            name = image_name(node)
            label = node.label.lower()
            return ('_g.' in name or '_em.' in name or 'glow' in name or
                    'emiss' in name or 'glow' in label or 'emiss' in label)
        glow = next((node for node in images if is_glow_image(node)), None)
        diffuse = next((node for node in images if node is not glow and 'normal' not in node.label.lower() and '_n.' not in image_name(node) and not is_glow_image(node)), images[0])
        normal = next((node for node in images if node is not diffuse and ('normal' in node.label.lower() or '_n.' in node.image.name.lower())), None)
        old_principled = next((node for node in tree.nodes if node.bl_idname == 'ShaderNodeBsdfPrincipled'), None)
        emission_link = None
        emission_color = None
        emission_strength = None
        if old_principled:
            emission_input = old_principled.inputs.get('Emission Color') or old_principled.inputs.get('Emission')
            if emission_input:
                emission_color = emission_input.default_value[:]
                if emission_input.links:
                    emission_link = emission_input.links[0].from_socket
            strength_input = old_principled.inputs.get('Emission Strength')
            if strength_input:
                emission_strength = strength_input.default_value
        principled = tree.nodes.new('ShaderNodeBsdfPrincipled')
        tree.links.new(diffuse.outputs['Color'], principled.inputs['Base Color'])
        new_emission = principled.inputs.get('Emission Color') or principled.inputs.get('Emission')
        if new_emission:
            if emission_link:
                tree.links.new(emission_link, new_emission)
            elif emission_color:
                new_emission.default_value = emission_color
        new_emission_strength = principled.inputs.get('Emission Strength')
        if new_emission_strength and emission_strength is not None:
            new_emission_strength.default_value = emission_strength
        if material.get('bevyout_emissive_bulb', False) and new_emission:
            for link in list(new_emission.links):
                tree.links.remove(link)
            tree.links.new(diffuse.outputs['Color'], new_emission)
            if new_emission_strength:
                new_emission_strength.default_value = max(emission_strength or 0.0, 2.0)
        if glow and new_emission:
            for link in list(new_emission.links): tree.links.remove(link)
            tree.links.new(glow.outputs['Color'], new_emission)
            if new_emission_strength:
                # Fallout's NIF shader properties provide the authored emissive
                # multiplier.  NIFTools imports it into the old Principled
                # material, which we copied above.  Glow maps are masks/colors,
                # not calibrated light intensities, so retain that multiplier
                # instead of replacing it with an arbitrary HDR boost.
                if emission_strength is None:
                    new_emission_strength.default_value = 1.0
        if alpha_blend or alpha_test:
            alpha_output = diffuse.outputs.get('Alpha')
            alpha_input = principled.inputs.get('Alpha')
            if alpha_output is not None and alpha_input is not None:
                if alpha_test and not alpha_blend:
                    clip = tree.nodes.new('ShaderNodeMath')
                    clip.operation = 'GREATER_THAN'
                    clip.inputs[1].default_value = alpha_threshold
                    tree.links.new(alpha_output, clip.inputs[0])
                    tree.links.new(clip.outputs[0], alpha_input)
                else:
                    tree.links.new(alpha_output, alpha_input)
                if hasattr(material, 'surface_render_method'):
                    material.surface_render_method = 'BLENDED' if alpha_blend else 'DITHERED'
        if normal:
            normal_map = tree.nodes.new('ShaderNodeNormalMap')
            tree.links.new(normal.outputs['Color'], normal_map.inputs['Color'])
            tree.links.new(normal_map.outputs['Normal'], principled.inputs['Normal'])
            # Fallout's normal maps store the authored specular strength in
            # alpha. Keep the RGB normal data and export the same image as the
            # glTF KHR_materials_specular scalar texture.
            specular_input = principled.inputs.get('Specular IOR Level')
            normal_alpha = normal.outputs.get('Alpha')
            if specular_input is not None and normal_alpha is not None:
                tree.links.new(normal_alpha, specular_input)
        for link in list(output.inputs['Surface'].links): tree.links.remove(link)
        tree.links.new(principled.outputs['BSDF'], output.inputs['Surface'])
    for mesh in bpy.data.meshes:
        # Bevy supports the primary glTF color stream, but not COLOR_1+.
        while len(mesh.color_attributes) > 1:
            mesh.color_attributes.remove(mesh.color_attributes[-1])
        if mesh.color_attributes:
            mesh.color_attributes.active_color_index = 0
            mesh.color_attributes.render_color_index = 0
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.export_scene.gltf(filepath=output_path, export_format='GLB', export_materials='EXPORT', export_image_format='AUTO', export_apply=True, export_vertex_color='ACTIVE', export_all_vertex_colors=False, export_extras=True)
"#;
    let result = Command::new(blender)
        .arg("--background")
        .arg("--factory-startup")
        .arg("--python-expr")
        .arg(script)
        .arg("--")
        .arg(&job_file)
        .arg(staging_dir)
        .current_dir(data_root)
        .output()?;
    if !result.status.success() {
        bail!(
            "Blender exited with {}:\n{}",
            result.status,
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for job in jobs {
        if !job.output.exists() {
            let stdout_tail = String::from_utf8_lossy(&result.stdout)
                .lines()
                .rev()
                .take(80)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            let stderr_tail = String::from_utf8_lossy(&result.stderr)
                .lines()
                .rev()
                .take(80)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            bail!(
                "Blender reported success but did not create {}\nstdout tail:\n{}\nstderr tail:\n{}",
                job.output.display(),
                stdout_tail,
                stderr_tail
            );
        }
        validate_glb_images(&job.output).with_context(|| {
            format!(
                "converted GLB failed texture validation: {}",
                job.output.display()
            )
        })?;
    }
    Ok(())
}

pub(crate) fn validate_glb_images(path: &Path) -> Result<()> {
    let bytes = fs::read(path)?;
    if bytes.len() < 20 || &bytes[0..4] != b"glTF" {
        bail!("invalid GLB header")
    }
    let json_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let json_start: usize = 20;
    let json_end = json_start
        .checked_add(json_length)
        .context("GLB JSON chunk length overflows")?;
    if json_end > bytes.len() {
        bail!("GLB JSON chunk extends beyond file")
    }
    let document: serde_json::Value = serde_json::from_slice(&bytes[json_start..json_end])?;
    let views = document
        .get("bufferViews")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let images = document
        .get("images")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let binary_start = json_end
        .checked_add(8)
        .context("GLB binary chunk header overflows")?;
    for image in images {
        let Some(view_index) = image.get("bufferView").and_then(serde_json::Value::as_u64) else {
            continue;
        };
        let Some(view) = views.get(view_index as usize) else {
            bail!("image references missing bufferView")
        };
        let offset = view
            .get("byteOffset")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as usize;
        let length = view
            .get("byteLength")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as usize;
        let data_start = binary_start
            .checked_add(offset)
            .context("image buffer offset overflows")?;
        let end = data_start
            .checked_add(length)
            .context("image buffer length overflows")?;
        if end > bytes.len() {
            bail!("image bufferView extends beyond GLB")
        }
        let data = &bytes[data_start..end];
        if data.starts_with(b"\\x89PNG\\r\\n\\x1a\\n") && data.len() >= 24 {
            let width = u32::from_be_bytes(data[16..20].try_into().unwrap());
            let height = u32::from_be_bytes(data[20..24].try_into().unwrap());
            if width <= 1 || height <= 1 {
                let name = image
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("unnamed");
                bail!("image {name} is a 1x1 placeholder")
            }
        }
    }
    Ok(())
}

fn blender_jobs_json(jobs: &[BlenderAssetJob]) -> String {
    let mut out = String::from("[");
    for (index, job) in jobs.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"input\":\"{}\",\"output\":\"{}\",\"model\":\"{}\",\"conversion\":\"{}\"}}",
            json_escape(&job.input.to_string_lossy()),
            json_escape(&job.output.to_string_lossy()),
            json_escape(&job.model),
            job.conversion.profile_tag(),
        ));
    }
    out.push(']');
    out
}

fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub(crate) fn load_archives(
    data_root: &Path,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<Vec<BsaArchive>> {
    let names = ["Fallout - Meshes.bsa", "Fallout - Textures.bsa"];
    let mut archives = Vec::new();
    for name in names {
        let path = data_root.join(name);
        if path.exists() {
            match BsaArchive::open(&path) {
                Ok(archive) => archives.push(archive),
                Err(error) => diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!("could not index {name}: {error}"),
                }),
            }
        }
    }
    Ok(archives)
}

pub(crate) fn resolve_asset(
    data_root: &Path,
    archives: &[BsaArchive],
    normalized: &str,
) -> Result<Option<Vec<u8>>> {
    let candidates = if normalized.starts_with("meshes/") || normalized.starts_with("textures/") {
        vec![normalized.to_string()]
    } else {
        vec![
            normalized.to_string(),
            format!("meshes/{normalized}"),
            format!("textures/{normalized}"),
        ]
    };
    for candidate in candidates {
        let loose = data_root.join(candidate.replace('/', std::path::MAIN_SEPARATOR_STR));
        if loose.exists() {
            return Ok(Some(fs::read(loose)?));
        }
        for archive in archives {
            if let Some(bytes) = archive
                .read(&candidate)
                .with_context(|| format!("reading archive asset {candidate}"))?
            {
                return Ok(Some(bytes));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_length_adjacent_texture_names_in_nif_bytes() {
        let references = texture_references(b"textures\\clutter\\machine\\panel.dds4");
        assert!(references.contains(&"textures/clutter/machine/panel.dds".to_string()));
    }

    #[test]
    fn keeps_texture_paths_with_spaces() {
        let references =
            texture_references(b"textures\\dungeons\\wasteland homes\\Wastehome01.dds3");
        assert!(
            references.contains(&"textures/dungeons/wasteland homes/wastehome01.dds".to_string())
        );
    }

    #[test]
    fn content_addressed_glb_names_are_stable_and_revision_sensitive() {
        let first = content_addressed_glb_name("converter-v1", b"nif-bytes");
        assert_eq!(
            first,
            content_addressed_glb_name("converter-v1", b"nif-bytes")
        );
        assert_ne!(
            first,
            content_addressed_glb_name("converter-v2", b"nif-bytes")
        );
        assert_ne!(
            first,
            content_addressed_glb_name("converter-v1", b"changed-nif")
        );
        assert!(first.ends_with(".glb"));
    }

    #[test]
    fn truncated_cached_glb_is_rejected_without_panicking() {
        let path =
            std::env::temp_dir().join(format!("bevyout-invalid-cache-{}.glb", std::process::id()));
        let mut bytes = vec![0_u8; 20];
        bytes[0..4].copy_from_slice(b"glTF");
        bytes[12..16].copy_from_slice(&1024_u32.to_le_bytes());
        std::fs::write(&path, bytes).unwrap();
        assert!(validate_glb_images(&path).is_err());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn static_assets_use_quick_ao_and_dynamic_assets_preserve_materials() {
        assert_eq!(asset_conversion(true), AssetConversion::QuickAo);
        assert_eq!(asset_conversion(false), AssetConversion::Preserve);
    }

    #[test]
    fn blender_job_json_carries_quick_ao_profile() {
        let json = blender_jobs_json(&[BlenderAssetJob {
            input: PathBuf::from("C:\\staging\\mesh.nif"),
            output: PathBuf::from("C:\\cache\\mesh.glb"),
            model: "architecture/test.nif".into(),
            conversion: AssetConversion::QuickAo,
        }]);
        assert!(json.contains("\"conversion\":\"ao-quick-v1\""));
        assert!(json.contains("architecture/test.nif"));
    }
}
