[bevy](../index.html)::[pbr](index.html)

# Struct MeshPipelineKey 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

```rust
pub struct MeshPipelineKey(/* private fields */);
```

## Implementations

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [NONE](#associatedconstant.NONE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [MORPH\_TARGETS](#associatedconstant.MORPH_TARGETS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_IN\_SHADER](#associatedconstant.TONEMAP_IN_SHADER): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [DEBAND\_DITHER](#associatedconstant.DEBAND_DITHER): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [DEPTH\_PREPASS](#associatedconstant.DEPTH_PREPASS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [NORMAL\_PREPASS](#associatedconstant.NORMAL_PREPASS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [DEFERRED\_PREPASS](#associatedconstant.DEFERRED_PREPASS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [MOTION\_VECTOR\_PREPASS](#associatedconstant.MOTION_VECTOR_PREPASS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [MAY\_DISCARD](#associatedconstant.MAY_DISCARD): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [ENVIRONMENT\_MAP](#associatedconstant.ENVIRONMENT_MAP): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_AMBIENT\_OCCLUSION](#associatedconstant.SCREEN_SPACE_AMBIENT_OCCLUSION): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [UNCLIPPED\_DEPTH\_ORTHO](#associatedconstant.UNCLIPPED_DEPTH_ORTHO): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TEMPORAL\_JITTER](#associatedconstant.TEMPORAL_JITTER): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [READS\_VIEW\_TRANSMISSION\_TEXTURE](#associatedconstant.READS_VIEW_TRANSMISSION_TEXTURE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [LIGHTMAPPED](#associatedconstant.LIGHTMAPPED): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [LIGHTMAP\_BICUBIC\_SAMPLING](#associatedconstant.LIGHTMAP_BICUBIC_SAMPLING): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [IRRADIANCE\_VOLUME](#associatedconstant.IRRADIANCE_VOLUME): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [VISIBILITY\_RANGE\_DITHER](#associatedconstant.VISIBILITY_RANGE_DITHER): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_REFLECTIONS](#associatedconstant.SCREEN_SPACE_REFLECTIONS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [HAS\_PREVIOUS\_SKIN](#associatedconstant.HAS_PREVIOUS_SKIN): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [HAS\_PREVIOUS\_MORPH](#associatedconstant.HAS_PREVIOUS_MORPH): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [OIT\_ENABLED](#associatedconstant.OIT_ENABLED): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [DISTANCE\_FOG](#associatedconstant.DISTANCE_FOG): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [ATMOSPHERE](#associatedconstant.ATMOSPHERE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [INVERT\_CULLING](#associatedconstant.INVERT_CULLING): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [PREPASS\_READS\_MATERIAL](#associatedconstant.PREPASS_READS_MATERIAL): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [CONTACT\_SHADOWS](#associatedconstant.CONTACT_SHADOWS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [LAST\_FLAG](#associatedconstant.LAST_FLAG): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [ALL\_PREPASS\_BITS](#associatedconstant.ALL_PREPASS_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [MSAA\_RESERVED\_BITS](#associatedconstant.MSAA_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [BLEND\_RESERVED\_BITS](#associatedconstant.BLEND_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [BLEND\_OPAQUE](#associatedconstant.BLEND_OPAQUE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [BLEND\_PREMULTIPLIED\_ALPHA](#associatedconstant.BLEND_PREMULTIPLIED_ALPHA): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [BLEND\_MULTIPLY](#associatedconstant.BLEND_MULTIPLY): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [BLEND\_ALPHA](#associatedconstant.BLEND_ALPHA): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [BLEND\_ALPHA\_TO\_COVERAGE](#associatedconstant.BLEND_ALPHA_TO_COVERAGE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_RESERVED\_BITS](#associatedconstant.TONEMAP_METHOD_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_NONE](#associatedconstant.TONEMAP_METHOD_NONE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_REINHARD](#associatedconstant.TONEMAP_METHOD_REINHARD): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_REINHARD\_LUMINANCE](#associatedconstant.TONEMAP_METHOD_REINHARD_LUMINANCE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_ACES\_FITTED](#associatedconstant.TONEMAP_METHOD_ACES_FITTED): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_AGX](#associatedconstant.TONEMAP_METHOD_AGX): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_SOMEWHAT\_BORING\_DISPLAY\_TRANSFORM](#associatedconstant.TONEMAP_METHOD_SOMEWHAT_BORING_DISPLAY_TRANSFORM): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_TONY\_MC\_MAPFACE](#associatedconstant.TONEMAP_METHOD_TONY_MC_MAPFACE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_BLENDER\_FILMIC](#associatedconstant.TONEMAP_METHOD_BLENDER_FILMIC): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [TONEMAP\_METHOD\_PBR\_NEUTRAL](#associatedconstant.TONEMAP_METHOD_PBR_NEUTRAL): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SHADOW\_FILTER\_METHOD\_RESERVED\_BITS](#associatedconstant.SHADOW_FILTER_METHOD_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SHADOW\_FILTER\_METHOD\_HARDWARE\_2X2](#associatedconstant.SHADOW_FILTER_METHOD_HARDWARE_2X2): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SHADOW\_FILTER\_METHOD\_GAUSSIAN](#associatedconstant.SHADOW_FILTER_METHOD_GAUSSIAN): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SHADOW\_FILTER\_METHOD\_TEMPORAL](#associatedconstant.SHADOW_FILTER_METHOD_TEMPORAL): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [VIEW\_PROJECTION\_RESERVED\_BITS](#associatedconstant.VIEW_PROJECTION_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [VIEW\_PROJECTION\_NONSTANDARD](#associatedconstant.VIEW_PROJECTION_NONSTANDARD): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [VIEW\_PROJECTION\_PERSPECTIVE](#associatedconstant.VIEW_PROJECTION_PERSPECTIVE): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [VIEW\_PROJECTION\_ORTHOGRAPHIC](#associatedconstant.VIEW_PROJECTION_ORTHOGRAPHIC): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [VIEW\_PROJECTION\_RESERVED](#associatedconstant.VIEW_PROJECTION_RESERVED): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_SPECULAR\_TRANSMISSION\_RESERVED\_BITS](#associatedconstant.SCREEN_SPACE_SPECULAR_TRANSMISSION_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_SPECULAR\_TRANSMISSION\_LOW](#associatedconstant.SCREEN_SPACE_SPECULAR_TRANSMISSION_LOW): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_SPECULAR\_TRANSMISSION\_MEDIUM](#associatedconstant.SCREEN_SPACE_SPECULAR_TRANSMISSION_MEDIUM): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_SPECULAR\_TRANSMISSION\_HIGH](#associatedconstant.SCREEN_SPACE_SPECULAR_TRANSMISSION_HIGH): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [SCREEN\_SPACE\_SPECULAR\_TRANSMISSION\_ULTRA](#associatedconstant.SCREEN_SPACE_SPECULAR_TRANSMISSION_ULTRA): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [COLOR\_TARGET\_FORMAT\_RESERVED\_BITS](#associatedconstant.COLOR_TARGET_FORMAT_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const [ALL\_RESERVED\_BITS](#associatedconstant.ALL_RESERVED_BITS): [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [empty](#method.empty)() -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Get a flags value with all bits unset.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/shader\_advanced/manual\_material.rs ([line 198](../../src/manual_material/manual_material.rs.html#198))

```rust
145    fn prepare_asset(
146        source_asset: Self::SourceAsset,
147        asset_id: AssetId<Self::SourceAsset>,
148        (
149            opaque_draw_functions,
150            material_layout,
151            asset_server,
152            bind_group_allocators,
153            render_material_bindings,
154            gpu_images,
155            image_material_sampler,
156        ): &mut SystemParamItem<Self::Param>,
157    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
158        let material_layout = material_layout.0.clone();
159        let draw_function_id = opaque_draw_functions.read().id::<DrawMaterial>();
160        let bind_group_allocator = bind_group_allocators
161            .get_mut(&TypeId::of::<ImageMaterial>())
162            .unwrap();
163        let Some(image) = gpu_images.get(&source_asset.image) else {
164            return Err(PrepareAssetError::RetryNextUpdate(source_asset));
165        };
166        let unprepared = UnpreparedBindGroup {
167            bindings: BindingResources(vec![
168                (
169                    0,
170                    OwnedBindingResource::TextureView(
171                        TextureViewDimension::D2,
172                        image.texture_view.clone(),
173                    ),
174                ),
175                (
176                    1,
177                    OwnedBindingResource::Sampler(
178                        SamplerBindingType::NonFiltering,
179                        image_material_sampler.0.clone(),
180                    ),
181                ),
182            ]),
183        };
184        let binding = match render_material_bindings.entry(asset_id.into()) {
185            Entry::Occupied(mut occupied_entry) => {
186                bind_group_allocator.free(*occupied_entry.get());
187                let new_binding =
188                    bind_group_allocator.allocate_unprepared(unprepared, &material_layout);
189                *occupied_entry.get_mut() = new_binding;
190                new_binding
191            }
192            Entry::Vacant(vacant_entry) => *vacant_entry
193                .insert(bind_group_allocator.allocate_unprepared(unprepared, &material_layout)),
194        };
195
196        let mut properties = MaterialProperties {
197            material_layout: Some(material_layout),
198            mesh_pipeline_key_bits: ErasedMeshPipelineKey::new(MeshPipelineKey::empty()),
199            base_specialize: Some(base_specialize),
200            ..Default::default()
201        };
202        properties.add_draw_function(MainPassOpaqueDrawFunction, draw_function_id);
203        properties.add_shader(MaterialFragmentShader, asset_server.load(SHADER_ASSET_PATH));
204
205        Ok(PreparedMaterial {
206            binding,
207            properties: Arc::new(properties),
208        })
209    }
```

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [all](#method.all)() -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Get a flags value with all known bits set.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [bits](#method.bits)(&self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Get the underlying bits value.

The returned value is exactly the bits set in this flags value.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [from\_bits](#method.from_bits)(bits: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\>

Convert from a bits value.

This method will return `None` if any unknown bits are set.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [from\_bits\_truncate](#method.from_bits_truncate)(bits: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Convert from a bits value, unsetting any unknown bits.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [from\_bits\_retain](#method.from_bits_retain)(bits: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Convert from a bits value exactly.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub fn [from\_name](#method.from_name)(name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\>

Get a flags value with the bits of a flag with the given name set.

This method will return `None` if `name` is empty or doesn’t correspond to any named flag.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all bits in `self` are unset.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [is\_all](#method.is_all)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all known bits in this flags value are set.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [intersects](#method.intersects)(&self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether any set bits in `other` are also set in `self`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [contains](#method.contains)(&self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all set bits in `other` are also set in `self`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub fn [insert](#method.insert)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub fn [remove](#method.remove)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `remove` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub fn [toggle](#method.toggle)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub fn [set](#method.set)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"), value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Call `insert` when `value` is `true` or `remove` when `value` is `false`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [intersection](#method.intersection)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise and (`&`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [union](#method.union)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [difference](#method.difference)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [symmetric\_difference](#method.symmetric_difference)( self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"), ) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [complement](#method.complement)(self) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise negation (`!`) of the bits in `self`, truncating the result.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [iter](#method.iter)(&self) -> [Iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\> [ⓘ](#)

Yield a set of contained flags values.

Each yielded flags value will correspond to a defined named flag. Any unknown bits will be yielded together as a final flags value.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### pub const fn [iter\_names](#method.iter_names)(&self) -> [IterNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html "struct bitflags::iter::IterNames")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\> [ⓘ](#)

Yield a set of contained named flags values.

This method is like [`iter`](#method.iter), except only yields bits in contained named flags. Any unknown bits, or bits not corresponding to a contained flag will not be yielded.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3079)

### impl [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3108)

#### pub fn [from\_msaa\_samples](#method.from_msaa_samples)(msaa\_samples: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3116)

#### pub fn [from\_target\_format](#method.from_target_format)(format: [TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Create a pipeline key from the view’s color target format.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3126)

#### pub fn [target\_format](#method.target_format)(&self) -> [TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

Color target format of the main pass for this pipeline key.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/shader\_advanced/custom\_render\_phase.rs ([line 221](../../src/custom_render_phase/custom_render_phase.rs.html#221))

```rust
185    fn specialize(
186        &self,
187        key: Self::Key,
188        layout: &MeshVertexBufferLayoutRef,
189    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
190        // We will only use the position of the mesh in our shader so we only need to specify that
191        let mut vertex_attributes = Vec::new();
192        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
193            // Make sure this matches the shader location
194            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
195        }
196        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
197        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
198        let view_layout = self
199            .mesh_pipeline
200            .get_view_layout(MeshPipelineViewLayoutKey::from(key));
201        Ok(RenderPipelineDescriptor {
202            label: Some("Specialized Mesh Pipeline".into()),
203            // We want to reuse the data from bevy so we use the same bind groups as the default
204            // mesh pipeline
205            layout: vec![
206                // Bind group 0 is the view uniform
207                view_layout.main_layout,
208                // Bind group 1 is empty
209                view_layout.empty_layout,
210                // Bind group 2 is the mesh uniform
211                self.mesh_pipeline.mesh_layouts.model_only.clone(),
212            ],
213            vertex: VertexState {
214                shader: self.shader_handle.clone(),
215                buffers: vec![vertex_buffer_layout],
216                ..default()
217            },
218            fragment: Some(FragmentState {
219                shader: self.shader_handle.clone(),
220                targets: vec![Some(ColorTargetState {
221                    format: key.target_format(),
222                    blend: None,
223                    write_mask: ColorWrites::ALL,
224                })],
225                ..default()
226            }),
227            primitive: PrimitiveState {
228                topology: key.primitive_topology(),
229                strip_index_format: key.strip_index_format(),
230                cull_mode: Some(Face::Back),
231                ..default()
232            },
233            // It's generally recommended to specialize your pipeline for MSAA,
234            // but it's not always possible
235            ..default()
236        })
237    }
```

Hide additional examples

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 232](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#232))

```rust
192    fn specialize(
193        &self,
194        mesh_key: Self::Key,
195        layout: &MeshVertexBufferLayoutRef,
196    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
197        // Define the vertex attributes based on a standard bevy [`Mesh`]
198        let mut vertex_attributes = Vec::new();
199        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
200            // Make sure this matches the shader location
201            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
202        }
203        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) {
204            // Make sure this matches the shader location
205            vertex_attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(1));
206        }
207        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
208        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
209
210        let view_layout = self
211            .mesh_pipeline
212            .get_view_layout(MeshPipelineViewLayoutKey::from(mesh_key));
213
214        Ok(RenderPipelineDescriptor {
215            label: Some("Specialized Mesh Pipeline".into()),
216            layout: vec![
217                view_layout.main_layout,
218                view_layout.empty_layout,
219                self.mesh_pipeline.mesh_layouts.model_only.clone(),
220            ],
221            vertex: VertexState {
222                shader: self.shader_handle.clone(),
223                // Customize how to store the meshes' vertex attributes in the vertex buffer
224                buffers: vec![vertex_buffer_layout],
225                ..default()
226            },
227            fragment: Some(FragmentState {
228                shader: self.shader_handle.clone(),
229                targets: vec![Some(ColorTargetState {
230                    // This isn't required, but bevy supports rendering different formats
231                    // so it's generally recommended to specialize the pipeline for that
232                    format: mesh_key.target_format(),
233                    // For this example we only use opaque meshes,
234                    // but if you wanted to use alpha blending you would need to set it here
235                    blend: None,
236                    write_mask: ColorWrites::ALL,
237                })],
238                ..default()
239            }),
240            primitive: PrimitiveState {
241                topology: mesh_key.primitive_topology(),
242                strip_index_format: mesh_key.strip_index_format(),
243                front_face: FrontFace::Ccw,
244                cull_mode: Some(Face::Back),
245                polygon_mode: PolygonMode::Fill,
246                ..default()
247            },
248            // Note that if your view has no depth buffer this will need to be
249            // changed.
250            depth_stencil: Some(DepthStencilState {
251                format: CORE_3D_DEPTH_FORMAT,
252                depth_write_enabled: Some(true),
253                depth_compare: Some(CompareFunction::GreaterEqual),
254                stencil: default(),
255                bias: default(),
256            }),
257            // It's generally recommended to specialize your pipeline for MSAA,
258            // but it's not always possible
259            multisample: MultisampleState {
260                count: mesh_key.msaa_samples(),
261                ..default()
262            },
263            ..default()
264        })
265    }
```

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3133)

#### pub fn [msaa\_samples](#method.msaa_samples)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 260](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#260))

```rust
192    fn specialize(
193        &self,
194        mesh_key: Self::Key,
195        layout: &MeshVertexBufferLayoutRef,
196    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
197        // Define the vertex attributes based on a standard bevy [`Mesh`]
198        let mut vertex_attributes = Vec::new();
199        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
200            // Make sure this matches the shader location
201            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
202        }
203        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) {
204            // Make sure this matches the shader location
205            vertex_attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(1));
206        }
207        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
208        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
209
210        let view_layout = self
211            .mesh_pipeline
212            .get_view_layout(MeshPipelineViewLayoutKey::from(mesh_key));
213
214        Ok(RenderPipelineDescriptor {
215            label: Some("Specialized Mesh Pipeline".into()),
216            layout: vec![
217                view_layout.main_layout,
218                view_layout.empty_layout,
219                self.mesh_pipeline.mesh_layouts.model_only.clone(),
220            ],
221            vertex: VertexState {
222                shader: self.shader_handle.clone(),
223                // Customize how to store the meshes' vertex attributes in the vertex buffer
224                buffers: vec![vertex_buffer_layout],
225                ..default()
226            },
227            fragment: Some(FragmentState {
228                shader: self.shader_handle.clone(),
229                targets: vec![Some(ColorTargetState {
230                    // This isn't required, but bevy supports rendering different formats
231                    // so it's generally recommended to specialize the pipeline for that
232                    format: mesh_key.target_format(),
233                    // For this example we only use opaque meshes,
234                    // but if you wanted to use alpha blending you would need to set it here
235                    blend: None,
236                    write_mask: ColorWrites::ALL,
237                })],
238                ..default()
239            }),
240            primitive: PrimitiveState {
241                topology: mesh_key.primitive_topology(),
242                strip_index_format: mesh_key.strip_index_format(),
243                front_face: FrontFace::Ccw,
244                cull_mode: Some(Face::Back),
245                polygon_mode: PolygonMode::Fill,
246                ..default()
247            },
248            // Note that if your view has no depth buffer this will need to be
249            // changed.
250            depth_stencil: Some(DepthStencilState {
251                format: CORE_3D_DEPTH_FORMAT,
252                depth_write_enabled: Some(true),
253                depth_compare: Some(CompareFunction::GreaterEqual),
254                stencil: default(),
255                bias: default(),
256            }),
257            // It's generally recommended to specialize your pipeline for MSAA,
258            // but it's not always possible
259            multisample: MultisampleState {
260                count: mesh_key.msaa_samples(),
261                ..default()
262            },
263            ..default()
264        })
265    }
```

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3140-3143)

#### pub fn [from\_primitive\_topology\_and\_strip\_index](#method.from_primitive_topology_and_strip_index)( primitive\_topology: [PrimitiveTopology](../mesh/enum.PrimitiveTopology.html "enum bevy::mesh::PrimitiveTopology"), strip\_index\_format: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IndexFormat](../render/render_resource/enum.IndexFormat.html "enum bevy::render::render_resource::IndexFormat")\>, ) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Create a [`BaseMeshPipelineKey`](../mesh/struct.BaseMeshPipelineKey.html "struct bevy::mesh::BaseMeshPipelineKey") from mesh primitive topology and index format.

For non-strip topologies, [`BaseMeshPipelineKey::STRIP_INDEX_FORMAT_NONE`](../mesh/struct.BaseMeshPipelineKey.html#associatedconstant.STRIP_INDEX_FORMAT_NONE "associated constant bevy::mesh::BaseMeshPipelineKey::STRIP_INDEX_FORMAT_NONE") is set regardless of the `strip_index_format` argument.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/shader\_advanced/custom\_shader\_instancing.rs ([lines 173-176](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#173-176))

```rust
137fn queue_custom(
138    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
139    custom_pipeline: Res<CustomPipeline>,
140    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
141    pipeline_cache: Res<PipelineCache>,
142    meshes: Res<RenderAssets<RenderMesh>>,
143    render_mesh_instances: Res<RenderMeshInstances>,
144    maybe_batched_instance_buffers: Option<
145        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
146    >,
147    material_meshes: Query<(Entity, &MainEntity), With<InstanceMaterialData>>,
148    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
149    views: Query<&ExtractedView>,
150    view_key_cache: Res<ViewKeyCache>,
151) {
152    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom>();
153
154    for view in &views {
155        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
156        else {
157            continue;
158        };
159
160        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
161            continue;
162        };
163
164        for (entity, main_entity) in &material_meshes {
165            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*main_entity)
166            else {
167                continue;
168            };
169            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id()) else {
170                continue;
171            };
172            let key = view_key
173                | MeshPipelineKey::from_primitive_topology_and_strip_index(
174                    mesh.primitive_topology(),
175                    mesh.index_format(),
176                );
177            let pipeline = pipelines
178                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
179                .unwrap();
180            transparent_phase.add_retained(Transparent3d {
181                sorting_info: TransparentSortingInfo3d::Sorted {
182                    mesh_center: pbr::get_mesh_instance_world_from_local(
183                        *main_entity,
184                        mesh_instance.current_uniform_index,
185                        &render_mesh_instances,
186                        maybe_batched_instance_buffers.as_deref(),
187                    )
188                    .transform_point3(
189                        meshes
190                            .get(mesh_instance.mesh_asset_id())
191                            .unwrap()
192                            .aabb_center,
193                    ),
194                    depth_bias: 0.0,
195                },
196                entity: (entity, *main_entity),
197                pipeline,
198                draw_function: draw_custom,
199                distance: 0.0,
200                batch_range: 0..1,
201                extra_index: PhaseItemExtraIndex::None,
202                indexed: true,
203            });
204        }
205    }
206}
```

Hide additional examples

examples/shader\_advanced/custom\_render\_phase.rs ([lines 600-603](../../src/custom_render_phase/custom_render_phase.rs.html#600-603))

```rust
531fn queue_custom_meshes(
532    custom_draw_functions: Res<DrawFunctions<Stencil3d>>,
533    mut pipelines: ResMut<SpecializedMeshPipelines<StencilPipeline>>,
534    pipeline_cache: Res<PipelineCache>,
535    custom_draw_pipeline: Res<StencilPipeline>,
536    render_meshes: Res<RenderAssets<RenderMesh>>,
537    render_mesh_instances: Res<RenderMeshInstances>,
538    maybe_batched_instance_buffers: Option<
539        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
540    >,
541    mut custom_render_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
542    mut views: Query<(&ExtractedView, &RenderVisibleEntities)>,
543    view_key_cache: Res<ViewKeyCache>,
544    dirty_specializations: Res<DirtySpecializations>,
545    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
546    has_marker: Query<(), With<DrawStencil>>,
547) {
548    for (view, visible_entities) in &mut views {
549        let Some(custom_phase) = custom_render_phases.get_mut(&view.retained_view_entity) else {
550            continue;
551        };
552        let draw_custom = custom_draw_functions.read().id::<DrawMesh3dStencil>();
553
554        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
555            continue;
556        };
557
558        // Since our phase can work on any 3d mesh we can reuse the default mesh 3d filter
559        let Some(render_visible_mesh_entities) = visible_entities.get::<Mesh3d>() else {
560            continue;
561        };
562
563        let view_pending_custom_mesh_queues =
564            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
565
566        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
567        for &main_entity in dirty_specializations
568            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
569        {
570            custom_phase.remove(Entity::PLACEHOLDER, main_entity);
571        }
572
573        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
574            view.retained_view_entity,
575            render_visible_mesh_entities,
576            &view_pending_custom_mesh_queues.prev_frame,
577        ) {
578            // We only want meshes with the marker component to be queued to our phase.
579            if has_marker.get(*render_entity).is_err() {
580                continue;
581            }
582            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
583            else {
584                // We couldn't fetch the mesh, probably because it hasn't been
585                // loaded yet. Add the entity to the list of pending custom mesh
586                // queues and bail.
587                view_pending_custom_mesh_queues
588                    .current_frame
589                    .insert((*render_entity, *visible_entity));
590                continue;
591            };
592            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
593                continue;
594            };
595
596            // Specialize the key for the current mesh entity
597            // For this example we only specialize based on the mesh topology
598            // but you could have more complex keys and that's where you'd need to create those keys
599            let mut mesh_key = view_key;
600            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
601                mesh.primitive_topology(),
602                mesh.index_format(),
603            );
604
605            let pipeline_id = pipelines.specialize(
606                &pipeline_cache,
607                &custom_draw_pipeline,
608                mesh_key,
609                &mesh.layout,
610            );
611            let pipeline_id = match pipeline_id {
612                Ok(id) => id,
613                Err(err) => {
614                    error!("{}", err);
615                    continue;
616                }
617            };
618            // At this point we have all the data we need to create a phase item and add it to our
619            // phase
620            custom_phase.add_retained(Stencil3d {
621                sorting_info: TransparentSortingInfo3d::Sorted {
622                    mesh_center: pbr::get_mesh_instance_world_from_local(
623                        *visible_entity,
624                        mesh_instance.current_uniform_index,
625                        &render_mesh_instances,
626                        maybe_batched_instance_buffers.as_deref(),
627                    )
628                    .transform_point3(
629                        render_meshes
630                            .get(mesh_instance.mesh_asset_id())
631                            .unwrap()
632                            .aabb_center,
633                    ),
634                    depth_bias: 0.0,
635                },
636                distance: FloatOrd(0.0),
637                entity: (Entity::PLACEHOLDER, *visible_entity),
638                pipeline: pipeline_id,
639                draw_function: draw_custom,
640                // Sorted phase items aren't batched
641                batch_range: 0..1,
642                extra_index: PhaseItemExtraIndex::None,
643                indexed: mesh.indexed(),
644            });
645        }
646    }
647}
```

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([lines 363-366](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#363-366))

```rust
277fn queue_custom_mesh_pipeline(
278    pipeline_cache: Res<PipelineCache>,
279    custom_mesh_pipeline: Res<CustomMeshPipeline>,
280    (mut opaque_render_phases, opaque_draw_functions): (
281        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
282        Res<DrawFunctions<Opaque3d>>,
283    ),
284    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
285    views: Query<(&RenderVisibleEntities, &ExtractedView)>,
286    view_key_cache: Res<ViewKeyCache>,
287    (render_meshes, render_mesh_instances): (
288        Res<RenderAssets<RenderMesh>>,
289        Res<RenderMeshInstances>,
290    ),
291    mut change_tick: Local<Tick>,
292    mesh_allocator: Res<MeshAllocator>,
293    gpu_preprocessing_support: Res<GpuPreprocessingSupport>,
294    dirty_specializations: Res<DirtySpecializations>,
295    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
296) {
297    // Get the id for our custom draw function
298    let draw_function = opaque_draw_functions
299        .read()
300        .id::<DrawSpecializedPipelineCommands>();
301
302    // Render phases are per-view, so we need to iterate over all views so that
303    // the entity appears in them. (In this example, we have only one view, but
304    // it's good practice to loop over all views anyway.)
305    for (view_visible_entities, view) in views.iter() {
306        let Some(opaque_phase) = opaque_render_phases.get_mut(&view.retained_view_entity) else {
307            continue;
308        };
309
310        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
311            continue;
312        };
313
314        let Some(render_visible_mesh_entities) =
315            view_visible_entities.get::<CustomRenderedEntity>()
316        else {
317            continue;
318        };
319
320        // Initialize the pending queues.
321        let view_pending_custom_mesh_queues =
322            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
323
324        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
325        for &main_entity in dirty_specializations
326            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
327        {
328            opaque_phase.remove(main_entity);
329        }
330
331        // Find all the custom rendered entities that are visible from this
332        // view.
333        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
334            view.retained_view_entity,
335            render_visible_mesh_entities,
336            &view_pending_custom_mesh_queues.prev_frame,
337        ) {
338            // Get the mesh instance
339            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
340            else {
341                // We couldn't fetch the mesh, probably because it hasn't been
342                // loaded yet. Add the entity to the list of pending custom
343                // meshes and bail.
344                view_pending_custom_mesh_queues
345                    .current_frame
346                    .insert((*render_entity, *visible_entity));
347                continue;
348            };
349
350            // Get the mesh data
351            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
352                continue;
353            };
354
355            let Some(mesh_slabs) = mesh_allocator.mesh_slabs(&mesh_instance.mesh_asset_id()) else {
356                continue;
357            };
358
359            // Specialize the key for the current mesh entity
360            // For this example we only specialize based on the mesh topology
361            // but you could have more complex keys and that's where you'd need to create those keys
362            let mut mesh_key = view_key;
363            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
364                mesh.primitive_topology(),
365                mesh.index_format(),
366            );
367
368            // Finally, we can specialize the pipeline based on the key
369            let pipeline_id = specialized_mesh_pipelines
370                .specialize(
371                    &pipeline_cache,
372                    &custom_mesh_pipeline,
373                    mesh_key,
374                    &mesh.layout,
375                )
376                // This should never happen with this example, but if your pipeline
377                // specialization can fail you need to handle the error here
378                .expect("Failed to specialize mesh pipeline");
379
380            // Bump the change tick so that Bevy is forced to rebuild the bin.
381            let next_change_tick = change_tick.get() + 1;
382            change_tick.set(next_change_tick);
383
384            // Add the mesh with our specialized pipeline
385            opaque_phase.add(
386                Opaque3dBatchSetKey {
387                    draw_function,
388                    pipeline: pipeline_id,
389                    material_bind_group_index: None,
390                    slabs: mesh_slabs,
391                    lightmap_slab: None,
392                },
393                // For this example we can use the mesh asset id as the bin key,
394                // but you can use any asset_id as a key
395                Opaque3dBinKey {
396                    asset_id: mesh_instance.mesh_asset_id().into(),
397                },
398                (*render_entity, *visible_entity),
399                mesh_instance.current_uniform_index,
400                // This example supports batching and multi draw indirect,
401                // but if your pipeline doesn't support it you can use
402                // `BinnedRenderPhaseType::UnbatchableMesh`
403                BinnedRenderPhaseType::mesh(
404                    mesh_instance.should_batch(),
405                    &gpu_preprocessing_support,
406                ),
407            );
408        }
409    }
410}
```

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3162)

#### pub fn [primitive\_topology](#method.primitive_topology)(&self) -> [PrimitiveTopology](../mesh/enum.PrimitiveTopology.html "enum bevy::mesh::PrimitiveTopology")

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/shader\_advanced/custom\_render\_phase.rs ([line 228](../../src/custom_render_phase/custom_render_phase.rs.html#228))

```rust
185    fn specialize(
186        &self,
187        key: Self::Key,
188        layout: &MeshVertexBufferLayoutRef,
189    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
190        // We will only use the position of the mesh in our shader so we only need to specify that
191        let mut vertex_attributes = Vec::new();
192        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
193            // Make sure this matches the shader location
194            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
195        }
196        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
197        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
198        let view_layout = self
199            .mesh_pipeline
200            .get_view_layout(MeshPipelineViewLayoutKey::from(key));
201        Ok(RenderPipelineDescriptor {
202            label: Some("Specialized Mesh Pipeline".into()),
203            // We want to reuse the data from bevy so we use the same bind groups as the default
204            // mesh pipeline
205            layout: vec![
206                // Bind group 0 is the view uniform
207                view_layout.main_layout,
208                // Bind group 1 is empty
209                view_layout.empty_layout,
210                // Bind group 2 is the mesh uniform
211                self.mesh_pipeline.mesh_layouts.model_only.clone(),
212            ],
213            vertex: VertexState {
214                shader: self.shader_handle.clone(),
215                buffers: vec![vertex_buffer_layout],
216                ..default()
217            },
218            fragment: Some(FragmentState {
219                shader: self.shader_handle.clone(),
220                targets: vec![Some(ColorTargetState {
221                    format: key.target_format(),
222                    blend: None,
223                    write_mask: ColorWrites::ALL,
224                })],
225                ..default()
226            }),
227            primitive: PrimitiveState {
228                topology: key.primitive_topology(),
229                strip_index_format: key.strip_index_format(),
230                cull_mode: Some(Face::Back),
231                ..default()
232            },
233            // It's generally recommended to specialize your pipeline for MSAA,
234            // but it's not always possible
235            ..default()
236        })
237    }
```

Hide additional examples

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 241](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#241))

```rust
192    fn specialize(
193        &self,
194        mesh_key: Self::Key,
195        layout: &MeshVertexBufferLayoutRef,
196    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
197        // Define the vertex attributes based on a standard bevy [`Mesh`]
198        let mut vertex_attributes = Vec::new();
199        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
200            // Make sure this matches the shader location
201            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
202        }
203        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) {
204            // Make sure this matches the shader location
205            vertex_attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(1));
206        }
207        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
208        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
209
210        let view_layout = self
211            .mesh_pipeline
212            .get_view_layout(MeshPipelineViewLayoutKey::from(mesh_key));
213
214        Ok(RenderPipelineDescriptor {
215            label: Some("Specialized Mesh Pipeline".into()),
216            layout: vec![
217                view_layout.main_layout,
218                view_layout.empty_layout,
219                self.mesh_pipeline.mesh_layouts.model_only.clone(),
220            ],
221            vertex: VertexState {
222                shader: self.shader_handle.clone(),
223                // Customize how to store the meshes' vertex attributes in the vertex buffer
224                buffers: vec![vertex_buffer_layout],
225                ..default()
226            },
227            fragment: Some(FragmentState {
228                shader: self.shader_handle.clone(),
229                targets: vec![Some(ColorTargetState {
230                    // This isn't required, but bevy supports rendering different formats
231                    // so it's generally recommended to specialize the pipeline for that
232                    format: mesh_key.target_format(),
233                    // For this example we only use opaque meshes,
234                    // but if you wanted to use alpha blending you would need to set it here
235                    blend: None,
236                    write_mask: ColorWrites::ALL,
237                })],
238                ..default()
239            }),
240            primitive: PrimitiveState {
241                topology: mesh_key.primitive_topology(),
242                strip_index_format: mesh_key.strip_index_format(),
243                front_face: FrontFace::Ccw,
244                cull_mode: Some(Face::Back),
245                polygon_mode: PolygonMode::Fill,
246                ..default()
247            },
248            // Note that if your view has no depth buffer this will need to be
249            // changed.
250            depth_stencil: Some(DepthStencilState {
251                format: CORE_3D_DEPTH_FORMAT,
252                depth_write_enabled: Some(true),
253                depth_compare: Some(CompareFunction::GreaterEqual),
254                stencil: default(),
255                bias: default(),
256            }),
257            // It's generally recommended to specialize your pipeline for MSAA,
258            // but it's not always possible
259            multisample: MultisampleState {
260                count: mesh_key.msaa_samples(),
261                ..default()
262            },
263            ..default()
264        })
265    }
```

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3176)

#### pub fn [strip\_index\_format](#method.strip_index_format)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[IndexFormat](../render/render_resource/enum.IndexFormat.html "enum bevy::render::render_resource::IndexFormat")\>

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/shader\_advanced/custom\_render\_phase.rs ([line 229](../../src/custom_render_phase/custom_render_phase.rs.html#229))

```rust
185    fn specialize(
186        &self,
187        key: Self::Key,
188        layout: &MeshVertexBufferLayoutRef,
189    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
190        // We will only use the position of the mesh in our shader so we only need to specify that
191        let mut vertex_attributes = Vec::new();
192        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
193            // Make sure this matches the shader location
194            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
195        }
196        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
197        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
198        let view_layout = self
199            .mesh_pipeline
200            .get_view_layout(MeshPipelineViewLayoutKey::from(key));
201        Ok(RenderPipelineDescriptor {
202            label: Some("Specialized Mesh Pipeline".into()),
203            // We want to reuse the data from bevy so we use the same bind groups as the default
204            // mesh pipeline
205            layout: vec![
206                // Bind group 0 is the view uniform
207                view_layout.main_layout,
208                // Bind group 1 is empty
209                view_layout.empty_layout,
210                // Bind group 2 is the mesh uniform
211                self.mesh_pipeline.mesh_layouts.model_only.clone(),
212            ],
213            vertex: VertexState {
214                shader: self.shader_handle.clone(),
215                buffers: vec![vertex_buffer_layout],
216                ..default()
217            },
218            fragment: Some(FragmentState {
219                shader: self.shader_handle.clone(),
220                targets: vec![Some(ColorTargetState {
221                    format: key.target_format(),
222                    blend: None,
223                    write_mask: ColorWrites::ALL,
224                })],
225                ..default()
226            }),
227            primitive: PrimitiveState {
228                topology: key.primitive_topology(),
229                strip_index_format: key.strip_index_format(),
230                cull_mode: Some(Face::Back),
231                ..default()
232            },
233            // It's generally recommended to specialize your pipeline for MSAA,
234            // but it's not always possible
235            ..default()
236        })
237    }
```

Hide additional examples

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 242](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#242))

```rust
192    fn specialize(
193        &self,
194        mesh_key: Self::Key,
195        layout: &MeshVertexBufferLayoutRef,
196    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
197        // Define the vertex attributes based on a standard bevy [`Mesh`]
198        let mut vertex_attributes = Vec::new();
199        if layout.0.contains(Mesh::ATTRIBUTE_POSITION) {
200            // Make sure this matches the shader location
201            vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
202        }
203        if layout.0.contains(Mesh::ATTRIBUTE_COLOR) {
204            // Make sure this matches the shader location
205            vertex_attributes.push(Mesh::ATTRIBUTE_COLOR.at_shader_location(1));
206        }
207        // This will automatically generate the correct `VertexBufferLayout` based on the vertex attributes
208        let vertex_buffer_layout = layout.0.get_layout(&vertex_attributes)?;
209
210        let view_layout = self
211            .mesh_pipeline
212            .get_view_layout(MeshPipelineViewLayoutKey::from(mesh_key));
213
214        Ok(RenderPipelineDescriptor {
215            label: Some("Specialized Mesh Pipeline".into()),
216            layout: vec![
217                view_layout.main_layout,
218                view_layout.empty_layout,
219                self.mesh_pipeline.mesh_layouts.model_only.clone(),
220            ],
221            vertex: VertexState {
222                shader: self.shader_handle.clone(),
223                // Customize how to store the meshes' vertex attributes in the vertex buffer
224                buffers: vec![vertex_buffer_layout],
225                ..default()
226            },
227            fragment: Some(FragmentState {
228                shader: self.shader_handle.clone(),
229                targets: vec![Some(ColorTargetState {
230                    // This isn't required, but bevy supports rendering different formats
231                    // so it's generally recommended to specialize the pipeline for that
232                    format: mesh_key.target_format(),
233                    // For this example we only use opaque meshes,
234                    // but if you wanted to use alpha blending you would need to set it here
235                    blend: None,
236                    write_mask: ColorWrites::ALL,
237                })],
238                ..default()
239            }),
240            primitive: PrimitiveState {
241                topology: mesh_key.primitive_topology(),
242                strip_index_format: mesh_key.strip_index_format(),
243                front_face: FrontFace::Ccw,
244                cull_mode: Some(Face::Back),
245                polygon_mode: PolygonMode::Fill,
246                ..default()
247            },
248            // Note that if your view has no depth buffer this will need to be
249            // changed.
250            depth_stencil: Some(DepthStencilState {
251                format: CORE_3D_DEPTH_FORMAT,
252                depth_write_enabled: Some(true),
253                depth_compare: Some(CompareFunction::GreaterEqual),
254                stencil: default(),
255                bias: default(),
256            }),
257            // It's generally recommended to specialize your pipeline for MSAA,
258            // but it's not always possible
259            multisample: MultisampleState {
260                count: mesh_key.msaa_samples(),
261                ..default()
262            },
263            ..default()
264        })
265    }
```

## Trait Implementations

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html#tymethod.fmt)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [BitAnd](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html "trait core::ops::bit::BitAnd") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bitand](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#tymethod.bitand)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise and (`&`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAnd.html#associatedtype.Output) = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The resulting type after applying the `&` operator.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html "trait core::ops::bit::BitAndAssign") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bitand\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitAndAssign.html#tymethod.bitand_assign)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The bitwise and (`&`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [BitOr](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html "trait core::ops::bit::BitOr") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bitor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#tymethod.bitor)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOr.html#associatedtype.Output) = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The resulting type after applying the `|` operator.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html "trait core::ops::bit::BitOrAssign") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bitor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitOrAssign.html#tymethod.bitor_assign)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [BitXor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html "trait core::ops::bit::BitXor") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bitxor](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#tymethod.bitxor)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXor.html#associatedtype.Output) = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The resulting type after applying the `^` operator.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html "trait core::ops::bit::BitXorAssign") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bitxor\_assign](https://doc.rust-lang.org/nightly/core/ops/bit/trait.BitXorAssign.html#tymethod.bitxor_assign)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [Extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html "trait core::iter::traits::collect::Extend")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\> for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [extend](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#tymethod.extend)<T>(&mut self, iterator: T)

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\>,

The bitwise or (`|`) of the bits in each flags value.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#420)

#### fn [extend\_one](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_one)(&mut self, item: A)

🔬This is a nightly-only experimental API. (`extend_one`)

Extends a collection with exactly one element.

[Source](https://doc.rust-lang.org/nightly/src/core/iter/traits/collect.rs.html#428)

#### fn [extend\_reserve](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

🔬This is a nightly-only experimental API. (`extend_one`)

Reserves capacity in a collection for the given number of additional elements. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.Extend.html#method.extend_reserve)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [Flags](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html "trait bitflags::traits::Flags") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### const [FLAGS](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedconstant.FLAGS): &'static \[[Flag](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/struct.Flag.html "struct bitflags::traits::Flag")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\>\]

The set of defined flags.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits) = [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

The underlying bits type.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits)(&self) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Get the underlying bits value. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.bits)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [from\_bits\_retain](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#tymethod.from_bits_retain)(bits: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Convert from a bits value exactly.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [all\_named](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all_named)() -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Get a flags value with all bits from named flags set. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all_named)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#140)

#### fn [empty](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.empty)() -> Self

Get a flags value with all bits unset.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#145)

#### fn [all](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.all)() -> Self

Get a flags value with all known bits set.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#168)

#### fn [known\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.known_bits)(&self) -> Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")

Get the known bits from a flags value.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#173)

#### fn [unknown\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.unknown_bits)(&self) -> Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")

Get the unknown bits from a flags value.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#178)

#### fn [contains\_unknown\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.contains_unknown_bits)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method will return `true` if any unknown bits are set.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#190)

#### fn [from\_bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits)(bits: Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

Convert from a bits value. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#201)

#### fn [from\_bits\_truncate](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_bits_truncate)(bits: Self::[Bits](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedtype.Bits "type bitflags::traits::Flags::Bits")) -> Self

Convert from a bits value, unsetting any unknown bits.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#212)

#### fn [from\_name](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name)(name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

Get a flags value with the bits of a flag with the given name set. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.from_name)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#231)

#### fn [iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter)(&self) -> [Iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter")<Self> [ⓘ](#)

Yield a set of contained flags values. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#239)

#### fn [iter\_names](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names)(&self) -> [IterNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html "struct bitflags::iter::IterNames")<Self> [ⓘ](#)

Yield a set of contained named flags values. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_names)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#244)

#### fn [iter\_defined\_names](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_defined_names)() -> [IterDefinedNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterDefinedNames.html "struct bitflags::iter::IterDefinedNames")<Self> [ⓘ](#)

Yield a set of all named flags defined by [`Self::FLAGS`](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#associatedconstant.FLAGS "associated constant bitflags::traits::Flags::FLAGS").

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#252)

#### fn [iter\_equal\_names](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_equal_names)(&self) -> [IterEqualNames](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.IterEqualNames.html "struct bitflags::iter::IterEqualNames")<Self> [ⓘ](#)

Get an iterator over all defined names for this flags value. [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.iter_equal_names)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#257)

#### fn [is\_empty](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all bits in this flags value are unset.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#262)

#### fn [is\_all](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.is_all)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether all known bits in this flags value are set.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#269-271)

#### fn [intersects](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.intersects)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Whether any set bits in `other` are also set in `self`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#277-279)

#### fn [contains](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.contains)(&self, other: Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Whether all set bits in `other` are also set in `self`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#285-287)

#### fn [truncate](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.truncate)(&mut self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Remove any unknown bits from the flags.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#293-295)

#### fn [insert](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert)(&mut self, other: Self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#304-306)

#### fn [remove](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove)(&mut self, other: Self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The intersection of `self` with the complement of `other` (`&!`). [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#312-314)

#### fn [toggle](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.toggle)(&mut self, other: Self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#320-322)

#### fn [set](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.set)(&mut self, other: Self, value: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Call [`Flags::insert`](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.insert "method bitflags::traits::Flags::insert") when `value` is `true` or [`Flags::remove`](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.remove "method bitflags::traits::Flags::remove") when `value` is `false`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#332-334)

#### fn [clear](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.clear)(&mut self)

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Unsets all bits in the flags.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#341)

#### fn [intersection](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.intersection)(self, other: Self) -> Self

The bitwise and (`&`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#347)

#### fn [union](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.union)(self, other: Self) -> Self

The bitwise or (`|`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#356)

#### fn [difference](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference)(self, other: Self) -> Self

The intersection of `self` with the complement of `other` (`&!`). [Read more](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.difference)

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#362)

#### fn [symmetric\_difference](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.symmetric_difference)(self, other: Self) -> Self

The bitwise exclusive-or (`^`) of the bits in `self` and `other`.

[Source](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/src/bitflags/traits.rs.html#368)

#### fn [complement](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html#method.complement)(self) -> Self

The bitwise negation (`!`) of the bits in `self`, truncating the result.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3197)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\> for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3198)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

Converts to this type from the input type.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh_view_bindings.rs.html#127)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\> for [MeshPipelineViewLayoutKey](struct.MeshPipelineViewLayoutKey.html "struct bevy::pbr::MeshPipelineViewLayoutKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh_view_bindings.rs.html#128)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineViewLayoutKey](struct.MeshPipelineViewLayoutKey.html "struct bevy::pbr::MeshPipelineViewLayoutKey")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3191)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\> for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3192)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [FromIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html "trait core::iter::traits::collect::FromIterator")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\> for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [from\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter)<T>(iterator: T) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\>,

The bitwise or (`|`) of the bits in each flags value.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

#### fn [hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)<\_\_H>(&self, state: [&mut \_\_H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where \_\_H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"),

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · [Source](https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#234-236)

#### fn [hash\_slice](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)<H>(data: &\[Self\], state: [&mut H](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where H: [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [Iter](https://docs.rs/bitflags/1.3.2/x86_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html "struct bitflags::iter::Iter")<[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")\>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey") as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html#tymethod.fmt)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [Not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [not](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#tymethod.not)(self) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The bitwise negation (`!`) of the bits in `self`, truncating the result.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html#associatedtype.Output) = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The resulting type after applying the `!` operator.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html#tymethod.fmt)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2985)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#tymethod.sub)(self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")) -> [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### type [Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output) = [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

The resulting type after applying the `-` operator.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [sub\_assign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html#tymethod.sub_assign)(&mut self, other: [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey"))

The intersection of `self` with the complement of `other` (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set. `difference` won’t truncate `other`, but the `!` operator will.

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

### impl [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2984-3077)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html#tymethod.fmt)

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [MeshPipelineKey](struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/src/parley/style/brush.rs.html#7)

### impl<T> [Brush](https://docs.rs/parley/0.9.0/x86_64-unknown-linux-gnu/parley/style/brush/trait.Brush.html "trait parley::style::brush::Brush") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#25-27)

### impl<T> [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#29)

#### fn [dyn\_eq](../app/trait.DynEq.html#tymethod.dyn_eq)(&self, other: &(dyn [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

This method tests for `self` and `other` values to be equal. [Read more](../app/trait.DynEq.html#tymethod.dyn_eq)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#47-49)

### impl<T> [DynHash](../ecs/label/trait.DynHash.html "trait bevy::ecs::label::DynHash") for T

where T: [DynEq](../app/trait.DynEq.html "trait bevy::app::DynEq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/label.rs.html#51)

#### fn [dyn\_hash](../ecs/label/trait.DynHash.html#tymethod.dyn_hash)(&self, state: &mut dyn [Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"))

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher").

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#88)

#### fn [equivalent](../platform/collections/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#151-154)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#156)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#166-169)

### impl<Q, K> [Equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html "trait hashbrown::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/lib.rs.html#171)

#### fn [equivalent](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if this value is equivalent to the given key. [Read more](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/trait.Equivalent.html#tymethod.equivalent)

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<MeshPipelineKey>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","Iter<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.Iter.html\\" title=\\"struct bitflags::iter::Iter\\">Iter</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = B;</div>","IterDefinedNames<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterDefinedNames.html\\" title=\\"struct bitflags::iter::IterDefinedNames\\">IterDefinedNames</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterDefinedNames.html\\" title=\\"struct bitflags::iter::IterDefinedNames\\">IterDefinedNames</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, B);</div>","IterEqualNames<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterEqualNames.html\\" title=\\"struct bitflags::iter::IterEqualNames\\">IterEqualNames</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterEqualNames.html\\" title=\\"struct bitflags::iter::IterEqualNames\\">IterEqualNames</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","IterNames<MeshPipelineKey>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, B);</div>","IterNames<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;B&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/iter/struct.IterNames.html\\" title=\\"struct bitflags::iter::IterNames\\">IterNames</a>&lt;B&gt;<div class=\\"where\\">where\\n B: <a class=\\"trait\\" href=\\"https://docs.rs/bitflags/1.3.2/x86\_64-unknown-linux-gnu/bitflags/traits/trait.Flags.html\\" title=\\"trait bitflags::traits::Flags\\">Flags</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'static <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, B);</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}