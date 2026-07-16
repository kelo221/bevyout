import bpy, gzip, json, math, os, sys
from mathutils import Matrix, Vector
def patch_niftools_blender52():
    from io_scene_niftools.modules.nif_import.geometry.vertex import Vertex
    from io_scene_niftools.modules.nif_import.property.nodes_wrapper import NodesWrapper
    from io_scene_niftools.modules.nif_import.collision import Collision, get_material
    from io_scene_niftools.modules.nif_import.collision.bound import Bound
    from io_scene_niftools.modules.nif_import.collision.havok import BhkCollision
    from io_scene_niftools.modules.nif_import.property.geometry.niproperty import NiPropertyProcessor
    from io_scene_niftools.modules.nif_import.property.material import Material, NiMaterial
    from io_scene_niftools.modules.nif_import.property.shader.bsshaderproperty import BSShaderPropertyProcessor
    from io_scene_niftools.modules.nif_import.geometry.vertex.groups import VertexGroup
    from io_scene_niftools.modules.nif_import.constraint import Constraint
    from nifgen.formats.nif.bshavok.niobjects.BhkConstraint import BhkConstraint
    def preserve_emission_multiplier(material, key, value):
        try:
            value = float(value)
        except (TypeError, ValueError):
            return False
        if not math.isfinite(value) or value < 0.0:
            return False
        material[key] = value
        material['bevyout_emissive_strength'] = value
        return True

    def nif_emission_multiplier(prop, primary_name, *fallback_names):
        value = getattr(prop, primary_name, None)
        if value is not None:
            return value
        for name in fallback_names:
            value = getattr(prop, name, None)
            if value is not None:
                return value
        return None

    # MeshPropertyProcessor binds process_nimaterial_property when its
    # singledispatch table is constructed. Patch the material importer itself
    # so the multiplier survives regardless of when that table was built.
    original_ni_material_import = NiMaterial.import_material
    def import_ni_material_with_emission(self, n_block, b_mat, n_mat_prop):
        result = original_ni_material_import(self, n_block, b_mat, n_mat_prop)
        preserve_emission_multiplier(
            b_mat,
            'bevyout_nimaterial_emit_multi',
            nif_emission_multiplier(n_mat_prop, 'emit_multi', 'emissive_mult'),
        )
        return result
    NiMaterial.import_material = import_ni_material_with_emission

    original_process_nimaterial_property = NiPropertyProcessor.process_nimaterial_property
    def process_nimaterial_property_with_emission(self, prop):
        original_process_nimaterial_property(self, prop)
        preserve_emission_multiplier(
            self.b_mat,
            'bevyout_nimaterial_emit_multi',
            nif_emission_multiplier(prop, 'emit_multi', 'emissive_mult'),
        )
    NiPropertyProcessor.process_nimaterial_property = process_nimaterial_property_with_emission

    original_import_bs_lighting_shader_property = BSShaderPropertyProcessor.import_bs_lighting_shader_property
    def import_bs_lighting_shader_property_with_emission(self, prop):
        original_import_bs_lighting_shader_property(self, prop)
        preserve_emission_multiplier(
            self._b_mat,
            'bevyout_bslighting_emissive_multiple',
            getattr(prop, 'emissive_multiple', None),
        )
    BSShaderPropertyProcessor.import_bs_lighting_shader_property = import_bs_lighting_shader_property_with_emission

    original_import_bs_effect_shader_property = BSShaderPropertyProcessor.import_bs_effect_shader_property
    def import_bs_effect_shader_property_with_emission(self, prop):
        original_import_bs_effect_shader_property(self, prop)
        preserve_emission_multiplier(
            self._b_mat,
            'bevyout_bseffect_base_color_scale',
            getattr(prop, 'base_color_scale', None),
        )
    BSShaderPropertyProcessor.import_bs_effect_shader_property = import_bs_effect_shader_property_with_emission

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
    def enum_name(value, default=''):
        return getattr(value, 'name', str(value) if value is not None else default)
    def vec3_value(value, default=(0.0, 0.0, 0.0)):
        if value is None: return list(default)
        return [float(getattr(value, axis, default[index])) for index, axis in enumerate(('x', 'y', 'z'))]
    def scaled_vec3(value, scale):
        return [component * scale for component in vec3_value(value)]
    original_rigid_body = BhkCollision._import_bhk_rigid_body
    def import_rigid_body_metadata(self, bhkshape, collision_objs):
        original_rigid_body(self, bhkshape, collision_objs)
        body_info = bhkshape.rigid_body_info
        next_group = getattr(self, '_bevyout_body_group', 0)
        self._bevyout_body_group = next_group + 1
        layer_filter = getattr(body_info, 'havok_filter', None)
        body_filter = getattr(bhkshape, 'havok_filter', None)
        layer = getattr(getattr(layer_filter, 'layer', None), 'value', 1)
        flags = getattr(body_filter, 'flags', 0)
        constrained = bool(getattr(bhkshape, 'constraints', None))
        inertia = getattr(body_info, 'inertia_tensor', None)
        source_inertia = [[float(getattr(inertia, f'm_{row}{column}', 0.0))
                           for column in (1, 2, 3)] for row in (1, 2, 3)]
        # Conjugate by the Blender/NIF -> Bevy basis (x, z, -y) and convert
        # Havok distance-squared units to the scaled GLB units.
        inertia_scale = self.HAVOK_SCALE * self.HAVOK_SCALE
        inertia_rows = [
            [source_inertia[0][0], source_inertia[0][2], -source_inertia[0][1]],
            [source_inertia[2][0], source_inertia[2][2], -source_inertia[2][1]],
            [-source_inertia[1][0], -source_inertia[1][2], source_inertia[1][1]],
        ]
        inertia_rows = [[value * inertia_scale for value in row] for row in inertia_rows]
        metadata = {
            'bevyout_body_group': next_group,
            'bevyout_motion_type': enum_name(getattr(body_info, 'motion_system', None), 'MO_SYS_FIXED'),
            'bevyout_quality_type': enum_name(getattr(body_info, 'quality_type', None), 'MO_QUAL_FIXED'),
            'bevyout_mass': float(getattr(body_info, 'mass', 0.0)),
            'bevyout_center_of_mass': scaled_vec3(getattr(body_info, 'center', None), self.HAVOK_SCALE),
            'bevyout_inertia': [value for row in inertia_rows for value in row],
            'bevyout_linear_velocity': scaled_vec3(getattr(body_info, 'linear_velocity', None), self.HAVOK_SCALE),
            'bevyout_angular_velocity': vec3_value(getattr(body_info, 'angular_velocity', None)),
            'bevyout_gravity_factor': float(getattr(body_info, 'gravity_factor', 1.0)),
            'bevyout_linear_damping': float(getattr(body_info, 'linear_damping', 0.0)),
            'bevyout_angular_damping': float(getattr(body_info, 'angular_damping', 0.0)),
            'bevyout_friction': float(getattr(body_info, 'friction', 0.8)),
            'bevyout_restitution': float(getattr(body_info, 'restitution', 0.0)),
            'bevyout_max_linear_velocity': float(getattr(body_info, 'max_linear_velocity', 0.0)) * self.HAVOK_SCALE,
            'bevyout_max_angular_velocity': float(getattr(body_info, 'max_angular_velocity', 0.0)),
            'bevyout_sleep_enabled': enum_name(getattr(body_info, 'deactivator_type', None)) != 'DEACTIVATOR_NEVER',
            'bevyout_ccd_enabled': 'BULLET' in enum_name(getattr(body_info, 'quality_type', None)).upper(),
            'bevyout_layer': int(layer),
            'bevyout_filter_flags': int(flags),
            'bevyout_constrained': constrained,
        }
        for obj in collision_objs:
            for key, value in metadata.items(): obj[key] = value
    BhkCollision._import_bhk_rigid_body = import_rigid_body_metadata
    original_phantom = BhkCollision.import_bhk_simple_shape_phantom
    def import_phantom_metadata(self, bhkshape):
        objects = original_phantom(self, bhkshape)
        transform_value = getattr(bhkshape, 'transform', None)
        if transform_value is not None:
            try:
                transform = __import__('mathutils').Matrix(transform_value.as_list())
                transform.translation = transform.translation * self.HAVOK_SCALE
                for obj in objects: obj.matrix_local = obj.matrix_local @ transform
            except Exception:
                pass
        for obj in objects: obj['bevyout_phantom'] = True
        return objects
    BhkCollision.import_bhk_simple_shape_phantom = import_phantom_metadata
    def mark_shape(original, kind):
        def wrapped(self, bhkshape):
            objects = original(self, bhkshape)
            for obj in objects:
                obj['bevyout_havok_shape_kind'] = kind
            return objects
        return wrapped
    BhkCollision.import_bhkbox_shape = mark_shape(BhkCollision.import_bhkbox_shape, 'Box')
    BhkCollision.import_bhksphere_shape = mark_shape(BhkCollision.import_bhksphere_shape, 'Sphere')
    BhkCollision.import_bhkcapsule_shape = mark_shape(BhkCollision.import_bhkcapsule_shape, 'Capsule')
    BhkCollision.import_bhkconvex_vertices_shape = mark_shape(BhkCollision.import_bhkconvex_vertices_shape, 'ConvexHull')
    BhkCollision.import_bhkpackednitristrips_shape = mark_shape(BhkCollision.import_bhkpackednitristrips_shape, 'TriangleMesh')
    BhkCollision.import_bhk_nitristrips_shape = mark_shape(BhkCollision.import_bhk_nitristrips_shape, 'TriangleMesh')
    BhkCollision.import_nitristrips = mark_shape(BhkCollision.import_nitristrips, 'TriangleMesh')
    # Issue #57: Blender 5.x's slotted actions removed Action.fcurves, which
    # niftools' animation importer (Animation.create_action/create_fcurves)
    # still assumes. These shims route curve creation through the
    # layer/strip/channelbag API and restore a read-only Action.fcurves for
    # the addon's two remaining read sites. Spiked on vdoorsliding01.nif with
    # Blender 5.1.2 + niftools v0.1.1 (see M2_WAVE3_PLAN.md's spike decision).
    from io_scene_niftools.modules.nif_import.animation import Animation
    def action_channelbag(action, id_type='OBJECT'):
        slot = action.slots[0] if len(action.slots) else action.slots.new(id_type=id_type, name='BevyOut')
        layer = action.layers[0] if len(action.layers) else action.layers.new('BevyOut')
        strip = layer.strips[0] if len(layer.strips) else layer.strips.new(type='KEYFRAME')
        return strip.channelbag(slot, ensure=True), slot
    original_create_action = Animation.create_action
    def create_action_compat(self, b_obj, action_name):
        b_action = original_create_action(self, b_obj, action_name)
        # Morph controllers animate shape-key (KEY) datablocks, not objects;
        # the slot type must match or the action_slot assignment raises.
        _, slot = action_channelbag(b_action, getattr(b_obj, 'id_type', 'OBJECT'))
        if b_obj.animation_data:
            b_obj.animation_data.action_slot = slot
        return b_action
    def create_fcurves_compat(self, action, dtype, drange, flags, bone_name=None, key_name=None):
        channelbag, _ = action_channelbag(action, 'KEY' if key_name else 'OBJECT')
        if bone_name:
            specs = [(f'pose.bones["{bone_name}"].{dtype}', i) for i in drange]
        elif key_name:
            specs = [(f'key_blocks["{key_name}"].{dtype}', 0)]
        else:
            specs = [(dtype, i) for i in drange]
        fcurves = [channelbag.fcurves.new(data_path=path, index=index) for path, index in specs]
        if flags:
            self.set_extrapolation(self.get_extend_from_flags(flags), fcurves)
        return fcurves
    def action_fcurves_compat(self):
        if not (len(self.slots) and len(self.layers) and len(self.layers[0].strips)):
            return []
        bag = self.layers[0].strips[0].channelbag(self.slots[0])
        return bag.fcurves if bag else []
    Animation.create_action = create_action_compat
    Animation.create_fcurves = create_fcurves_compat
    bpy.types.Action.fcurves = property(action_fcurves_compat)
    # With animation=True the addon also imports material (UV/alpha ramp)
    # controllers, and its get_controller_data crashes on NiBlendFloat
    # interpolators (no `.data`). We only consume node transform sequences
    # (Open/Close), so material animation import is disabled wholesale.
    from io_scene_niftools.modules.nif_import.animation.material import MaterialAnimation
    MaterialAnimation.import_material_controllers = lambda self, n_geom, b_mat: None
    # Same `.data` assumption crashes on NiBlendBoolInterpolator visibility
    # controllers; hide_viewport animation is never exported to the GLB, so
    # skip it, and let every other caller see None instead of crashing (the
    # transform importer isinstance-guards its NiKeyframeData use).
    from io_scene_niftools.modules.nif_import.animation.object import ObjectAnimation
    ObjectAnimation.import_visibility = lambda self, n_node, b_obj: None
    # Morph (shape-key) controllers also assume interpolator `.data` and
    # shape keys cannot ride this GLB export (export_apply=True flattens
    # meshes), so skip them entirely.
    from io_scene_niftools.modules.nif_import.animation.morph import MorphAnimation
    MorphAnimation.import_morph_controller = lambda self, n_node, b_obj: None
    original_get_controller_data = Animation.get_controller_data
    def get_controller_data_compat(ctrl):
        if hasattr(ctrl, 'interpolator') and ctrl.interpolator and not hasattr(ctrl.interpolator, 'data'):
            return None
        return original_get_controller_data(ctrl)
    Animation.get_controller_data = staticmethod(get_controller_data_compat)

