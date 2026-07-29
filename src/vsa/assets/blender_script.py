import bmesh, bpy, gzip, json, math, os, re, sys
from mathutils import Matrix, Vector

RIGID_BODY_TYPES = {'bhkRigidBody', 'bhkRigidBodyT'}
FALLOUT_EMISSIVE_SCALE = 0.25
FALLOUT_EMISSIVE_MAX = 1.0

def perceptual_roughness_from_glossiness(glossiness):
    try:
        exponent = float(glossiness)
    except (TypeError, ValueError):
        exponent = 10.0
    if not math.isfinite(exponent) or exponent < 0.0:
        exponent = 10.0
    return max(0.0, min(1.0, 1.75 * (2.0 / (exponent + 2.0)) ** 0.25))

def canonical_texture_reference(path):
    normalized = str(path or '').strip().replace('\\', '/').casefold()
    marker = normalized.find('textures/')
    return normalized[marker:] if marker >= 0 else None

def set_material_roughness(material, glossiness):
    roughness = perceptual_roughness_from_glossiness(glossiness)
    material.roughness = roughness
    material['bevyout_perceptual_roughness'] = roughness
    if material.use_nodes:
        for node in material.node_tree.nodes:
            if node.bl_idname == 'ShaderNodeBsdfPrincipled':
                socket = node.inputs.get('Roughness')
                if socket is not None and not socket.is_linked:
                    socket.default_value = roughness
    return roughness

def canonical_nif_path(path):
    return os.path.normcase(os.path.abspath(os.fspath(path))).replace('\\', '/')

def nif_block_index(blocks, target):
    for index, block in enumerate(blocks):
        if block is target:
            return index
    raise RuntimeError(
        'Imported Havok body is absent from the active NIF block table: '
        + type(target).__name__
    )

def nif_body_key(path, block_index):
    return '{}#{}'.format(canonical_nif_path(path), int(block_index))

def resolve_authored_joint_body_groups(source_joints, group_id_by_key):
    resolved = []
    for source_joint in source_joints:
        key_a = str(source_joint.get('body_a_key', ''))
        key_b = str(source_joint.get('body_b_key', ''))
        body_a = group_id_by_key.get(key_a)
        body_b = group_id_by_key.get(key_b)
        if body_a is None or body_b is None:
            raise RuntimeError(
                'Unresolved authored Havok constraint endpoint: {} -> {}'.format(
                    key_a or '<missing>', key_b or '<missing>')
            )
        if body_a == body_b:
            raise RuntimeError(
                'Authored Havok constraint resolves to one body: {}'.format(key_a)
            )
        joint = {
            key: value for key, value in source_joint.items()
            if key not in {'body_a_key', 'body_b_key'}
        }
        joint['body_a'] = int(body_a)
        joint['body_b'] = int(body_b)
        resolved.append(joint)
    return resolved

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
    def import_material_gloss_ggx(b_mat, glossiness):
        set_material_roughness(b_mat, glossiness)
    Material.import_material_gloss = staticmethod(import_material_gloss_ggx)
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
        from io_scene_niftools.utils.singleton import NifData
        body_info = bhkshape.rigid_body_info
        body_block_index = nif_block_index(NifData.data.blocks, bhkshape)
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
            'bevyout_nif_body_block': body_block_index,
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
        from io_scene_niftools.utils.singleton import NifData
        body_block_index = nif_block_index(NifData.data.blocks, bhkshape)
        transform_value = getattr(bhkshape, 'transform', None)
        if transform_value is not None:
            try:
                transform = __import__('mathutils').Matrix(transform_value.as_list())
                transform.translation = transform.translation * self.HAVOK_SCALE
                for obj in objects: obj.matrix_local = obj.matrix_local @ transform
            except Exception:
                pass
        for obj in objects:
            obj['bevyout_phantom'] = True
            obj['bevyout_nif_body_block'] = body_block_index
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

def actor_bone_name(armature, *hints):
    """Return the authored armature bone matching any of the given hints."""
    bones = list(getattr(getattr(armature, 'data', None), 'bones', []))
    normalized = [hint.casefold().replace(' ', '').replace('_', '') for hint in hints]
    for bone in bones:
        name = bone.name.casefold().replace(' ', '').replace('_', '')
        if name in normalized:
            return bone.name
    for hint in normalized:
        for bone in bones:
            if hint in bone.name.casefold().replace(' ', '').replace('_', ''):
                return bone.name
    return None

def actor_node_key(name):
    parts = [part.casefold() for part in re.findall(r'[A-Za-z0-9]+', str(name))]
    if (len(parts) >= 3 and parts[0].startswith('bip')
            and parts[-1] in {'l', 'r'}):
        side = parts.pop()
        parts.insert(1, side)
    return ''.join(parts)

def actor_ragdoll_weight_target(bone, ragdoll_names_by_key):
    current = bone
    while current is not None:
        target = ragdoll_names_by_key.get(actor_node_key(current.name))
        if target is not None:
            return target
        current = current.parent
    return None

def collapse_actor_ragdoll_weights(ragdoll_nodes):
    ragdoll_names_by_key = {
        actor_node_key(name): str(name)
        for name in ragdoll_nodes if name
    }
    if not ragdoll_names_by_key:
        return 0
    moved_groups = 0
    for mesh in bpy.context.scene.objects:
        if mesh.type != 'MESH' or not mesh.vertex_groups:
            continue
        armature = next((
            modifier.object for modifier in mesh.modifiers
            if modifier.type == 'ARMATURE' and modifier.object is not None
        ), None)
        if armature is None:
            continue
        for source_group in list(mesh.vertex_groups):
            if actor_node_key(source_group.name) in ragdoll_names_by_key:
                continue
            bone = armature.data.bones.get(source_group.name)
            if bone is None:
                continue
            target_name = actor_ragdoll_weight_target(
                bone.parent, ragdoll_names_by_key)
            if target_name is None:
                continue
            target_bone = next((
                candidate for candidate in armature.data.bones
                if actor_node_key(candidate.name) == actor_node_key(target_name)
            ), None)
            if target_bone is None:
                continue
            target_group = mesh.vertex_groups.get(target_bone.name)
            if target_group is None:
                target_group = mesh.vertex_groups.new(name=target_bone.name)
            moved = False
            for vertex in mesh.data.vertices:
                try:
                    weight = source_group.weight(vertex.index)
                except RuntimeError:
                    continue
                if weight <= 0.0:
                    continue
                target_group.add([vertex.index], weight, 'ADD')
                source_group.remove([vertex.index])
                moved = True
            if moved:
                moved_groups += 1
                print('[convert] actor ragdoll weights {}:{} -> {}'.format(
                    mesh.name, source_group.name, target_bone.name), flush=True)
    return moved_groups

def normalize_actor_assembly():
    """Normalize imported body parts and attach standalone gear.

    Fallout actor parts are authored as separate NIFs. NIFTools therefore
    creates one partial armature per input, while static weapons/helmets stay
    at their NIF origin. Keep each weighted part on its owning armature and
    add explicit bone parents for meshes with no weights; the exported bind
    hierarchy and runtime duplicate-name sync provide the shared pose.
    """
    armatures = [obj for obj in bpy.context.scene.objects if obj.type == 'ARMATURE']
    if not armatures:
        print('[convert] actor assembly has no armature', flush=True)
        return
    main = max(armatures, key=lambda obj: (
        len(getattr(getattr(obj, 'data', None), 'bones', [])),
        int(any('bip01' in bone.name.casefold() for bone in obj.data.bones)),
        obj.name.casefold(),
    ))
    meshes = [obj for obj in bpy.context.scene.objects if obj.type == 'MESH']
    # Keep each body-part mesh on the armature that owns its vertex groups.
    # The Fallout skeleton NIF is a plain bind-pose node hierarchy (not a
    # Blender armature), while hands/head carry their own valid partial
    # armatures. Rebinding those meshes to the torso armature would discard
    # their finger/head weights; ragdoll pose propagation below updates every
    # duplicate named bone node instead.
    weapon_slot = 0
    for root in [obj for obj in bpy.context.scene.objects
                 if obj.get('bevyout_actor_source_path') and obj.parent is None]:
        source = str(root.get('bevyout_actor_source_path', '')).replace('\\', '/').casefold()
        if 'weapons/' in source or source.startswith('weapons/'):
            explicit_left = any(token in source for token in ('lefthand', 'left/', '/left'))
            explicit_right = any(token in source for token in ('righthand', 'right/', '/right'))
            use_left = explicit_left or (not explicit_right and weapon_slot % 2 == 0)
            weapon_slot += 1
            bone = actor_bone_name(
                main,
                'Bip01 L Hand' if use_left else 'Bip01 R Hand',
                'Bip01 Hand.L' if use_left else 'Bip01 Hand.R',
                'LeftHand' if use_left else 'RightHand',
            )
        elif any(token in source for token in ('helmet', 'headgear', '/head/', '/hat', 'hatgo')):
            bone = actor_bone_name(main, 'Bip01 Head', 'Head')
        elif 'armor/' in source or source.startswith('armor/'):
            # Clothing/armor NIFs are separate scene roots. Their meshes are
            # commonly weighted to a tiny local armature (or only contain a
            # pair of placeholder bones), so leaving that root at the NIF
            # origin makes the outfit appear on the floor. Move the complete
            # outfit root with the torso; the runtime duplicate-bone sync
            # still updates any authored partial armature beneath it.
            bone = actor_bone_name(
                main,
                'Bip01 Spine2', 'Bip01 Spine1', 'Bip01 Spine',
                'Spine2', 'Spine1', 'Spine',
            )
        else:
            bone = None
        if bone is None:
            # Body armor and weighted meshes follow the shared armature via
            # their modifiers; retain their authored actor-space transform.
            continue
        root.parent = main
        root.parent_type = 'BONE'
        root.parent_bone = bone
        root.matrix_parent_inverse = Matrix.Identity(4)
        root.matrix_basis = Matrix.Identity(4)
        print('[convert] actor gear bound {} -> {}'.format(source, bone), flush=True)
    print('[convert] actor armature normalized main={} bones={} parts={}'.format(
        main.name, len(main.data.bones), len(meshes)), flush=True)

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

