[bevy](../../index.html)::[input\_focus](../index.html)::[navigator](index.html)

# Function find\_best\_candidate 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/navigator.rs.html#144-149)

```rust
pub fn find_best_candidate(
    origin: &FocusableArea,
    direction: CompassOctant,
    candidates: &[FocusableArea],
    config: &AutoNavigationConfig,
) -> Option<Entity>
```

Finds the best entity to navigate to from the origin towards the given direction.

For details on what “best” means here, refer to [`AutoNavigationConfig`](../directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig"), which configures how candidates are scored.