def blender_point_to_bevy(point):
    return [float(point.x), float(point.z), float(-point.y)]

def blender_vector_to_bevy(vector):
    return Vector((vector.x, vector.z, -vector.y))

def mesh_points(obj):
    return [blender_point_to_bevy(obj.matrix_world @ vertex.co) for vertex in obj.data.vertices]

def shape_from_collision_object(obj):
    kind = obj.get('bevyout_havok_shape_kind')
    if not kind:
        kind = {'SPHERE': 'Sphere', 'CAPSULE': 'Capsule', 'BOX': 'Box',
                'CONVEX_HULL': 'ConvexHull', 'MESH': 'TriangleMesh'}.get(
                    getattr(obj.rigid_body, 'collision_shape', ''), 'TriangleMesh')
    if kind == 'Sphere':
        radius = float(getattr(obj.rigid_body, 'collision_margin', 0.0))
        scale = max(abs(value) for value in obj.matrix_world.to_scale())
        return {'kind': 'Sphere', 'center': blender_point_to_bevy(obj.matrix_world.translation),
                'radius': radius * scale}
    if kind == 'Capsule':
        radius = float(getattr(obj.rigid_body, 'collision_margin', 0.0))
        zs = [corner[2] for corner in obj.bound_box]
        half_segment = max(0.0, (max(zs) - min(zs)) * 0.5 - radius)
        point1 = obj.matrix_world @ Vector((0.0, 0.0, -half_segment))
        point2 = obj.matrix_world @ Vector((0.0, 0.0, half_segment))
        scale = max(abs(obj.matrix_world.to_scale().x), abs(obj.matrix_world.to_scale().y))
        return {'kind': 'Capsule', 'point1': blender_point_to_bevy(point1),
                'point2': blender_point_to_bevy(point2), 'radius': radius * scale}
    if kind == 'Box':
        mins = Vector((min(corner[i] for corner in obj.bound_box) for i in range(3)))
        maxs = Vector((max(corner[i] for corner in obj.bound_box) for i in range(3)))
        center = obj.matrix_world @ ((mins + maxs) * 0.5)
        half = (maxs - mins) * 0.5
        basis = obj.matrix_world.to_3x3()
        sx, sy, sz = (basis @ Vector((1, 0, 0))).length, (basis @ Vector((0, 1, 0))).length, (basis @ Vector((0, 0, 1))).length
        axis_x = blender_vector_to_bevy(basis @ Vector((1, 0, 0))).normalized()
        axis_y = blender_vector_to_bevy(basis @ Vector((0, 0, 1))).normalized()
        axis_z = blender_vector_to_bevy(basis @ Vector((0, -1, 0))).normalized()
        rotation = __import__('mathutils').Matrix((axis_x, axis_y, axis_z)).transposed().to_quaternion()
        return {'kind': 'Box', 'center': blender_point_to_bevy(center),
                'half_extents': [float(half.x * sx), float(half.z * sz), float(half.y * sy)],
                'rotation_xyzw': [float(rotation.x), float(rotation.y), float(rotation.z), float(rotation.w)]}
    points = mesh_points(obj)
    if kind == 'ConvexHull':
        return {'kind': 'ConvexHull', 'points': points}
    indices = []
    for polygon in obj.data.polygons:
        vertices = list(polygon.vertices)
        for index in range(1, len(vertices) - 1):
            indices.extend((int(vertices[0]), int(vertices[index]), int(vertices[index + 1])))
    return {'kind': 'TriangleMesh', 'vertices': points, 'indices': indices}

