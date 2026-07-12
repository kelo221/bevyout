[bevy](../../index.html)::[ui](../index.html)::[update](index.html)

# Function update\_clipping\_system 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/update.rs.html#21-32)

```rust
pub fn update_clipping_system(
    commands: Commands<'_, '_>,
    root_nodes: UiRootNodes<'_, '_>,
    node_query: Query<'_, '_, (&Node, &ComputedNode, &UiGlobalTransform, Option<&mut CalculatedClip>, Has<OverrideClip>)>,
    ui_children: UiChildren<'_, '_>,
)
```

Updates clipping for all nodes