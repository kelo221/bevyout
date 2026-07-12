import bpy
import json
import math
import os
import sys
import time
from mathutils import Matrix, Quaternion, Vector


# Blender is Z-up while the prepared manifest uses Bevy/glTF coordinates
# (x, y, z) = (Fallout x, Fallout z, -Fallout y).  The glTF exporter applies
# this basis conversion to the final scene, so placement transforms must be
# conjugated into Blender space before the composed scene is exported.
BLENDER_TO_BEVY = Matrix((
    (1.0, 0.0, 0.0, 0.0),
    (0.0, 0.0, 1.0, 0.0),
    (0.0, -1.0, 0.0, 0.0),
    (0.0, 0.0, 0.0, 1.0),
))
BEVY_TO_BLENDER = BLENDER_TO_BEVY.transposed()


def bevy_transform_to_blender(translation, rotation_xyzw, scale=1.0):
    rotation = Quaternion((
        rotation_xyzw[3], rotation_xyzw[0], rotation_xyzw[1], rotation_xyzw[2]
    )).to_matrix().to_4x4()
    bevy = Matrix.Translation(Vector(translation)) @ rotation
    bevy @= Matrix.Diagonal((scale, scale, scale, 1.0))
    return BEVY_TO_BLENDER @ bevy @ BLENDER_TO_BEVY


def clear_scene():
    bpy.ops.object.select_all(action="SELECT")
    bpy.ops.object.delete(use_global=False)
    for collection in (bpy.data.meshes, bpy.data.materials, bpy.data.images,
                       bpy.data.cameras, bpy.data.lights):
        for item in list(collection):
            if item.users == 0:
                collection.remove(item)


def set_emission_scale(material, scale):
    if not material or not material.use_nodes:
        return
    for node in material.node_tree.nodes:
        if node.bl_idname != "ShaderNodeBsdfPrincipled":
            continue
        strength = node.inputs.get("Emission Strength")
        if strength is not None:
            strength.default_value *= scale


def object_uv1(obj, rect, page_size, gutter):
    mesh = obj.data
    if not mesh.uv_layers:
        mesh.uv_layers.new(name="UVMap")
    lightmap = mesh.uv_layers.get("Lightmap")
    if lightmap is None:
        lightmap = mesh.uv_layers.new(name="Lightmap")

    # Fallout material UVs are frequently tiled or overlapping. They are
    # suitable for sampling a diffuse texture, but not for a baked lightmap:
    # two different faces would write to the same texels. Generate a separate
    # non-overlapping chart layout before placing this object in its atlas
    # rectangle. Smart Project is available in Blender 4.x and 5.x and is
    # considerably more reliable here than trying to normalize the primary UV
    # layer. The small fallback keeps unusual/degenerate meshes bakeable while
    # making the problem visible in the Blender log.
    mesh.uv_layers.active_index = list(mesh.uv_layers).index(lightmap)
    bpy.ops.object.select_all(action="DESELECT")
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj
    for polygon in mesh.polygons:
        polygon.select = True
    unwrapped = False
    try:
        if obj.mode != "OBJECT":
            bpy.ops.object.mode_set(mode="OBJECT")
        bpy.ops.object.mode_set(mode="EDIT")
        bpy.ops.mesh.select_all(action="SELECT")
        bpy.ops.uv.smart_project(
            angle_limit=math.radians(66.0),
            island_margin=max(0.001, 2.0 * gutter / float(page_size)),
            area_weight=0.0,
            correct_aspect=True,
            scale_to_bounds=False,
        )
        unwrapped = True
    except (RuntimeError, TypeError) as error:
        print("[bake] smart UV unwrap failed for %s: %s" % (obj.name, error), flush=True)
    finally:
        if obj.mode != "OBJECT":
            bpy.ops.object.mode_set(mode="OBJECT")

    if not unwrapped:
        # Keep a deterministic fallback for meshes Blender cannot unwrap. It
        # is better to retain the source layout than to abort an entire cell;
        # the warning above identifies the mesh for later cleanup.
        lightmap = mesh.uv_layers.get("Lightmap")
        if lightmap is None:
            raise RuntimeError("Lightmap UV layer disappeared during fallback for %s" % obj.name)
        source = mesh.uv_layers[0]
        for loop in mesh.loops:
            lightmap.data[loop.index].uv = source.data[loop.index].uv

    # Blender may invalidate the RNA object held by `lightmap` while leaving
    # edit mode (the smart-project operator temporarily exposes an internal
    # selection attribute). Reacquire the layer before reading its UV data.
    lightmap = mesh.uv_layers.get("Lightmap")
    if lightmap is None:
        raise RuntimeError("Lightmap UV layer disappeared during unwrap for %s" % obj.name)
    values = [lightmap.data[loop.index].uv.copy() for loop in mesh.loops]
    if not values:
        return
    min_x = min(uv.x for uv in values)
    max_x = max(uv.x for uv in values)
    min_y = min(uv.y for uv in values)
    max_y = max(uv.y for uv in values)
    span_x = max(max_x - min_x, 1e-5)
    span_y = max(max_y - min_y, 1e-5)
    inset = gutter / float(page_size)
    x0, y0, x1, y1 = rect
    x0 += inset
    y0 += inset
    x1 -= inset
    y1 -= inset
    for loop in mesh.loops:
        uv = lightmap.data[loop.index].uv
        lightmap.data[loop.index].uv = (
            x0 + (uv.x - min_x) / span_x * (x1 - x0),
            y0 + (uv.y - min_y) / span_y * (y1 - y0),
        )
    mesh.uv_layers.active_index = list(mesh.uv_layers).index(lightmap)


