import bpy
import hashlib
import json
import math
import os
import struct
import sys
import tempfile
import time
import traceback
from collections import defaultdict
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
STATIC_BATCH_SIZE_METERS = 64.0


def bevy_transform_to_blender(translation, rotation_xyzw, scale=1.0):
    """Conjugate one prepared Bevy transform into Blender's Z-up space.

    Imported GLB objects already carry their asset-local hierarchy transforms.
    Only the placement is conjugated here; the caller composes it once with the
    imported template's world matrix before unlinking the template hierarchy.
    """
    rotation = Quaternion((
        rotation_xyzw[3], rotation_xyzw[0], rotation_xyzw[1], rotation_xyzw[2]
    )).to_matrix().to_4x4()
    bevy = Matrix.Translation(Vector(translation)) @ rotation
    bevy @= Matrix.Diagonal((scale, scale, scale, 1.0))
    return BEVY_TO_BLENDER @ bevy @ BLENDER_TO_BEVY


def stable_value(value):
    """Return a deterministic, JSON-compatible representation of Blender values."""
    if isinstance(value, float):
        return round(value, 8)
    if isinstance(value, (bool, int, str)) or value is None:
        return value
    try:
        return tuple(stable_value(item) for item in value)
    except TypeError:
        return str(value)


def image_content_signature(image, cache):
    if image is None:
        return None
    key = image.as_pointer()
    if key in cache:
        return cache[key]
    digest = hashlib.sha256()
    digest.update(str(tuple(image.size)).encode("ascii"))
    digest.update(image.source.encode("utf8"))
    digest.update(image.colorspace_settings.name.encode("utf8"))
    packed = image.packed_file
    path = bpy.path.abspath(image.filepath_raw or image.filepath)
    if packed is not None:
        digest.update(bytes(packed.data))
    elif path and os.path.isfile(path):
        with open(path, "rb") as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(block)
    else:
        # Generated images are uncommon in converted GLBs. Pixel text is slower
        # than packed bytes, but keeps equivalent generated fixtures canonical.
        digest.update(repr(tuple(round(float(value), 7) for value in image.pixels)).encode("ascii"))
    cache[key] = digest.hexdigest()
    return cache[key]


def shader_socket_signature(socket, image_cache, visiting):
    links = sorted(
        (link for link in socket.links if link.is_valid),
        key=lambda link: (link.from_node.bl_idname, link.from_socket.identifier),
    )
    if not links:
        return ("value", stable_value(getattr(socket, "default_value", None)))
    return tuple(node_signature(link.from_node, image_cache, visiting) for link in links)


def node_signature(node, image_cache, visiting):
    pointer = node.as_pointer()
    if pointer in visiting:
        return (node.bl_idname, "cycle")
    visiting.add(pointer)
    properties = []
    for name in (
        "blend_type", "clamp", "data_type", "extension", "interpolation",
        "invert", "operation", "projection", "projection_blend", "space", "uv_map",
    ):
        if hasattr(node, name):
            properties.append((name, stable_value(getattr(node, name))))
    if node.bl_idname == "ShaderNodeTexImage":
        properties.append(("image", image_content_signature(node.image, image_cache)))
    inputs = tuple(
        (socket.identifier, shader_socket_signature(socket, image_cache, visiting))
        for socket in node.inputs
        if socket.enabled and socket.name != "BevyOutLightmap"
    )
    visiting.remove(pointer)
    return (node.bl_idname, tuple(properties), inputs)


def canonical_material_signature(material, image_cache=None):
    """Describe all render-affecting imported glTF PBR state, independent of names."""
    if material is None:
        return ("default-material",)
    image_cache = image_cache if image_cache is not None else {}
    state = (
        stable_value(material.diffuse_color),
        stable_value(getattr(material, "metallic", 0.0)),
        stable_value(getattr(material, "roughness", 0.5)),
        stable_value(getattr(material, "surface_render_method", None)),
        stable_value(getattr(material, "blend_method", None)),
        stable_value(getattr(material, "alpha_threshold", 0.5)),
        stable_value(getattr(material, "use_backface_culling", False)),
        stable_value(getattr(material, "show_transparent_back", True)),
    )
    if not material.use_nodes or material.node_tree is None:
        return ("material", state, None)
    output = next(
        (node for node in material.node_tree.nodes
         if node.bl_idname == "ShaderNodeOutputMaterial" and node.is_active_output),
        None,
    )
    surface = output.inputs.get("Surface") if output is not None else None
    graph = shader_socket_signature(surface, image_cache, set()) if surface is not None else None
    return ("material", state, graph)


