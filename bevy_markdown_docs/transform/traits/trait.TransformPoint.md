[bevy](../../index.html)::[transform](../index.html)::[traits](index.html)

# Trait TransformPoint 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#6)

```rust
pub trait TransformPoint {
    // Required method
    fn transform_point(&self, point: impl Into<Vec3>) -> Vec3;
}
```

A trait for point transformation methods.

## Required Methods

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#8)

#### fn [transform\_point](#tymethod.transform_point)(&self, point: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

Transform a point.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#32)

### impl [TransformPoint](../../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [Affine3A](../../math/struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#18)

### impl [TransformPoint](../../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [GlobalTransform](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#39)

### impl [TransformPoint](../../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [Isometry3d](../../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#25)

### impl [TransformPoint](../../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [Mat4](../../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/traits.rs.html#11)

### impl [TransformPoint](../../prelude/trait.TransformPoint.html "trait bevy::prelude::TransformPoint") for [Transform](../../prelude/struct.Transform.html "struct bevy::prelude::Transform")