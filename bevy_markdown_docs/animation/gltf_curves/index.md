[bevy](../../index.html)::[animation](../index.html)

# Module gltf\_curves 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#14)

Concrete curve structures used to load glTF curves into the animation system.

## Structs

[CubicKeyframeCurve](struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")

A keyframe-defined curve that uses cubic spline interpolation, backed by a contiguous buffer.

[CubicRotationCurve](struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

A keyframe-defined curve that uses cubic spline interpolation, special-cased for quaternions since it uses `Vec4` internally.

[SteppedKeyframeCurve](struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")

A keyframe-defined curve that “interpolates” by stepping at `t = 1.0` to the next keyframe.

[WideCubicKeyframeCurve](struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")

A keyframe-defined curve that uses cubic interpolation over many samples at once, backed by a contiguous buffer.

[WideLinearKeyframeCurve](struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")

A keyframe-defined curve that uses linear interpolation over many samples at once, backed by a contiguous buffer.

[WideSteppedKeyframeCurve](struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")

A keyframe-defined curve that uses stepped “interpolation” over many samples at once, backed by a contiguous buffer.

## Enums

[WideKeyframeCurveError](enum.WideKeyframeCurveError.html "enum bevy::animation::gltf_curves::WideKeyframeCurveError")

An error indicating that a multisampling keyframe curve could not be constructed.