def fallout_material_semantics(material, glow_node=None):
    """Return the flag-shaped material contract exported with each GLB.

    NIFTools owns the shader graph, so this metadata mirrors the semantics that
    survived import rather than attempting to recreate Bethesda's shader in
    Blender.  The JSON string is intentional: Blender ID properties export
    reliably as scalar extras across the supported Blender versions.
    """
    shader = getattr(material, 'niftools_shader', None)

    def integer_value(*names):
        for owner in (shader, material):
            for name in names:
                value = getattr(owner, name, None) if owner is not None else None
                if value is None and owner is material:
                    value = material.get(name)
                try:
                    return int(value)
                except (TypeError, ValueError):
                    continue
        return 0

    shader_type_value = integer_value('Shader_Type', 'shader_type')
    shader_type_name = str(getattr(shader, 'bs_shadertype', '')).casefold()
    effect_shader = shader_type_name == 'bseffectshaderproperty'
    no_lighting_shader = 'nolighting' in shader_type_name.replace('_', '')
    shader_type = {
        'environment_map': 1,
        'environment map': 1,
        'glow_shader': 2,
        'glow shader': 2,
        'skin_tint': 5,
        'skin tint': 5,
        'hair_tint': 6,
        'hair tint': 6,
    }.get(shader_type_name, shader_type_value)
    flags1 = integer_value('Shader_Flags_1', 'shaderflags1', 'shader_flags_1')
    flags2 = integer_value('Shader_Flags_2', 'shaderflags2', 'shader_flags_2')
    # A filename suffix is only a texture naming convention.  Fallout's
    # shader flags/type decide whether slot 2 is actually sampled as glow.
    # In particular, environment-map materials can carry an adjacent `_g`
    # texture and a nonzero authored color without being emissive.
    glow_map = bool(flags2 & (1 << 6)) or shader_type == 2
    specular = bool(flags1 & (1 << 0)) or bool(flags1 & (1 << 12))
    parallax = bool(flags1 & (1 << 11)) or bool(flags2 & (1 << 24))
    environment_mapping = bool(flags1 & (1 << 7)) or shader_type == 1
    soft_lighting = bool(flags2 & (1 << 25))
    back_lighting = bool(flags2 & (1 << 27))
    translucent_candidate = (
        soft_lighting or back_lighting or
        shader_type in (5, 6) or
        'skin_tint' in shader_type_name or 'hair_tint' in shader_type_name
    )
    strength = 0.35 if back_lighting else 0.2 if soft_lighting else 0.15 if shader_type in (5, 6) else 0.0
    semantics = {
        'schema': 1,
        'shader_type': shader_type,
        'shader_flags_1': flags1,
        'shader_flags_2': flags2,
        'features': {
            'glow_map': glow_map,
            'specular': specular,
            'parallax': parallax,
            'environment_mapping': environment_mapping,
            'double_sided': not bool(getattr(material, 'use_backface_culling', True)),
            'vertex_colors': False,
            'vertex_alpha': False,
            'soft_lighting': soft_lighting,
            'back_lighting': back_lighting,
        },
        'effect_shader': effect_shader,
        'no_lighting_shader': no_lighting_shader,
        'emission_authorized': glow_map or effect_shader,
        'translucency_enabled': translucent_candidate,
        'translucency_strength': strength,
        'emissive_multiplier': float(material.get('bevyout_emissive_strength', 1.0) or 1.0),
        'emissive_max': FALLOUT_EMISSIVE_MAX,
        'emissive_scale': FALLOUT_EMISSIVE_SCALE,
    }
    material['bevyout_fallout_material'] = json.dumps(
        semantics, sort_keys=True, separators=(',', ':')
    )
    return semantics

def collision_body_key(obj):
    source_path = obj.get('bevyout_nif_source_path')
    block_index = obj.get('bevyout_nif_body_block')
    if source_path and block_index is not None:
        return nif_body_key(source_path, block_index)
    # Bounds and other non-rigid collision helpers are never constraint
    # endpoints. Keep them deterministic without allowing their import order
    # to enter the authored ragdoll identity namespace.
    source = canonical_nif_path(source_path) if source_path else '<unknown>'
    return '{}#helper:{}'.format(source, str(obj.name).casefold())

def build_physics_asset():
    collision_objects = [obj for obj in bpy.context.scene.objects
                         if obj.type == 'MESH' and obj.get('bevyout_collision', False)]
    groups_by_key = {}
    for obj in collision_objects:
        groups_by_key.setdefault(collision_body_key(obj), []).append(obj)
    authored_bodies = []
    group_id_by_key = {}
    for source_key, objects in sorted(groups_by_key.items()):
        group_id = len(authored_bodies)
        body = physics_body_from_objects(group_id, objects)
        if not body['shapes']:
            continue
        group_id_by_key[source_key] = group_id
        authored_bodies.append(body)
    joints = resolve_authored_joint_body_groups(
        globals().get('current_joint_defs', []), group_id_by_key)
    body_by_group = {int(body['group_id']): body for body in authored_bodies}
    for joint in joints:
        node_a = body_by_group[int(joint['body_a'])].get('node') or '<unnamed>'
        node_b = body_by_group[int(joint['body_b'])].get('node') or '<unnamed>'
        print('[convert] authored joint resolved {} -> {}'.format(
            node_a, node_b), flush=True)
    # Keep every valid authored actor edge. Only connect genuinely disconnected
    # components of a partial/custom skeleton with the conservative Bip01
    # fallback. Fallout's authored ragdoll is a complete tree whose topology
    # intentionally differs from the visual bone-parent hierarchy, so filling
    # every absent visual-parent edge would over-constrain it.
    if (globals().get('assembly_inputs') is not None
            and any('bip01' in str(body.get('node', '')).casefold()
                    for body in authored_bodies)):
        joints = actor_completed_joints(joints, authored_bodies)
    if any(body_blocks_player(body) and body['shapes'] for body in authored_bodies):
        return {'schema_version': 3, 'source': 'AuthoredHavok', 'bodies': authored_bodies, 'joints': joints}
    render_objects = [obj for obj in bpy.context.scene.objects
                      if obj.type == 'MESH' and len(obj.data.polygons)
                      and not obj.get('bevyout_collision', False)
                      and obj.visible_get()
                      and obj.data.materials
                      and all(fallback_material_eligible(material) for material in obj.data.materials)]
    fallback = render_fallback_body(render_objects)
    if not fallback['shapes'][0]['indices']:
        return {'schema_version': 3, 'source': 'GeneratedRender', 'bodies': authored_bodies, 'joints': joints}
    # Keep the render-only fallback out of the authored body-id namespace.
    # Actor body IDs start at zero; reusing zero would overwrite the first
    # ragdoll body's lookup entry when the runtime builds its joint map.
    fallback['group_id'] = max(
        (int(body['group_id']) for body in authored_bodies), default=-1
    ) + 1
    return {'schema_version': 3, 'source': 'GeneratedRender',
            'bodies': authored_bodies + [fallback], 'joints': joints}

def actor_shape_anchor(shape):
    kind = shape.get('kind')
    if kind == 'Sphere':
        return list(shape.get('center', [0.0, 0.0, 0.0]))
    if kind == 'Capsule':
        first = shape.get('point1', [0.0, 0.0, 0.0])
        second = shape.get('point2', [0.0, 0.0, 0.0])
        return [(float(first[i]) + float(second[i])) * 0.5 for i in range(3)]
    if kind == 'Box':
        return list(shape.get('center', [0.0, 0.0, 0.0]))
    points = shape.get('points') or shape.get('vertices') or []
    if not points:
        return [0.0, 0.0, 0.0]
    return [sum(float(point[i]) for point in points) / len(points) for i in range(3)]

def actor_body_anchor(body):
    shapes = body.get('shapes') or []
    if not shapes:
        return [0.0, 0.0, 0.0]
    anchors = [actor_shape_anchor(shape) for shape in shapes]
    return [sum(anchor[i] for anchor in anchors) / len(anchors) for i in range(3)]

