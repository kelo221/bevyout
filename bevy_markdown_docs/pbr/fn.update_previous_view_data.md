[bevy](../index.html)::[pbr](index.html)

# Function update\_previous\_view\_data 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#195-198)

```rust
pub fn update_previous_view_data(
    commands: Commands<'_, '_>,
    query: Query<'_, '_, (Entity, &Camera, &GlobalTransform), Or<(With<Camera3d>, With<ShadowView>)>>,
)
```