def add_bake_image(material, image):
    if not material or not material.use_nodes:
        return
    tree = material.node_tree
    node = next((node for node in tree.nodes
                 if node.bl_idname == "ShaderNodeTexImage" and node.name == "BevyOutLightmap"), None)
    if node is None:
        node = tree.nodes.new("ShaderNodeTexImage")
        node.name = "BevyOutLightmap"
    node.image = image
    for item in tree.nodes:
        item.select = False
    node.select = True
    tree.nodes.active = node


def mesh_diagonal(obj):
    bounds = [obj.matrix_world @ Vector(corner) for corner in obj.bound_box]
    extent = Vector((
        max(value.x for value in bounds) - min(value.x for value in bounds),
        max(value.y for value in bounds) - min(value.y for value in bounds),
        max(value.z for value in bounds) - min(value.z for value in bounds),
    ))
    return extent.length


NON_RENDERING_PREFIXES = (
    "shadefade",
    "fx",
    "editormarker",
    "marker",
    "collision",
)


def is_non_rendering_object(obj):
    """Identify marker/effect helper meshes that must not reach the bake or GLB."""
    name = obj.name.casefold().replace("_", "").replace(" ", "")
    return name.startswith(NON_RENDERING_PREFIXES)


def is_lightmap_receiver(obj):
    """Keep atlas texels for static surfaces that can visibly benefit from baking.

    Fallout cells contain many tiny props, markers, glow cards and effect meshes.
    Giving every one a unique atlas island makes Cycles trace a large number of
    mostly useless texels. Marker/effect helper meshes are removed entirely;
    other small props remain dynamically lit in the exported scene.
    """
    if obj.type != "MESH" or not obj.data.polygons:
        return False
    if is_non_rendering_object(obj):
        return False
    name = obj.name.casefold()
    if any(token in name for token in ("marker", "fx", "glow", "effect", "collision")):
        return False
    structural_tokens = (
        "shack", "wall", "floor", "ceiling", "door", "roof", "ground", "terrain",
        "rock", "building", "structure", "pillar", "stair", "ramp", "tunnel",
        "bridge", "rail", "platform", "support",
    )
    if any(token in name for token in structural_tokens):
        return True
    # A few architecture meshes have generic names. Keep only genuinely large
    # receivers from that group; furniture and clutter remain dynamically lit.
    return len(obj.data.polygons) >= 1024 or (
        mesh_diagonal(obj) >= 12.0 and len(obj.data.polygons) >= 256
    )