def actor_synthetic_joints(bodies):
    """Return a stable Bip01 ragdoll tree for partially authored actors."""
    by_name = {}
    for body in bodies:
        node = body.get('node')
        if not node:
            continue
        compact = ''.join(character for character in str(node).casefold()
                          if character.isalnum())
        by_name.setdefault(compact, body)
    if len(by_name) < 2:
        return []

    parents = {
        'bip01footl': 'bip01calfl', 'bip01calfl': 'bip01thighl',
        'bip01thighl': 'bip01pelvis',
        'bip01footr': 'bip01calfr', 'bip01calfr': 'bip01thighr',
        'bip01thighr': 'bip01pelvis',
        'bip01pelvis': 'bip01nonaccum',
        'bip01spine': 'bip01nonaccum',
        'bip01spine1': 'bip01spine', 'bip01spine2': 'bip01spine1',
        'bip01neck1': 'bip01spine2', 'bip01head': 'bip01neck1',
        'bip01upperarml': 'bip01spine2', 'bip01forearml': 'bip01upperarml',
        'bip01handl': 'bip01forearml',
        'bip01upperarmr': 'bip01spine2', 'bip01forearmr': 'bip01upperarmr',
        'bip01handr': 'bip01forearmr',
    }
    # Pelvis/Clavicles are absent in some FO3 skeleton exports. Walk upward
    # through the canonical hierarchy until the nearest exported body exists.
    resolved = {}
    for child, parent in parents.items():
        if child not in by_name:
            continue
        candidate = parent
        visited = set()
        while candidate not in by_name and candidate not in visited:
            visited.add(candidate)
            candidate = parents.get(candidate)
            if candidate is None:
                break
        if candidate in by_name and candidate != child:
            resolved[child] = candidate

    existing = set()
    joints = []
    hinge_axes = {
        # Bind-pose legs point down; X-axis hinges let the calves fold in the
        # sagittal plane while blocking sideways and hyperextension collapse.
        'bip01calfl': [1.0, 0.0, 0.0],
        'bip01calfr': [1.0, 0.0, 0.0],
        # Bind-pose arms point in opposite X directions. Mirroring the right
        # hinge axis makes the same positive angular range flex both elbows.
        'bip01forearml': [0.0, 0.0, 1.0],
        'bip01forearmr': [0.0, 0.0, -1.0],
    }
    for child, parent in sorted(resolved.items()):
        child_body = by_name[child]
        parent_body = by_name[parent]
        body_a = int(parent_body.get('group_id', -1))
        body_b = int(child_body.get('group_id', -1))
        pair = tuple(sorted((body_a, body_b)))
        if body_a < 0 or body_b < 0 or pair in existing:
            continue
        existing.add(pair)
        anchor = [(left + right) * 0.5
                  for left, right in zip(actor_body_anchor(parent_body),
                                         actor_body_anchor(child_body))]
        hinge_axis = hinge_axes.get(child)
        cone_limit = 1.8 if child in {'bip01thighl', 'bip01thighr'} else 1.0
        frame = joint_frame_quaternion(
            hinge_axis or [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0] if hinge_axis and abs(hinge_axis[1]) < 0.9
            else [1.0, 0.0, 0.0],
        )
        joints.append({
            'kind': 'revolute' if hinge_axis is not None else 'spherical',
            'body_a': body_a, 'body_b': body_b,
            'anchor_a': anchor, 'anchor_b': anchor,
            'frame_a_rotation_xyzw': frame,
            'frame_b_rotation_xyzw': frame,
            'lower_limit': -0.05 if hinge_axis is not None else None,
            'upper_limit': 2.4 if hinge_axis is not None else None,
            'cone_limit': None if hinge_axis is not None else cone_limit,
            'plane_lower_limit': None if hinge_axis is not None else -1.0,
            'plane_upper_limit': None if hinge_axis is not None else 1.0,
            'twist_lower_limit': None if hinge_axis is not None else -1.0,
            'twist_upper_limit': None if hinge_axis is not None else 1.0,
            'malleable_strength': None,
            'source': 'SyntheticFallback',
        })
    return joints

def actor_completed_joints(authored, bodies):
    completed = list(authored)
    parent = {
        int(body['group_id']): int(body['group_id'])
        for body in bodies
    }

    def root(body_id):
        while parent[body_id] != body_id:
            parent[body_id] = parent[parent[body_id]]
            body_id = parent[body_id]
        return body_id

    def connect(body_a, body_b):
        root_a = root(body_a)
        root_b = root(body_b)
        if root_a == root_b:
            return False
        parent[root_b] = root_a
        return True

    for joint in completed:
        connect(int(joint['body_a']), int(joint['body_b']))
    for fallback in actor_synthetic_joints(bodies):
        pair = tuple(sorted((int(fallback['body_a']), int(fallback['body_b']))))
        if not connect(*pair):
            continue
        completed.append(fallback)
        print('[convert] actor ragdoll fallback edge {}-{}'.format(*pair), flush=True)
    completed.sort(key=lambda joint: (
        min(int(joint['body_a']), int(joint['body_b'])),
        max(int(joint['body_a']), int(joint['body_b'])),
        str(joint.get('source', '')),
    ))
    return completed

def nif_raw_vector(value):
    if value is None:
        return Vector((0.0, 0.0, 0.0))
    return Vector((float(getattr(value, 'x', 0.0)),
                   float(getattr(value, 'y', 0.0)),
                   float(getattr(value, 'z', 0.0))))

def joint_frame_quaternion(axis_z, reference_x):
    """Return a deterministic BoxDDD frame with authored twist/hinge on Z."""
    z_axis = Vector(axis_z)
    x_axis = Vector(reference_x)
    if (not all(math.isfinite(value) for value in (*z_axis, *x_axis))
            or z_axis.length_squared < 1.0e-10):
        return None
    z_axis.normalize()
    x_axis -= z_axis * x_axis.dot(z_axis)
    if x_axis.length_squared < 1.0e-10:
        return None
    x_axis.normalize()
    y_axis = z_axis.cross(x_axis)
    if y_axis.length_squared < 1.0e-10:
        return None
    y_axis.normalize()
    x_axis = y_axis.cross(z_axis).normalized()
    rotation = Matrix((x_axis, y_axis, z_axis)).transposed().to_quaternion()
    rotation.normalize()
    values = [float(rotation.x), float(rotation.y),
              float(rotation.z), float(rotation.w)]
    # q and -q encode the same frame. Canonicalize the sign so sidecars are
    # deterministic across Blender/mathutils builds.
    if values[3] < 0.0:
        values = [-value for value in values]
    return values

def nif_constraint_payload(block):
    """Unwrap the concrete descriptor carried by a Havok constraint block."""
    name = type(block).__name__
    payload = getattr(block, 'constraint', None)
    strength = None
    if name == 'bhkMalleableConstraint':
        descriptor = payload
        type_value = getattr(descriptor, 'type', None)
        constraint_type = str(getattr(type_value, 'name', type_value or '')).upper()
        strength_value = getattr(descriptor, 'strength', None)
        strength = float(strength_value) if strength_value is not None else None
        if 'RAGDOLL' in constraint_type:
            return ('bhkRagdollConstraint', getattr(descriptor, 'ragdoll', None), strength)
        if 'LIMITED_HINGE' in constraint_type:
            return ('bhkLimitedHingeConstraint',
                    getattr(descriptor, 'limited_hinge', None), strength)
        return (None, None, strength)
    if name == 'bhkBreakableConstraint':
        descriptor = getattr(block, 'constraint_data', None)
        return ('bhkLimitedHingeConstraint',
                getattr(descriptor, 'limited_hinge', None), None)
    return (name, payload, strength)

def nif_target_rest_matrix(nif, target):
    if target is None:
        return None
    from io_scene_niftools.utils.math import nifformat_to_mathutils_matrix
    for root in getattr(nif, 'roots', []):
        try:
            matrix = nifformat_to_mathutils_matrix(target.get_transform(root))
            matrix.translation *= 1.0 / 70.0
            return matrix
        except (ValueError, AttributeError):
            continue
    try:
        matrix = nifformat_to_mathutils_matrix(target.get_transform())
        matrix.translation *= 1.0 / 70.0
        return matrix
    except (ValueError, AttributeError):
        return None

def nif_rigid_body_matrix(body, havok_scale):
    matrix = Matrix.Identity(4)
    if type(body).__name__ != 'bhkRigidBodyT':
        return matrix
    info = getattr(body, 'rigid_body_info', None)
    rotation = getattr(body, 'rotation', None)
    if rotation is None:
        rotation = getattr(info, 'rotation', None)
    if rotation is not None:
        quaternion = __import__('mathutils').Quaternion((
            float(getattr(rotation, 'w', 1.0)),
            float(getattr(rotation, 'x', 0.0)),
            float(getattr(rotation, 'y', 0.0)),
            float(getattr(rotation, 'z', 0.0)),
        ))
        matrix = quaternion.to_matrix().to_4x4()
    translation = getattr(body, 'translation', None)
    if translation is None:
        translation = getattr(info, 'translation', None)
    if translation is not None:
        matrix.translation = nif_raw_vector(translation) * (havok_scale / 70.0)
    return matrix

def nif_body_actor_matrix(nif, body, target):
    target_matrix = nif_target_rest_matrix(nif, target)
    if target_matrix is None:
        return None
    return target_matrix @ nif_rigid_body_matrix(body, float(nif.havok_scale))

def nif_actor_point(value, body_matrix, havok_scale):
    local = nif_raw_vector(value) * (havok_scale / 70.0)
    return blender_point_to_bevy(body_matrix @ local)

def nif_actor_vector(value, body_matrix):
    actor = body_matrix.to_quaternion().to_matrix() @ nif_raw_vector(value)
    bevy = blender_vector_to_bevy(actor)
    return list(bevy)

def nif_joint_frame(body_matrix, axis, reference):
    return joint_frame_quaternion(
        nif_actor_vector(axis, body_matrix),
        nif_actor_vector(reference, body_matrix),
    )