def body_owner_node(obj):
    parent = obj.parent
    while parent is not None:
        if not parent.get('bevyout_collision', False):
            return parent.name
        parent = parent.parent
    return None

def physics_body_from_objects(group_id, objects):
    first = objects[0]
    flat_inertia = list(first.get('bevyout_inertia', [0.0] * 9))
    while len(flat_inertia) < 9: flat_inertia.append(0.0)
    def basis_vector(values):
        values = list(values) if values is not None else [0.0, 0.0, 0.0]
        return [float(values[0]), float(values[2]), float(-values[1])]
    shapes = [shape_from_collision_object(obj) for obj in objects]
    shapes = [shape for shape in shapes if (
        (shape['kind'] == 'TriangleMesh' and len(shape['vertices']) >= 3 and len(shape['indices']) >= 3)
        or (shape['kind'] == 'ConvexHull' and len(shape['points']) >= 4)
        or shape['kind'] in {'Box', 'Sphere', 'Capsule'})]
    return {
        'group_id': int(group_id),
        'node': body_owner_node(first),
        'motion_type': str(first.get('bevyout_motion_type', 'MO_SYS_FIXED')),
        'quality_type': str(first.get('bevyout_quality_type', 'MO_QUAL_FIXED')),
        'mass': float(first.get('bevyout_mass', 0.0)),
        'center_of_mass': basis_vector(first.get('bevyout_center_of_mass')),
        'inertia': [flat_inertia[0:3], flat_inertia[3:6], flat_inertia[6:9]],
        'linear_velocity': basis_vector(first.get('bevyout_linear_velocity')),
        'angular_velocity': basis_vector(first.get('bevyout_angular_velocity')),
        'gravity_factor': float(first.get('bevyout_gravity_factor', 1.0)),
        'linear_damping': max(0.0, float(first.get('bevyout_linear_damping', 0.0))),
        'angular_damping': max(0.0, float(first.get('bevyout_angular_damping', 0.0))),
        'friction': max(0.0, float(first.get('bevyout_friction', 0.8))),
        'restitution': max(0.0, float(first.get('bevyout_restitution', 0.0))),
        'max_linear_velocity': max(0.0, float(first.get('bevyout_max_linear_velocity', 0.0))),
        'max_angular_velocity': max(0.0, float(first.get('bevyout_max_angular_velocity', 0.0))),
        'sleep_enabled': bool(first.get('bevyout_sleep_enabled', True)),
        'ccd_enabled': bool(first.get('bevyout_ccd_enabled', False)),
        'layer': int(first.get('bevyout_layer', 1)),
        'filter_flags': int(first.get('bevyout_filter_flags', 0)),
        'material': int(first['bevyout_havok_material']) if 'bevyout_havok_material' in first else None,
        'material_name': str(first['bevyout_havok_material_name']) if 'bevyout_havok_material_name' in first else None,
        'phantom': any(bool(obj.get('bevyout_phantom', False)) for obj in objects),
        'constrained': bool(first.get('bevyout_constrained', False)),
        'shapes': shapes,
    }

