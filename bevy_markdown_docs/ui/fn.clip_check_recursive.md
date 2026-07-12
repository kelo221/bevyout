[bevy](../index.html)::[ui](index.html)

# Function clip\_check\_recursive 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#345-350)

```rust
pub fn clip_check_recursive(
    point: Vec2,
    entity: Entity,
    clipping_query: &Query<'_, '_, (&ComputedNode, &UiGlobalTransform, &Node)>,
    child_of_query: &Query<'_, '_, &ChildOf, Without<OverrideClip>>,
) -> bool
```

Walk up the tree child-to-parent checking that `point` is not clipped by any ancestor node. If `entity` has an [`OverrideClip`](../prelude/struct.OverrideClip.html "struct bevy::prelude::OverrideClip") component it ignores any inherited clipping and returns true.