def nif_constraint_joints(paths):
    """Extract constraint records without asking NIFTools to create Blender
    rigid-body constraints (that importer still assumes removed 2.x fields).
    Endpoints retain their source NIF block identity until collision bodies
    have been imported and grouped."""
    try:
        from nifgen.formats.nif import NifFile
    except Exception:
        return []
    joints = []
    for path in paths:
        try:
            nif = NifFile.from_path(path)
        except Exception:
            continue
        block_indices = {id(block): index for index, block in enumerate(nif.blocks)}
        body_targets = {
            id(getattr(block, 'body', None)): getattr(block, 'target', None)
            for block in nif.blocks
            if 'CollisionObject' in type(block).__name__
            and getattr(block, 'body', None) is not None
        }
        for block in nif.blocks:
            name = type(block).__name__
            if not name.endswith('Constraint'):
                continue
            outer = getattr(block, 'constraint_info', None)
            entity_a = getattr(outer, 'entity_a', None)
            entity_b = getattr(outer, 'entity_b', None)
            body_a_block = block_indices.get(id(entity_a))
            body_b_block = block_indices.get(id(entity_b))
            if (body_a_block is None or body_b_block is None
                    or body_a_block == body_b_block
                    or type(entity_a).__name__ not in RIGID_BODY_TYPES
                    or type(entity_b).__name__ not in RIGID_BODY_TYPES):
                continue
            body_a_key = nif_body_key(path, body_a_block)
            body_b_key = nif_body_key(path, body_b_block)
            kind, payload, strength = nif_constraint_payload(block)
            if kind is None or payload is None:
                continue
            matrix_a = nif_body_actor_matrix(nif, entity_a, body_targets.get(id(entity_a)))
            matrix_b = nif_body_actor_matrix(nif, entity_b, body_targets.get(id(entity_b)))
            if matrix_a is None or matrix_b is None:
                print('[convert] skipped authored joint without body rest frame {}-{}'.format(
                    body_a_key, body_b_key), flush=True)
                continue
            anchor_a = nif_actor_point(
                getattr(payload, 'pivot_a', None), matrix_a, float(nif.havok_scale))
            anchor_b = nif_actor_point(
                getattr(payload, 'pivot_b', None), matrix_b, float(nif.havok_scale))
            pivot_error = (Vector(anchor_a) - Vector(anchor_b)).length
            if not math.isfinite(pivot_error) or pivot_error > 0.05:
                print('[convert] skipped authored joint {}-{} pivot mismatch {:.6f}m'.format(
                    body_a_key, body_b_key, pivot_error), flush=True)
                continue
            joint = {
                'kind': 'fixed',
                'body_a_key': body_a_key, 'body_b_key': body_b_key,
                'anchor_a': anchor_a, 'anchor_b': anchor_b,
                'frame_a_rotation_xyzw': [0.0, 0.0, 0.0, 1.0],
                'frame_b_rotation_xyzw': [0.0, 0.0, 0.0, 1.0],
                'lower_limit': None, 'upper_limit': None,
                'cone_limit': None,
                'plane_lower_limit': None, 'plane_upper_limit': None,
                'twist_lower_limit': None, 'twist_upper_limit': None,
                'malleable_strength': strength,
                'source': 'Authored',
            }
            if kind == 'bhkRagdollConstraint':
                joint['kind'] = 'spherical'
                joint['frame_a_rotation_xyzw'] = nif_joint_frame(
                    matrix_a, getattr(payload, 'twist_a', None),
                    getattr(payload, 'plane_a', None))
                joint['frame_b_rotation_xyzw'] = nif_joint_frame(
                    matrix_b, getattr(payload, 'twist_b', None),
                    getattr(payload, 'plane_b', None))
                joint['cone_limit'] = float(getattr(payload, 'cone_max_angle', 0.0))
                joint['plane_lower_limit'] = float(getattr(payload, 'plane_min_angle', 0.0))
                joint['plane_upper_limit'] = float(getattr(payload, 'plane_max_angle', 0.0))
                joint['twist_lower_limit'] = float(getattr(payload, 'twist_min_angle', 0.0))
                joint['twist_upper_limit'] = float(getattr(payload, 'twist_max_angle', 0.0))
            elif kind == 'bhkLimitedHingeConstraint':
                joint['kind'] = 'revolute'
                joint['frame_a_rotation_xyzw'] = nif_joint_frame(
                    matrix_a, getattr(payload, 'axis_a', None),
                    getattr(payload, 'perp_axis_in_a_1', None))
                joint['frame_b_rotation_xyzw'] = nif_joint_frame(
                    matrix_b, getattr(payload, 'axis_b', None),
                    getattr(payload, 'perp_axis_in_b_1', None))
                joint['lower_limit'] = float(getattr(payload, 'min_angle', 0.0))
                joint['upper_limit'] = float(getattr(payload, 'max_angle', 0.0))
            elif kind == 'bhkPrismaticConstraint':
                joint['kind'] = 'prismatic'
                joint['frame_a_rotation_xyzw'] = nif_joint_frame(
                    matrix_a, getattr(payload, 'sliding_a', None),
                    getattr(payload, 'rotation_a', None))
                joint['frame_b_rotation_xyzw'] = nif_joint_frame(
                    matrix_b, getattr(payload, 'sliding_b', None),
                    getattr(payload, 'rotation_b', None))
                distance_scale = float(nif.havok_scale) / 70.0
                joint['lower_limit'] = float(getattr(payload, 'min_distance', 0.0)) * distance_scale
                joint['upper_limit'] = float(getattr(payload, 'max_distance', 0.0)) * distance_scale
            else:
                continue
            if (joint['frame_a_rotation_xyzw'] is None
                    or joint['frame_b_rotation_xyzw'] is None):
                print('[convert] skipped authored joint {}-{} with degenerate frame'.format(
                    body_a_key, body_b_key), flush=True)
                continue
            joints.append(joint)
    print('[convert] authored constraints extracted {}'.format(len(joints)), flush=True)
    return joints

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

