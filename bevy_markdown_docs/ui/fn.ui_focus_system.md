[bevy](../index.html)::[ui](index.html)

# Function ui\_focus\_system 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#148-160)

```rust
pub fn ui_focus_system(
    hovered_nodes: Local<'_, Vec<Entity>>,
    state: Local<'_, State>,
    camera_query: Query<'_, '_, (Entity, &Camera, &RenderTarget)>,
    primary_window: Query<'_, '_, Entity, With<PrimaryWindow>>,
    windows: Query<'_, '_, &Window>,
    mouse_button_input: Res<'_, ButtonInput<MouseButton>>,
    touches_input: Res<'_, Touches>,
    ui_stack: Res<'_, UiStack>,
    node_query: Query<'_, '_, NodeQuery>,
    clipping_query: Query<'_, '_, (&ComputedNode, &UiGlobalTransform, &Node)>,
    child_of_query: Query<'_, '_, &ChildOf, Without<OverrideClip>>,
)
```

The system that sets Interaction for all UI elements based on the mouse cursor activity

Entities with a hidden [`InheritedVisibility`](../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility") are always treated as released.