def world_bounds(obj):
    values = [obj.matrix_world @ Vector(corner) for corner in obj.bound_box]
    minimum = Vector(tuple(min(value[axis] for value in values) for axis in range(3)))
    maximum = Vector(tuple(max(value[axis] for value in values) for axis in range(3)))
    return minimum, maximum


def static_chunk(center, size=STATIC_BATCH_SIZE_METERS):
    return tuple(math.floor(float(center[axis]) / size) for axis in range(3))


def fits_static_chunk(extent, size=STATIC_BATCH_SIZE_METERS):
    return all(float(extent[axis]) <= size + 1e-6 for axis in range(3))


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
    # A physical bulb promoted from a LightGlow card must remain bright at
    # runtime.  The regular emission scale is only for static bake lighting;
    # applying it to the bulb would remove the HDR value that drives Bloom.
    if material.get("bevyout_emissive_bulb", False):
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


def neutralize_quick_vertex_ao(obj):
    """Remove generated quick AO from a copied lightmap receiver.

    The receiver's placement-specific atlas already contains local and
    neighbouring geometry occlusion. Dynamic/non-receiver objects keep their
    original vertex colors.
    """
    if obj.get("bevyout_ao_mode", "ao-none") != "ao-quick-v1":
        return
    for attribute in obj.data.color_attributes:
        for item in attribute.data:
            values = tuple(item.color)
            alpha = values[3] if len(values) > 3 else 1.0
            item.color = (1.0, 1.0, 1.0, alpha)


def require_renderable_visual_templates(placement, path, templates):
    visual_templates = [
        obj for obj in templates
        if obj.type == "MESH" and bool(obj.data.polygons)
        and not obj.get("bevyout_collision", False)
        and not is_non_rendering_object(obj)
    ]
    if not visual_templates:
        raise RuntimeError(
            "placement %08X imported no renderable visual meshes from %s"
            % (int(placement["reference_form_id"]), path)
        )
    return visual_templates


def stamp_placement_provenance(obj, reference_form_id):
    vertex_attribute = obj.data.attributes.new(
        "bevyout_reference_vertex_id", type="INT", domain="POINT"
    )
    for item in vertex_attribute.data:
        item.value = int(reference_form_id)
    face_attribute = obj.data.attributes.new(
        "bevyout_reference_face_id", type="INT", domain="FACE"
    )
    for item in face_attribute.data:
        item.value = int(reference_form_id)


def placement_geometry(reference_form_id, objects):
    minimum = Vector((float("inf"),) * 3)
    maximum = Vector((float("-inf"),) * 3)
    for obj in objects:
        for vertex in obj.data.vertices:
            world_vertex = obj.matrix_world @ vertex.co
            minimum = Vector(tuple(
                min(minimum[i], world_vertex[i]) for i in range(3)
            ))
            maximum = Vector(tuple(
                max(maximum[i], world_vertex[i]) for i in range(3)
            ))
    return {
        "reference_form_id": int(reference_form_id),
        "visual_meshes": len(objects),
        "vertices": sum(len(obj.data.vertices) for obj in objects),
        "triangles": sum(len(obj.data.loop_triangles) for obj in objects),
        "world_bounds_min": list(minimum),
        "world_bounds_max": list(maximum),
    }


def placement_fragment_adjustment(reference_form_id, template_name):
    adjusted_fragments = {
        0x0002943E: (":32",),
        0x00029522: (":32", ":41"),
        0x000AB2FD: (":32", ":41"),
        0x000AB30D: (":32",),
    }
    if template_name.endswith(adjusted_fragments.get(int(reference_form_id), ())):
        return Matrix.Rotation(-math.pi, 4, "Z")
    return Matrix.Identity(4)