def run_ragdoll_identity_self_test():
    source_keys = {
        'Pelvis': 'fixture/skeleton.nif#10',
        'Forearm.L': 'fixture/skeleton.nif#21',
        'Neck1': 'fixture/skeleton.nif#32',
        'Foot.L': 'fixture/skeleton.nif#43',
    }
    importer_order = ['Foot.L', 'Pelvis', 'Neck1', 'Forearm.L']
    groups_by_key = {
        source_keys[node]: {'node': node}
        for node in importer_order
    }
    ordered_keys = sorted(groups_by_key)
    group_id_by_key = {
        key: group_id for group_id, key in enumerate(ordered_keys)
    }
    resolved = resolve_authored_joint_body_groups([{
        'kind': 'spherical',
        'body_a_key': source_keys['Forearm.L'],
        'body_b_key': source_keys['Neck1'],
    }], group_id_by_key)
    node_by_group = {
        group_id_by_key[key]: value['node']
        for key, value in groups_by_key.items()
    }
    assert node_by_group[resolved[0]['body_a']] == 'Forearm.L'
    assert node_by_group[resolved[0]['body_b']] == 'Neck1'
    class FixtureBone:
        def __init__(self, name, parent=None):
            self.name = name
            self.parent = parent
    spine2 = FixtureBone('Bip01 Spine2')
    clavicle = FixtureBone('Bip01 L Clavicle', spine2)
    upper_arm = FixtureBone('Bip01 L UpperArm', clavicle)
    forearm = FixtureBone('Bip01 L Forearm', upper_arm)
    fore_twist = FixtureBone('Bip01 L ForeTwist', forearm)
    weight_targets = {
        actor_node_key('Bip01 Spine2'): 'Bip01 Spine2',
        actor_node_key('Bip01 UpperArm.L'): 'Bip01 UpperArm.L',
        actor_node_key('Bip01 Forearm.L'): 'Bip01 Forearm.L',
    }
    assert actor_ragdoll_weight_target(clavicle, weight_targets) == 'Bip01 Spine2'
    assert actor_ragdoll_weight_target(fore_twist, weight_targets) == 'Bip01 Forearm.L'
    print('[convert-test] ragdoll source identity passed', flush=True)

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
    assert is_pynifly_render_helper('BSBound:BBX')
    assert is_pynifly_render_helper('bsbound:actor bounds')
    assert is_pynifly_render_helper('bodymeat')
    assert is_pynifly_render_helper('headmeat.001')
    assert is_pynifly_render_helper('MeatCapBody')
    assert is_pynifly_render_helper('MeatCapLimbs')
    assert is_pynifly_render_helper('llegmeatcapbody')
    assert not is_pynifly_render_helper('RaiderArmor01M_GO')
    assert not is_pynifly_render_helper('RaiderArmor01M_GO.001')
    assert not is_pynifly_render_helper('UpperBody')
    assert not is_pynifly_render_helper('DomeRoot:0')
    assert partition_is_editor_visible(0x0001)
    assert not partition_is_editor_visible(0x0000)
    class FixtureShader:
        def _readtexture(self, file_handle, shape_handle, slot):
            assert file_handle == 'fixture-file'
            assert shape_handle == 'fixture-shape'
            return {
                1: 'textures\\characters\\female\\UpperBodyFemale.dds',
                2: 'textures\\characters\\female\\UpperBodyFemale_n.dds',
            }.get(slot, '')
    fixture_texture_node = type('TextureNode', (), {
        'textures': {},
        'shader': FixtureShader(),
        'file': type('TextureFile', (), {'_handle': 'fixture-file'})(),
        '_handle': 'fixture-shape',
    })()
    assert actor_shape_texture_values(
        fixture_texture_node,
        ['textures\\armor\\raiderarmor01\\OutfitF.dds'],
    ) == [
        'textures\\characters\\female\\UpperBodyFemale.dds',
        'textures\\characters\\female\\UpperBodyFemale_n.dds',
    ]
    assert actor_material_alpha_policy(0) == 'OPAQUE'
    assert actor_material_alpha_policy(1) == 'BLEND'
    assert actor_material_alpha_policy(1 << 9) == 'MASK'
    assert actor_material_alpha_policy(1 | (1 << 9)) == 'BLEND'
    partition_mesh = bpy.data.meshes.new('ActorPartitionFixture')
    partition_mesh.from_pydata(
        [(0, 0, 0), (1, 0, 0), (0, 1, 0),
         (2, 0, 0), (3, 0, 0), (2, 1, 0)],
        [],
        [(0, 1, 2), (3, 4, 5)],
    )
    partition_obj = bpy.data.objects.new('ActorPartitionFixture', partition_mesh)
    bpy.context.collection.objects.link(partition_obj)
    visible_material = bpy.data.materials.new('VisibleSkinMaterial')
    hidden_material = bpy.data.materials.new('HiddenGoreMaterial')
    partition_mesh.materials.append(visible_material)
    partition_mesh.materials.append(hidden_material)
    partition_mesh.polygons[0].material_index = 0
    partition_mesh.polygons[1].material_index = 1
    visible_weights = partition_obj.vertex_groups.new(name='Bip01 Spine2')
    visible_weights.add([0, 1, 2], 1.0, 'REPLACE')
    fixture_node = type('PartitionNode', (), {
        'partitions': [
            type('Partition', (), {'flags': 0x0001})(),
            type('Partition', (), {'flags': 0x0000})(),
        ],
        'partition_tris': [0, 1],
    })()
    assert prune_hidden_actor_partitions(partition_obj, fixture_node) == 1
    assert len(partition_mesh.polygons) == 1
    assert partition_mesh.polygons[0].material_index == 0
    assert visible_weights.weight(0) == 1.0
    bpy.data.objects.remove(partition_obj, do_unlink=True)
    fixture_bodies = [
        {'group_id': 0, 'node': 'Bip01 NonAccum',
         'shapes': [{'kind': 'Sphere', 'center': [0.0, 0.9, 0.0]}]},
        {'group_id': 1, 'node': 'Bip01 Thigh.L',
         'shapes': [{'kind': 'Sphere', 'center': [0.2, 0.65, 0.0]}]},
        {'group_id': 2, 'node': 'Bip01 Thigh.R',
         'shapes': [{'kind': 'Sphere', 'center': [-0.2, 0.65, 0.0]}]},
        {'group_id': 3, 'node': 'Bip01 Calf.L',
         'shapes': [{'kind': 'Sphere', 'center': [0.2, 0.3, 0.0]}]},
        {'group_id': 4, 'node': 'Bip01 Spine2',
         'shapes': [{'kind': 'Sphere', 'center': [0.0, 1.3, 0.0]}]},
        {'group_id': 5, 'node': 'Bip01 UpperArm.L',
         'shapes': [{'kind': 'Sphere', 'center': [-0.25, 1.35, 0.0]}]},
        {'group_id': 6, 'node': 'Bip01 Forearm.L',
         'shapes': [{'kind': 'Sphere', 'center': [-0.55, 1.35, 0.0]}]},
    ]
    fixture_joints = actor_synthetic_joints(fixture_bodies)
    fixture_by_pair = {
        (joint['body_a'], joint['body_b']): joint
        for joint in fixture_joints
    }
    assert set(fixture_by_pair) == {
        (0, 1), (0, 2), (1, 3), (0, 4), (4, 5), (5, 6)
    }, fixture_by_pair
    assert fixture_by_pair[(1, 3)]['kind'] == 'revolute'
    assert fixture_by_pair[(5, 6)]['kind'] == 'revolute'
    assert fixture_by_pair[(0, 4)]['kind'] == 'spherical'
    assert fixture_by_pair[(0, 1)]['cone_limit'] == 1.8
    assert fixture_by_pair[(0, 2)]['cone_limit'] == 1.8
    for pair, expected_axis in {
        (1, 3): Vector((1.0, 0.0, 0.0)),
        (5, 6): Vector((0.0, 0.0, 1.0)),
    }.items():
        values = fixture_by_pair[pair]['frame_a_rotation_xyzw']
        rotation = __import__('mathutils').Quaternion(
            (values[3], values[0], values[1], values[2]))
        assert (rotation @ Vector((0.0, 0.0, 1.0)) - expected_axis).length < 1e-5
    assert all(joint['source'] == 'SyntheticFallback' for joint in fixture_joints)
    authored_fixture = dict(fixture_by_pair[(1, 3)])
    authored_fixture['source'] = 'Authored'
    completed_fixture = actor_completed_joints([authored_fixture], fixture_bodies)
    completed_by_pair = {
        (joint['body_a'], joint['body_b']): joint
        for joint in completed_fixture
    }
    assert completed_by_pair[(1, 3)]['source'] == 'Authored'
    assert len(completed_fixture) == len(fixture_joints)
    authored_tree = []
    for fixture_joint in fixture_joints:
        authored_joint = dict(fixture_joint)
        authored_joint['source'] = 'Authored'
        authored_tree.append(authored_joint)
    completed_tree = actor_completed_joints(authored_tree, fixture_bodies)
    assert len(completed_tree) == len(authored_tree)
    assert all(joint['source'] == 'Authored' for joint in completed_tree)
    descriptor = type('MalleableDescriptor', (), {
        'type': type('ConstraintType', (), {'name': 'RAGDOLL'})(),
        'ragdoll': object(),
        'limited_hinge': None,
        'strength': 0.9,
    })()
    malleable = type('bhkMalleableConstraint', (), {'constraint': descriptor})()
    kind, payload, strength = nif_constraint_payload(malleable)
    assert kind == 'bhkRagdollConstraint'
    assert payload is descriptor.ragdoll
    assert abs(strength - 0.9) < 1e-6
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

def clear_imported_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    for datablocks in (
            bpy.data.meshes, bpy.data.curves, bpy.data.materials,
            bpy.data.cameras, bpy.data.lights, bpy.data.actions,
            bpy.data.armatures):
        for datablock in list(datablocks):
            if datablock.users == 0:
                datablocks.remove(datablock)

def is_pynifly_render_helper(name):
    # Fallout skeleton NIFs carry BSBound nodes for editor/gameplay bounds.
    # PyNifly 28 imports them as ordinary cube meshes even when collision
    # import is disabled, producing a giant untextured box around the actor.
    normalized = str(name).casefold()
    if normalized.startswith('bsbound:'):
        return True
    base = normalized.rsplit('.', 1)[0] if re.search(r'\.\d{3}$', normalized) else normalized
    return base in {'bodymeat', 'headmeat'} or 'meatcap' in base

def partition_is_editor_visible(flags):
    return int(flags) & 0x0001 != 0

def prune_hidden_actor_partitions(obj, nifnode):
    """Drop BSDismember triangles hidden in the authored intact state."""
    partitions = list(getattr(nifnode, 'partitions', ()) or ())
    partition_tris = list(getattr(nifnode, 'partition_tris', ()) or ())
    if not partitions or not partition_tris:
        return 0
    polygons = list(obj.data.polygons)
    if len(partition_tris) != len(polygons):
        print('[convert] actor partition map mismatch object={} polygons={} entries={}'.format(
            obj.name, len(polygons), len(partition_tris)), flush=True)
        return 0
    hidden_faces = []
    for polygon, partition_index in zip(polygons, partition_tris):
        if partition_index >= len(partitions):
            continue
        if not partition_is_editor_visible(getattr(partitions[partition_index], 'flags', 0)):
            hidden_faces.append(polygon.index)
    if not hidden_faces:
        return 0
    mesh = bmesh.new()
    mesh.from_mesh(obj.data)
    mesh.faces.ensure_lookup_table()
    bmesh.ops.delete(
        mesh,
        geom=[mesh.faces[index] for index in hidden_faces],
        context='FACES',
    )
    mesh.to_mesh(obj.data)
    mesh.free()
    obj.data.update()
    return len(hidden_faces)

def actor_source_key(path):
    return os.path.abspath(str(path)).replace('\\', '/').casefold()

def actor_body_part_visible(index, occupied_slots):
    covered_slot = {0: 0x00000004, 1: 0x00000008, 2: 0x00000010}.get(int(index), 0)
    return covered_slot == 0 or occupied_slots & covered_slot == 0

def actor_shape_texture_values(nifnode, source_fallback):
    """Return the texture set authored for one PyNifly actor shape.

    PyNifly 28 leaves ``NiShape.textures`` empty for Fallout 3's
    BSShaderPPLightingProperty even though its native shader-slot accessor can
    still read the referenced BSShaderTextureSet. Query those per-shape slots
    before falling back to file-wide string recovery; a NIF can contain both
    clothing and race-skin texture sets and must not apply the first one to
    every mesh.
    """
    texture_map = getattr(nifnode, 'textures', None)
    if texture_map:
        values = [value for value in texture_map.values() if value]
        if values:
            return values
    shader = getattr(nifnode, 'shader', None)
    source_file = getattr(nifnode, 'file', None)
    read_texture = getattr(shader, '_readtexture', None)
    if callable(read_texture) and source_file is not None:
        values = []
        for slot in range(1, 9):
            try:
                value = read_texture(source_file._handle, nifnode._handle, slot)
            except (AttributeError, RuntimeError, TypeError, ValueError):
                value = ''
            if value and value.casefold() not in {
                    existing.casefold() for existing in values}:
                values.append(value)
        if values:
            return values
    return list(source_fallback)