def body_blocks_player(body):
    return (not body['phantom'] and not (body['filter_flags'] & 0x40)
            and body['layer'] not in {0, 8, 12, 15, 16, 18, 21, 22, 23, 24, 25,
                                     29, 30, 31, 33, 34, 35, 36, 37, 38, 39, 40, 43})

def render_fallback_body(objects):
    vertices, indices = [], []
    for obj in objects:
        offset = len(vertices)
        vertices.extend(mesh_points(obj))
        for polygon in obj.data.polygons:
            polygon_vertices = list(polygon.vertices)
            for index in range(1, len(polygon_vertices) - 1):
                indices.extend((offset + int(polygon_vertices[0]),
                                offset + int(polygon_vertices[index]),
                                offset + int(polygon_vertices[index + 1])))
    return {
        'group_id': 0, 'motion_type': 'MO_SYS_FIXED', 'quality_type': 'MO_QUAL_FIXED',
        'mass': 0.0, 'center_of_mass': [0.0, 0.0, 0.0],
        'inertia': [[0.0, 0.0, 0.0]] * 3,
        'linear_velocity': [0.0, 0.0, 0.0], 'angular_velocity': [0.0, 0.0, 0.0],
        'gravity_factor': 1.0, 'linear_damping': 0.0, 'angular_damping': 0.0,
        'friction': 0.8, 'restitution': 0.0, 'max_linear_velocity': 0.0,
        'max_angular_velocity': 0.0, 'sleep_enabled': True, 'ccd_enabled': False,
        'layer': 1, 'filter_flags': 0, 'material': None, 'material_name': None,
        'phantom': False, 'constrained': False,
        'shapes': [{'kind': 'TriangleMesh', 'vertices': vertices, 'indices': indices}],
    }

