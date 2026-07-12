[bevy](../../index.html)::[ui](../index.html)

# Module experimental 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#33)

Experimental features are not yet stable and may change or be removed in the future.

These features are not recommended for production use, but are available to ease experimentation within Bevy’s ecosystem. Please let us know how you are using these features and what you would like to see improved!

These may be feature-flagged: check the `Cargo.toml` for `bevy_ui` to see what options are available.

## Warning

Be careful when using these features, especially in concert with third-party crates, as they may not be fully supported, functional or stable.

## Structs

[GhostNode](struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")`ghost_nodes`

Marker component for entities that should be ignored within UI hierarchies.

[UiChildren](struct.UiChildren.html "struct bevy::ui::experimental::UiChildren")`ghost_nodes`

System param that gives access to UI children utilities, skipping over [`GhostNode`](struct.GhostNode.html "struct bevy::ui::experimental::GhostNode").

[UiChildrenIter](struct.UiChildrenIter.html "struct bevy::ui::experimental::UiChildrenIter")`ghost_nodes`

[UiRootNodes](struct.UiRootNodes.html "struct bevy::ui::experimental::UiRootNodes")`ghost_nodes`

System param that allows iteration of all UI root nodes.