def actor_shape_glossiness(nifnode):
    shader = getattr(nifnode, 'shader', None)
    properties = getattr(shader, 'properties', None)
    for source in (properties, shader, nifnode):
        if source is None:
            continue
        for name in ('Glossiness', 'glossiness'):
            value = getattr(source, name, None)
            if value is not None:
                return value
    return None

def actor_material_alpha_policy(alpha_flags):
    """Map authored NiAlphaProperty flags to a glTF alpha policy."""
    flags = int(alpha_flags)
    if flags & 1:
        return 'BLEND'
    if flags & (1 << 9):
        return 'MASK'
    return 'OPAQUE'

def import_pynifly_actor(
        skeleton_path, visual_paths, body_parts, apparel, eye_geometry,
        eye_texture, ragdoll_nodes, model, policy):
    """Import one actor visual assembly with PyNifly 28.

    NIFTools remains responsible for the isolated Havok pass. PyNifly owns the
    exported armatures, inverse bind matrices, weights, and shader materials.
    """
    import addon_utils
    if 'io_scene_nifly' not in bpy.context.preferences.addons:
        addon_utils.enable('io_scene_nifly', default_set=True, persistent=False)
    if 'io_scene_nifly' not in bpy.context.preferences.addons:
        raise RuntimeError('PyNifly is installed but could not be enabled for headless conversion')
    from io_scene_nifly import bl_info as pynifly_info
    from io_scene_nifly import blender_defs as pynifly_blender_defs
    from io_scene_nifly.nif.import_nif import NifImporter
    from io_scene_nifly.pyn import pynifly as pynifly_api
    from io_scene_nifly.util.settings import ImportSettings

    version = tuple(int(value) for value in pynifly_info.get('version', (0, 0, 0)))
    if version < (28, 0, 0):
        raise RuntimeError(
            'PyNifly 28.0.0 or newer is required for actor conversion; found '
            + '.'.join(str(value) for value in version)
        )
    clear_imported_scene()
    settings = ImportSettings(
        rename_bones=False,
        rename_bones_niftools=False,
        rotate_bones_pretty=False,
        blender_xf=False,
        create_bones=True,
        import_tris=False,
        import_cutpoints=False,
        import_animations=False,
        import_shapekeys=False,
        apply_skinning=True,
        smart_editor_markers=False,
        create_collection=False,
        mesh_only=False,
        import_collisions=False,
        import_pose=False,
        reference_skeleton=skeleton_path,
    )
    reference = pynifly_api.NifFile(skeleton_path)
    # PyNifly's standard display transform is 0.1 units; Fallout preparation
    # uses 1/70, so apply the remaining 1/7 scale while retaining its axis fix.
    import_xf = pynifly_blender_defs.blender_import_xf @ Matrix.Scale(1.0 / 7.0, 4)
    importer = NifImporter(
        visual_paths,
        import_settings=settings,
        collection=bpy.context.scene.collection,
        reference_skel=reference,
        base_transform=import_xf,
        scale=1.0,
    )
    importer.execute()

    def source_texture_references(path):
        """Recover Fallout 3 texture paths PyNifly 28 does not expose.

        Some BSShaderPPLightingProperty blocks import with an empty
        ``nifnode.textures`` mapping even though their BSShaderTextureSet has
        valid, null-terminated paths. Limit the fallback to printable texture
        references embedded in that same source NIF.
        """
        try:
            with open(path, 'rb') as source_file:
                payload = source_file.read()
        except OSError:
            return []
        references = []
        for match in re.finditer(
                rb'textures[\\/][\x20-\x7e]*?\.(?:dds|tga|png)',
                payload, flags=re.IGNORECASE):
            value = match.group(0).decode('ascii', errors='ignore')
            if value and value.casefold() not in {
                    existing.casefold() for existing in references}:
                references.append(value)
        return references

    source_texture_values = {
        os.path.abspath(path).casefold(): source_texture_references(path)
        for path in visual_paths
    }
    eye_sources = {actor_source_key(path) for path in eye_geometry}

    def staged_texture_path(value):
        if not value:
            return None
        normalized = str(value).replace('\\', '/').lower()
        marker = normalized.find('textures/')
        if marker >= 0:
            normalized = normalized[marker:]
        root = os.path.abspath(bpy.context.preferences.filepaths.texture_directory)
        if os.path.basename(root).casefold() == 'textures':
            root = os.path.dirname(root)
        candidate = os.path.join(root, *normalized.split('/'))
        png = os.path.splitext(candidate)[0] + '.png'
        if os.path.isfile(png):
            return png
        if os.path.isfile(candidate):
            return candidate
        return None

    def actor_texture_kind(path):
        stem = os.path.basename(path).casefold()
        if '_n.' in stem or 'normal' in stem:
            return 'normal'
        if '_g.' in stem or '_em.' in stem or 'glow' in stem:
            return 'glow'
        if '_sk.' in stem or 'spec' in stem:
            return 'specular'
        return 'diffuse'

    def install_source_material(material, texture_values, glossiness=None):
        resolved = [staged_texture_path(value) for value in texture_values]
        resolved = [value for value in resolved if value]
        if not resolved:
            return False
        diffuse_path = next((path for path in resolved if actor_texture_kind(path) == 'diffuse'), None)
        if diffuse_path is None:
            return False
        diffuse_reference = next((
            canonical_texture_reference(value) for value in texture_values
            if canonical_texture_reference(value) and
            actor_texture_kind(str(value)) == 'diffuse'
        ), None)
        if diffuse_reference:
            material['bevyout_diffuse_texture_path'] = diffuse_reference
        normal_path = next((path for path in resolved if actor_texture_kind(path) == 'normal'), None)
        material.use_nodes = True
        tree = material.node_tree
        tree.nodes.clear()
        output = tree.nodes.new('ShaderNodeOutputMaterial')
        principled = tree.nodes.new('ShaderNodeBsdfPrincipled')
        principled.inputs['Roughness'].default_value = perceptual_roughness_from_glossiness(glossiness)
        set_material_roughness(material, glossiness)
        diffuse = tree.nodes.new('ShaderNodeTexImage')
        diffuse.label = 'Diffuse'
        diffuse.image = bpy.data.images.load(diffuse_path, check_existing=True)
        diffuse.image.colorspace_settings.name = 'sRGB'
        tree.links.new(diffuse.outputs['Color'], principled.inputs['Base Color'])
        alpha_flags = int(getattr(
            getattr(material, 'niftools_alpha', None), 'alphaflag', 0))
        alpha_policy = actor_material_alpha_policy(alpha_flags)
        alpha_output = diffuse.outputs.get('Alpha')
        alpha_input = principled.inputs.get('Alpha')
        if (alpha_policy != 'OPAQUE' and
                alpha_output is not None and alpha_input is not None):
            if alpha_policy == 'MASK':
                clip = tree.nodes.new('ShaderNodeMath')
                clip.operation = 'GREATER_THAN'
                clip.inputs[1].default_value = float(
                    getattr(material, 'alpha_threshold', 0.5))
                tree.links.new(alpha_output, clip.inputs[0])
                tree.links.new(clip.outputs[0], alpha_input)
            else:
                tree.links.new(alpha_output, alpha_input)
            if hasattr(material, 'surface_render_method'):
                material.surface_render_method = (
                    'BLENDED' if alpha_policy == 'BLEND' else 'DITHERED')
        if normal_path:
            normal = tree.nodes.new('ShaderNodeTexImage')
            normal.label = 'Normal'
            normal.image = bpy.data.images.load(normal_path, check_existing=True)
            normal.image.colorspace_settings.name = 'Non-Color'
            normal_map = tree.nodes.new('ShaderNodeNormalMap')
            tree.links.new(normal.outputs['Color'], normal_map.inputs['Color'])
            tree.links.new(normal_map.outputs['Normal'], principled.inputs['Normal'])
            specular_input = principled.inputs.get('Specular IOR Level')
            normal_alpha = normal.outputs.get('Alpha')
            if specular_input is not None and normal_alpha is not None:
                tree.links.new(normal_alpha, specular_input)
        tree.links.new(principled.outputs['BSDF'], output.inputs['Surface'])
        return True

    repaired_materials = 0
    removed_helpers = 0
    removed_hidden_faces = 0
    for represented in importer.objects_created:
        obj = represented.blender_obj
        nifnode = represented.nifnode
        if obj is None or obj.type != 'MESH' or nifnode is None:
            continue
        if is_pynifly_render_helper(obj.name) or is_pynifly_render_helper(
                getattr(nifnode, 'name', '')):
            bpy.data.objects.remove(obj, do_unlink=True)
            removed_helpers += 1
            continue
        hidden_faces = prune_hidden_actor_partitions(obj, nifnode)
        removed_hidden_faces += hidden_faces
        if len(obj.data.polygons) == 0:
            bpy.data.objects.remove(obj, do_unlink=True)
            continue
        source_file = getattr(getattr(nifnode, 'file', None), 'filepath', '')
        if source_file:
            obj['bevyout_actor_source_path'] = str(source_file).replace('\\', '/')
        texture_values = actor_shape_texture_values(
            nifnode,
            source_texture_values.get(
                os.path.abspath(source_file).casefold(), []) if source_file else [],
        )
        glossiness = actor_shape_glossiness(nifnode)
        is_selected_eye = actor_source_key(source_file) in eye_sources
        for slot, material in enumerate(list(obj.data.materials)):
            if material is None:
                continue
            if is_selected_eye and eye_texture:
                # Eye NIFs provide geometry/UVs; the NPC/RACE-selected EYES
                # record owns the diffuse identity. Copy the material so a
                # shared head material cannot be changed accidentally, and
                # retain the authored normal input after the override.
                selected = material.copy()
                obj.data.materials[slot] = selected
                if install_source_material(
                        selected, [eye_texture] + list(texture_values), glossiness):
                    repaired_materials += 1
                    continue
                material = selected
            diffuse_reference = next((
                canonical_texture_reference(value) for value in texture_values
                if canonical_texture_reference(value) and
                actor_texture_kind(str(value)) == 'diffuse'
            ), None)
            if diffuse_reference:
                material['bevyout_diffuse_texture_path'] = diffuse_reference
            set_material_roughness(material, glossiness)
            images = [] if not material.use_nodes else [
                node.image for node in material.node_tree.nodes
                if node.bl_idname == 'ShaderNodeTexImage' and node.image
            ]
            if not images and install_source_material(material, texture_values, glossiness):
                repaired_materials += 1

    # BSBound can arrive outside PyNifly's represented-object list, so sweep
    # the finished scene as the authoritative export set as well.
    for obj in list(bpy.context.scene.objects):
        if obj.type == 'MESH' and is_pynifly_render_helper(obj.name):
            bpy.data.objects.remove(obj, do_unlink=True)
            removed_helpers += 1

    body_part_by_source = {
        actor_source_key(item.get('path', '')): int(item.get('index', -1))
        for item in body_parts if item.get('path')
    }
    apparel_by_source = {
        actor_source_key(item.get('path', '')): item
        for item in apparel if item.get('path')
    }
    successful_apparel = set()
    for obj in bpy.context.scene.objects:
        if obj.type != 'MESH' or not len(obj.data.polygons):
            continue
        source = actor_source_key(obj.get('bevyout_actor_source_path', ''))
        if source not in apparel_by_source:
            continue
        has_armature = any(modifier.type == 'ARMATURE' and modifier.object
                           for modifier in obj.modifiers)
        if obj.vertex_groups and has_armature:
            successful_apparel.add(source)

    occupied_slots = 0
    for source in successful_apparel:
        occupied_slots |= int(apparel_by_source[source].get('biped_slot_mask', 0))
    for source, item in apparel_by_source.items():
        if source in successful_apparel:
            continue
        for obj in list(bpy.context.scene.objects):
            if (obj.type == 'MESH' and
                    actor_source_key(obj.get('bevyout_actor_source_path', '')) == source):
                bpy.data.objects.remove(obj, do_unlink=True)
        print('[convert] actor apparel fallback form_id={:08x} path={}'.format(
            int(item.get('form_id', 0)), item.get('path', '')), flush=True)
    for source, index in body_part_by_source.items():
        if actor_body_part_visible(index, occupied_slots):
            continue
        for obj in list(bpy.context.scene.objects):
            if (obj.type == 'MESH' and
                    actor_source_key(obj.get('bevyout_actor_source_path', '')) == source):
                bpy.data.objects.remove(obj, do_unlink=True)

    remapped_ragdoll_groups = collapse_actor_ragdoll_weights(ragdoll_nodes)

    armatures = [obj for obj in bpy.context.scene.objects if obj.type == 'ARMATURE']
    meshes = [obj for obj in bpy.context.scene.objects
              if obj.type == 'MESH' and len(obj.data.polygons)]
    if not armatures:
        raise RuntimeError('PyNifly actor import produced no armature')
    if not meshes:
        raise RuntimeError('PyNifly actor import produced no render meshes')
    weighted_meshes = []
    for mesh in meshes:
        has_armature = any(modifier.type == 'ARMATURE' and modifier.object
                           for modifier in mesh.modifiers)
        if mesh.vertex_groups and has_armature:
            weighted_meshes.append(mesh)
    if not weighted_meshes:
        raise RuntimeError('PyNifly actor import produced no weighted skinned mesh')
    for mesh in weighted_meshes:
        for material in mesh.data.materials:
            images = [] if material is None or not material.use_nodes else [
                node.image for node in material.node_tree.nodes
                if node.bl_idname == 'ShaderNodeTexImage' and node.image
            ]
            if not images:
                raise RuntimeError(
                    'PyNifly actor material has no resolved texture: '
                    + mesh.name + '/' + (material.name if material else '<none>')
                )

    carrier = max(armatures, key=lambda obj: (
        len(getattr(getattr(obj, 'data', None), 'bones', [])), obj.name.casefold()
    ))
    carrier['bevyout_source_model'] = str(model)
    carrier['bevyout_root_transform_policy'] = str(policy)
    carrier['bevyout_record_zero_non_identity'] = False
    carrier['bevyout_record_zero_transform'] = [
        float(Matrix.Identity(4)[row][column])
        for row in range(4) for column in range(4)
    ]
    print('[convert] PyNifly actor imported version={} armatures={} meshes={} weighted={}'.format(
        '.'.join(str(value) for value in version), len(armatures), len(meshes),
        len(weighted_meshes)), 'repaired_materials={} removed_helpers={} hidden_faces={} apparel={}/{} ragdoll_weight_groups={}'.format(
            repaired_materials, removed_helpers, removed_hidden_faces,
            len(successful_apparel), len(apparel_by_source), remapped_ragdoll_groups), flush=True)
    return [], 0