def should_bake_object(obj, job):
    return not is_non_rendering_object(obj) and obj.type == "MESH" and bool(obj.data.polygons) and (
        bool(job.get("bake_all", False)) or is_lightmap_receiver(obj)
    )


def neutralize_intrinsic_vertex_ao(obj):
    """Remove source intrinsic AO from a copied lightmap receiver.

    The receiver's placement-specific atlas already contains local and
    neighbouring geometry occlusion. Dynamic/non-receiver objects keep their
    original vertex colors.
    """
    if obj.get("bevyout_vertex_color_mode", "vertex-preserve") != "vertex-intrinsic-ao-035":
        return
    for attribute in obj.data.color_attributes:
        for item in attribute.data:
            values = tuple(item.color)
            alpha = values[3] if len(values) > 3 else 1.0
            item.color = (1.0, 1.0, 1.0, alpha)


def import_placements(job):
    objects = []
    template_cache = {}
    imported_templates = []
    excluded = 0
    for placement in job["placements"]:
        path = placement["asset_path"]
        if not os.path.isabs(path):
            path = os.path.join(job["asset_root"], path)
        if path not in template_cache:
            before = set(bpy.context.scene.objects)
            result = bpy.ops.import_scene.gltf(filepath=path)
            if "FINISHED" not in result:
                raise RuntimeError("glTF import failed: " + path)
            imported_all = [obj for obj in bpy.context.scene.objects if obj not in before]
            imported = []
            for obj in sorted((obj for obj in imported_all if obj.type == "MESH"),
                              key=lambda obj: obj.name):
                if is_non_rendering_object(obj):
                    excluded += 1
                    continue
                imported.append(obj)
            template_cache[path] = imported
            imported_templates.extend(imported_all)
        templates = template_cache[path]
        placement_matrix = bevy_transform_to_blender(
            placement["translation"],
            placement["rotation_xyzw"],
            placement["scale"],
        )
        local_index = 0
        for template in templates:
            obj = template.copy()
            if not job.get("preview_only", False) and should_bake_object(template, job):
                obj.data = template.data.copy()
            obj.parent = None
            obj.matrix_world = placement_matrix @ template.matrix_world
            bpy.context.collection.objects.link(obj)
            obj["bevyout_reference_form_id"] = placement["reference_form_id"]
            obj["bevyout_vertex_color_mode"] = placement.get("vertex_color_mode", "vertex-preserve")
            objects.append((obj, placement["reference_form_id"], local_index))
            local_index += 1
    for template in imported_templates:
        bpy.data.objects.remove(template, do_unlink=True)
    print("[bake] excluded non-rendering meshes %d" % excluded, flush=True)
    return objects


