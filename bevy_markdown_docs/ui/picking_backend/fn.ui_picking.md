[bevy](../../index.html)::[ui](../index.html)::[picking\_backend](index.html)

# Function ui\_picking 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#101-112)

```rust
pub fn ui_picking(
    pointers: Query<'_, '_, (&PointerId, &PointerLocation)>,
    camera_query: Query<'_, '_, (Entity, &Camera, &RenderTarget, Has<UiPickingCamera>)>,
    primary_window: Query<'_, '_, Entity, With<PrimaryWindow>>,
    settings: Res<'_, UiPickingSettings>,
    ui_stack: Res<'_, UiStack>,
    node_query: Query<'_, '_, NodeQuery>,
    output: MessageWriter<'_, PointerHits>,
    clipping_query: Query<'_, '_, (&ComputedNode, &UiGlobalTransform, &Node)>,
    child_of_query: Query<'_, '_, &ChildOf, Without<OverrideClip>>,
    pickable_query: Query<'_, '_, &Pickable>,
)
```

Available on **crate feature `bevy_picking`** only.

Computes the UI node entities under each pointer.

Bevy’s [`UiStack`](../struct.UiStack.html "struct bevy::ui::UiStack") orders all nodes in the order they will be rendered, which is the same order we need for determining picking.