if sys.argv[-1] == '--self-test-root-policy':
    run_root_transform_self_test()
    raise SystemExit(0)
if sys.argv[-1] == '--self-test-ragdoll-identity':
    run_ragdoll_identity_self_test()
    raise SystemExit(0)
if len(sys.argv) >= 3 and sys.argv[-2] == '--inspect-ragdoll':
    inspected_joints = nif_constraint_joints([sys.argv[-1]])
    print('[convert-test] ragdoll joints={} authored={} fallback={}'.format(
        len(inspected_joints),
        sum(joint.get('source') == 'Authored' for joint in inspected_joints),
        sum(joint.get('source') == 'SyntheticFallback' for joint in inspected_joints),
    ), flush=True)
    print(json.dumps(inspected_joints, sort_keys=True), flush=True)
    raise SystemExit(0)
bpy.context.preferences.filepaths.texture_directory = os.path.abspath(sys.argv[-1])
with open(sys.argv[-2], 'r', encoding='utf8') as f: jobs=json.load(f)
for job in jobs:
    nif_path = job['input']
    assembly_inputs = None
    assembly_skeleton = None
    assembly_body_parts = []
    assembly_apparel = []
    assembly_eye_geometry = []
    assembly_eye_texture = None
    assembly_used_niftools_fallback = False
    if nif_path.casefold().endswith('.actor.json'):
        with open(nif_path, 'r', encoding='utf8') as assembly_file:
            assembly = json.load(assembly_file)
            assembly_skeleton = assembly.get('skeleton')
            assembly_inputs = assembly.get('visual_inputs', [])
            assembly_body_parts = assembly.get('body_parts', [])
            assembly_apparel = assembly.get('apparel', [])
            assembly_eye_geometry = assembly.get('eye_geometry', [])
            assembly_eye_texture = assembly.get('eye_texture')
        if not assembly_skeleton:
            raise RuntimeError('actor assembly manifest has no skeleton: ' + nif_path)
        if not assembly_inputs:
            raise RuntimeError('actor assembly manifest has no visual inputs: ' + nif_path)
    current_joint_defs = nif_constraint_joints(
        [assembly_skeleton] if assembly_skeleton is not None else [nif_path]
    )
    output_path = job['output']
    physics_output_path = job['physics_output']
    conversion = job.get('conversion', 'ao-none')
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    os.makedirs(os.path.dirname(physics_output_path), exist_ok=True)
    non_rendering_prefixes=('shadefade','fx','editormarker','marker','collision')
    def is_non_rendering_object(obj):
        name=obj.name.casefold().replace('_','').replace(' ','')
        return name.startswith(non_rendering_prefixes)
    def import_nif_scene(with_animation, append=False, source_paths=None):
        if not append:
            clear_imported_scene()
        paths = source_paths if source_paths is not None else [nif_path]
        spatial_corrections = []
        collision_corrections = 0
        for source_path in paths:
            before_objects = {obj.as_pointer() for obj in bpy.context.scene.objects}
            result=bpy.ops.import_scene.nif(filepath=source_path, process='EVERYTHING', animation=with_animation, scale_correction=1.0/70.0, use_custom_normals=False, use_embedded_texture=False)
            if 'FINISHED' not in result: raise RuntimeError('NIF import failed: '+source_path)
            # Keep provenance on every imported node so the actor assembly
            # pass can distinguish weapons/helmets from the body parts after
            # NIFTools has created its separate scene roots.
            for imported in bpy.context.scene.objects:
                if imported.as_pointer() not in before_objects:
                    imported['bevyout_actor_source_path'] = source_path.replace('\\', '/')
                    imported['bevyout_nif_source_path'] = canonical_nif_path(source_path)
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
            spatial, collision = apply_verified_spatial_corrections(
                list(bpy.context.scene.objects), job.get('model', '')
            )
            spatial_corrections.extend(spatial)
            collision_corrections += collision
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
        return spatial_corrections, collision_corrections
    if assembly_inputs is not None:
        # Physics is extracted from the explicit reference skeleton through the
        # established NIFTools/nifgen path, then that scratch scene is replaced
        # wholesale by the PyNifly visual import.
        import_nif_scene(False, append=False, source_paths=[assembly_skeleton])
        physics_asset = build_physics_asset()
        try:
            spatial_corrections, collision_corrections = import_pynifly_actor(
                assembly_skeleton,
                assembly_inputs,
                assembly_body_parts,
                assembly_apparel,
                assembly_eye_geometry,
                assembly_eye_texture,
                [body.get('node') for body in physics_asset.get('bodies', [])],
                job.get('model', ''),
                job.get('root_transform_policy', 'preserve_verified'),
            )
        except Exception as error:
            # PyNifly 28 can reject creature attachment NIFs whose shapes are
            # parented directly to bones. NIFTools imports those authored
            # hierarchies correctly, so rebuild the visual scene through the
            # established compatibility path while keeping skeleton-derived
            # physics and the same deterministic assembly inputs.
            print('[convert] actor PyNifly import failed; retrying NIFTools compatibility path: {}'.format(error), flush=True)
            assembly_used_niftools_fallback = True
            spatial_corrections, collision_corrections = import_nif_scene(
                False,
                append=False,
                source_paths=assembly_inputs,
            )
            normalize_actor_assembly()
    else:
        spatial_corrections, collision_corrections = import_nif_scene(
            True,
            append=False,
        )
    if assembly_inputs is None and bpy.data.actions:
        # Controller import bakes an animated pose into the collision
        # objects' transforms (a door's colliders land in the Open position,
        # leaving the doorway hole open), so physics must come from a
        # rest-pose import; the animated scene is rebuilt after for the GLB.
        import_nif_scene(False)
        physics_asset = build_physics_asset()
        spatial_corrections, collision_corrections = import_nif_scene(True)
    elif assembly_inputs is None:
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
    if assembly_inputs is not None:
        source_render_meshes = len(render_meshes)
        source_render_vertices = sum(len(obj.data.vertices) for obj in render_meshes)
        source_render_triangles = sum(
            sum(max(0, len(polygon.vertices) - 2) for polygon in obj.data.polygons)
            for obj in render_meshes
        )
    else:
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
        source_render_meshes = len(source_render_geometries)
        source_render_vertices = sum(
            int(geometry.data.num_vertices) for geometry in source_render_geometries
        )
        source_render_triangles = sum(
            len(geometry.get_triangles()) for geometry in source_render_geometries
        )
    metadata_carrier = next(
        (obj for obj in bpy.context.scene.objects
         if obj.get('bevyout_source_model') == job.get('model', '')),
        None,
    )
    if metadata_carrier is None:
        raise RuntimeError('converted scene lost source metadata carrier: ' + nif_path)
    metadata_carrier['bevyout_source_render_meshes'] = source_render_meshes
    metadata_carrier['bevyout_source_render_vertices'] = source_render_vertices
    metadata_carrier['bevyout_source_render_triangles'] = source_render_triangles
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
        glow_candidate = next((node for node in images if is_glow_image(node)), None)
        semantics = fallout_material_semantics(material)
        if not images:
            # An untextured BSEffectShaderProperty is the native form of a
            # physical light card (RCLightBox01 is one example). NIFTools can
            # import it with zero NiMaterialProperty emission and no image
            # nodes, so use its authored diffuse color as the bulb emission.
            if semantics['effect_shader']:
                principled = next(
                    (node for node in tree.nodes
                     if node.bl_idname == 'ShaderNodeBsdfPrincipled'),
                    None,
                )
                if principled:
                    base_color = principled.inputs.get('Base Color')
                    if base_color:
                        for link in list(base_color.links):
                            tree.links.remove(link)
                        base_color.default_value = (0.0, 0.0, 0.0, material.diffuse_color[3])
                    new_emission = principled.inputs.get('Emission Color') or principled.inputs.get('Emission')
                    authored = authored_emission_color(material)
                    diffuse_color = tuple(float(channel) for channel in material.diffuse_color[:3])
                    emission_color = authored or diffuse_color
                    if new_emission and any(channel != 0.0 for channel in emission_color):
                        for link in list(new_emission.links):
                            tree.links.remove(link)
                        new_emission.default_value = (*emission_color, 1.0)
                        strength = principled.inputs.get('Emission Strength')
                        if strength:
                            multiplier, has_multiplier = source_emission_multiplier(material)
                            strength.default_value = min(
                                FALLOUT_EMISSIVE_MAX,
                                max(0.0, (multiplier if has_multiplier else 1.0) * FALLOUT_EMISSIVE_SCALE),
                            )
            continue
        glow = glow_candidate if semantics['features']['glow_map'] else None
        diffuse = next((node for node in images if node is not glow and 'normal' not in node.label.lower() and '_n.' not in image_name(node) and not is_glow_image(node)), images[0])
        normal = next((node for node in images if node is not diffuse and ('normal' in node.label.lower() or '_n.' in node.image.name.lower())), None)
        diffuse_reference = canonical_texture_reference(
            getattr(diffuse.image, 'filepath', '') or diffuse.image.name
        )
        if diffuse_reference:
            material['bevyout_diffuse_texture_path'] = diffuse_reference
        material_roughness = material.get(
            'bevyout_perceptual_roughness',
            perceptual_roughness_from_glossiness(None),
        )
        material['bevyout_perceptual_roughness'] = material_roughness
        material.roughness = material_roughness
        for node in tree.nodes:
            if node.bl_idname == 'ShaderNodeBsdfPrincipled':
                roughness_input = node.inputs.get('Roughness')
                if roughness_input is not None and not roughness_input.is_linked:
                    roughness_input.default_value = material_roughness
        # NIFTools can leave imported actor maps tagged as Non-Color. That
        # makes the skin atlas sample as linear data in the exported GLB and
        # produces the uniformly pale/pink appearance seen in the viewer.
        # Base-color maps must be sRGB; normals/specular alpha must remain
        # Non-Color data.
        try:
            diffuse.image.colorspace_settings.name = 'sRGB'
            if normal:
                normal.image.colorspace_settings.name = 'Non-Color'
        except (AttributeError, TypeError, ValueError):
            # Older NIFTools/Blender combinations may not expose the color
            # space property; the material links below are still valid.
            pass
        if assembly_inputs is not None and not assembly_used_niftools_fallback:
            # PyNifly already authored its Principled/alpha/normal graph. Keep
            # it intact; only normalize texture color spaces above.
            continue
        bulb_override = material.get('bevyout_emissive_bulb', False)
        explicit_emission_source = authored_emission_color(material)
        emission_multiplier, has_emission_multiplier = source_emission_multiplier(material)
        explicit_environment_emission = (
            semantics['shader_type'] == 1 and
            explicit_emission_source is not None and
            glow_candidate is None and
            (
                bool(semantics['shader_flags_1'] & (1 << 7)) or
                (has_emission_multiplier and emission_multiplier >= 10.0)
            )
        )
        effect_shader_fallback = bool(semantics['effect_shader']) and explicit_emission_source is None
        emission_authorized = (
            bool(semantics['emission_authorized']) or
            bool(bulb_override) or
            (
                explicit_emission_source is not None and
                semantics['shader_type'] != 1 and
                not semantics['no_lighting_shader']
            ) or
            explicit_environment_emission or
            effect_shader_fallback
        )
        semantics['emission_authorized'] = emission_authorized
        material['bevyout_fallout_material'] = json.dumps(
            semantics, sort_keys=True, separators=(',', ':')
        )
        authored_emission = explicit_emission_source if emission_authorized else None
        if not emission_authorized:
            # Do not carry NIFTools' imported Principled emission through the
            # rebuild for non-glow materials.  This is the path that prevents
            # environment-map props such as RadAway from becoming uniformly
            # orange/yellow when their authored multiplier is nonzero.
            has_emission_multiplier = False
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
        if not emission_authorized:
            emission_link = None
            emission_color = None
            emission_strength = None
        if semantics['effect_shader'] and not emission_link:
            emission_link = diffuse.outputs['Color']
        principled = tree.nodes.new('ShaderNodeBsdfPrincipled')
        principled.inputs['Roughness'].default_value = material_roughness
        if semantics['effect_shader']:
            principled.inputs['Base Color'].default_value = (0.0, 0.0, 0.0, 1.0)
        else:
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
        if new_emission_strength and not emission_authorized:
            new_emission_strength.default_value = 0.0
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
        if semantics['effect_shader'] and new_emission_strength and (
                emission_strength is None or
                not math.isfinite(emission_strength) or
                emission_strength <= 0.0 or
                emission_strength == 1.0):
            new_emission_strength.default_value = emission_multiplier if has_emission_multiplier else 1.0
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
        if new_emission_strength and not new_emission_strength.is_linked:
            # The imported Fallout values are calibrated for the original
            # renderer; Bevy's additive bloom needs a quarter-strength export.
            new_emission_strength.default_value = max(
                0.0,
                min(
                    FALLOUT_EMISSIVE_MAX,
                    float(new_emission_strength.default_value) * FALLOUT_EMISSIVE_SCALE,
                ),
            )
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