def import_placements(job):
    objects = []
    contributed_reference_form_ids = []
    template_cache = {}
    imported_templates = []
    excluded = 0
    placement_objects = defaultdict(list)
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
                if is_non_rendering_object(obj) and not obj.get("bevyout_collision", False):
                    excluded += 1
                    continue
                imported.append(obj)
            template_cache[path] = imported
            imported_templates.extend(imported_all)
        templates = template_cache[path]
        require_renderable_visual_templates(placement, path, templates)
        placement_matrix = bevy_transform_to_blender(
            placement["translation"],
            placement["rotation_xyzw"],
            placement["scale"],
        )
        local_index = 0
        for template in templates:
            obj = template.copy()
            batchable_static = bool(placement.get("batchable_static", False))
            if not job.get("preview_only", False) and (
                should_bake_object(template, job)
                or (batchable_static and not template.get("bevyout_collision", False))
            ):
                obj.data = template.data.copy()
            # The imported template matrix is the asset-local GLB/NIF hierarchy.
            # Apply the prepared placement exactly once, then detach so Blender
            # does not apply the imported parent chain a second time on export.
            obj.parent = None
            obj.matrix_world = (
                placement_matrix
                @ template.matrix_world
                @ placement_fragment_adjustment(
                    placement["reference_form_id"], template.name
                )
            )
            bpy.context.collection.objects.link(obj)
            obj["bevyout_reference_form_id"] = placement["reference_form_id"]
            obj["bevyout_ao_mode"] = placement.get("ao_mode", "ao-none")
            obj["bevyout_batchable_static"] = batchable_static
            objects.append((obj, placement["reference_form_id"], local_index))
            if not obj.get("bevyout_collision", False) and not is_non_rendering_object(obj):
                obj.data.calc_loop_triangles()
                stamp_placement_provenance(obj, placement["reference_form_id"])
                placement_objects[int(placement["reference_form_id"])].append(obj)
            local_index += 1
        contributed_reference_form_ids.append(int(placement["reference_form_id"]))
    for template in imported_templates:
        bpy.data.objects.remove(template, do_unlink=True)
    print("[bake] excluded non-rendering meshes %d" % excluded, flush=True)
    geometry = [
        placement_geometry(reference_form_id, placement_objects[reference_form_id])
        for reference_form_id in contributed_reference_form_ids
    ]
    return objects, {
        "expected_placements": len(job["placements"]),
        "contributed_placements": len(contributed_reference_form_ids),
        "reference_form_ids": contributed_reference_form_ids,
        "post_batch_verified": False,
        "placements": geometry,
    }


def verify_post_batch_placement_geometry(expected_geometry):
    actual = defaultdict(lambda: {
        "vertices": 0,
        "triangles": 0,
        "minimum": Vector((float("inf"),) * 3),
        "maximum": Vector((float("-inf"),) * 3),
    })
    for obj in visual_mesh_objects():
        vertex_ids = obj.data.attributes.get("bevyout_reference_vertex_id")
        face_ids = obj.data.attributes.get("bevyout_reference_face_id")
        if vertex_ids is None or face_ids is None:
            raise RuntimeError("static batching dropped placement provenance attributes")
        for vertex, item in zip(obj.data.vertices, vertex_ids.data):
            record = actual[int(item.value)]
            world_vertex = obj.matrix_world @ vertex.co
            record["vertices"] += 1
            record["minimum"] = Vector(tuple(
                min(record["minimum"][axis], world_vertex[axis]) for axis in range(3)
            ))
            record["maximum"] = Vector(tuple(
                max(record["maximum"][axis], world_vertex[axis]) for axis in range(3)
            ))
        for polygon, item in zip(obj.data.polygons, face_ids.data):
            actual[int(item.value)]["triangles"] += max(0, polygon.loop_total - 2)

    tolerance = 0.001
    for expected in expected_geometry:
        reference_form_id = int(expected["reference_form_id"])
        record = actual.get(reference_form_id)
        expected_minimum = Vector(expected["world_bounds_min"])
        expected_maximum = Vector(expected["world_bounds_max"])
        if record is None or (
            record["vertices"] < int(expected["vertices"])
            or record["triangles"] != int(expected["triangles"])
            or (record["minimum"] - expected_minimum).length > tolerance
            or (record["maximum"] - expected_maximum).length > tolerance
        ):
            raise RuntimeError(
                "static batching changed geometry for placement %08X: expected vertices >= %d, triangles %d, bounds %s..%s; got vertices %s, triangles %s, bounds %s..%s"
                % (
                    reference_form_id,
                    int(expected["vertices"]),
                    int(expected["triangles"]),
                    tuple(expected_minimum),
                    tuple(expected_maximum),
                    None if record is None else record["vertices"],
                    None if record is None else record["triangles"],
                    None if record is None else tuple(record["minimum"]),
                    None if record is None else tuple(record["maximum"]),
                )
            )


def render_preview(job, objects):
    scene = bpy.context.scene
    for obj, _, _ in objects:
        if obj.get("bevyout_collision", False):
            obj.hide_render = True
    set_eevee_engine(scene)
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


def set_eevee_engine(scene):
    for engine in ("BLENDER_EEVEE_NEXT", "BLENDER_EEVEE"):
        try:
            scene.render.engine = engine
            return engine
        except TypeError:
            continue
    raise RuntimeError("this Blender build does not expose a supported EEVEE engine")


