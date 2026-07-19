"""Build animation-only GLB clip packs from Fallout external KF files.

PyNifly creates the exact armature/bind pose used by the prepared actor
appearance path. NIFTools then applies each external KF to that armature.
Every clip is isolated in the report so a malformed or incompatible KF does
not prevent the remaining clips from being exported.
"""

import json
import math
import os
import re
import sys
import traceback

import addon_utils
import bpy
from mathutils import Matrix


class _LegacyActionFCurves:
    """Expose Blender 5 layered curves through NIFTools' legacy API."""

    def __init__(self, action):
        self.action = action

    def _collection(self):
        if not self.action.slots or not self.action.layers:
            return ()
        strips = self.action.layers[0].strips
        if not strips:
            return ()
        bag = strips[0].channelbag(self.action.slots[0])
        return () if bag is None else bag.fcurves

    def new(self, data_path, index=0, action_group=""):
        if not self.action.slots or not self.action.layers:
            raise RuntimeError("animation action has no layered channel storage")
        strips = self.action.layers[0].strips
        if not strips:
            raise RuntimeError("animation action has no keyframe strip")
        bag = strips[0].channelbag(self.action.slots[0], ensure=True)
        return bag.fcurves.new(
            data_path=data_path,
            index=index,
            group_name=action_group,
        )

    def __iter__(self):
        return iter(self._collection())

    def __len__(self):
        return len(self._collection())

    def __bool__(self):
        return bool(len(self))


def install_niftools_layered_action_adapter():
    """Bridge NIFTools' Action.fcurves writes to Blender 5's slot API."""
    if hasattr(bpy.types.Action, "fcurves"):
        return

    from io_scene_niftools.modules.nif_import.animation import Animation

    original_create_action = Animation.create_action

    def create_layered_action(importer, b_obj, action_name):
        action = original_create_action(importer, b_obj, action_name)
        slot = action.slots[0] if action.slots else action.slots.new(
            b_obj.id_type, b_obj.name
        )
        if not action.layers:
            layer = action.layers.new("KF Layer")
            layer.strips.new(type="KEYFRAME")
        b_obj.animation_data.action = action
        b_obj.animation_data.action_slot = slot
        return action

    Animation.create_action = create_layered_action
    bpy.types.Action.fcurves = property(lambda action: _LegacyActionFCurves(action))


def enable_addon(module, label):
    if module not in bpy.context.preferences.addons:
        addon_utils.enable(module, default_set=True, persistent=False)
    if module not in bpy.context.preferences.addons:
        raise RuntimeError(label + " addon could not be enabled")


def clear_scene():
    bpy.ops.object.select_all(action="SELECT")
    bpy.ops.object.delete(use_global=False)
    for collection in (
        bpy.data.actions,
        bpy.data.armatures,
        bpy.data.meshes,
        bpy.data.materials,
        bpy.data.images,
    ):
        for item in list(collection):
            collection.remove(item)


def import_pynifly_skeleton(skeleton_path):
    enable_addon("io_scene_nifly", "PyNifly")
    from io_scene_nifly import blender_defs as pynifly_blender_defs
    from io_scene_nifly.nif.import_nif import NifImporter
    from io_scene_nifly.pyn import pynifly as pynifly_api
    from io_scene_nifly.util.settings import ImportSettings

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
    import_xf = pynifly_blender_defs.blender_import_xf @ Matrix.Scale(1.0 / 7.0, 4)
    importer = NifImporter(
        [skeleton_path],
        import_settings=settings,
        collection=bpy.context.scene.collection,
        reference_skel=reference,
        base_transform=import_xf,
        scale=1.0,
    )
    importer.execute()
    armatures = [obj for obj in bpy.context.scene.objects if obj.type == "ARMATURE"]
    if not armatures:
        raise RuntimeError("PyNifly skeleton import created no armature")
    armature = sorted(armatures, key=lambda obj: obj.name.casefold())[0]
    bpy.ops.object.select_all(action="DESELECT")
    armature.select_set(True)
    bpy.context.view_layer.objects.active = armature
    # Clip packs intentionally contain only the target hierarchy and animation.
    for obj in list(bpy.context.scene.objects):
        if obj is not armature:
            bpy.data.objects.remove(obj, do_unlink=True)
    return armature