def render_preview(job, objects):
    scene = bpy.context.scene
    scene.render.engine = "BLENDER_EEVEE"
    # Eevee's screen-space ray tracing and Fast GI are useful for a quick
    # lighting read, but they are intentionally preview-only: off-screen
    # surfaces and final indirect transport still require the Cycles bake.
    eevee = scene.eevee
    eevee.use_raytracing = True
    eevee.ray_tracing_method = "SCREEN"
    eevee.ray_tracing_options.resolution_scale = "2"
    eevee.ray_tracing_options.use_denoise = True
    eevee.use_fast_gi = True
    eevee.fast_gi_method = "GLOBAL_ILLUMINATION"
    eevee.fast_gi_quality = 0.25
    eevee.fast_gi_ray_count = 2
    eevee.fast_gi_step_count = 8
    eevee.gi_diffuse_bounces = 1
    scene.render.resolution_x = 1280
    scene.render.resolution_y = 720
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    minimum = Vector((float("inf"),) * 3)
    maximum = Vector((float("-inf"),) * 3)
    for obj, _, _ in objects:
        for corner in obj.bound_box:
            world_corner = obj.matrix_world @ Vector(corner)
            minimum = Vector(tuple(min(minimum[i], world_corner[i]) for i in range(3)))
            maximum = Vector(tuple(max(maximum[i], world_corner[i]) for i in range(3)))
    center = (minimum + maximum) * 0.5
    radius = max((maximum - minimum).length * 0.5, 10.0)
    camera_data = bpy.data.cameras.new("BevyOutPreviewCamera")
    camera = bpy.data.objects.new("BevyOutPreviewCamera", camera_data)
    bpy.context.collection.objects.link(camera)
    camera.location = center + Vector((radius * 0.8, -radius * 1.4, radius * 0.7))
    camera.rotation_euler = (center - camera.location).to_track_quat("-Z", "Y").to_euler()
    camera_data.lens = 38.0
    camera_data.clip_end = radius * 20.0
    scene.camera = camera
    scene.render.filepath = job["preview_output"]
    bpy.ops.render.render(write_still=True)


def add_lights(job):
    for index, light in enumerate(job["lights"]):
        data = bpy.data.lights.new("BevyOutLight_%04d" % index, "POINT")
        data.energy = max(float(light["intensity_lumens"]) / 683.0, 0.01)
        data.color = tuple(light["color_rgba"][:3])
        data.shadow_soft_size = max(float(light["radius"]) * 0.05, 0.01)
        obj = bpy.data.objects.new(data.name, data)
        bpy.context.collection.objects.link(obj)
        obj.matrix_world = bevy_transform_to_blender(
            light["translation"], light["rotation_xyzw"]
        )


def add_cell_directional_light(job):
    """Add the resolved CELL directional light to the bake scene.

    Rust sends the already-converted Bevy quaternion so this script does not
    duplicate Fallout's rotation convention.  A SUN light uses irradiance;
    Blender's watts are converted from Bevy lux using the same 683 lm/W
    reference used for placed lights.
    """
    illuminance = float(job.get("cell_directional_illuminance", 0.0))
    color = job.get("cell_directional_rgba", [0.0, 0.0, 0.0, 1.0])
    if illuminance <= 1e-6 or sum(float(value) for value in color[:3]) <= 1e-6:
        return
    data = bpy.data.lights.new("BevyOutCellDirectional", "SUN")
    data.energy = max(illuminance / 683.0, 0.01)
    data.color = tuple(float(value) for value in color[:3])
    obj = bpy.data.objects.new(data.name, data)
    bpy.context.collection.objects.link(obj)
    obj.matrix_world = bevy_transform_to_blender(
        [0.0, 0.0, 0.0], job.get("cell_directional_rotation_xyzw", [0.0, 0.0, 0.0, 1.0])
    )


def configure_cycles(scene, requested_device, denoise):
    scene.cycles.use_denoising = denoise
    if denoise and hasattr(scene.cycles, "denoiser"):
        # Explicitly prefer Intel OpenImageDenoise where Blender exposes it;
        # this is independent of whether the render device is CPU or OptiX.
        scene.cycles.denoiser = "OPENIMAGEDENOISE"
    if requested_device == "CPU":
        scene.cycles.device = "CPU"
        return
    preferences = bpy.context.preferences.addons["cycles"].preferences
    preferences.compute_device_type = requested_device
    preferences.get_devices()
    devices = [device for device in preferences.devices if device.type == requested_device]
    if not devices:
        raise RuntimeError("requested Cycles device is unavailable: " + requested_device)
    for device in preferences.devices:
        device.use = device.type == requested_device
    scene.cycles.device = "GPU"