def irradiance_volume_bounds(objects, spacing):
    minimum = Vector((float("inf"),) * 3)
    maximum = Vector((float("-inf"),) * 3)
    for obj in objects:
        object_minimum, object_maximum = world_bounds(obj)
        minimum = Vector(tuple(min(minimum[i], object_minimum[i]) for i in range(3)))
        maximum = Vector(tuple(max(maximum[i], object_maximum[i]) for i in range(3)))
    if not objects:
        raise RuntimeError("cannot create an irradiance volume without renderable meshes")
    # Put the outermost probes one spacing inside the scene bounds. This gives
    # interpolation room at walls without adding a second, mostly empty cell.
    padding = Vector((float(spacing),) * 3)
    minimum -= padding
    maximum += padding
    extent = maximum - minimum
    resolution = tuple(
        max(2, int(math.ceil(float(extent[axis]) / float(spacing))) + 1)
        for axis in range(3)
    )
    center = (minimum + maximum) * 0.5
    return center, extent, resolution


def bake_irradiance_volume(job, objects):
    if tuple(bpy.app.version[:2]) != (4, 5):
        raise RuntimeError(
            "irradiance volume baking is pinned to Blender 4.5 LTS; found %s"
            % bpy.app.version_string
        )
    spacing = max(float(job.get("irradiance_spacing_meters", 8.0)), 0.01)
    center, extent, resolution = irradiance_volume_bounds(objects, spacing)
    bpy.ops.object.select_all(action="DESELECT")
    bpy.ops.object.lightprobe_add(type="VOLUME", location=center)
    probe = bpy.context.object
    probe.name = "BevyOutIrradianceVolume"
    probe.data.name = "BevyOutIrradianceVolume"
    probe.scale = extent
    probe.data.resolution_x = resolution[0]
    probe.data.resolution_y = resolution[1]
    probe.data.resolution_z = resolution[2]
    probe.data.bake_samples = int(job.get("irradiance_samples", 64))
    probe.data.surfel_density = 1
    probe.data.capture_world = False
    probe.data.capture_indirect = True
    probe.data.capture_emission = True
    if hasattr(probe.data, "visibility_buffer_bias"):
        probe.data.visibility_buffer_bias = 0.5
    bpy.context.view_layer.objects.active = probe
    probe.select_set(True)
    print(
        "[bake] irradiance volume bounds center=%s extent=%s resolution=%s samples=%d"
        % (tuple(round(value, 2) for value in center),
           tuple(round(value, 2) for value in extent),
           resolution, probe.data.bake_samples),
        flush=True,
    )
    result = bpy.ops.object.lightprobe_cache_bake(subset="ACTIVE")
    if "FINISHED" not in result:
        raise RuntimeError("Blender irradiance volume cache bake failed")
    # The cache is stored in Blender's DNA rather than exposed through the
    # Python API. Save uncompressed and let Rust extract its SH coefficients.
    bpy.ops.wm.save_as_mainfile(filepath=job["irradiance_blend"], compress=False)
    if not os.path.exists(job["irradiance_blend"]):
        raise RuntimeError("Blender irradiance volume bake did not save its .blend cache")
    return {
        "blend_path": job["irradiance_blend"],
        "resolution": list(resolution),
        "translation": [float(center.x), float(center.z), float(-center.y)],
        "rotation_xyzw": [0.0, 0.0, 0.0, 1.0],
        "scale": [float(extent.x), float(extent.z), float(extent.y)],
    }


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


def used_materials(obj):
    indices = sorted({polygon.material_index for polygon in obj.data.polygons})
    materials = []
    for index in indices:
        materials.append(obj.data.materials[index] if index < len(obj.data.materials) else None)
    return materials or [None]


def visual_mesh_objects():
    return [
        obj for obj in bpy.context.scene.objects
        if obj.type == "MESH" and obj.data.polygons
        and not obj.get("bevyout_collision", False)
        and not is_non_rendering_object(obj)
    ]


def render_primitive_count(objects):
    return sum(len(used_materials(obj)) for obj in objects)


def referenced_material_count(objects):
    return len({
        material.as_pointer()
        for obj in objects
        for material in used_materials(obj)
        if material is not None
    })


def split_object_by_material(obj):
    if len(used_materials(obj)) <= 1:
        return [obj]
    before = set(bpy.context.scene.objects)
    if bpy.context.object and bpy.context.object.mode != "OBJECT":
        bpy.ops.object.mode_set(mode="OBJECT")
    bpy.ops.object.select_all(action="DESELECT")
    obj.select_set(True)
    bpy.context.view_layer.objects.active = obj
    bpy.ops.object.mode_set(mode="EDIT")
    bpy.ops.mesh.select_all(action="SELECT")
    bpy.ops.mesh.separate(type="MATERIAL")
    bpy.ops.object.mode_set(mode="OBJECT")
    fragments = [obj] + [
        item for item in bpy.context.scene.objects
        if item not in before and item.type == "MESH"
    ]
    for index, fragment in enumerate(sorted(fragments, key=lambda item: item.name)):
        bpy.ops.object.select_all(action="DESELECT")
        fragment.select_set(True)
        bpy.context.view_layer.objects.active = fragment
        bpy.ops.object.material_slot_remove_unused()
        fragment.name = "%s_mat_%02d" % (obj.name, index)
        fragment.data.name = fragment.name
    return fragments