def import_niftools_skeleton(skeleton_path):
    """Compatibility path for skeleton NIFs with no skinned render shapes.

    PyNifly materializes an armature while importing an actor assembly, but a
    bare Fallout skeleton can contain only nodes/collision. NIFTools creates
    the equivalent named armature directly, which is also the armature its KF
    importer is designed to consume.
    """
    clear_scene()
    result = bpy.ops.import_scene.nif(
        filepath=skeleton_path,
        process="SKELETON_ONLY",
        animation=False,
        scale_correction=1.0 / 70.0,
        use_custom_normals=False,
        use_embedded_texture=False,
    )
    if "FINISHED" not in result:
        raise RuntimeError("NIFTools skeleton import returned " + repr(result))
    armatures = [obj for obj in bpy.context.scene.objects if obj.type == "ARMATURE"]
    if not armatures:
        raise RuntimeError("NIFTools skeleton import created no armature")
    armature = sorted(armatures, key=lambda obj: obj.name.casefold())[0]
    bpy.ops.object.select_all(action="DESELECT")
    armature.select_set(True)
    bpy.context.view_layer.objects.active = armature
    for obj in list(bpy.context.scene.objects):
        if obj is not armature:
            bpy.data.objects.remove(obj, do_unlink=True)
    return armature


def normalized_cycle_type(value):
    name = getattr(value, "name", str(value)).rsplit(".", 1)[-1].casefold()
    return {
        "cycle_loop": "loop",
        "cycle_clamp": "clamp",
        "cycle_reverse": "reverse",
    }.get(name, "unknown")


def source_metadata(kf_path):
    from io_scene_niftools.file_io.nif import NifFile as KFFile
    from io_scene_niftools.modules.nif_import.object import block_registry
    from io_scene_niftools.utils.logging import NifLog, _MockOperator

    # Blender destroys an operator's StructRNA as soon as bpy.ops returns.
    # NIFTools retains that operator as its logger, so reset the logger before
    # parsing the next standalone file; the KF operator installs itself again.
    NifLog.op = _MockOperator()

    data = KFFile.load_nif(kf_path)
    targets = []
    controller_types = []
    interpolator_types = []
    sequence_names = []
    start_times = []
    stop_times = []
    frequencies = []
    phases = []
    cycle_types = []
    accumulation_roots = []
    text_keys = []
    for root in data.roots:
        sequence_name = str(getattr(root, "name", "")).strip()
        if sequence_name:
            sequence_names.append(sequence_name)
        for values, attribute in (
            (start_times, "start_time"),
            (stop_times, "stop_time"),
            (frequencies, "frequency"),
            (phases, "phase"),
        ):
            value = getattr(root, attribute, None)
            if value is not None and math.isfinite(float(value)):
                values.append(float(value))
        cycle_types.append(normalized_cycle_type(getattr(root, "cycle_type", None)))
        accumulation_root = str(getattr(root, "accum_root_name", "")).strip()
        if accumulation_root:
            accumulation_roots.append(accumulation_root)
        root_text_keys = getattr(root, "text_keys", None)
        if root_text_keys:
            for key in root_text_keys.text_keys:
                time_seconds = float(key.time)
                value = str(key.value)
                if math.isfinite(time_seconds):
                    text_keys.append(
                        {"time_seconds": time_seconds, "value": value}
                    )
        for block in getattr(root, "controlled_blocks", []):
            value = getattr(block, "target_name", "")
            if not value:
                try:
                    value = block.get_node_name()
                except Exception:
                    value = ""
            if value:
                targets.append(block_registry.get_bone_name_for_blender(value))
            controller_type = str(getattr(block, "controller_type", "")).strip()
            if controller_type:
                controller_types.append(controller_type)
            interpolator = getattr(block, "interpolator", None)
            if interpolator is not None:
                interpolator_types.append(type(interpolator).__name__)
    cycle_types = sorted(set(cycle_types))
    sequence_names = sorted(set(sequence_names), key=str.casefold)
    accumulation_roots = sorted(set(accumulation_roots), key=str.casefold)
    text_keys.sort(key=lambda item: (item["time_seconds"], item["value"].casefold(), item["value"]))
    return {
        "source_sequence_name": sequence_names[0] if len(sequence_names) == 1 else None,
        "source_start_seconds": min(start_times) if start_times else None,
        "source_end_seconds": max(stop_times) if stop_times else None,
        "source_frequency": frequencies[0] if len(set(frequencies)) == 1 else None,
        "source_phase": phases[0] if len(set(phases)) == 1 else None,
        "loop_mode": (
            cycle_types[0]
            if len(cycle_types) == 1
            else ("mixed" if cycle_types else "unknown")
        ),
        "root_motion_policy": "preserve_authored",
        "accumulation_root": accumulation_roots[0] if len(accumulation_roots) == 1 else None,
        "required_targets": sorted(set(targets), key=str.casefold),
        "controller_types": sorted(set(controller_types), key=str.casefold),
        "interpolator_types": sorted(set(interpolator_types), key=str.casefold),
        "text_keys": text_keys,
    }


def animated_targets(action):
    pattern = re.compile(r'^pose\.bones\["([^"]+)"\]')
    targets = []
    for curve in action.fcurves:
        match = pattern.match(curve.data_path)
        if match:
            targets.append(match.group(1))
    return sorted(set(targets), key=str.casefold)