def main(job_path):
    started = time.perf_counter()

    def stage(name):
        print("[bake] %-12s %6.1fs" % (name, time.perf_counter() - started), flush=True)

    with open(job_path, "r", encoding="utf8") as stream:
        job = json.load(stream)
    clear_scene()
    scene = bpy.context.scene
    if job.get("preview_only", False):
        scene.render.engine = "BLENDER_EEVEE"
    else:
        scene.render.engine = "CYCLES"
        configure_cycles(scene, job.get("device", "CPU"), bool(job.get("denoise", True)))
        scene.cycles.samples = int(job["samples"])
        scene.cycles.max_bounces = int(job["bounces"])
        scene.cycles.sample_clamp_indirect = float(job.get("indirect_clamp", 0.0))
        scene.cycles.use_fast_gi = bool(job.get("fast_gi", False))
        if scene.cycles.use_fast_gi:
            scene.cycles.fast_gi_method = "REPLACE"
        if hasattr(scene.cycles, "use_adaptive_sampling"):
            scene.cycles.use_adaptive_sampling = True
        if hasattr(scene.cycles, "adaptive_threshold"):
            scene.cycles.adaptive_threshold = 0.1
        if hasattr(scene.cycles, "max_diffuse_bounces"):
            scene.cycles.max_diffuse_bounces = int(job["bounces"])
        if hasattr(scene.cycles, "max_glossy_bounces"):
            scene.cycles.max_glossy_bounces = 0
        if hasattr(scene.cycles, "max_transmission_bounces"):
            scene.cycles.max_transmission_bounces = 0
        if hasattr(scene.cycles, "max_volume_bounces"):
            scene.cycles.max_volume_bounces = 0
        if hasattr(scene.cycles, "use_caustics"):
            scene.cycles.use_caustics = False
    scene.render.resolution_x = int(job["page_size"])
    scene.render.resolution_y = int(job["page_size"])
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "OPEN_EXR"
    scene.render.image_settings.color_depth = "32"
    scene.render.image_settings.color_mode = "RGBA"
    scene.world.color = tuple(job["ambient_rgba"][:3])
    scene.world.use_nodes = True
    background = scene.world.node_tree.nodes.get("Background")
    if background:
        background.inputs["Color"].default_value = tuple(job["ambient_rgba"][:3]) + (1.0,)
        background.inputs["Strength"].default_value = 0.1

    objects = import_placements(job)
    stage("import")
    add_lights(job)
    add_cell_directional_light(job)
    if not objects:
        raise RuntimeError("no mesh objects were imported")
    if job.get("preview_only", False):
        render_preview(job, objects)
        stage("preview")
        return
    bake_objects = [entry for entry in objects if should_bake_object(entry[0], job)]
    print("[bake] receivers     %d / %d mesh objects" % (len(bake_objects), len(objects)), flush=True)
    if not bake_objects:
        raise RuntimeError("no substantial mesh objects were selected for lightmap baking")
    for obj, _, _ in bake_objects:
        neutralize_intrinsic_vertex_ao(obj)
    image = bpy.data.images.new("BevyOutLightmap", width=job["page_size"],
                                height=job["page_size"], alpha=False, float_buffer=True)
    grid = int(math.ceil(math.sqrt(len(bake_objects))))
    bindings = []
    for index, (obj, reference_id, local_index) in enumerate(bake_objects):
        gx = index % grid
        gy = index // grid
        rect = (gx / grid, gy / grid, (gx + 1) / grid, (gy + 1) / grid)
        object_uv1(obj, rect, job["page_size"], job["gutter"])
        mesh_name = "lm_%08x_%03d" % (reference_id, local_index)
        obj.name = mesh_name
        obj.data.name = mesh_name
        for material in obj.data.materials:
            set_emission_scale(material, float(job["emission_scale"]))
        # Blender and Cycles use an OpenGL-style lower-left image origin;
        # Bevy's Lightmap rect uses the top-left Vulkan convention.
        bindings.append({
            "mesh_name": mesh_name,
            "page": 0,
            "uv_rect": [rect[0], 1.0 - rect[3], rect[2], 1.0 - rect[1]],
        })

    if not job.get("bake_all", False) and len(bake_objects) > 1:
        # A Cycles bake has significant per-object overhead. Quick joins the
        # static receiver meshes after assigning non-overlapping UV islands;
        # the final GLB still contains all props as separate runtime objects.
        bpy.ops.object.select_all(action="DESELECT")
        for obj, _, _ in bake_objects:
            obj.select_set(True)
        bpy.context.view_layer.objects.active = bake_objects[0][0]
        bpy.ops.object.join()
        joined = bake_objects[0][0]
        joined.name = "lm_cell_static"
        joined.data.name = "lm_cell_static"
        for material in joined.data.materials:
            add_bake_image(material, image)
        bake_objects = [(joined, 0, 0)]
        bindings = [{
            "mesh_name": joined.name,
            "page": 0,
            "uv_rect": [0.0, 0.0, 1.0, 1.0],
        }]
    else:
        for obj, _, _ in bake_objects:
            for material in obj.data.materials:
                add_bake_image(material, image)

    stage("uv setup")

    bake_object_ids = {id(obj) for obj, _, _ in bake_objects}
    for obj in bpy.context.scene.objects:
        if id(obj) not in bake_object_ids:
            # Keep props in the exported scene, but remove them from the Cycles
            # dependency graph for Quick. Their runtime Bevy lights remain.
            obj.hide_render = True

    bpy.ops.object.select_all(action="DESELECT")
    for obj, _, _ in bake_objects:
        obj.select_set(True)
    bpy.context.view_layer.objects.active = bake_objects[0][0]
    scene.render.filepath = job["output_exr"]
    pass_filter = {"DIRECT"}
    if job.get("include_indirect", True):
        pass_filter.add("INDIRECT")
    print("[bake] cycles start (passes: %s)" % ",".join(sorted(pass_filter)), flush=True)
    result = bpy.ops.object.bake(type="DIFFUSE", pass_filter=pass_filter,
                                 filepath=job["output_exr"], width=job["page_size"],
                                 height=job["page_size"], margin=job["gutter"],
                                 margin_type="ADJACENT_FACES", use_clear=True)
    if "FINISHED" not in result:
        raise RuntimeError("Cycles lightmap bake failed")
    image.filepath_raw = job["output_exr"]
    image.file_format = "OPEN_EXR"
    image.save()
    if not os.path.exists(job["output_exr"]):
        raise RuntimeError("Cycles lightmap bake did not save the EXR")
    stage("cycles bake")
    for obj in bpy.context.scene.objects:
        obj.hide_render = False

    # The lightmap is a separate Bevy asset. Do not embed the temporary bake
    # image in every material of the composed GLB.
    for material in bpy.data.materials:
        if not material.use_nodes:
            continue
        for node in list(material.node_tree.nodes):
            if node.name == "BevyOutLightmap":
                material.node_tree.nodes.remove(node)
    # Keep the source UV set active for ordinary material textures in the
    # exported GLB. The generated Lightmap layer remains TEXCOORD_1 for
    # Bevy's lightmap shader, but it must not become TEXCOORD_0 just because it
    # was active while Cycles was baking.
    for obj in bpy.context.scene.objects:
        if obj.type == "MESH" and obj.data.uv_layers:
            obj.data.uv_layers.active_index = 0
    if image.users == 0:
        bpy.data.images.remove(image)
    bpy.ops.export_scene.gltf(filepath=job["output_scene"], export_format="GLB",
                              export_materials="EXPORT", export_image_format="AUTO",
                              export_apply=True)
    stage("scene export")
    with open(job["result_json"], "w", encoding="utf8") as stream:
        json.dump({"bindings": bindings}, stream, indent=2)


if __name__ == "__main__":
    if "--" not in sys.argv:
        raise RuntimeError("expected -- job.json")
    main(sys.argv[sys.argv.index("--") + 1])