def remap_fragment_material(obj, material):
    obj.data.materials.clear()
    if material is not None:
        obj.data.materials.append(material)
    for polygon in obj.data.polygons:
        polygon.material_index = 0


def join_static_group(group, name, material):
    for obj in group:
        remap_fragment_material(obj, material)
    bpy.ops.object.select_all(action="DESELECT")
    for obj in group:
        obj.select_set(True)
    joined = group[0]
    bpy.context.view_layer.objects.active = joined
    bpy.ops.object.join()
    joined.name = name
    joined.data.name = name
    joined["bevyout_batch_size"] = len(group)
    if joined.get("bevyout_lightmapped", False):
        joined["bevyout_lightmap_rect"] = [0.0, 0.0, 1.0, 1.0]
    return joined


def batch_static_meshes(chunk_size=STATIC_BATCH_SIZE_METERS):
    """Batch static render fragments while preserving collision and spatial culling."""
    before = visual_mesh_objects()
    stats = {
        "chunk_size_meters": float(chunk_size),
        "visual_objects_before": len(before),
        "render_primitives_before": render_primitive_count(before),
        "materials_before": referenced_material_count(before),
        "batches_created": 0,
        "largest_batch": 0,
        "excluded_collision": sum(
            1 for obj in bpy.context.scene.objects
            if obj.type == "MESH" and obj.data.polygons
            and obj.get("bevyout_collision", False)
        ),
        "excluded_large": 0,
        "excluded_non_static": sum(
            1 for obj in before if not obj.get("bevyout_batchable_static", False)
        ),
    }

    image_cache = {}
    representatives = {}
    groups = defaultdict(list)
    candidates = [obj for obj in before if obj.get("bevyout_batchable_static", False)]
    fragments = []
    for obj in candidates:
        fragments.extend(split_object_by_material(obj))

    for fragment in fragments:
        minimum, maximum = world_bounds(fragment)
        extent = maximum - minimum
        if not fits_static_chunk(extent, chunk_size):
            stats["excluded_large"] += 1
            continue
        material = used_materials(fragment)[0]
        signature = canonical_material_signature(material, image_cache)
        representative = representatives.setdefault(signature, material)
        remap_fragment_material(fragment, representative)
        center = (minimum + maximum) * 0.5
        lightmapped = bool(fragment.get("bevyout_lightmapped", False))
        groups[(static_chunk(center, chunk_size), signature, lightmapped)].append(fragment)

    for (chunk, signature, lightmapped), group in sorted(
        groups.items(), key=lambda item: (item[0][0], repr(item[0][1]), item[0][2])
    ):
        if len(group) < 2:
            continue
        digest = hashlib.sha256(repr(signature).encode("utf8")).hexdigest()[:10]
        name = "batch_%s_%d_%d_%d_%s" % (
            "lm" if lightmapped else "dyn", chunk[0], chunk[1], chunk[2], digest
        )
        join_static_group(group, name, representatives[signature])
        stats["batches_created"] += 1
        stats["largest_batch"] = max(stats["largest_batch"], len(group))

    after = visual_mesh_objects()
    stats.update({
        "visual_objects_after": len(after),
        "render_primitives_after": render_primitive_count(after),
        "materials_after": referenced_material_count(after),
    })
    print(
        "[bake] static batch %.1f m chunks: objects %d -> %d, primitives %d -> %d, materials %d -> %d, batches %d (largest %d)"
        % (
            stats["chunk_size_meters"],
            stats["visual_objects_before"], stats["visual_objects_after"],
            stats["render_primitives_before"], stats["render_primitives_after"],
            stats["materials_before"], stats["materials_after"],
            stats["batches_created"], stats["largest_batch"],
        ),
        flush=True,
    )
    return stats


def lightmap_bindings():
    bindings = []
    for obj in sorted(visual_mesh_objects(), key=lambda item: item.name):
        if not obj.get("bevyout_lightmapped", False):
            continue
        obj.data.name = obj.name
        rect = list(obj.get("bevyout_lightmap_rect", [0.0, 0.0, 1.0, 1.0]))
        bindings.append({"mesh_name": obj.name, "page": 0, "uv_rect": rect})
    return bindings


def self_test_material(name, color, image=None):
    material = bpy.data.materials.new(name)
    material.use_nodes = True
    principled = next(
        node for node in material.node_tree.nodes
        if node.bl_idname == "ShaderNodeBsdfPrincipled"
    )
    principled.inputs["Base Color"].default_value = color
    if image is not None:
        texture = material.node_tree.nodes.new("ShaderNodeTexImage")
        texture.image = image
        material.node_tree.links.new(texture.outputs["Color"], principled.inputs["Base Color"])
    return material


