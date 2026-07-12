[bevy](../../index.html)::[input\_focus](../index.html)::[directional\_navigation](index.html)

# Function auto\_generate\_navigation\_edges 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#518-522)

```rust
pub fn auto_generate_navigation_edges(
    nav_map: &mut DirectionalNavigationMap,
    nodes: &[FocusableArea],
    config: &AutoNavigationConfig,
)
```

Automatically generates directional navigation edges for a collection of nodes.

This function takes a slice of navigation nodes with their positions and sizes, and populates the navigation map with edges to the nearest neighbor in each compass direction. Manual edges already in the map are preserved and not overwritten.

## Arguments

*   `nav_map` - The navigation map to populate
*   `nodes` - A slice of [`FocusableArea`](struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea") structs containing entity, position, and size data
*   `config` - Configuration for the auto-generation algorithm

## Example

```rust
let mut nav_map = DirectionalNavigationMap::default();
let config = AutoNavigationConfig::default();

let nodes = vec![
    FocusableArea { entity: Entity::PLACEHOLDER, position: Vec2::new(100.0, 100.0), size: Vec2::new(50.0, 50.0) },
    FocusableArea { entity: Entity::PLACEHOLDER, position: Vec2::new(200.0, 100.0), size: Vec2::new(50.0, 50.0) },
];

auto_generate_navigation_edges(&mut nav_map, &nodes, &config);
```