def fallback_material_eligible(material):
    if material is None or not material.use_nodes:
        return False
    alpha_flags = int(getattr(getattr(material, 'niftools_alpha', None), 'alphaflag', 0))
    if alpha_flags & (1 | (1 << 9)):
        return False
    if material.get('bevyout_emissive_bulb', False):
        return False
    for node in material.node_tree.nodes:
        if node.bl_idname == 'ShaderNodeTexImage' and node.image:
            name = node.image.name.casefold()
            label = node.label.casefold()
            if '_g.' in name or '_em.' in name or 'glow' in name or 'emiss' in name or 'glow' in label or 'emiss' in label:
                return False
        if node.bl_idname == 'ShaderNodeBsdfPrincipled':
            emission = node.inputs.get('Emission Color') or node.inputs.get('Emission')
            strength = node.inputs.get('Emission Strength')
            if emission and (emission.links or max(emission.default_value[:3]) > 0.0):
                if strength is None or strength.default_value > 0.0:
                    return False
    return True

def authored_emission_color(material):
    """Return the imported NIFTools emission color when it is authored."""
    niftools = getattr(material, 'niftools', None)
    color = getattr(niftools, 'emissive_color', None)
    if color is None:
        return None
    try:
        values = tuple(float(channel) for channel in color[:3])
    except (TypeError, ValueError):
        return None
    if len(values) != 3 or not all(math.isfinite(channel) for channel in values):
        return None
    return values if any(channel != 0.0 for channel in values) else None

def source_emission_multiplier(material):
    """Return the active NIF shader multiplier and whether it was authored."""
    shader = getattr(material, 'niftools_shader', None)
    shader_type = str(getattr(shader, 'bs_shadertype', ''))
    keys = {
        'BSLightingShaderProperty': 'bevyout_bslighting_emissive_multiple',
        'BSEffectShaderProperty': 'bevyout_bseffect_base_color_scale',
    }
    if shader_type in keys:
        candidates = [keys[shader_type], 'bevyout_emissive_strength']
    else:
        candidates = [
            'bevyout_emissive_strength',
            'bevyout_nimaterial_emit_multi',
            'bevyout_bslighting_emissive_multiple',
            'bevyout_bseffect_base_color_scale',
        ]
    for key in candidates:
        value = material.get(key)
        if value is None:
            continue
        try:
            value = float(value)
        except (TypeError, ValueError):
            continue
        if math.isfinite(value) and value >= 0.0:
            # NIFTools' default emit_multi is 1.0. When a material has an
            # authored nonzero color, let the matching source block below
            # recover a more specific multiplier instead of accepting that
            # default as the final value.
            if value == 1.0 and authored_emission_color(material) is not None:
                continue
            return value, True
    # Some NIFTools import paths construct their material dispatch table
    # before the compatibility patch is installed. Recover the source value
    # directly from the loaded NIF property when the authored color uniquely
    # identifies one source material.
    authored = authored_emission_color(material)
    if authored is not None:
        matches = []
        for block in NifData.data.blocks:
            block_name = type(block).__name__
            if block_name == 'NiMaterialProperty':
                color_value = getattr(block, 'emissive_color', None)
                strength_value = getattr(block, 'emit_multi', None)
                if strength_value is None:
                    strength_value = getattr(block, 'emissive_mult', None)
            elif block_name == 'BSLightingShaderProperty':
                color_value = getattr(block, 'emissive_color', None)
                strength_value = getattr(block, 'emissive_multiple', None)
            elif block_name == 'BSEffectShaderProperty':
                color_value = getattr(block, 'base_color', None)
                strength_value = getattr(block, 'base_color_scale', None)
            else:
                continue
            if color_value is None:
                continue
            source_color = tuple(float(getattr(color_value, channel))
                                 for channel in ('r', 'g', 'b'))
            if any(abs(source_color[index] - authored[index]) > 1e-6
                   for index in range(3)):
                continue
            try:
                strength_value = float(strength_value)
            except (TypeError, ValueError):
                continue
            if math.isfinite(strength_value) and strength_value >= 0.0:
                matches.append(strength_value)
        if matches and all(value == matches[0] for value in matches):
            return matches[0], True
    return 1.0, False

def build_physics_asset():
    collision_objects = [obj for obj in bpy.context.scene.objects
                         if obj.type == 'MESH' and obj.get('bevyout_collision', False)]
    groups = {}
    for index, obj in enumerate(collision_objects):
        group = int(obj.get('bevyout_body_group', 1000000 + index))
        groups.setdefault(group, []).append(obj)
    authored_bodies = [physics_body_from_objects(group, objects)
                       for group, objects in sorted(groups.items())]
    authored_bodies = [body for body in authored_bodies if body['shapes']]
    if any(body_blocks_player(body) and body['shapes'] for body in authored_bodies):
        return {'schema_version': 1, 'source': 'AuthoredHavok', 'bodies': authored_bodies}
    render_objects = [obj for obj in bpy.context.scene.objects
                      if obj.type == 'MESH' and len(obj.data.polygons)
                      and not obj.get('bevyout_collision', False)
                      and obj.visible_get()
                      and obj.data.materials
                      and all(fallback_material_eligible(material) for material in obj.data.materials)]
    fallback = render_fallback_body(render_objects)
    if not fallback['shapes'][0]['indices']:
        return {'schema_version': 1, 'source': 'GeneratedRender', 'bodies': authored_bodies}
    return {'schema_version': 1, 'source': 'GeneratedRender',
            'bodies': authored_bodies + [fallback]}

def matrix_identity_error(matrix):
    return max(abs(float(matrix[row][column]) - (1.0 if row == column else 0.0))
               for row in range(4) for column in range(4))

def matrix_max_error(left, right):
    return max(abs(float(left[row][column]) - float(right[row][column]))
               for row in range(4) for column in range(4))