def self_test_cube(name, location, material, *, batchable=True, lightmapped=True, size=1.0):
    bpy.ops.mesh.primitive_cube_add(size=size, location=location)
    obj = bpy.context.object
    obj.name = name
    obj.data.name = name
    obj.data.materials.append(material)
    obj["bevyout_batchable_static"] = batchable
    obj["bevyout_lightmapped"] = lightmapped
    obj["bevyout_lightmap_rect"] = [0.0, 0.0, 1.0, 1.0]
    obj.data.uv_layers.new(name="Lightmap")
    color = obj.data.color_attributes.new(
        name="BevyOutQuickAO", type="FLOAT_COLOR", domain="CORNER"
    )
    for item in color.data:
        item.color = (0.8, 0.8, 0.8, 1.0)
    obj.data.color_attributes.active_color_index = list(obj.data.color_attributes).index(color)
    return obj


def glb_mesh_attributes(path):
    with open(path, "rb") as stream:
        magic, version, _ = struct.unpack("<III", stream.read(12))
        if magic != 0x46546C67 or version != 2:
            raise AssertionError("self-test export is not a glTF 2 GLB")
        json_length, json_type = struct.unpack("<II", stream.read(8))
        if json_type != 0x4E4F534A:
            raise AssertionError("self-test GLB has no JSON chunk")
        document = json.loads(stream.read(json_length).decode("utf8").rstrip("\x00 "))
    meshes = document.get("meshes", [])
    return {
        node.get("name", ""): [
            set(primitive.get("attributes", {}))
            for primitive in meshes[node["mesh"]].get("primitives", [])
        ]
        for node in document.get("nodes", [])
        if "mesh" in node
    }


def matrix_max_error(actual, expected):
    return max(
        abs(float(actual[row][column]) - float(expected[row][column]))
        for row in range(4)
        for column in range(4)
    )


def run_transform_self_test():
    translation = (12.5, -3.25, 8.75)
    rotation = Quaternion((0.91, 0.17, -0.28, 0.22)).normalized()
    rotation_xyzw = (rotation.x, rotation.y, rotation.z, rotation.w)
    scale = 1.75
    expected_bevy = (
        Matrix.Translation(Vector(translation))
        @ rotation.to_matrix().to_4x4()
        @ Matrix.Diagonal((scale, scale, scale, 1.0))
    )
    blender = bevy_transform_to_blender(translation, rotation_xyzw, scale)
    recovered_bevy = BLENDER_TO_BEVY @ blender @ BEVY_TO_BLENDER
    assert matrix_max_error(recovered_bevy, expected_bevy) < 1e-5

    template_bevy = (
        Matrix.Translation(Vector((-2.0, 1.5, 0.75)))
        @ Quaternion((0.97, -0.11, 0.08, 0.19)).normalized().to_matrix().to_4x4()
    )
    template = BEVY_TO_BLENDER @ template_bevy @ BLENDER_TO_BEVY
    composed = blender @ template
    recovered_composed = BLENDER_TO_BEVY @ composed @ BEVY_TO_BLENDER
    expected_composed = expected_bevy @ template_bevy
    assert matrix_max_error(recovered_composed, expected_composed) < 1e-5
    assert matrix_max_error(composed, blender @ blender @ template) > 1e-4

    clear_scene()
    material = self_test_material("transform_material", (0.6, 0.7, 0.8, 1.0))
    bpy.ops.mesh.primitive_cube_add(size=2.0)
    cube = bpy.context.object
    cube.data.materials.append(material)
    cube.matrix_world = composed
    minimum, maximum = world_bounds(cube)
    with tempfile.TemporaryDirectory() as directory:
        output = os.path.join(directory, "transform_fixture.glb")
        bpy.ops.export_scene.gltf(
            filepath=output, export_format="GLB", export_materials="EXPORT",
            export_image_format="AUTO", export_apply=True, export_extras=True,
        )
        assert os.path.isfile(output)
        clear_scene()
        bpy.ops.import_scene.gltf(filepath=output)
        imported = next(obj for obj in bpy.context.scene.objects if obj.type == "MESH")
        imported_minimum, imported_maximum = world_bounds(imported)
        assert (imported_minimum - minimum).length < 1e-5
        assert (imported_maximum - maximum).length < 1e-5
    clear_scene()
    print("[bake-test] transform conjugation/export fixtures passed", flush=True)


