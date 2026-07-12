[bevy](../index.html)::[ui](index.html)

# Function ui\_layout\_system 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/layout/mod.rs.html#77-105)

```rust
pub fn ui_layout_system(
    ui_surface: ResMut<'_, UiSurface>,
    ui_root_node_query: UiRootNodes<'_, '_>,
    ui_children: UiChildren<'_, '_>,
    node_query: Query<'_, '_, (Entity, Ref<'_, Node>, &mut ContentSize, Ref<'_, ComputedUiRenderTargetInfo>)>,
    added_node_query: Query<'_, '_, (), Added<Node>>,
    node_update_query: Query<'_, '_, (&mut ComputedNode, &UiTransform, &mut UiGlobalTransform, &Node, Option<&LayoutConfig>, Option<&Outline>, Option<&ScrollPosition>, Option<&IgnoreScroll>)>,
    buffer_query: Query<'_, '_, &mut ComputedTextBlock>,
    font_system: ResMut<'_, FontCx>,
    removed_children: RemovedComponents<'_, '_, Children>,
    removed_nodes: RemovedComponents<'_, '_, Node>,
    removed_ghost_nodes: RemovedComponents<'_, '_, GhostNode>,
    added_ghost_node_query: Query<'_, '_, Entity, Added<GhostNode>>,
    ghost_node_query: Query<'_, '_, (), With<GhostNode>>,
)
```

Updates the UI’s layout tree, computes the new layout geometry and then updates the sizes and transforms of all the UI nodes.