def apply_verified_spatial_corrections(objects, model):
    corrections = {
        'dungeons/vault/room/vdnwallendcorinr01.nif': (':32', ':41'),
        'dungeons/vault/room/vdnwallendcoroutr01.nif': (':32',),
    }.get(str(model).casefold(), ())
    verified = []
    verified_collision = 0
    correction = Matrix.Rotation(-math.pi, 4, 'Z')
    for obj in objects:
        if obj.type != 'MESH':
            continue
        if obj.get('bevyout_collision', False):
            if corrections:
                expected = obj.matrix_local @ correction
                obj.matrix_local = expected
                bpy.context.view_layer.update()
                if matrix_max_error(obj.matrix_local, expected) > 1e-5:
                    raise RuntimeError('collision spatial correction failed for ' + obj.name)
                obj['bevyout_spatial_policy'] = 'verified_local_z_180'
                obj['bevyout_spatial_verified'] = True
                verified_collision += 1
            continue
        niftools = getattr(obj, 'niftools', None)
        name = str(niftools.longname if niftools and niftools.longname else obj.name)
        if not name.endswith(corrections):
            continue
        before = obj.matrix_local.copy()
        expected = before @ correction
        obj.matrix_local = expected
        bpy.context.view_layer.update()
        if matrix_max_error(obj.matrix_local, expected) > 1e-5:
            raise RuntimeError('spatial correction failed for ' + name)
        obj['bevyout_spatial_policy'] = 'verified_local_z_180'
        obj['bevyout_spatial_verified'] = True
        verified.append(name)
    if len(verified) != len(corrections):
        raise RuntimeError(
            'spatial correction coverage mismatch for %s: expected %d, verified %d'
            % (model, len(corrections), len(verified))
        )
    return sorted(verified), verified_collision

def apply_record_zero_transform_policy(
        objects, model, policy, record_zero_name, record_zero_is_node):
    """Annotate the NIF root and apply the Rust-selected compatibility policy.

    NIFTools imports a record-0 NiNode branch as a top-level EMPTY or ARMATURE.
    Rust owns the normalized-model policy registry. Blender records the original
    transform for cache-hit audits and only resets roots explicitly marked as a
    verified discard. Bip01 remains protected regardless of the supplied policy.
    """
    record_zero_name = str(record_zero_name)
    if policy not in {
            'preserve_review_required', 'preserve_verified',
            'preserve_verified', 'discard_verified'}:
        raise RuntimeError('unknown root transform policy: ' + str(policy))
    changed = []
    candidates = []
    for obj in sorted(objects, key=lambda item: item.name):
        if obj.parent is not None:
            continue
        niftools = getattr(obj, 'niftools', None)
        imported_name = (niftools.longname if niftools and niftools.longname
                         else obj.name)
        if imported_name.casefold() == record_zero_name.casefold():
            candidates.insert(0, obj)
        else:
            candidates.append(obj)
    if not candidates:
        return changed
    carrier = candidates[0]
    original = carrier.matrix_local.copy()
    non_identity = matrix_identity_error(original) > 1e-5
    carrier['bevyout_source_model'] = str(model)
    carrier['bevyout_root_transform_policy'] = str(policy)
    carrier['bevyout_record_zero_non_identity'] = non_identity
    carrier['bevyout_record_zero_transform'] = [
        float(original[row][column])
        for row in range(4) for column in range(4)
    ]
    if (policy == 'discard_verified'
            and record_zero_is_node
            and carrier.type in {'EMPTY', 'ARMATURE'}
            and record_zero_name.casefold() != 'bip01'):
        if non_identity:
            changed.append(carrier.name)
        carrier.matrix_local = Matrix.Identity(4)
    return changed

def run_root_transform_self_test():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    root = bpy.data.objects.new('FixtureRoot', None)
    child = bpy.data.objects.new('FixtureChild', None)
    other_root = bpy.data.objects.new('OtherRoot', None)
    bip01 = bpy.data.objects.new('Bip01', None)
    mesh = bpy.data.objects.new('GeometryRoot', bpy.data.meshes.new('GeometryRoot'))
    for obj in (root, child, other_root, bip01, mesh):
        bpy.context.collection.objects.link(obj)
    root.niftools.nodetype = 'NiNode'
    other_root.niftools.nodetype = 'NiNode'
    bip01.niftools.nodetype = 'NiNode'
    root.rotation_euler = (math.pi, 0.0, 0.0)
    child.parent = root
    child.rotation_euler = (math.pi, 0.0, 0.0)
    other_root.rotation_euler = (0.0, -0.5, 0.0)
    bip01.rotation_euler = (0.0, 0.0, 0.5)
    mesh.rotation_euler = (0.0, 0.25, 0.0)
    bpy.context.view_layer.update()

    assert apply_record_zero_transform_policy(
        list(bpy.context.scene.objects),
        'dungeons/rivetcity/roomsmall/rcsmdoor01.nif',
        'preserve_review_required',
        'FixtureRoot',
        True,
    ) == []
    assert matrix_identity_error(root.matrix_local) > 0.1
    assert root['bevyout_source_model'] == 'dungeons/rivetcity/roomsmall/rcsmdoor01.nif'
    assert root['bevyout_root_transform_policy'] == 'preserve_review_required'
    assert root['bevyout_record_zero_non_identity']
    assert len(root['bevyout_record_zero_transform']) == 16

    changed = apply_record_zero_transform_policy(
        list(bpy.context.scene.objects),
        'dungeons/vault/room/vrmwallscreen01.nif',
        'discard_verified',
        'FixtureRoot',
        True,
    )
    bpy.context.view_layer.update()
    assert changed == ['FixtureRoot'], (changed, [
        (obj.name, obj.type, obj.parent.name if obj.parent else None,
         obj.niftools.nodetype, matrix_identity_error(obj.matrix_local))
        for obj in bpy.context.scene.objects
    ])
    assert matrix_identity_error(root.matrix_local) < 1e-6
    assert matrix_identity_error(child.matrix_local) > 0.1
    assert matrix_identity_error(other_root.matrix_local) > 0.1
    assert matrix_identity_error(bip01.matrix_local) > 0.1
    assert matrix_identity_error(mesh.matrix_local) > 0.1
    root.rotation_euler = (0.0, math.pi, 0.0)
    bpy.context.view_layer.update()
    changed = apply_record_zero_transform_policy(
        list(bpy.context.scene.objects),
        'dungeons/vault/room/vdnwallendcoroutr01.nif',
        'preserve_verified',
        'FixtureRoot',
        True,
    )
    bpy.context.view_layer.update()
    assert changed == []
    assert matrix_identity_error(root.matrix_local) > 0.1
    assert root['bevyout_source_model'] == 'dungeons/vault/room/vdnwallendcoroutr01.nif'
    assert root['bevyout_root_transform_policy'] == 'preserve_verified'
    assert apply_record_zero_transform_policy(
        list(bpy.context.scene.objects),
        'architecture/geometryroot.nif',
        'discard_verified',
        'GeometryRoot',
        False,
    ) == []
    assert mesh['bevyout_source_model'] == 'architecture/geometryroot.nif'
    assert matrix_identity_error(mesh.matrix_local) > 0.1
    assert apply_record_zero_transform_policy(
        list(bpy.context.scene.objects),
        'dungeons/vault/room/vrmwallscreen01.nif',
        'discard_verified',
        'Bip01',
        True,
    ) == []
    assert matrix_identity_error(bip01.matrix_local) > 0.1
    print('[convert-test] model root transform policy passed', flush=True)

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
from io_scene_niftools.utils.singleton import NifData
from nifgen.formats.nif import classes as NifClasses

