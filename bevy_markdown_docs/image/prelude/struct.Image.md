[bevy](../../index.html)::[image](../index.html)::[prelude](index.html)

# Struct Image 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#613)

```rust
pub struct Image {
    pub data: Option<Vec<u8>>,
    pub data_order: TextureDataOrder,
    pub texture_descriptor: TextureDescriptor<Option<&'static str>, &'static [TextureFormat]>,
    pub sampler: ImageSampler,
    pub texture_view_descriptor: Option<TextureViewDescriptor<Option<&'static str>>>,
    pub asset_usage: RenderAssetUsages,
    pub copy_on_resize: bool,
}
```

An image, optimized for usage in rendering.

### Remote Inspection

To transmit an [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") between two running Bevy apps, e.g. through BRP, use [`SerializedImage`](../struct.SerializedImage.html "struct bevy::image::SerializedImage"). This type is only meant for short-term transmission between same versions and should not be stored anywhere.

## Fields

`data: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)>>`

Raw pixel data. If the image is being used as a storage texture which doesn’t need to be initialized by the CPU, then this should be `None`. Otherwise, it should always be `Some`.

`data_order: [TextureDataOrder](../../render/render_resource/enum.TextureDataOrder.html "enum bevy::render::render_resource::TextureDataOrder")`

For texture data with layers and mips, this field controls how wgpu interprets the buffer layout.

Use [`TextureDataOrder::default()`](../../render/render_resource/enum.TextureDataOrder.html#method.default "associated function bevy::render::render_resource::TextureDataOrder::default") for all other cases.

`texture_descriptor: [TextureDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureDescriptor.html "struct wgpu_types::texture::TextureDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>, &'static [[TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")]>`

Describes the data layout of the GPU texture.  
For example, whether a texture contains 1D/2D/3D data, and what the format of the texture data is.

#### Field Usage Notes

*   [`TextureDescriptor::label`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureDescriptor.html#structfield.label "field wgpu_types::texture::TextureDescriptor::label") is used for caching purposes when not using `Asset<Image>`.  
    If you use assets, the label is purely a debugging aid.
*   [`TextureDescriptor::view_formats`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureDescriptor.html#structfield.view_formats "field wgpu_types::texture::TextureDescriptor::view_formats") is currently unused by Bevy.

`sampler: [ImageSampler](../enum.ImageSampler.html "enum bevy::image::ImageSampler")`

The [`ImageSampler`](../enum.ImageSampler.html "enum bevy::image::ImageSampler") to use during rendering.

`texture_view_descriptor: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextureViewDescriptor](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureViewDescriptor.html "struct wgpu_types::texture::TextureViewDescriptor")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>>>`

Describes how the GPU texture should be interpreted.  
For example, 2D image data could be read as plain 2D, an array texture of layers of 2D with the same dimensions (and the number of layers in that case), a cube map, an array of cube maps, etc.

#### Field Usage Notes

*   [`TextureViewDescriptor::label`](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/texture/struct.TextureViewDescriptor.html#structfield.label "field wgpu_types::texture::TextureViewDescriptor::label") is used for caching purposes when not using `Asset<Image>`.  
    If you use assets, the label is purely a debugging aid.

`asset_usage: [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")`

Where this image asset will be used. See [`RenderAssetUsages`](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages") for more.

`copy_on_resize: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)`

Whether this image should be copied on the GPU when resized.

## Implementations

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1096)

### impl [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1102-1108)

#### pub fn [new](#method.new)( size: [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), dimension: [TextureDimension](../../render/render_resource/enum.TextureDimension.html "enum bevy::render::render_resource::TextureDimension"), data: [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, format: [TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat"), asset\_usage: [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages"), ) -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Creates a new image from raw binary data and the corresponding metadata.

##### Panics

Panics if the length of the `data`, volume of the `size` and the size of the `format` do not match.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/testbed/3d.rs ([lines 538-548](../../../src/testbed_3d/3d.rs.html#538-548))

```rust
517    fn create_white_cubemap(size: u32) -> Image {
518        // f16 bytes for 1.0 (white): [0, 60] in little-endian
519        const WHITE_F16: [u8; 2] = [0, 60];
520        const WHITE_PIXEL: [u8; 8] = [
521            WHITE_F16[0],
522            WHITE_F16[1], // R
523            WHITE_F16[0],
524            WHITE_F16[1], // G
525            WHITE_F16[0],
526            WHITE_F16[1], // B
527            WHITE_F16[0],
528            WHITE_F16[1], // A
529        ];
530
531        let pixel_data: Vec<u8> = (0..6 * size * size).flat_map(|_| WHITE_PIXEL).collect();
532
533        Image {
534            texture_view_descriptor: Some(TextureViewDescriptor {
535                dimension: Some(TextureViewDimension::Cube),
536                ..Default::default()
537            }),
538            ..Image::new(
539                Extent3d {
540                    width: size,
541                    height: size,
542                    depth_or_array_layers: 6,
543                },
544                TextureDimension::D2,
545                pixel_data,
546                TextureFormat::Rgba16Float,
547                RenderAssetUsages::RENDER_WORLD,
548            )
549        }
550    }
551
552    pub fn setup(
553        mut commands: Commands,
554        mut meshes: ResMut<Assets<Mesh>>,
555        mut materials: ResMut<Assets<StandardMaterial>>,
556        mut images: ResMut<Assets<Image>>,
557    ) {
558        let sphere_mesh = meshes.add(Sphere::new(0.45));
559
560        // Light should come from the environment map only
561        commands.insert_resource(GlobalAmbientLight::NONE);
562
563        // add entities to the world
564        for y in -2..=2 {
565            for x in -5..=5 {
566                let x01 = (x + 5) as f32 / 10.0;
567                let y01 = (y + 2) as f32 / 4.0;
568                // sphere
569                commands.spawn((
570                    Mesh3d(sphere_mesh.clone()),
571                    MeshMaterial3d(materials.add(StandardMaterial {
572                        base_color: LinearRgba::WHITE.into(),
573                        // vary key PBR parameters on a grid of spheres to show the effect
574                        metallic: y01,
575                        perceptual_roughness: x01,
576                        ..default()
577                    })),
578                    Transform::from_xyz(x as f32, y as f32 + 0.5, 0.0),
579                    DespawnOnExit(CURRENT_SCENE),
580                ));
581            }
582        }
583
584        // Create a pure white cubemap
585        let white_cubemap = create_white_cubemap(256);
586        let white_cubemap_handle = images.add(white_cubemap);
587
588        let mut solid_color_light = EnvironmentMapLight::solid_color(&mut images, Color::WHITE);
589        solid_color_light.intensity = 500.0;
590
591        // camera
592        commands.spawn((
593            Camera3d::default(),
594            Hdr,
595            Tonemapping::None,
596            Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
597            Projection::from(OrthographicProjection {
598                scale: 0.01,
599                scaling_mode: ScalingMode::WindowSize,
600                ..OrthographicProjection::default_3d()
601            }),
602            Skybox {
603                image: Some(white_cubemap_handle),
604                // middle gray
605                brightness: 500.0,
606                ..default()
607            },
608            solid_color_light,
609            DespawnOnExit(CURRENT_SCENE),
610        ));
611    }
612}
613
614mod white_furnace_environment_map_light {
615    use bevy::{
616        asset::RenderAssetUsages,
617        camera::{Hdr, ScalingMode},
618        core_pipeline::tonemapping::Tonemapping,
619        light::Skybox,
620        prelude::*,
621        render::render_resource::{
622            Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
623        },
624    };
625
626    const CURRENT_SCENE: super::Scene = super::Scene::WhiteFurnaceEnvironmentMapLight;
627
628    /// Creates a pure white cubemap
629    fn create_white_cubemap(size: u32) -> Image {
630        // f16 bytes for 1.0 (white): [0, 60] in little-endian
631        const WHITE_F16: [u8; 2] = [0, 60];
632        const WHITE_PIXEL: [u8; 8] = [
633            WHITE_F16[0],
634            WHITE_F16[1], // R
635            WHITE_F16[0],
636            WHITE_F16[1], // G
637            WHITE_F16[0],
638            WHITE_F16[1], // B
639            WHITE_F16[0],
640            WHITE_F16[1], // A
641        ];
642
643        let pixel_data: Vec<u8> = (0..6 * size * size).flat_map(|_| WHITE_PIXEL).collect();
644
645        Image {
646            texture_view_descriptor: Some(TextureViewDescriptor {
647                dimension: Some(TextureViewDimension::Cube),
648                ..Default::default()
649            }),
650            ..Image::new(
651                Extent3d {
652                    width: size,
653                    height: size,
654                    depth_or_array_layers: 6,
655                },
656                TextureDimension::D2,
657                pixel_data,
658                TextureFormat::Rgba16Float,
659                RenderAssetUsages::RENDER_WORLD,
660            )
661        }
662    }
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1122-1127)

#### pub fn [new\_uninit](#method.new_uninit)( size: [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), dimension: [TextureDimension](../../render/render_resource/enum.TextureDimension.html "enum bevy::render::render_resource::TextureDimension"), format: [TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat"), asset\_usage: [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages"), ) -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Exactly the same as [`Image::new`](../../prelude/struct.Image.html#method.new "associated function bevy::prelude::Image::new"), but doesn’t initialize the image

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/3d/mirror.rs ([lines 430-435](../../../src/mirror/mirror.rs.html#430-435))

```rust
423fn create_mirror_texture_image(images: &mut Assets<Image>, window_size: UVec2) -> Handle<Image> {
424    let mirror_image_extent = Extent3d {
425        width: window_size.x,
426        height: window_size.y,
427        depth_or_array_layers: 1,
428    };
429
430    let mut image = Image::new_uninit(
431        mirror_image_extent,
432        TextureDimension::D2,
433        TextureFormat::Bgra8UnormSrgb,
434        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
435    );
436    image.texture_descriptor.usage |=
437        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
438
439    images.add(image)
440}
```

Hide additional examples

examples/app/externally\_driven\_headless\_renderer.rs ([lines 76-85](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#76-85))

```rust
75    fn new_render_target(&mut self, width: u32, height: u32) -> RenderTarget {
76        let mut target = Image::new_uninit(
77            Extent3d {
78                width,
79                height,
80                depth_or_array_layers: 1,
81            },
82            TextureDimension::D2,
83            TextureFormat::Rgba8UnormSrgb,
84            RenderAssetUsages::RENDER_WORLD,
85        );
86        // We're going to render to this image, mark it as such
87        target.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;
88        self.0
89            .main
90            .world_mut()
91            .resource_mut::<Assets<Image>>()
92            .add(target)
93            .into()
94    }
```

examples/2d/dynamic\_mip\_generation.rs ([lines 773-782](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#773-782))

```rust
766    fn regenerate_mipmap_source_image(
767        &mut self,
768        commands: &mut Commands,
769        images: &mut Assets<Image>,
770    ) -> Handle<Image> {
771        let image_data = self.generate_image_data();
772
773        let mut image = Image::new_uninit(
774            Extent3d {
775                width: self.image_width as u32,
776                height: self.image_height as u32,
777                depth_or_array_layers: 1,
778            },
779            TextureDimension::D2,
780            TextureFormat::Rgba8Unorm,
781            RenderAssetUsages::all(),
782        );
783        image.texture_descriptor.mip_level_count = self.image_mip_level_count();
784        image.texture_descriptor.usage |= TextureUsages::STORAGE_BINDING;
785        image.data = Some(image_data);
786
787        let image_handle = images.add(image);
788        commands.insert_resource(MipmapSourceImage(image_handle.clone()));
789
790        image_handle
791    }
```

examples/shader\_advanced/render\_depth\_to\_texture.rs ([lines 311-320](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#311-320))

```rust
307    fn from_world(world: &mut World) -> Self {
308        let mut images = world.resource_mut::<Assets<Image>>();
309
310        // Create a new 32-bit floating point depth texture.
311        let mut depth_image = Image::new_uninit(
312            Extent3d {
313                width: DEPTH_TEXTURE_SIZE,
314                height: DEPTH_TEXTURE_SIZE,
315                depth_or_array_layers: 1,
316            },
317            TextureDimension::D2,
318            TextureFormat::Depth32Float,
319            RenderAssetUsages::default(),
320        );
321
322        // Create a sampler. Note that this needs to specify a `compare`
323        // function in order to be compatible with depth textures.
324        depth_image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
325            label: Some("custom depth image sampler".to_owned()),
326            compare: Some(ImageCompareFunction::Always),
327            ..ImageSamplerDescriptor::default()
328        });
329
330        let depth_image_handle = images.add(depth_image);
331        DemoDepthTexture(depth_image_handle)
332    }
```

examples/ui/widgets/viewport\_node.rs ([lines 36-41](../../../src/viewport_node/viewport_node.rs.html#36-41))

```rust
25fn test(
26    mut commands: Commands,
27    mut images: ResMut<Assets<Image>>,
28    mut meshes: ResMut<Assets<Mesh>>,
29    mut materials: ResMut<Assets<StandardMaterial>>,
30) {
31    // Spawn a UI camera
32    commands.spawn(Camera3d::default());
33
34    // Set up an texture for the 3D camera to render to.
35    // The size of the texture will be based on the viewport's ui size.
36    let mut image = Image::new_uninit(
37        default(),
38        TextureDimension::D2,
39        TextureFormat::Bgra8UnormSrgb,
40        RenderAssetUsages::all(),
41    );
42    image.texture_descriptor.usage =
43        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
44    let image_handle = images.add(image);
45
46    // Spawn the 3D camera
47    let camera = commands
48        .spawn((
49            Camera3d::default(),
50            Camera {
51                // Render this camera before our UI camera
52                order: -1,
53                ..default()
54            },
55            RenderTarget::Image(image_handle.clone().into()),
56        ))
57        .id();
58
59    // Spawn something for the 3D camera to look at
60    commands
61        .spawn((
62            Mesh3d(meshes.add(Cuboid::new(5.0, 5.0, 5.0))),
63            MeshMaterial3d(materials.add(Color::WHITE)),
64            Transform::from_xyz(0.0, 0.0, -10.0),
65            Shape,
66        ))
67        // We can observe pointer events on our objects as normal, the
68        // `bevy::ui::widget::viewport_picking` system will take care of ensuring our viewport
69        // clicks pass through
70        .observe(on_drag_cuboid);
71
72    // Spawn our viewport widget
73    commands
74        .spawn((
75            Node {
76                position_type: PositionType::Absolute,
77                top: px(50),
78                left: px(50),
79                width: px(200),
80                height: px(200),
81                border: UiRect::all(px(5)),
82                ..default()
83            },
84            BorderColor::all(Color::WHITE),
85            ViewportNode::new(camera),
86        ))
87        .observe(on_drag_viewport);
88}
```

examples/shader/gpu\_readback.rs ([lines 87-92](../../../src/gpu_readback/gpu_readback.rs.html#87-92))

```rust
67fn setup(
68    mut commands: Commands,
69    mut images: ResMut<Assets<Image>>,
70    mut buffers: ResMut<Assets<ShaderBuffer>>,
71) {
72    // Create a storage buffer with some data
73    let buffer: Vec<u32> = (0..BUFFER_LEN as u32).collect();
74    let mut buffer = ShaderBuffer::from(buffer);
75    // We need to enable the COPY_SRC usage so we can copy the buffer to the cpu
76    buffer.buffer_description.usage |= BufferUsages::COPY_SRC;
77    let buffer = buffers.add(buffer);
78
79    // Create a storage texture with some data
80    let size = Extent3d {
81        width: BUFFER_LEN as u32,
82        height: 1,
83        ..default()
84    };
85    // We create an uninitialized image since this texture will only be used for getting data out
86    // of the compute shader, not getting data in, so there's no reason for it to exist on the CPU
87    let mut image = Image::new_uninit(
88        size,
89        TextureDimension::D2,
90        TextureFormat::R32Uint,
91        RenderAssetUsages::RENDER_WORLD,
92    );
93    // We also need to enable the COPY_SRC, as well as STORAGE_BINDING so we can use it in the
94    // compute shader
95    image.texture_descriptor.usage |= TextureUsages::COPY_SRC | TextureUsages::STORAGE_BINDING;
96    let image = images.add(image);
97
98    // Spawn the readback components. For each frame, the data will be read back from the GPU
99    // asynchronously and trigger the `ReadbackComplete` event on this entity. Despawn the entity
100    // to stop reading back the data.
101    commands
102        .spawn(Readback::buffer(buffer.clone()))
103        .observe(|event: On<ReadbackComplete>| {
104            // This matches the type which was used to create the `ShaderBuffer` above,
105            // and is a convenient way to interpret the data.
106            let data: Vec<u32> = event.to_shader_type();
107            info!("Buffer {:?}", data);
108        });
109
110    // It is also possible to read only a range of the buffer.
111    commands
112        .spawn(Readback::buffer_range(
113            buffer.clone(),
114            4 * u32::SHADER_SIZE.get(), // skip the first four elements
115            8 * u32::SHADER_SIZE.get(), // read eight elements
116        ))
117        .observe(|event: On<ReadbackComplete>| {
118            let data: Vec<u32> = event.to_shader_type();
119            info!("Buffer range {:?}", data);
120        });
121
122    // This is just a simple way to pass the buffer handle to the render app for our compute node
123    commands.insert_resource(ReadbackBuffer(buffer));
124
125    // Textures can also be read back from the GPU. Pay careful attention to the format of the
126    // texture, as it will affect how the data is interpreted.
127    commands
128        .spawn(Readback::texture(image.clone()))
129        .observe(|event: On<ReadbackComplete>| {
130            // You probably want to interpret the data as a color rather than a `ShaderType`,
131            // but in this case we know the data is a single channel storage texture, so we can
132            // interpret it as a `Vec<u32>`
133            let data: Vec<u32> = event.to_shader_type();
134            info!("Image {:?}", data);
135        });
136    commands.insert_resource(ReadbackImage(image));
137}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1153)

#### pub fn [transparent](#method.transparent)() -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

A transparent white 1x1x1 image.

Contrast to [`Image::default`](../../prelude/struct.Image.html#method.default "associated function bevy::prelude::Image::default"), which is opaque.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1171)

#### pub fn [default\_uninit](#method.default_uninit)() -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Creates a new uninitialized 1x1x1 image

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1185-1191)

#### pub fn [new\_fill](#method.new_fill)( size: [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), dimension: [TextureDimension](../../render/render_resource/enum.TextureDimension.html "enum bevy::render::render_resource::TextureDimension"), pixel: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], format: [TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat"), asset\_usage: [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages"), ) -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Creates a new image from raw binary data and the corresponding metadata, by filling the image data with the `pixel` data repeated multiple times.

##### Panics

Panics if the size of the `format` is not a multiple of the length of the `pixel` data.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/stress\_tests/bevymark\_3d.rs ([lines 490-500](../../../src/bevymark_3d/bevymark_3d.rs.html#490-500))

```rust
481fn init_textures(textures: &mut Vec<Handle<Image>>, args: &Args, images: &mut Assets<Image>) {
482    let mut color_rng = ChaCha8Rng::seed_from_u64(42);
483    while textures.len() < args.material_texture_count {
484        let pixel = [
485            color_rng.random(),
486            color_rng.random(),
487            color_rng.random(),
488            255,
489        ];
490        textures.push(images.add(Image::new_fill(
491            Extent3d {
492                width: CUBE_TEXTURE_SIZE as u32,
493                height: CUBE_TEXTURE_SIZE as u32,
494                depth_or_array_layers: 1,
495            },
496            TextureDimension::D2,
497            &pixel,
498            TextureFormat::Rgba8UnormSrgb,
499            RenderAssetUsages::RENDER_WORLD,
500        )));
501    }
502}
```

Hide additional examples

examples/3d/3d\_shapes.rs ([lines 247-257](../../../src/3d_shapes/3d_shapes.rs.html#247-257))

```rust
232fn uv_debug_texture() -> Image {
233    const TEXTURE_SIZE: usize = 8;
234
235    let mut palette: [u8; 32] = [
236        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
237        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
238    ];
239
240    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
241    for y in 0..TEXTURE_SIZE {
242        let offset = TEXTURE_SIZE * y * 4;
243        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
244        palette.rotate_right(4);
245    }
246
247    Image::new_fill(
248        Extent3d {
249            width: TEXTURE_SIZE as u32,
250            height: TEXTURE_SIZE as u32,
251            depth_or_array_layers: 1,
252        },
253        TextureDimension::D2,
254        &texture_data,
255        TextureFormat::Rgba8UnormSrgb,
256        RenderAssetUsages::RENDER_WORLD,
257    )
258}
```

examples/stress\_tests/many\_cubes.rs ([lines 375-381](../../../src/many_cubes/many_cubes.rs.html#375-381))

```rust
359fn init_textures(args: &Args, images: &mut Assets<Image>) -> Vec<Handle<Image>> {
360    // We're seeding the PRNG here to make this example deterministic for testing purposes.
361    // This isn't strictly required in practical use unless you need your app to be deterministic.
362    let mut color_rng = ChaCha8Rng::seed_from_u64(42);
363    let color_bytes: Vec<u8> = (0..(args.material_texture_count * 4))
364        .map(|i| {
365            if (i % 4) == 3 {
366                255
367            } else {
368                color_rng.random()
369            }
370        })
371        .collect();
372    color_bytes
373        .chunks(4)
374        .map(|pixel| {
375            images.add(Image::new_fill(
376                Extent3d::default(),
377                TextureDimension::D2,
378                pixel,
379                TextureFormat::Rgba8UnormSrgb,
380                RenderAssetUsages::RENDER_WORLD,
381            ))
382        })
383        .collect()
384}
```

examples/stress\_tests/bevymark.rs ([lines 632-642](../../../src/bevymark/bevymark.rs.html#632-642))

```rust
621fn init_textures(textures: &mut Vec<Handle<Image>>, args: &Args, images: &mut Assets<Image>) {
622    // We're seeding the PRNG here to make this example deterministic for testing purposes.
623    // This isn't strictly required in practical use unless you need your app to be deterministic.
624    let mut color_rng = ChaCha8Rng::seed_from_u64(42);
625    while textures.len() < args.material_texture_count {
626        let pixel = [
627            color_rng.random(),
628            color_rng.random(),
629            color_rng.random(),
630            255,
631        ];
632        textures.push(images.add(Image::new_fill(
633            Extent3d {
634                width: BIRD_TEXTURE_SIZE as u32,
635                height: BIRD_TEXTURE_SIZE as u32,
636                depth_or_array_layers: 1,
637            },
638            TextureDimension::D2,
639            &pixel,
640            TextureFormat::Rgba8UnormSrgb,
641            RenderAssetUsages::RENDER_WORLD,
642        )));
643    }
644}
```

examples/3d/anti\_aliasing.rs ([lines 519-529](../../../src/anti_aliasing/anti_aliasing.rs.html#519-529))

```rust
504fn uv_debug_texture() -> Image {
505    const TEXTURE_SIZE: usize = 8;
506
507    let mut palette: [u8; 32] = [
508        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
509        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
510    ];
511
512    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
513    for y in 0..TEXTURE_SIZE {
514        let offset = TEXTURE_SIZE * y * 4;
515        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
516        palette.rotate_right(4);
517    }
518
519    let mut img = Image::new_fill(
520        Extent3d {
521            width: TEXTURE_SIZE as u32,
522            height: TEXTURE_SIZE as u32,
523            depth_or_array_layers: 1,
524        },
525        TextureDimension::D2,
526        &texture_data,
527        TextureFormat::Rgba8UnormSrgb,
528        RenderAssetUsages::RENDER_WORLD,
529    );
530    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::default());
531    img
532}
```

examples/3d/motion\_blur.rs ([lines 371-381](../../../src/motion_blur/motion_blur.rs.html#371-381))

```rust
355fn uv_debug_texture() -> Image {
356    use bevy::{asset::RenderAssetUsages, render::render_resource::*};
357    const TEXTURE_SIZE: usize = 7;
358
359    let mut palette = [
360        164, 164, 164, 255, 168, 168, 168, 255, 153, 153, 153, 255, 139, 139, 139, 255, 153, 153,
361        153, 255, 177, 177, 177, 255, 159, 159, 159, 255,
362    ];
363
364    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
365    for y in 0..TEXTURE_SIZE {
366        let offset = TEXTURE_SIZE * y * 4;
367        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
368        palette.rotate_right(12);
369    }
370
371    let mut img = Image::new_fill(
372        Extent3d {
373            width: TEXTURE_SIZE as u32,
374            height: TEXTURE_SIZE as u32,
375            depth_or_array_layers: 1,
376        },
377        TextureDimension::D2,
378        &texture_data,
379        TextureFormat::Rgba8UnormSrgb,
380        RenderAssetUsages::RENDER_WORLD,
381    );
382    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
383        address_mode_u: ImageAddressMode::Repeat,
384        address_mode_v: ImageAddressMode::MirrorRepeat,
385        mag_filter: ImageFilterMode::Nearest,
386        ..ImageSamplerDescriptor::linear()
387    });
388    img
389}
```

Additional examples can be found in:  

*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#42-55)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#128-138)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#467-477)
*   [examples/ui/render\_ui\_to\_texture.rs](../../../src/render_ui_to_texture/render_ui_to_texture.rs.html#48-54)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1232-1237)

#### pub fn [new\_target\_texture](#method.new_target_texture)( width: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), height: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), format: [TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat"), view\_format: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")\>, ) -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Create a new zero-filled image with a given size, which can be rendered to. Useful for mirrors, UI, or exporting images for example. This is primarily for use as a render target for a [`Camera`](https://docs.rs/bevy/latest/bevy/render/camera/struct.Camera.html). See [`RenderTarget::Image`](https://docs.rs/bevy/latest/bevy/render/camera/enum.RenderTarget.html#variant.Image).

For Standard Dynamic Range (SDR) images you can use [`TextureFormat::Rgba8UnormSrgb`](../../render/render_resource/enum.TextureFormat.html#variant.Rgba8UnormSrgb "variant bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb"). For High Dynamic Range (HDR) images you can use [`TextureFormat::Rgba16Float`](../../render/render_resource/enum.TextureFormat.html#variant.Rgba16Float "variant bevy::render::render_resource::TextureFormat::Rgba16Float").

The default [`TextureUsages`](../../render/render_resource/struct.TextureUsages.html "struct bevy::render::render_resource::TextureUsages") are [`TEXTURE_BINDING`](../../render/render_resource/struct.TextureUsages.html#associatedconstant.TEXTURE_BINDING "associated constant bevy::render::render_resource::TextureUsages::TEXTURE_BINDING"), [`COPY_DST`](../../render/render_resource/struct.TextureUsages.html#associatedconstant.COPY_DST "associated constant bevy::render::render_resource::TextureUsages::COPY_DST"), [`RENDER_ATTACHMENT`](../../render/render_resource/struct.TextureUsages.html#associatedconstant.RENDER_ATTACHMENT "associated constant bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT").

The default [`RenderAssetUsages`](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages") is [`MAIN_WORLD | RENDER_WORLD`](../../asset/struct.RenderAssetUsages.html#method.default "associated function bevy::asset::RenderAssetUsages::default") so that it is accessible from the CPU and GPU. You can customize this by changing the [`asset_usage`](../../prelude/struct.Image.html#structfield.asset_usage "field bevy::prelude::Image::asset_usage") field.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/shader/compute\_shader\_game\_of\_life.rs ([line 55](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#55))

```rust
54fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
55    let mut image = Image::new_target_texture(SIZE.x, SIZE.y, TextureFormat::Rgba32Float, None);
56    image.asset_usage = RenderAssetUsages::RENDER_WORLD;
57    image.texture_descriptor.usage =
58        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
59    let image0 = images.add(image.clone());
60    let image1 = images.add(image);
61
62    commands.spawn((
63        Sprite {
64            image: image0.clone(),
65            custom_size: Some(SIZE.as_vec2()),
66            ..default()
67        },
68        Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)),
69    ));
70    commands.spawn(Camera2d);
71
72    commands.insert_resource(GameOfLifeImages {
73        texture_a: image0,
74        texture_b: image1,
75    });
76
77    commands.insert_resource(GameOfLifeUniforms {
78        alive_color: LinearRgba::RED,
79    });
80}
```

Hide additional examples

examples/app/headless\_renderer.rs ([line 243](../../../src/headless_renderer/headless_renderer.rs.html#243))

```rust
227fn setup_render_target(
228    commands: &mut Commands,
229    images: &mut ResMut<Assets<Image>>,
230    render_device: &Res<RenderDevice>,
231    scene_controller: &mut ResMut<SceneController>,
232    pre_roll_frames: u32,
233    scene_name: String,
234) -> RenderTarget {
235    let size = Extent3d {
236        width: scene_controller.width,
237        height: scene_controller.height,
238        ..Default::default()
239    };
240
241    // This is the texture that will be rendered to.
242    let mut render_target_image =
243        Image::new_target_texture(size.width, size.height, TextureFormat::Rgba8UnormSrgb, None);
244    render_target_image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
245    let render_target_image_handle = images.add(render_target_image);
246
247    // This is the texture that will be copied to.
248    let cpu_image =
249        Image::new_target_texture(size.width, size.height, TextureFormat::Rgba8UnormSrgb, None);
250    let cpu_image_handle = images.add(cpu_image);
251
252    commands.spawn(ImageCopier::new(
253        render_target_image_handle.clone(),
254        size,
255        render_device,
256    ));
257
258    commands.spawn(ImageToSave(cpu_image_handle));
259
260    scene_controller.state = SceneState::Render(pre_roll_frames);
261    scene_controller.name = scene_name;
262    RenderTarget::Image(render_target_image_handle.into())
263}
```

examples/3d/render\_to\_texture.rs ([lines 31-36](../../../src/render_to_texture/render_to_texture.rs.html#31-36))

```rust
24fn setup(
25    mut commands: Commands,
26    mut meshes: ResMut<Assets<Mesh>>,
27    mut materials: ResMut<Assets<StandardMaterial>>,
28    mut images: ResMut<Assets<Image>>,
29) {
30    // This is the texture that will be rendered to.
31    let image = Image::new_target_texture(
32        512,
33        512,
34        TextureFormat::Rgba8Unorm,
35        Some(TextureFormat::Rgba8UnormSrgb),
36    );
37
38    let image_handle = images.add(image);
39
40    let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
41    let cube_material_handle = materials.add(StandardMaterial {
42        base_color: Color::srgb(0.8, 0.7, 0.6),
43        reflectance: 0.02,
44        unlit: false,
45        ..default()
46    });
47
48    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
49    let first_pass_layer = RenderLayers::layer(1);
50
51    // The cube that will be rendered to the texture.
52    commands.spawn((
53        Mesh3d(cube_handle),
54        MeshMaterial3d(cube_material_handle),
55        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
56        FirstPassCube,
57        first_pass_layer.clone(),
58    ));
59
60    // Light
61    // NOTE: we add the light to both layers so it affects both the rendered-to-texture cube, and the cube on which we display the texture
62    // Setting the layer to RenderLayers::layer(0) would cause the main view to be lit, but the rendered-to-texture cube to be unlit.
63    // Setting the layer to RenderLayers::layer(1) would cause the rendered-to-texture cube to be lit, but the main view to be unlit.
64    commands.spawn((
65        PointLight::default(),
66        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
67        RenderLayers::layer(0).with(1),
68    ));
69
70    commands.spawn((
71        Camera3d::default(),
72        Camera {
73            // render before the "main pass" camera
74            order: -1,
75            clear_color: Color::WHITE.into(),
76            ..default()
77        },
78        RenderTarget::Image(image_handle.clone().into()),
79        Transform::from_translation(Vec3::new(0.0, 0.0, 15.0)).looking_at(Vec3::ZERO, Vec3::Y),
80        first_pass_layer,
81    ));
82
83    let cube_size = 4.0;
84    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));
85
86    // This material has the texture that has been rendered.
87    let material_handle = materials.add(StandardMaterial {
88        base_color_texture: Some(image_handle),
89        reflectance: 0.02,
90        unlit: false,
91        ..default()
92    });
93
94    // Main pass cube, with material containing the rendered first pass texture.
95    commands.spawn((
96        Mesh3d(cube_handle),
97        MeshMaterial3d(material_handle),
98        Transform::from_xyz(0.0, 0.0, 1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
99        MainPassCube,
100    ));
101
102    // The main pass camera.
103    commands.spawn((
104        Camera3d::default(),
105        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
106    ));
107}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1283)

#### pub fn [width](#method.width)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the width of a 2D image.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/ui/text/font\_atlas\_debug.rs ([line 58](../../../src/font_atlas_debug/font_atlas_debug.rs.html#58))

```rust
39fn atlas_render_system(
40    mut commands: Commands,
41    mut state: ResMut<State>,
42    font_atlas_set: Res<FontAtlasSet>,
43    images: Res<Assets<Image>>,
44) {
45    if let Some(font_atlases) = font_atlas_set.values().next() {
46        let x_offset = state.atlas_count as f32;
47        if state.atlas_count == font_atlases.len() as u32 {
48            return;
49        }
50        let font_atlas = &font_atlases[state.atlas_count as usize];
51        let image = images.get(&font_atlas.texture).unwrap();
52        state.atlas_count += 1;
53        commands.spawn((
54            ImageNode::new(font_atlas.texture.clone()),
55            Node {
56                position_type: PositionType::Absolute,
57                top: Val::ZERO,
58                left: px(image.width() as f32 * x_offset),
59                ..default()
60            },
61        ));
62    }
63}
```

Hide additional examples

examples/3d/skybox.rs ([line 160](../../../src/skybox/skybox.rs.html#160))

```rust
148fn asset_loaded(
149    asset_server: Res<AssetServer>,
150    mut images: ResMut<Assets<Image>>,
151    mut cubemap: ResMut<Cubemap>,
152    mut skyboxes: Query<&mut Skybox>,
153) {
154    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
155        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
156        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
157        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
158        // so they appear as one texture. The following code reconfigures the texture as necessary.
159        if image.texture_descriptor.array_layer_count() == 1 {
160            let layers = image.height() / image.width();
161            image
162                .reinterpret_stacked_2d_as_array(layers)
163                .expect("asset should be 2d texture and height will always be evenly divisible with the given layers");
164            image.texture_view_descriptor = Some(TextureViewDescriptor {
165                dimension: Some(TextureViewDimension::Cube),
166                ..default()
167            });
168        }
169
170        for mut skybox in &mut skyboxes {
171            skybox.image = Some(cubemap.image_handle.clone());
172        }
173
174        cubemap.is_loaded = true;
175    }
176}
```

examples/app/headless\_renderer.rs ([line 476](../../../src/headless_renderer/headless_renderer.rs.html#476))

```rust
450fn update(
451    images_to_save: Query<&ImageToSave>,
452    receiver: Res<MainWorldReceiver>,
453    mut images: ResMut<Assets<Image>>,
454    mut scene_controller: ResMut<SceneController>,
455    mut app_exit_writer: MessageWriter<AppExit>,
456    mut file_number: Local<u32>,
457) {
458    if let SceneState::Render(n) = scene_controller.state {
459        if n < 1 {
460            // We don't want to block the main world on this,
461            // so we use try_recv which attempts to receive without blocking
462            let mut image_data = Vec::new();
463            while let Ok(data) = receiver.try_recv() {
464                // image generation could be faster than saving to fs,
465                // that's why use only last of them
466                image_data = data;
467            }
468            if !image_data.is_empty() {
469                for image in images_to_save.iter() {
470                    // Fill correct data from channel to image
471                    let mut img_bytes = images.get_mut(image.id()).unwrap();
472
473                    // We need to ensure that this works regardless of the image dimensions
474                    // If the image became wider when copying from the texture to the buffer,
475                    // then the data is reduced to its original size when copying from the buffer to the image.
476                    let row_bytes = img_bytes.width() as usize
477                        * img_bytes.texture_descriptor.format.pixel_size().unwrap();
478                    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
479                    if row_bytes == aligned_row_bytes {
480                        img_bytes.data.as_mut().unwrap().clone_from(&image_data);
481                    } else {
482                        // shrink data to original image size
483                        img_bytes.data = Some(
484                            image_data
485                                .chunks(aligned_row_bytes)
486                                .take(img_bytes.height() as usize)
487                                .flat_map(|row| &row[..row_bytes.min(row.len())])
488                                .cloned()
489                                .collect(),
490                        );
491                    }
492
493                    // Create RGBA Image Buffer
494                    let img = match img_bytes.clone().try_into_dynamic() {
495                        Ok(img) => img.to_rgba8(),
496                        Err(e) => panic!("Failed to create image buffer {e:?}"),
497                    };
498
499                    // Prepare directory for images, test_images in bevy folder is used here for example
500                    // You should choose the path depending on your needs
501                    let images_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_images");
502                    info!("Saving image to: {images_dir:?}");
503                    std::fs::create_dir_all(&images_dir).unwrap();
504
505                    // Choose filename starting from 000.png
506                    let image_path = images_dir.join(format!("{:03}.png", file_number.deref()));
507                    *file_number.deref_mut() += 1;
508
509                    // Finally saving image to file, this heavy blocking operation is kept here
510                    // for example simplicity, but in real app you should move it to a separate task
511                    if let Err(e) = img.save(image_path) {
512                        panic!("Failed to save image: {e}");
513                    };
514                }
515                if scene_controller.single_image {
516                    app_exit_writer.write(AppExit::Success);
517                }
518            }
519        } else {
520            // clears channel for skipped frames
521            while receiver.try_recv().is_ok() {}
522            scene_controller.state = SceneState::Render(n - 1);
523        }
524    }
525}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1289)

#### pub fn [height](#method.height)(&self) -> [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

Returns the height of a 2D image.

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/3d/skybox.rs ([line 160](../../../src/skybox/skybox.rs.html#160))

```rust
148fn asset_loaded(
149    asset_server: Res<AssetServer>,
150    mut images: ResMut<Assets<Image>>,
151    mut cubemap: ResMut<Cubemap>,
152    mut skyboxes: Query<&mut Skybox>,
153) {
154    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
155        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
156        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
157        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
158        // so they appear as one texture. The following code reconfigures the texture as necessary.
159        if image.texture_descriptor.array_layer_count() == 1 {
160            let layers = image.height() / image.width();
161            image
162                .reinterpret_stacked_2d_as_array(layers)
163                .expect("asset should be 2d texture and height will always be evenly divisible with the given layers");
164            image.texture_view_descriptor = Some(TextureViewDescriptor {
165                dimension: Some(TextureViewDimension::Cube),
166                ..default()
167            });
168        }
169
170        for mut skybox in &mut skyboxes {
171            skybox.image = Some(cubemap.image_handle.clone());
172        }
173
174        cubemap.is_loaded = true;
175    }
176}
```

Hide additional examples

examples/app/headless\_renderer.rs ([line 486](../../../src/headless_renderer/headless_renderer.rs.html#486))

```rust
450fn update(
451    images_to_save: Query<&ImageToSave>,
452    receiver: Res<MainWorldReceiver>,
453    mut images: ResMut<Assets<Image>>,
454    mut scene_controller: ResMut<SceneController>,
455    mut app_exit_writer: MessageWriter<AppExit>,
456    mut file_number: Local<u32>,
457) {
458    if let SceneState::Render(n) = scene_controller.state {
459        if n < 1 {
460            // We don't want to block the main world on this,
461            // so we use try_recv which attempts to receive without blocking
462            let mut image_data = Vec::new();
463            while let Ok(data) = receiver.try_recv() {
464                // image generation could be faster than saving to fs,
465                // that's why use only last of them
466                image_data = data;
467            }
468            if !image_data.is_empty() {
469                for image in images_to_save.iter() {
470                    // Fill correct data from channel to image
471                    let mut img_bytes = images.get_mut(image.id()).unwrap();
472
473                    // We need to ensure that this works regardless of the image dimensions
474                    // If the image became wider when copying from the texture to the buffer,
475                    // then the data is reduced to its original size when copying from the buffer to the image.
476                    let row_bytes = img_bytes.width() as usize
477                        * img_bytes.texture_descriptor.format.pixel_size().unwrap();
478                    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
479                    if row_bytes == aligned_row_bytes {
480                        img_bytes.data.as_mut().unwrap().clone_from(&image_data);
481                    } else {
482                        // shrink data to original image size
483                        img_bytes.data = Some(
484                            image_data
485                                .chunks(aligned_row_bytes)
486                                .take(img_bytes.height() as usize)
487                                .flat_map(|row| &row[..row_bytes.min(row.len())])
488                                .cloned()
489                                .collect(),
490                        );
491                    }
492
493                    // Create RGBA Image Buffer
494                    let img = match img_bytes.clone().try_into_dynamic() {
495                        Ok(img) => img.to_rgba8(),
496                        Err(e) => panic!("Failed to create image buffer {e:?}"),
497                    };
498
499                    // Prepare directory for images, test_images in bevy folder is used here for example
500                    // You should choose the path depending on your needs
501                    let images_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_images");
502                    info!("Saving image to: {images_dir:?}");
503                    std::fs::create_dir_all(&images_dir).unwrap();
504
505                    // Choose filename starting from 000.png
506                    let image_path = images_dir.join(format!("{:03}.png", file_number.deref()));
507                    *file_number.deref_mut() += 1;
508
509                    // Finally saving image to file, this heavy blocking operation is kept here
510                    // for example simplicity, but in real app you should move it to a separate task
511                    if let Err(e) = img.save(image_path) {
512                        panic!("Failed to save image: {e}");
513                    };
514                }
515                if scene_controller.single_image {
516                    app_exit_writer.write(AppExit::Success);
517                }
518            }
519        } else {
520            // clears channel for skipped frames
521            while receiver.try_recv().is_ok() {}
522            scene_controller.state = SceneState::Render(n - 1);
523        }
524    }
525}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1295)

#### pub fn [aspect\_ratio](#method.aspect_ratio)(&self) -> [AspectRatio](../../math/struct.AspectRatio.html "struct bevy::math::AspectRatio")

Returns the aspect ratio (width / height) of a 2D image.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1303)

#### pub fn [size\_f32](#method.size_f32)(&self) -> [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

Returns the size of a 2D image as f32.

##### [Examples found in repository](#scraped-examples-6)[?](../../../scrape-examples-help.html)

examples/3d/tonemapping.rs ([line 256](../../../src/tonemapping/tonemapping.rs.html#256))

```rust
227fn resize_image(
228    image_mesh: Query<(&MeshMaterial3d<StandardMaterial>, &Mesh3d), With<HDRViewer>>,
229    materials: Res<Assets<StandardMaterial>>,
230    mut meshes: ResMut<Assets<Mesh>>,
231    images: Res<Assets<Image>>,
232    mut image_event_reader: MessageReader<AssetEvent<Image>>,
233) {
234    for event in image_event_reader.read() {
235        let (AssetEvent::Added { id } | AssetEvent::Modified { id }) = event else {
236            continue;
237        };
238
239        for (mat_h, mesh_h) in &image_mesh {
240            let Some(mat) = materials.get(mat_h) else {
241                continue;
242            };
243
244            let Some(ref base_color_texture) = mat.base_color_texture else {
245                continue;
246            };
247
248            if *id != base_color_texture.id() {
249                continue;
250            };
251
252            let Some(image_changed) = images.get(*id) else {
253                continue;
254            };
255
256            let size = image_changed.size_f32().normalize_or_zero() * 1.4;
257            // Resize Mesh
258            let quad = Mesh::from(Rectangle::from_size(size));
259            meshes.insert(mesh_h, quad).unwrap();
260        }
261    }
262}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1309)

#### pub fn [size](#method.size)(&self) -> [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

Returns the size of a 2D image.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1317)

#### pub fn [resize](#method.resize)(&mut self, size: [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"))

Resizes the image to the new size, by removing information or appending 0 to the `data`. Does not properly scale the contents of the image.

If you need to keep pixel data intact, use [`Image::resize_in_place`](../../prelude/struct.Image.html#method.resize_in_place "method bevy::prelude::Image::resize_in_place").

##### [Examples found in repository](#scraped-examples-7)[?](../../../scrape-examples-help.html)

examples/2d/pixel\_grid\_snap.rs ([line 109](../../../src/pixel_grid_snap/pixel_grid_snap.rs.html#109))

```rust
84fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
85    let canvas_size = Extent3d {
86        width: RES_WIDTH,
87        height: RES_HEIGHT,
88        ..default()
89    };
90
91    // This Image serves as a canvas representing the low-resolution game screen
92    let mut canvas = Image {
93        texture_descriptor: TextureDescriptor {
94            label: None,
95            size: canvas_size,
96            dimension: TextureDimension::D2,
97            format: TextureFormat::Bgra8UnormSrgb,
98            mip_level_count: 1,
99            sample_count: 1,
100            usage: TextureUsages::TEXTURE_BINDING
101                | TextureUsages::COPY_DST
102                | TextureUsages::RENDER_ATTACHMENT,
103            view_formats: &[],
104        },
105        ..default()
106    };
107
108    // Fill image.data with zeroes
109    canvas.resize(canvas_size);
110
111    let image_handle = images.add(canvas);
112
113    // This camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
114    commands.spawn((
115        Camera2d,
116        Camera {
117            // Render before the "main pass" camera
118            order: -1,
119            clear_color: ClearColorConfig::Custom(GRAY.into()),
120            ..default()
121        },
122        RenderTarget::Image(image_handle.clone().into()),
123        Msaa::Off,
124        InGameCamera,
125        PIXEL_PERFECT_LAYERS,
126    ));
127
128    // Spawn the canvas
129    commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));
130
131    // The "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
132    // here, the canvas and one of the sample sprites will be rendered by this camera
133    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
134}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1328-1331)

#### pub fn [reinterpret\_size](#method.reinterpret_size)( &mut self, new\_size: [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TextureReinterpretationError](../enum.TextureReinterpretationError.html "enum bevy::image::TextureReinterpretationError")\>

Changes the `size` if the total number of data elements (pixels) remains the same. If not, returns [`TextureReinterpretationError::IncompatibleSizes`](../enum.TextureReinterpretationError.html#variant.IncompatibleSizes "variant bevy::image::TextureReinterpretationError::IncompatibleSizes").

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1347)

#### pub fn [resize\_in\_place](#method.resize_in_place)(&mut self, new\_size: [Extent3d](../../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d"))

Resizes the image to the new size, keeping the pixel data intact, anchored at the top-left. When growing, the new space is filled with 0. When shrinking, the image is clipped.

For faster resizing when keeping pixel data intact is not important, use [`Image::resize`](../../prelude/struct.Image.html#method.resize "method bevy::prelude::Image::resize").

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1395-1398)

#### pub fn [reinterpret\_stacked\_2d\_as\_array](#method.reinterpret_stacked_2d_as_array)( &mut self, layers: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TextureReinterpretationError](../enum.TextureReinterpretationError.html "enum bevy::image::TextureReinterpretationError")\>

Takes a 2D image containing vertically stacked images of the same size, and reinterprets it as a 2D array texture, where each of the stacked images becomes one layer of the array. This is primarily for use with the `texture2DArray` shader uniform type.

##### Errors

Returns [`TextureReinterpretationError`](../enum.TextureReinterpretationError.html "enum bevy::image::TextureReinterpretationError") if the texture is not 2D, has more than one layers or is not evenly dividable into the `layers`.

##### [Examples found in repository](#scraped-examples-8)[?](../../../scrape-examples-help.html)

examples/3d/skybox.rs ([line 162](../../../src/skybox/skybox.rs.html#162))

```rust
148fn asset_loaded(
149    asset_server: Res<AssetServer>,
150    mut images: ResMut<Assets<Image>>,
151    mut cubemap: ResMut<Cubemap>,
152    mut skyboxes: Query<&mut Skybox>,
153) {
154    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
155        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
156        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
157        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
158        // so they appear as one texture. The following code reconfigures the texture as necessary.
159        if image.texture_descriptor.array_layer_count() == 1 {
160            let layers = image.height() / image.width();
161            image
162                .reinterpret_stacked_2d_as_array(layers)
163                .expect("asset should be 2d texture and height will always be evenly divisible with the given layers");
164            image.texture_view_descriptor = Some(TextureViewDescriptor {
165                dimension: Some(TextureViewDimension::Cube),
166                ..default()
167            });
168        }
169
170        for mut skybox in &mut skyboxes {
171            skybox.image = Some(cubemap.image_handle.clone());
172        }
173
174        cubemap.is_loaded = true;
175    }
176}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1433-1437)

#### pub fn [create\_stacked\_array\_from\_2d\_grid](#method.create_stacked_array_from_2d_grid)( &self, rows: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), columns: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image"), [TextureReinterpretationError](../enum.TextureReinterpretationError.html "enum bevy::image::TextureReinterpretationError")\>

Returns a newly constructed 2D image using the same properties as &self from a grid of tiles of the specified size, The new image is constructed in a vertical stack of tiles to be used as a 2D array texture.

This is primarily for preparing grid based tilesets.

##### Errors

Returns [`TextureReinterpretationError`](../enum.TextureReinterpretationError.html "enum bevy::image::TextureReinterpretationError") if the texture is not 2D, has more than one layers or is not evenly dividable by `size_in_tiles`.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1535)

#### pub fn [convert](#method.convert)(&self, new\_format: [TextureFormat](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>

Convert a texture from a format to another. Only a few formats are supported as input and output:

*   `TextureFormat::R8Unorm`
*   `TextureFormat::Rg8Unorm`
*   `TextureFormat::Rgba8UnormSrgb`

To get [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") as a [`image::DynamicImage`](https://docs.rs/image/0.25.9/x86_64-unknown-linux-gnu/image/images/dynimage/enum.DynamicImage.html "enum image::images::dynimage::DynamicImage") see: [`Image::try_into_dynamic`](../../prelude/struct.Image.html#method.try_into_dynamic "method bevy::prelude::Image::try_into_dynamic").

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1557-1568)

#### pub fn [from\_buffer](#method.from_buffer)( buffer: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], image\_type: [ImageType](../enum.ImageType.html "enum bevy::image::ImageType")<'\_>, supported\_compressed\_formats: [CompressedImageFormats](../struct.CompressedImageFormats.html "struct bevy::image::CompressedImageFormats"), is\_srgb: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), image\_sampler: [ImageSampler](../enum.ImageSampler.html "enum bevy::image::ImageSampler"), asset\_usage: [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image"), [TextureError](../../prelude/enum.TextureError.html "enum bevy::prelude::TextureError")\>

Load a bytes buffer in a [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image"), according to type `image_type`, using the `image` crate

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1613)

#### pub fn [is\_compressed](#method.is_compressed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether the texture format is compressed or uncompressed

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1635)

#### pub fn [pixel\_data\_offset](#method.pixel_data_offset)( &self, coords: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Compute the byte offset where the data of a specific pixel is stored

Returns an error if the provided coordinates are out of bounds.

For 2D textures, Z is the layer number. For 1D textures, Y and Z are ignored.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1669)

#### pub fn [pixel\_bytes](#method.pixel_bytes)(&self, coords: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Get a reference to the data bytes where a specific pixel’s value is stored.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1680)

#### pub fn [pixel\_bytes\_mut](#method.pixel_bytes_mut)( &mut self, coords: [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\], [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Get a mutable reference to the data bytes where a specific pixel’s value is stored.

##### [Examples found in repository](#scraped-examples-9)[?](../../../scrape-examples-help.html)

examples/2d/cpu\_draw.rs ([line 70](../../../src/cpu_draw/cpu_draw.rs.html#70))

```rust
38fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
39    commands.spawn(Camera2d);
40
41    // Create an image that we are going to draw into
42    let mut image = Image::new_fill(
43        // 2D image of size 256x256
44        Extent3d {
45            width: IMAGE_WIDTH,
46            height: IMAGE_HEIGHT,
47            depth_or_array_layers: 1,
48        },
49        TextureDimension::D2,
50        // Initialize it with a beige color
51        &(css::BEIGE.to_u8_array()),
52        // Use the same encoding as the color we set
53        TextureFormat::Rgba8UnormSrgb,
54        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
55    );
56
57    // To make it extra fancy, we can set the Alpha of each pixel,
58    // so that it fades out in a circular fashion.
59    for y in 0..IMAGE_HEIGHT {
60        for x in 0..IMAGE_WIDTH {
61            let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
62            let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
63            let r = Vec2::new(x as f32, y as f32).distance(center);
64            let a = 1.0 - (r / max_radius).clamp(0.0, 1.0);
65
66            // Here we will set the A value by accessing the raw data bytes.
67            // (it is the 4th byte of each pixel, as per our `TextureFormat`)
68
69            // Find our pixel by its coordinates
70            let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();
71            // Convert our f32 to u8
72            pixel_bytes[3] = (a * u8::MAX as f32) as u8;
73        }
74    }
75
76    // Add it to Bevy's assets, so it can be used for rendering
77    // this will give us a handle we can use
78    // (to display it in a sprite, or as part of UI, etc.)
79    let handle = images.add(image);
80
81    // Create a sprite entity using our image
82    commands.spawn(Sprite::from_image(handle.clone()));
83    commands.insert_resource(MyProcGenImage(handle));
84
85    // We're seeding the PRNG here to make this example deterministic for testing purposes.
86    // This isn't strictly required in practical use unless you need your app to be deterministic.
87    let seeded_rng = ChaCha8Rng::seed_from_u64(19878367467712);
88    commands.insert_resource(SeededRng(seeded_rng));
89}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1694)

#### pub fn [clear](#method.clear)(&mut self, pixel: &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\])

Clears the content of the image with the given pixel. The image needs to be initialized on the cpu otherwise this is a noop.

This does nothing if the image data is not already initialized

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1721)

#### pub fn [get\_color\_at\_1d](#method.get_color_at_1d)(&self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Read the color of a specific pixel (1D texture).

See [`get_color_at`](../../prelude/struct.Image.html#method.get_color_at "method bevy::prelude::Image::get_color_at") for more details.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1752)

#### pub fn [get\_color\_at](#method.get_color_at)(&self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Read the color of a specific pixel (2D texture).

This function will find the raw byte data of a specific pixel and decode it into a user-friendly [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") struct for you.

Supports many of the common [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")s:

*   RGBA/BGRA 8-bit unsigned integer, both sRGB and Linear
*   16-bit and 32-bit unsigned integer
*   16-bit and 32-bit float

Be careful: as the data is converted to [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") (which uses `f32` internally), there may be issues with precision when using non-f32 [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")s. If you read a value you previously wrote using `set_color_at`, it will not match. If you are working with a 32-bit integer [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat"), the value will be inaccurate (as `f32` does not have enough bits to represent it exactly).

Single channel (R) formats are assumed to represent grayscale, so the value will be copied to all three RGB channels in the resulting [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color").

Other [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")s are unsupported, such as:

*   block-compressed formats
*   non-byte-aligned formats like 10-bit
*   signed integer formats

##### [Examples found in repository](#scraped-examples-10)[?](../../../scrape-examples-help.html)

examples/2d/cpu\_draw.rs ([line 124](../../../src/cpu_draw/cpu_draw.rs.html#124))

```rust
92fn draw(
93    my_handle: Res<MyProcGenImage>,
94    mut images: ResMut<Assets<Image>>,
95    // Used to keep track of where we are
96    mut i: Local<u32>,
97    mut draw_color: Local<Color>,
98    mut seeded_rng: ResMut<SeededRng>,
99) {
100    if *i == 0 {
101        // Generate a random color on first run.
102        *draw_color = Color::linear_rgb(
103            seeded_rng.0.random(),
104            seeded_rng.0.random(),
105            seeded_rng.0.random(),
106        );
107    }
108
109    // Get the image from Bevy's asset storage.
110    let mut image = images.get_mut(&my_handle.0).expect("Image not found");
111
112    // Compute the position of the pixel to draw.
113
114    let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
115    let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
116    let rot_speed = 0.0123;
117    let period = 0.12345;
118
119    let r = ops::sin(*i as f32 * period) * max_radius;
120    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
121    let (x, y) = (xy.x as u32, xy.y as u32);
122
123    // Get the old color of that pixel.
124    let old_color = image.get_color_at(x, y).unwrap();
125
126    // If the old color is our current color, change our drawing color.
127    let tolerance = 1.0 / 255.0;
128    if old_color.distance(&draw_color) <= tolerance {
129        *draw_color = Color::linear_rgb(
130            seeded_rng.0.random(),
131            seeded_rng.0.random(),
132            seeded_rng.0.random(),
133        );
134    }
135
136    // Set the new color, but keep old alpha value from image.
137    image
138        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
139        .unwrap();
140
141    *i += 1;
142}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1763)

#### pub fn [get\_color\_at\_3d](#method.get_color_at_3d)( &self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Read the color of a specific pixel (2D texture with layers or 3D texture).

See [`get_color_at`](../../prelude/struct.Image.html#method.get_color_at "method bevy::prelude::Image::get_color_at") for more details.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1779)

#### pub fn [set\_color\_at\_1d](#method.set_color_at_1d)( &mut self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Change the color of a specific pixel (1D texture).

See [`set_color_at`](../../prelude/struct.Image.html#method.set_color_at "method bevy::prelude::Image::set_color_at") for more details.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1811)

#### pub fn [set\_color\_at](#method.set_color_at)( &mut self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Change the color of a specific pixel (2D texture).

This function will find the raw byte data of a specific pixel and change it according to a [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") you provide. The [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") struct will be encoded into the [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image")’s [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat").

Supports many of the common [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")s:

*   RGBA/BGRA 8-bit unsigned integer, both sRGB and Linear
*   16-bit and 32-bit unsigned integer (with possibly-limited precision, as [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") uses `f32`)
*   16-bit and 32-bit float

Be careful: writing to non-f32 [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")s is lossy! The data has to be converted, so if you read it back using `get_color_at`, the `Color` you get will not equal the value you used when writing it using this function.

For RG formats, only the respective values from the linear RGB [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") will be used.

For R formats the linear RGB [`Color`](../../prelude/enum.Color.html "enum bevy::prelude::Color") will be converted to grayscale and the R channel will be the luminance.

Other [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")s are unsupported, such as:

*   block-compressed formats
*   non-byte-aligned formats like 10-bit
*   signed integer formats

##### [Examples found in repository](#scraped-examples-11)[?](../../../scrape-examples-help.html)

examples/asset/asset\_saving.rs ([line 241](../../../src/asset_saving/asset_saving.rs.html#241))

```rust
208fn try_plot(
209    event: On<TryPlot>,
210    sprite: Query<(&Sprite, &Anchor, &GlobalTransform), With<SpriteToSave>>,
211    camera: Single<(&Camera, &GlobalTransform)>,
212    texture_atlases: Res<Assets<TextureAtlasLayout>>,
213    draw_color: Res<DrawColor>,
214    mut images: ResMut<Assets<Image>>,
215) {
216    let Ok((sprite, anchor, sprite_transform)) = sprite.get(event.entity) else {
217        return;
218    };
219    let (camera, camera_transform) = camera.into_inner();
220    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, event.location.position)
221    else {
222        return;
223    };
224    let relative_to_sprite = sprite_transform
225        .affine()
226        .inverse()
227        .transform_point3(world_position.extend(0.0));
228    let Ok(pixel_space) = sprite.compute_pixel_space_point(
229        relative_to_sprite.xy(),
230        *anchor,
231        &images,
232        &texture_atlases,
233    ) else {
234        return;
235    };
236    let pixel_coordinates = pixel_space.floor().as_uvec2();
237    let mut image = images.get_mut(&sprite.image).unwrap();
238    // For an actual drawing app, you'd at least draw a line from the last point, but this is
239    // simpler.
240    image
241        .set_color_at(pixel_coordinates.x, pixel_coordinates.y, draw_color.0)
242        .unwrap();
243}
```

Hide additional examples

examples/2d/cpu\_draw.rs ([line 138](../../../src/cpu_draw/cpu_draw.rs.html#138))

```rust
92fn draw(
93    my_handle: Res<MyProcGenImage>,
94    mut images: ResMut<Assets<Image>>,
95    // Used to keep track of where we are
96    mut i: Local<u32>,
97    mut draw_color: Local<Color>,
98    mut seeded_rng: ResMut<SeededRng>,
99) {
100    if *i == 0 {
101        // Generate a random color on first run.
102        *draw_color = Color::linear_rgb(
103            seeded_rng.0.random(),
104            seeded_rng.0.random(),
105            seeded_rng.0.random(),
106        );
107    }
108
109    // Get the image from Bevy's asset storage.
110    let mut image = images.get_mut(&my_handle.0).expect("Image not found");
111
112    // Compute the position of the pixel to draw.
113
114    let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
115    let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
116    let rot_speed = 0.0123;
117    let period = 0.12345;
118
119    let r = ops::sin(*i as f32 * period) * max_radius;
120    let xy = Vec2::from_angle(*i as f32 * rot_speed) * r + center;
121    let (x, y) = (xy.x as u32, xy.y as u32);
122
123    // Get the old color of that pixel.
124    let old_color = image.get_color_at(x, y).unwrap();
125
126    // If the old color is our current color, change our drawing color.
127    let tolerance = 1.0 / 255.0;
128    if old_color.distance(&draw_color) <= tolerance {
129        *draw_color = Color::linear_rgb(
130            seeded_rng.0.random(),
131            seeded_rng.0.random(),
132            seeded_rng.0.random(),
133        );
134    }
135
136    // Set the new color, but keep old alpha value from image.
137    image
138        .set_color_at(x, y, draw_color.with_alpha(old_color.alpha()))
139        .unwrap();
140
141    *i += 1;
142}
```

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1822-1828)

#### pub fn [set\_color\_at\_3d](#method.set_color_at_3d)( &mut self, x: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), y: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), z: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), color: [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TextureAccessError](../enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Change the color of a specific pixel (2D texture with layers or 3D texture).

See [`set_color_at`](../../prelude/struct.Image.html#method.set_color_at "method bevy::prelude::Image::set_color_at") for more details.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_texture_conversion.rs.html#7)

### impl [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_texture_conversion.rs.html#9-13)

#### pub fn [from\_dynamic](#method.from_dynamic)( dyn\_img: [DynamicImage](https://docs.rs/image/0.25.9/x86_64-unknown-linux-gnu/image/images/dynimage/enum.DynamicImage.html "enum image::images::dynimage::DynamicImage"), is\_srgb: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), asset\_usage: [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages"), ) -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Converts a [`DynamicImage`](https://docs.rs/image/0.25.9/x86_64-unknown-linux-gnu/image/images/dynimage/enum.DynamicImage.html "enum image::images::dynimage::DynamicImage") to an [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image").

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image_texture_conversion.rs.html#168)

#### pub fn [try\_into\_dynamic](#method.try_into_dynamic)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[DynamicImage](https://docs.rs/image/0.25.9/x86_64-unknown-linux-gnu/image/images/dynimage/enum.DynamicImage.html "enum image::images::dynimage::DynamicImage"), [IntoDynamicImageError](../enum.IntoDynamicImageError.html "enum bevy::image::IntoDynamicImageError")\>

Convert a [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") to a [`DynamicImage`](https://docs.rs/image/0.25.9/x86_64-unknown-linux-gnu/image/images/dynimage/enum.DynamicImage.html "enum image::images::dynimage::DynamicImage"). Useful for editing image data. Not all [`TextureFormat`](../../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") are covered, therefore it will return an error if the format is unsupported. Supported formats are:

*   `TextureFormat::R8Unorm`
*   `TextureFormat::Rg8Unorm`
*   `TextureFormat::Rgba8UnormSrgb`
*   `TextureFormat::Bgra8UnormSrgb`

To convert [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") to a different format see: [`Image::convert`](../../prelude/struct.Image.html#method.convert "method bevy::prelude::Image::convert").

##### [Examples found in repository](#scraped-examples-12)[?](../../../scrape-examples-help.html)

examples/app/headless\_renderer.rs ([line 494](../../../src/headless_renderer/headless_renderer.rs.html#494))

```rust
450fn update(
451    images_to_save: Query<&ImageToSave>,
452    receiver: Res<MainWorldReceiver>,
453    mut images: ResMut<Assets<Image>>,
454    mut scene_controller: ResMut<SceneController>,
455    mut app_exit_writer: MessageWriter<AppExit>,
456    mut file_number: Local<u32>,
457) {
458    if let SceneState::Render(n) = scene_controller.state {
459        if n < 1 {
460            // We don't want to block the main world on this,
461            // so we use try_recv which attempts to receive without blocking
462            let mut image_data = Vec::new();
463            while let Ok(data) = receiver.try_recv() {
464                // image generation could be faster than saving to fs,
465                // that's why use only last of them
466                image_data = data;
467            }
468            if !image_data.is_empty() {
469                for image in images_to_save.iter() {
470                    // Fill correct data from channel to image
471                    let mut img_bytes = images.get_mut(image.id()).unwrap();
472
473                    // We need to ensure that this works regardless of the image dimensions
474                    // If the image became wider when copying from the texture to the buffer,
475                    // then the data is reduced to its original size when copying from the buffer to the image.
476                    let row_bytes = img_bytes.width() as usize
477                        * img_bytes.texture_descriptor.format.pixel_size().unwrap();
478                    let aligned_row_bytes = RenderDevice::align_copy_bytes_per_row(row_bytes);
479                    if row_bytes == aligned_row_bytes {
480                        img_bytes.data.as_mut().unwrap().clone_from(&image_data);
481                    } else {
482                        // shrink data to original image size
483                        img_bytes.data = Some(
484                            image_data
485                                .chunks(aligned_row_bytes)
486                                .take(img_bytes.height() as usize)
487                                .flat_map(|row| &row[..row_bytes.min(row.len())])
488                                .cloned()
489                                .collect(),
490                        );
491                    }
492
493                    // Create RGBA Image Buffer
494                    let img = match img_bytes.clone().try_into_dynamic() {
495                        Ok(img) => img.to_rgba8(),
496                        Err(e) => panic!("Failed to create image buffer {e:?}"),
497                    };
498
499                    // Prepare directory for images, test_images in bevy folder is used here for example
500                    // You should choose the path depending on your needs
501                    let images_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_images");
502                    info!("Saving image to: {images_dir:?}");
503                    std::fs::create_dir_all(&images_dir).unwrap();
504
505                    // Choose filename starting from 000.png
506                    let image_path = images_dir.join(format!("{:03}.png", file_number.deref()));
507                    *file_number.deref_mut() += 1;
508
509                    // Finally saving image to file, this heavy blocking operation is kept here
510                    // for example simplicity, but in real app you should move it to a separate task
511                    if let Err(e) = img.save(image_path) {
512                        panic!("Failed to save image: {e}");
513                    };
514                }
515                if scene_controller.single_image {
516                    app_exit_writer.write(AppExit::Success);
517                }
518            }
519        } else {
520            // clears channel for skipped frames
521            while receiver.try_recv().is_ok() {}
522            scene_controller.state = SceneState::Render(n - 1);
523        }
524    }
525}
```

Hide additional examples

examples/remote/integration\_test.rs ([line 83](../../../src/integration_test/integration_test.rs.html#83))

```rust
29fn main() -> AnyhowResult<()> {
30    let url = format!("http://{DEFAULT_ADDR}:{DEFAULT_PORT}/");
31
32    // Step 1: Take a screenshot via BRP
33    // The window must be visible (not fully occluded) for the GPU to render content
34    // If the window is hidden, the screenshot will be black
35    println!("Spawning Screenshot entity...");
36    let spawn_response = brp_request(
37        &url,
38        BRP_SPAWN_ENTITY_METHOD,
39        1,
40        &BrpSpawnEntityParams {
41            components: HashMap::from([(
42                type_name::<Screenshot>().to_string(),
43                serde_json::json!({"Window": "Primary"}),
44            )]),
45        },
46    )?;
47    let screenshot_entity = &spawn_response["result"]["entity"];
48
49    println!("Observing ScreenshotCaptured on entity {screenshot_entity}...");
50    let observe_response = ureq::post(&url).send_json(BrpRequest {
51        method: BRP_OBSERVE_METHOD.to_string(),
52        id: Some(serde_json::to_value(2)?),
53        params: Some(serde_json::to_value(BrpObserveParams {
54            event: type_name::<ScreenshotCaptured>().to_string(),
55            entity: Some(serde_json::from_value(screenshot_entity.clone())?),
56        })?),
57    })?;
58
59    println!("Waiting for screenshot capture...");
60    let reader = std::io::BufReader::new(observe_response.into_body().into_reader());
61    for line in reader.lines() {
62        let line = line?;
63        if let Some(json_str) = line.strip_prefix("data: ") {
64            let response: serde_json::Value = serde_json::from_str(json_str)?;
65            if let Some(error) = response.get("error") {
66                anyhow::bail!("Observe error: {error}");
67            }
68            if let Some(result) = response.get("result") {
69                let events = result.as_array().expect("Expected events array");
70                let event = &events[0];
71
72                let image_data = &event["image"];
73                let width = image_data["texture_descriptor"]["size"]["width"]
74                    .as_u64()
75                    .unwrap();
76                let height = image_data["texture_descriptor"]["size"]["height"]
77                    .as_u64()
78                    .unwrap();
79                println!("Screenshot captured! Image size: {width}x{height}");
80
81                let image: bevy::image::Image = serde_json::from_value(image_data.clone())?;
82                let dyn_img = image
83                    .try_into_dynamic()
84                    .expect("Failed to convert screenshot to dynamic image");
85                let path = "screenshot.png";
86                dyn_img.to_rgb8().save(path)?;
87                println!("Screenshot saved to {path}");
88                break;
89            }
90        }
91    }
92
93    // Step 2: Find the button entity, and its global transform
94    println!("Querying for button entity...");
95    let button_query = brp_request(
96        &url,
97        BRP_QUERY_METHOD,
98        3,
99        &BrpQueryParams {
100            data: BrpQuery {
101                components: vec![type_name::<UiGlobalTransform>().to_string()],
102                option: ComponentSelector::default(),
103                has: Vec::default(),
104            },
105            strict: false,
106            filter: BrpQueryFilter {
107                with: vec![type_name::<Button>().to_string()],
108                without: Vec::default(),
109            },
110        },
111    )?;
112
113    let button_result = button_query["result"]
114        .as_array()
115        .expect("Expected result array");
116    let button = &button_result[0];
117
118    // UiGlobalTransform wraps an Affine2, serialized as a flat array:
119    // [_, _, _, _, translation_x, translation_y]
120    // The translation gives the node's center in physical pixels.
121    let transform = &button["components"][type_name::<UiGlobalTransform>()];
122    let transform_arr = transform.as_array().expect("Expected transform array");
123    let phys_x = transform_arr[4].as_f64().unwrap();
124    let phys_y = transform_arr[5].as_f64().unwrap();
125    println!("Found button at physical ({phys_x}, {phys_y})");
126
127    // Step 3: Find the window entity and scale factor
128    println!("Querying for window entity...");
129    let window_query = brp_request(
130        &url,
131        BRP_QUERY_METHOD,
132        4,
133        &BrpQueryParams {
134            data: BrpQuery {
135                components: vec![type_name::<Window>().to_string()],
136                option: ComponentSelector::default(),
137                has: Vec::default(),
138            },
139            strict: false,
140            filter: BrpQueryFilter::default(),
141        },
142    )?;
143
144    let window_result = window_query["result"]
145        .as_array()
146        .expect("Expected result array");
147    let window = &window_result[0];
148    let window_entity = &window["entity"];
149    let window_data = &window["components"][type_name::<Window>()];
150    let scale_factor = window_data["resolution"]["scale_factor"].as_f64().unwrap();
151    println!("Found window entity: {window_entity}, scale_factor: {scale_factor}");
152
153    // Step 4: Convert button center from physical to logical pixels
154    let logical_x = phys_x / scale_factor;
155    let logical_y = phys_y / scale_factor;
156    println!("Clicking at logical position: ({logical_x}, {logical_y})");
157
158    // Step 5: Send CursorMoved via WindowEvent message
159    // This lets the picking system know where the pointer is.
160    println!("Sending CursorMoved message...");
161    brp_request(
162        &url,
163        BRP_WRITE_MESSAGE_METHOD,
164        5,
165        &BrpWriteMessageParams {
166            message: type_name::<WindowEvent>().to_string(),
167            value: Some(serde_json::json!({
168                "CursorMoved": {
169                    "window": window_entity,
170                    "position": [logical_x, logical_y],
171                    "delta": null
172                }
173            })),
174        },
175    )?;
176
177    // Step 6: Send MouseButtonInput Pressed + Released via WindowEvent messages.
178    // The picking system needs both press and release to generate a Pointer<Click>.
179    println!("Sending mouse press...");
180    brp_request(
181        &url,
182        BRP_WRITE_MESSAGE_METHOD,
183        6,
184        &BrpWriteMessageParams {
185            message: type_name::<WindowEvent>().to_string(),
186            value: Some(serde_json::json!({
187                "MouseButtonInput": {
188                    "button": "Left",
189                    "state": "Pressed",
190                    "window": window_entity,
191                }
192            })),
193        },
194    )?;
195
196    println!("Sending mouse release...");
197    brp_request(
198        &url,
199        BRP_WRITE_MESSAGE_METHOD,
200        7,
201        &BrpWriteMessageParams {
202            message: type_name::<WindowEvent>().to_string(),
203            value: Some(serde_json::json!({
204                "MouseButtonInput": {
205                    "button": "Left",
206                    "state": "Released",
207                    "window": window_entity,
208                }
209            })),
210        },
211    )?;
212
213    Ok(())
214}
```

## Trait Implementations

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1080)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#1082)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

default is a 1x1x1 all ‘1.0’ texture

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#659)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#660)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<D>( deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image"), <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#82)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(\_registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [to\_dynamic](../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#609)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#609)

#### fn [reflect\_clone](../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#343)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, \_value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#350)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, \_value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#652)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#653)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<S>( &self, serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

### impl [VisitAssetDependencies](../../asset/trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#605)

#### fn [visit\_dependencies](../../asset/trait.VisitAssetDependencies.html#tymethod.visit_dependencies)(&self, visit: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([UntypedAssetId](../../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")))

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#157)

### impl<T> [DynamicTypePath](../../reflect/trait.DynamicTypePath.html "trait bevy::reflect::DynamicTypePath") for T

where T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#159)

#### fn [reflect\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::type_path`](../../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#164)

#### fn [reflect\_short\_type\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_short_type_path)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

See [`TypePath::short_type_path`](../../prelude/trait.TypePath.html#tymethod.short_type_path "associated function bevy::prelude::TypePath::short_type_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#169)

#### fn [reflect\_type\_ident](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_type_ident)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::type_ident`](../../prelude/trait.TypePath.html#method.type_ident "associated function bevy::prelude::TypePath::type_ident").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#174)

#### fn [reflect\_crate\_name](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_crate_name)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::crate_name`](../../prelude/trait.TypePath.html#method.crate_name "associated function bevy::prelude::TypePath::crate_name").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#179)

#### fn [reflect\_module\_path](../../reflect/trait.DynamicTypePath.html#tymethod.reflect_module_path)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

See [`TypePath::module_path`](../../prelude/trait.TypePath.html#method.module_path "associated function bevy::prelude::TypePath::module_path").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#165)

### impl<T> [DynamicTyped](../../reflect/trait.DynamicTyped.html "trait bevy::reflect::DynamicTyped") for T

where T: [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_info.rs.html#167)

#### fn [reflect\_type\_info](../../reflect/trait.DynamicTyped.html#tymethod.reflect_type_info)(&self) -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

See [`Typed::type_info`](../../reflect/trait.Typed.html#tymethod.type_info "associated function bevy::reflect::Typed::type_info").

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

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](../../prelude/trait.FromTemplate.html#associatedtype.Template) = T

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#295)

### impl<T> [GetPath](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#256)

#### fn [reflect\_path](../../prelude/trait.GetPath.html#method.reflect_path)<'p>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#264-267)

#### fn [reflect\_path\_mut](../../prelude/trait.GetPath.html#method.reflect_path_mut)<'p>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

Returns a mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.reflect_path_mut)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#278)

#### fn [path](../../prelude/trait.GetPath.html#method.path)<'p, T>( &self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#289)

#### fn [path\_mut](../../prelude/trait.GetPath.html#method.path_mut)<'p, T>( &mut self, path: impl [ReflectPath](../../prelude/trait.ReflectPath.html "trait bevy::prelude::ReflectPath")<'p>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ReflectPathError](../../reflect/enum.ReflectPathError.html "enum bevy::reflect::ReflectPathError")<'p>>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a statically typed mutable reference to the value specified by `path`. [Read more](../../prelude/trait.GetPath.html#method.path_mut)

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#311)

### impl<G> [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate") for G

where G: [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#312)

#### type [Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template) = <G as [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate")\>::[Template](../../prelude/trait.FromTemplate.html#associatedtype.Template "type bevy::prelude::FromTemplate::Template")

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") that will be patched.

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#313-315)

#### fn [patch](../../prelude/trait.PatchFromTemplate.html#tymethod.patch)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template")\>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut <G as [PatchFromTemplate](../../prelude/trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")\>::[Template](../../prelude/trait.PatchFromTemplate.html#associatedtype.Template "type bevy::prelude::PatchFromTemplate::Template"), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func`, and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#327)

### impl<T> [PatchTemplate](../../prelude/trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#328-330)

#### fn [patch\_template](../../prelude/trait.PatchTemplate.html#tymethod.patch_template)<F>(func: F) -> [TemplatePatch](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch")<F, T>

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), &mut [ResolveContext](../../scene/struct.ResolveContext.html "struct bevy::scene::ResolveContext")<'\_>),

Takes a “patch function” `func` that patches this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"), and turns it into a [`TemplatePatch`](../../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch").

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

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](../../prelude/trait.Template.html#associatedtype.Output) = T

The type of value produced by this [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#393)

#### fn [build\_template](../../prelude/trait.Template.html#tymethod.build_template)( &self, \_context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](../../prelude/trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#397)

#### fn [clone\_template](../../prelude/trait.Template.html#tymethod.clone_template)(&self) -> T

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../../prelude/trait.ToOwned.html#method.clone_into)

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

### impl<T> [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}