def import_clip(armature, clip, metadata):
    before = {action.as_pointer() for action in bpy.data.actions}
    result = bpy.ops.import_scene.kf(
        filepath=clip["path"],
        files=[{"name": os.path.basename(clip["path"])}],
        scale_correction=1.0 / 70.0,
    )
    if "FINISHED" not in result:
        raise RuntimeError("NIFTools KF import returned " + repr(result))
    actions = [
        action for action in bpy.data.actions if action.as_pointer() not in before
    ]
    actions.sort(key=lambda action: (-len(action.fcurves), action.name.casefold()))
    if not actions or not actions[0].fcurves:
        raise RuntimeError("KF produced no animated channels on the prepared skeleton")
    action = actions[0]
    for extra in actions[1:]:
        bpy.data.actions.remove(extra)
    action.name = clip["name"]
    armature.animation_data_create()
    armature.animation_data.action = None
    track = armature.animation_data.nla_tracks.new()
    track.name = clip["name"]
    track.strips.new(clip["name"], int(round(action.frame_range[0])), action)
    targets = animated_targets(action)
    target_lookup = {target.casefold() for target in targets}
    missing = [
        target
        for target in metadata["required_targets"]
        if target.casefold() not in target_lookup
    ]
    fps = max(float(bpy.context.scene.render.fps), 1.0)
    duration = max(0.0, float(action.frame_range[1] - action.frame_range[0]) / fps)
    return metadata | {
        "name": clip["name"],
        "source_path": clip["source_path"],
        "success": True,
        "duration_seconds": duration if math.isfinite(duration) else 0.0,
        "animated_channel_count": len(action.fcurves),
        "animated_target_count": len(targets),
        "animated_targets": targets,
        "missing_targets": missing,
        "error": None,
    }


def failed_clip(clip, error, metadata=None):
    return (metadata or {
        "source_sequence_name": None,
        "source_start_seconds": None,
        "source_end_seconds": None,
        "source_frequency": None,
        "source_phase": None,
        "loop_mode": "unknown",
        "root_motion_policy": "unknown",
        "accumulation_root": None,
        "required_targets": [],
        "controller_types": [],
        "interpolator_types": [],
        "text_keys": [],
    }) | {
        "name": clip["name"],
        "source_path": clip["source_path"],
        "success": False,
        "duration_seconds": None,
        "animated_channel_count": 0,
        "animated_target_count": 0,
        "animated_targets": [],
        "missing_targets": [],
        "error": str(error),
    }


def process_job(job):
    report = {
        "revision": job["revision"],
        "skeleton_path": job["skeleton_path"],
        "clips": [],
        "pack_error": None,
    }
    try:
        clear_scene()
        enable_addon("io_scene_niftools", "NIFTools")
        install_niftools_layered_action_adapter()
        try:
            armature = import_pynifly_skeleton(job["skeleton"])
        except Exception as pynifly_error:
            print(
                "[actor-animation] bare PyNifly skeleton path unavailable; "
                "using NIFTools armature compatibility path: {}".format(pynifly_error),
                flush=True,
            )
            armature = import_niftools_skeleton(job["skeleton"])
        for index, clip in enumerate(job["clips"]):
            metadata = None
            try:
                metadata = source_metadata(clip["path"])
                report["clips"].append(import_clip(armature, clip, metadata))
            except Exception as error:
                report["clips"].append(failed_clip(clip, error, metadata))
                print(
                    "[actor-animation] failed {}/{} {}: {}".format(
                        index + 1, len(job["clips"]), clip["source_path"], error
                    ),
                    flush=True,
                )
            if (index + 1) % 50 == 0 or index + 1 == len(job["clips"]):
                print(
                    "[actor-animation] imported {}/{} for {}".format(
                        index + 1, len(job["clips"]), job["skeleton_path"]
                    ),
                    flush=True,
                )
        if not any(clip["success"] for clip in report["clips"]):
            raise RuntimeError("no compatible KF clips were imported")
        os.makedirs(os.path.dirname(job["output"]), exist_ok=True)
        armature.animation_data.action = None
        bpy.ops.export_scene.gltf(
            filepath=job["output"],
            export_format="GLB",
            export_materials="NONE",
            export_apply=False,
            export_extras=True,
            export_animations=True,
            export_animation_mode="NLA_TRACKS",
            export_skins=True,
        )
    except Exception as error:
        report["pack_error"] = str(error)
        print("[actor-animation] pack failed: " + traceback.format_exc(), flush=True)
        if os.path.exists(job["output"]):
            os.remove(job["output"])
    os.makedirs(os.path.dirname(job["report"]), exist_ok=True)
    with open(job["report"], "w", encoding="utf8") as output:
        json.dump(report, output, sort_keys=True, separators=(",", ":"))


with open(sys.argv[-1], "r", encoding="utf8") as jobs_file:
    jobs = json.load(jobs_file)
for current_job in jobs:
    process_job(current_job)