def collect_animation_sound_cues():
    cues = []
    for block in NifData.data.blocks:
        if not isinstance(block, NifClasses.NiControllerSequence):
            continue
        sequence = str(block.name).strip()
        text_keys = getattr(block, 'text_keys', None)
        if not text_keys:
            continue
        for key in text_keys.text_keys:
            text = str(key.value).replace('\r', '\n')
            for line in text.split('\n'):
                prefix, separator, value = line.partition(':')
                if separator and prefix.strip().casefold() == 'sound':
                    editor_id = value.strip()
                    if editor_id:
                        cues.append({
                            'sequence': sequence,
                            'time': float(key.time),
                            'editor_id': editor_id,
                        })
    cues.sort(key=lambda cue: (
        cue['sequence'].casefold(), cue['time'], cue['editor_id'].casefold(), cue['editor_id']
    ))
    return cues

if sys.argv[-1] == '--self-test-root-policy':
    run_root_transform_self_test()
    raise SystemExit(0)
bpy.context.preferences.filepaths.texture_directory = os.path.abspath(sys.argv[-1])
with open(sys.argv[-2], 'r', encoding='utf8') as f: jobs=json.load(f)
for job in jobs:
    nif_path = job['input']
    output_path = job['output']
    physics_output_path = job['physics_output']
    conversion = job.get('conversion', 'ao-none')
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    os.makedirs(os.path.dirname(physics_output_path), exist_ok=True)
    non_rendering_prefixes=('shadefade','fx','editormarker','marker','collision')
    def is_non_rendering_object(obj):
        name=obj.name.casefold().replace('_','').replace(' ','')
        return name.startswith(non_rendering_prefixes)
    def import_nif_scene(with_animation):
        bpy.ops.object.select_all(action='SELECT')
        bpy.ops.object.delete(use_global=False)
        for datablocks in (bpy.data.meshes, bpy.data.curves, bpy.data.materials, bpy.data.cameras, bpy.data.lights, bpy.data.actions):
            for datablock in list(datablocks):
                if datablock.users == 0: datablocks.remove(datablock)
        result=bpy.ops.import_scene.nif(filepath=nif_path, process='EVERYTHING', animation=with_animation, scale_correction=1.0/70.0, use_custom_normals=False, use_embedded_texture=False)
        if 'FINISHED' not in result: raise RuntimeError('NIF import failed: '+nif_path)
        record_zero = NifData.data.blocks[0] if NifData.data.blocks else None
        reset_roots = apply_record_zero_transform_policy(
            list(bpy.context.scene.objects),
            job.get('model', ''),
            job.get('root_transform_policy', 'preserve_review_required'),
            record_zero.name if record_zero else '',
            isinstance(record_zero, NifClasses.NiNode),
        )
        if reset_roots:
            print('[convert] discarded audited root transform(s): ' + ', '.join(reset_roots), flush=True)
        corrections = apply_verified_spatial_corrections(
            list(bpy.context.scene.objects), job.get('model', '')
        )
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
        return corrections
    spatial_corrections, collision_corrections = import_nif_scene(True)
    if bpy.data.actions:
        # Controller import bakes an animated pose into the collision
        # objects' transforms (a door's colliders land in the Open position,
        # leaving the doorway hole open), so physics must come from a
        # rest-pose import; the animated scene is rebuilt after for the GLB.
        import_nif_scene(False)
        physics_asset = build_physics_asset()
        spatial_corrections, collision_corrections = import_nif_scene(True)
    else:
        physics_asset = build_physics_asset()
    with gzip.open(physics_output_path, 'wt', encoding='utf8', compresslevel=6) as physics_file:
        json.dump(physics_asset, physics_file, separators=(',', ':'))
    for obj in list(bpy.context.scene.objects):
        if obj.get('bevyout_collision', False):
            bpy.data.objects.remove(obj, do_unlink=True)
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
    render_meshes = [obj for obj in bpy.context.scene.objects
                     if obj.type == 'MESH' and not obj.get('bevyout_collision', False)]
    retained_names = {
        (obj.niftools.longname if obj.niftools.longname else obj.name)
        for obj in render_meshes
    }
    source_render_geometries = [
        block for block in NifData.data.blocks
        if str(getattr(block, 'name', '')) in retained_names
        and getattr(block, 'data', None) is not None
        and callable(getattr(block, 'get_triangles', None))
    ]
    metadata_carrier = next(
        (obj for obj in bpy.context.scene.objects
         if obj.get('bevyout_source_model') == job.get('model', '')),
        None,
    )
    if metadata_carrier is None:
        raise RuntimeError('converted scene lost source metadata carrier: ' + nif_path)
    metadata_carrier['bevyout_source_render_meshes'] = len(source_render_geometries)
    metadata_carrier['bevyout_source_render_vertices'] = sum(
        int(geometry.data.num_vertices) for geometry in source_render_geometries
    )
    metadata_carrier['bevyout_source_render_triangles'] = sum(
        len(geometry.get_triangles()) for geometry in source_render_geometries
    )
    metadata_carrier['bevyout_spatial_audit_version'] = 1
    metadata_carrier['bevyout_expected_spatial_corrections'] = len(spatial_corrections)
    metadata_carrier['bevyout_verified_spatial_corrections'] = sum(
        1 for obj in render_meshes if obj.get('bevyout_spatial_verified', False)
    )
    metadata_carrier['bevyout_expected_collision_corrections'] = collision_corrections
    metadata_carrier['bevyout_verified_collision_corrections'] = collision_corrections
    metadata_carrier['bevyout_animation_sound_cues'] = json.dumps(
        collect_animation_sound_cues(), separators=(',', ':')
    )
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
        authored_emission = authored_emission_color(material)
        emission_multiplier, has_emission_multiplier = source_emission_multiplier(material)
        old_principled = next((node for node in tree.nodes if node.bl_idname == 'ShaderNodeBsdfPrincipled'), None)
        emission_link = None
        emission_color = None
        emission_strength = None
        authored_emission_fallback = False
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
            # Blender 5.2 defaults the rebuilt Principled emission socket to
            # white. Start from Fallout's non-emissive baseline so authored
            # zero colors cannot become accidental emitters.
            new_emission.default_value = (0.0, 0.0, 0.0, 1.0)
            if emission_link:
                tree.links.new(emission_link, new_emission)
            elif emission_color is not None and any(channel != 0.0 for channel in emission_color[:3]):
                new_emission.default_value = emission_color
            elif authored_emission is not None:
                new_emission.default_value = (*authored_emission, 1.0)
                authored_emission_fallback = True
        source_strength_applies = (
            authored_emission_fallback and
            has_emission_multiplier and
            (emission_strength is None or
             not math.isfinite(emission_strength) or
             emission_strength <= 0.0 or
             emission_strength == 1.0)
        )
        new_emission_strength = principled.inputs.get('Emission Strength')
        if new_emission_strength and source_strength_applies:
            # NIFTools can leave the imported Principled strength at its
            # zero-valued default even though niftools.emissive_color and the
            # source shader multiplier are authored. A zero strength makes
            # glTF omit the otherwise nonzero emissive material.
            new_emission_strength.default_value = emission_multiplier
        elif new_emission_strength and authored_emission_fallback and (
                emission_strength is None or
                not math.isfinite(emission_strength) or
                emission_strength <= 0.0 or
                emission_strength == 1.0):
            new_emission_strength.default_value = 1.0
        elif new_emission_strength and emission_strength is not None:
            new_emission_strength.default_value = emission_strength
        if new_emission_strength and emission_strength is None and has_emission_multiplier:
            new_emission_strength.default_value = emission_multiplier
        bulb_override = material.get('bevyout_emissive_bulb', False)
        if bulb_override and new_emission:
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
                # multiplier. Glow maps are masks/colors, not calibrated light
                # intensities, so retain that multiplier instead of replacing it
                # with an arbitrary HDR boost. A bulb override keeps its existing
                # minimum strength even when the glow card is also present.
                if source_strength_applies and not bulb_override:
                    new_emission_strength.default_value = emission_multiplier
                elif emission_strength is None and not bulb_override:
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
    # Issue #57: niftools names each imported action `<Sequence>_<NodeName>`
    # (e.g. `Open_VDoorBottom01`) and leaves it as the object's single active
    # action. The glTF exporter only exports an object's active action, so
    # regroup every sequence's actions onto an NLA track named by the
    # sequence (stripping the `_<object name>` suffix) before exporting with
    # NLA_TRACKS mode, which merges same-named tracks across objects into one
    # glTF animation (spike-verified: `Open`/`Close`, each animating every
    # door node).
    for obj in bpy.context.scene.objects:
        ad = obj.animation_data
        if not ad:
            continue
        obj_actions = [a for a in bpy.data.actions if a.name.endswith('_' + obj.name)]
        ad.action = None
        for act in obj_actions:
            sequence = act.name[: -(len(obj.name) + 1)]
            track = ad.nla_tracks.new()
            track.name = sequence
            track.strips.new(act.name, 0, act)
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.export_scene.gltf(filepath=output_path, export_format='GLB', export_materials='EXPORT', export_image_format='AUTO', export_apply=True, export_vertex_color='ACTIVE', export_all_vertex_colors=False, export_extras=True, export_animations=True, export_animation_mode='NLA_TRACKS')