def run_self_tests():
    clear_scene()
    run_transform_self_test()
    adjusted = placement_fragment_adjustment(0x000AB2FD, "VDnWallEndCorInR01:32")
    assert matrix_max_error(adjusted, Matrix.Rotation(-math.pi, 4, "Z")) < 1e-6
    adjusted_floor = placement_fragment_adjustment(0x000AB2FD, "VDnWallEndCorInR01:41")
    assert matrix_max_error(adjusted_floor, Matrix.Rotation(-math.pi, 4, "Z")) < 1e-6
    assert matrix_max_error(
        placement_fragment_adjustment(0x000AB2FD, "VDnWallEndCorInR01:13"),
        Matrix.Identity(4),
    ) < 1e-6
    assert matrix_max_error(
        placement_fragment_adjustment(0x0002943E, "VDnWallEndCorOutR01:32"),
        Matrix.Rotation(-math.pi, 4, "Z"),
    ) < 1e-6
    material = self_test_material("contribution_material", (0.5, 0.5, 0.5, 1.0))
    visual = self_test_cube("contribution_visual", (0.0, 0.0, 0.0), material)
    assert visual.type == "MESH" and bool(visual.data.polygons)
    visual["bevyout_collision"] = True
    try:
        require_renderable_visual_templates(
            {"reference_form_id": 0x54426}, "collision-only.glb", [visual]
        )
        raise AssertionError("collision-only placement was accepted")
    except RuntimeError as error:
        assert "00054426" in str(error)
        assert "collision-only.glb" in str(error)
    visual["bevyout_collision"] = False
    assert require_renderable_visual_templates(
        {"reference_form_id": 0x54426}, "visual.glb", [visual]
    ) == [visual]
    clear_scene()
    assert static_chunk(Vector((63.999, 0.0, -0.001))) == (0, 0, -1)
    assert static_chunk(Vector((64.0, -64.0, 0.0))) == (1, -1, 0)
    assert fits_static_chunk(Vector((64.0, 1.0, 1.0)))
    assert not fits_static_chunk(Vector((64.001, 1.0, 1.0)))

    image_a = bpy.data.images.new("canonical_a", width=1, height=1)
    image_b = bpy.data.images.new("canonical_b", width=1, height=1)
    image_a.pixels = [0.25, 0.5, 0.75, 1.0]
    image_b.pixels = [0.25, 0.5, 0.75, 1.0]
    material_a = self_test_material("material_a", (1.0, 1.0, 1.0, 1.0), image_a)
    material_b = self_test_material("material_b", (1.0, 1.0, 1.0, 1.0), image_b)
    material_other = self_test_material("material_other", (0.2, 0.3, 0.4, 0.5))
    assert canonical_material_signature(material_a) == canonical_material_signature(material_b)
    assert canonical_material_signature(material_a) != canonical_material_signature(material_other)

    self_test_cube("equivalent_a", (0.5, 0.5, 0.5), material_a)
    self_test_cube("equivalent_b", (1.5, 0.5, 0.5), material_b)
    multi = self_test_cube("multi_material", (2.5, 0.5, 0.5), material_a)
    multi.data.materials.append(material_other)
    for index, polygon in enumerate(multi.data.polygons):
        if index % 2 == 0:
            polygon.material_index = 1
    self_test_cube("different_material", (3.5, 0.5, 0.5), material_other)
    self_test_cube("other_chunk", (65.0, 0.5, 0.5), material_a)
    self_test_cube("large_static", (160.0, 0.5, 0.5), material_a, size=65.0)
    self_test_cube("non_static", (4.5, 0.5, 0.5), material_a, batchable=False)
    collision = self_test_cube("physics_surface", (0.5, 0.5, -1.0), material_a)
    collision["bevyout_collision"] = True
    collision["bevyout_havok_material"] = 17

    fixture_geometry = []
    for index, obj in enumerate(visual_mesh_objects()):
        reference_form_id = 0x54426 + index
        obj.data.calc_loop_triangles()
        stamp_placement_provenance(obj, reference_form_id)
        fixture_geometry.append(placement_geometry(reference_form_id, [obj]))
    stats = batch_static_meshes()
    verify_post_batch_placement_geometry(fixture_geometry)
    assert stats["chunk_size_meters"] == 64.0
    assert stats["batches_created"] >= 2
    assert stats["largest_batch"] >= 2
    assert stats["excluded_collision"] == 1
    assert stats["excluded_large"] == 1
    assert stats["excluded_non_static"] == 1
    assert collision.name in bpy.context.scene.objects
    assert collision["bevyout_havok_material"] == 17
    assert all(
        "Lightmap" in obj.data.uv_layers
        for obj in visual_mesh_objects()
        if obj.get("bevyout_lightmapped", False)
    )

    bindings = lightmap_bindings()
    bound_names = {binding["mesh_name"] for binding in bindings}
    assert bound_names
    assert bound_names == {
        obj.name for obj in visual_mesh_objects()
        if obj.get("bevyout_lightmapped", False)
    }
    expected_names = set(bound_names)
    with tempfile.TemporaryDirectory() as directory:
        output = os.path.join(directory, "batch_fixture.glb")
        bpy.ops.export_scene.gltf(
            filepath=output, export_format="GLB", export_materials="EXPORT",
            export_image_format="AUTO", export_apply=True, export_extras=True,
        )
        assert os.path.isfile(output)
        attributes = glb_mesh_attributes(output)
        assert expected_names.issubset(attributes)
        for name in expected_names:
            assert attributes[name]
            assert all("TEXCOORD_1" in item for item in attributes[name])
            assert all("COLOR_0" in item for item in attributes[name])
        clear_scene()
        bpy.ops.import_scene.gltf(filepath=output)
        imported_meshes = {
            obj.data.name: obj for obj in bpy.context.scene.objects if obj.type == "MESH"
        }
        assert expected_names.issubset(imported_meshes)
        for name in expected_names:
            assert len(imported_meshes[name].data.uv_layers) >= 2
            assert imported_meshes[name].data.color_attributes
        imported_collision = next(
            obj for obj in bpy.context.scene.objects
            if obj.get("bevyout_collision", False)
        )
        assert imported_collision.get("bevyout_havok_material") == 17
    print("[bake-test] static batching fixtures passed", flush=True)

    clear_scene()
    material = self_test_material("irradiance_material", (0.6, 0.7, 0.8, 1.0))
    bpy.ops.mesh.primitive_cube_add(size=2.0, location=(0.0, 0.0, 0.0))
    cube = bpy.context.object
    cube.data.materials.append(material)
    bpy.ops.object.light_add(type="POINT", location=(2.0, -2.0, 3.0))
    bpy.context.object.data.energy = 1000.0
    with tempfile.TemporaryDirectory() as directory:
        irradiance = bake_irradiance_volume(
            {
                "irradiance_spacing_meters": 8.0,
                "irradiance_samples": 1,
                "irradiance_blend": os.path.join(directory, "irradiance.blend"),
            },
            [cube],
        )
        assert tuple(irradiance["resolution"]) == (4, 4, 4)
        assert os.path.isfile(irradiance["blend_path"])
    print("[bake-test] irradiance volume fixture passed", flush=True)


