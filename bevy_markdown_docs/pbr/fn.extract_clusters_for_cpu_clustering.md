[bevy](../index.html)::[pbr](index.html)

# Function extract\_clusters\_for\_cpu\_clustering 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#385-395)

```rust
pub fn extract_clusters_for_cpu_clustering(
    commands: Commands<'_, '_>,
    views: Extract<'_, '_, Query<'_, '_, (RenderEntity, &Clusters, &Camera)>>,
    mapper: Extract<'_, '_, Query<'_, '_, (Option<&RenderEntity>, (Has<PointLight>, Has<SpotLight>, Has<EnvironmentMapLight>, Has<IrradianceVolume>, Has<ClusteredDecal>)), Or<(With<PointLight>, With<SpotLight>, With<EnvironmentMapLight>, With<IrradianceVolume>, With<ClusteredDecal>)>>>,
    global_cluster_settings: Extract<'_, '_, Res<'_, GlobalClusterSettings>>,
)
```

Extracts the clusters that the CPU produced into the render world.