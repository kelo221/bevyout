[bevy](../index.html)::[pbr](index.html)

# Function prepare\_clusters\_for\_cpu\_clustering 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#468-480)

```rust
pub fn prepare_clusters_for_cpu_clustering(
    commands: Commands<'_, '_>,
    render_device: Res<'_, RenderDevice>,
    render_queue: Res<'_, RenderQueue>,
    mesh_pipeline: Res<'_, MeshPipeline>,
    global_clusterable_object_meta: Res<'_, GlobalClusterableObjectMeta>,
    views: Query<'_, '_, (Entity, &ExtractedClusterableObjects, Option<&RenderViewLightProbes<EnvironmentMapLight>>, Option<&RenderViewLightProbes<IrradianceVolume>>)>,
)
```

Creates and populates the GPU buffers that store clusters when CPU clustering is being used.