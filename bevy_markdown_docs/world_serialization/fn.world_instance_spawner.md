[bevy](../index.html)::[world\_serialization](index.html)

# Function world\_instance\_spawner 

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#688-699)

```rust
pub fn world_instance_spawner(
    commands: Commands<'_, '_>,
    world_assets_to_spawn: Query<'_, '_, (Entity, &WorldAssetRoot, Option<&mut WorldInstance>), (Changed<WorldAssetRoot>, Without<DynamicWorldRoot>)>,
    dynamic_worlds_to_spawn: Query<'_, '_, (Entity, &DynamicWorldRoot, Option<&mut WorldInstance>), (Changed<DynamicWorldRoot>, Without<WorldAssetRoot>)>,
    world_asset_spawner: ResMut<'_, WorldInstanceSpawner>,
)
```

System that will spawn instances from the [`WorldAssetRoot`](../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot") and [`DynamicWorldRoot`](../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot") components.