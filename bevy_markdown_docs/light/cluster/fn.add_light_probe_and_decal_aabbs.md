[bevy](../../index.html)::[light](../index.html)::[cluster](index.html)

# Function add\_light\_probe\_and\_decal\_aabbs 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#514-520)

```rust
pub fn add_light_probe_and_decal_aabbs(
    commands: Commands<'_, '_>,
    light_probes_and_decals_query: Query<'_, '_, Entity, (Or<(With<ClusteredDecal>, With<LightProbe>)>, Without<Aabb>)>,
)
```

A system that adds AABBs to light probes and decals so that the visibility determination works for them.