[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function color\_plane\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#193)

```rust
pub fn color_plane_bundle<B>(
    plane: FeathersColorPlane,
    overrides: B,
) -> impl Bundlewhere
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the color\_plane() BSN function

Template function to spawn a “color plane”, which is a 2d picker that allows selecting two components of a color space.

The control emits a [`ValueChange<Vec2>`](../../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") representing the current x and y values, ranging from 0 to 1. The control accepts a [`Vec3`](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") input value, where the third component (‘z’) is used to provide the fixed constant channel for the background gradient.

The control does not do any color space conversions internally, other than the shader code for displaying gradients. Avoiding excess conversions helps avoid gimble-lock problems when implementing a color picker for cylindrical color spaces such as HSL.

## Arguments

*   `overrides` - a bundle of components that are merged in with the normal swatch components.