def main(job_path):
    started = time.perf_counter()

    def stage(name):
        print("[bake] %-12s %6.1fs" % (name, time.perf_counter() - started), flush=True)

    with open(job_path, "r", encoding="utf8") as stream:
        job = json.load(stream)
    clear_scene()
    scene = bpy.context.scene
    set_eevee_engine(scene)
    scene.render.resolution_percentage = 100
    scene.world.color = tuple(job["ambient_rgba"][:3])
    scene.world.use_nodes = True
    background = scene.world.node_tree.nodes.get("Background")
    if background:
        background.inputs["Color"].default_value = tuple(job["ambient_rgba"][:3]) + (1.0,)
        background.inputs["Strength"].default_value = 0.1

    objects, placement_contribution = import_placements(job)
    stage("import")
    add_lights(job)
    add_cell_directional_light(job)
    if not objects:
        raise RuntimeError("no mesh objects were imported")
    if job.get("preview_only", False):
        render_preview(job, objects)
        stage("preview")
        return

    irradiance = bake_irradiance_volume(job, [obj for obj, _, _ in objects])
    stage("irradiance bake")

    batching = batch_static_meshes(float(job.get(
        "static_batch_chunk_meters", STATIC_BATCH_SIZE_METERS
    )))
    verify_post_batch_placement_geometry(placement_contribution["placements"])
    placement_contribution["post_batch_verified"] = True
    stage("static batch")

    # Keep the source UV set active for ordinary material textures in the
    # exported GLB. The irradiance volume is independent of mesh UVs.
    for obj in bpy.context.scene.objects:
        if obj.type == "MESH" and obj.data.uv_layers:
            obj.data.uv_layers.active_index = 0
    bpy.ops.export_scene.gltf(filepath=job["output_scene"], export_format="GLB",
                              export_materials="EXPORT", export_image_format="AUTO",
                              export_apply=True, export_extras=True)
    stage("scene export")
    with open(job["result_json"], "w", encoding="utf8") as stream:
        json.dump({
            "irradiance": irradiance,
            "batching": batching,
            "placement_contribution": placement_contribution,
        }, stream, indent=2)


if __name__ == "__main__":
    try:
        if "--" not in sys.argv:
            raise RuntimeError("expected -- job.json")
        argument = sys.argv[sys.argv.index("--") + 1]
        if argument == "--self-test":
            run_self_tests()
        else:
            main(argument)
    except Exception:
        traceback.print_exc()
        sys.exit(1)
