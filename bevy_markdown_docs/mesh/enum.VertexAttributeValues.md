[bevy](../index.html)::[mesh](index.html)

# Enum VertexAttributeValues 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#238)

```rust
pub enum VertexAttributeValues {
    Uint8(Vec<u8>),
    Uint8x2(Vec<[u8; 2]>),
    Uint8x4(Vec<[u8; 4]>),
    Sint8(Vec<i8>),
    Sint8x2(Vec<[i8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Unorm8(Vec<u8>),
    Unorm8x2(Vec<[u8; 2]>),
    Unorm8x4(Vec<[u8; 4]>),
    Snorm8(Vec<i8>),
    Snorm8x2(Vec<[i8; 2]>),
    Snorm8x4(Vec<[i8; 4]>),
    Uint16(Vec<u16>),
    Uint16x2(Vec<[u16; 2]>),
    Uint16x4(Vec<[u16; 4]>),
    Sint16(Vec<i16>),
    Sint16x2(Vec<[i16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Unorm16(Vec<u16>),
    Unorm16x2(Vec<[u16; 2]>),
    Unorm16x4(Vec<[u16; 4]>),
    Snorm16(Vec<i16>),
    Snorm16x2(Vec<[i16; 2]>),
    Snorm16x4(Vec<[i16; 4]>),
    Float16(Vec<f16>),
    Float16x2(Vec<[f16; 2]>),
    Float16x4(Vec<[f16; 4]>),
    Float32(Vec<f32>),
    Float32x2(Vec<[f32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Uint32(Vec<u32>),
    Uint32x2(Vec<[u32; 2]>),
    Uint32x3(Vec<[u32; 3]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint32(Vec<i32>),
    Sint32x2(Vec<[i32; 2]>),
    Sint32x3(Vec<[i32; 3]>),
    Sint32x4(Vec<[i32; 4]>),
    Float64(Vec<f64>),
    Float64x2(Vec<[f64; 2]>),
    Float64x3(Vec<[f64; 3]>),
    Float64x4(Vec<[f64; 4]>),
    Unorm10_10_10_2(Vec<u32>),
    Unorm8x4Bgra(Vec<[u8; 4]>),
}
```

Contains an array where each entry describes a property of a single vertex. Matches the [`VertexFormats`](enum.VertexFormat.html "enum bevy::mesh::VertexFormat").

## Variants

### Uint8([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>)

One unsigned byte (u8). `u32` in shaders.

### Uint8x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two unsigned bytes (u8). `vec2<u32>` in shaders.

### Uint8x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four unsigned bytes (u8). `vec4<u32>` in shaders.

### Sint8([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>)

One signed byte (i8). `i32` in shaders.

### Sint8x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two signed bytes (i8). `vec2<i32>` in shaders.

### Sint8x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four signed bytes (i8). `vec4<i32>` in shaders.

### Unorm8([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>)

One unsigned byte (u8). \[0, 255\] converted to float \[0, 1\] `f32` in shaders.

### Unorm8x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two unsigned bytes (u8). \[0, 255\] converted to float \[0, 1\] `vec2<f32>` in shaders.

### Unorm8x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four unsigned bytes (u8). \[0, 255\] converted to float \[0, 1\] `vec4<f32>` in shaders.

### Snorm8([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>)

One signed byte (i8). \[−127, 127\] converted to float \[−1, 1\] `f32` in shaders.

### Snorm8x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two signed bytes (i8). \[−127, 127\] converted to float \[−1, 1\] `vec2<f32>` in shaders.

### Snorm8x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four signed bytes (i8). \[−127, 127\] converted to float \[−1, 1\] `vec4<f32>` in shaders.

### Uint16([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>)

One unsigned short (u16). `u32` in shaders.

### Uint16x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two unsigned shorts (u16). `vec2<u32>` in shaders.

### Uint16x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four unsigned shorts (u16). `vec4<u32>` in shaders.

### Sint16([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>)

One signed short (i16). `i32` in shaders.

### Sint16x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two signed shorts (i16). `vec2<i32>` in shaders.

### Sint16x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four signed shorts (i16). `vec4<i32>` in shaders.

### Unorm16([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>)

One unsigned short (u16). \[0, 65535\] converted to float \[0, 1\] `f32` in shaders.

### Unorm16x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two unsigned shorts (u16). \[0, 65535\] converted to float \[0, 1\] `vec2<f32>` in shaders.

### Unorm16x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four unsigned shorts (u16). \[0, 65535\] converted to float \[0, 1\] `vec4<f32>` in shaders.

### Snorm16([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>)

One signed short (i16). \[−32767, 32767\] converted to float \[−1, 1\] `f32` in shaders.

### Snorm16x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two signed shorts (i16). \[−32767, 32767\] converted to float \[−1, 1\] `vec2<f32>` in shaders.

### Snorm16x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four signed shorts (i16). \[−32767, 32767\] converted to float \[−1, 1\] `vec4<f32>` in shaders.

### Float16([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f16](https://docs.rs/half/2.7.1/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html "struct half::binary16::f16")\>)

One half-precision float (no Rust equiv). `f32` in shaders.

### Float16x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f16](https://docs.rs/half/2.7.1/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html "struct half::binary16::f16"); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two half-precision floats (no Rust equiv). `vec2<f32>` in shaders.

### Float16x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f16](https://docs.rs/half/2.7.1/x86_64-unknown-linux-gnu/half/binary16/struct.f16.html "struct half::binary16::f16"); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four half-precision floats (no Rust equiv). `vec4<f32>` in shaders.

### Float32([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>)

One single-precision float (f32). `f32` in shaders.

### Float32x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two single-precision floats (f32). `vec2<f32>` in shaders.

### Float32x3([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Three single-precision floats (f32). `vec3<f32>` in shaders.

### Float32x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four single-precision floats (f32). `vec4<f32>` in shaders.

### Uint32([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>)

One unsigned int (u32). `u32` in shaders.

### Uint32x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two unsigned ints (u32). `vec2<u32>` in shaders.

### Uint32x3([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Three unsigned ints (u32). `vec3<u32>` in shaders.

### Uint32x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four unsigned ints (u32). `vec4<u32>` in shaders.

### Sint32([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>)

One signed int (i32). `i32` in shaders.

### Sint32x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two signed ints (i32). `vec2<i32>` in shaders.

### Sint32x3([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Three signed ints (i32). `vec3<i32>` in shaders.

### Sint32x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four signed ints (i32). `vec4<i32>` in shaders.

### Float64([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>)

One double-precision float (f64). `f32` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`](../render/render_resource/struct.WgpuFeatures.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT "associated constant bevy::render::render_resource::WgpuFeatures::VERTEX_ATTRIBUTE_64BIT").

### Float64x2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Two double-precision floats (f64). `vec2<f32>` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`](../render/render_resource/struct.WgpuFeatures.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT "associated constant bevy::render::render_resource::WgpuFeatures::VERTEX_ATTRIBUTE_64BIT").

### Float64x3([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Three double-precision floats (f64). `vec3<f32>` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`](../render/render_resource/struct.WgpuFeatures.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT "associated constant bevy::render::render_resource::WgpuFeatures::VERTEX_ATTRIBUTE_64BIT").

### Float64x4([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four double-precision floats (f64). `vec4<f32>` in shaders. Requires [`wgpu_types::Features::VERTEX_ATTRIBUTE_64BIT`](../render/render_resource/struct.WgpuFeatures.html#associatedconstant.VERTEX_ATTRIBUTE_64BIT "associated constant bevy::render::render_resource::WgpuFeatures::VERTEX_ATTRIBUTE_64BIT").

### Unorm10\_10\_10\_2([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>)

Three unsigned 10-bit integers and one 2-bit integer, packed into a 32-bit integer (u32). \[0, 1023\] and \[0, 3\] converted to float \[0, 1\] `vec4<f32>` in shaders.

### Unorm8x4Bgra([Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>)

Four unsigned 8-bit integers (u8) in BGRA. \[0, 255\] converted to float \[0, 1\] `vec4<f32>` RGBA in shaders.

## Implementations

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

### impl [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

#### pub fn [enum\_variant\_index](#method.enum_variant_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

#### pub fn [enum\_variant\_name](#method.enum_variant_name)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#331)

### impl [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#389)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of vertices in this [`VertexAttributeValues`](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"). For a single mesh, all of the [`VertexAttributeValues`](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues") must have the same length.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#440)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if there are no vertices in this [`VertexAttributeValues`](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues").

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#445)

#### pub fn [as\_float3](#method.as_float3)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&\[\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]\]>

Returns the values as float triples if possible.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/occlusion\_culling.rs ([line 294](../../src/occlusion_culling/occlusion_culling.rs.html#294))

```rust
254fn spawn_small_cubes(
255    commands: &mut Commands,
256    meshes: &mut Assets<Mesh>,
257    materials: &mut Assets<StandardMaterial>,
258) {
259    // Add the cube mesh.
260    let small_cube = meshes.add(Cuboid::new(
261        SMALL_CUBE_SIZE,
262        SMALL_CUBE_SIZE,
263        SMALL_CUBE_SIZE,
264    ));
265
266    // Add the cube material.
267    let small_cube_material = materials.add(StandardMaterial {
268        base_color: SILVER.into(),
269        ..default()
270    });
271
272    // Create the entity that the small cubes will be parented to. This is the
273    // entity that we rotate.
274    let sphere_parent = commands
275        .spawn(Transform::from_translation(Vec3::ZERO))
276        .insert(Visibility::default())
277        .insert(SphereParent)
278        .id();
279
280    // Now we have to figure out where to place the cubes. To do that, we create
281    // a sphere mesh, but we don't add it to the scene. Instead, we inspect the
282    // sphere mesh to find the positions of its vertices, and spawn a small cube
283    // at each one. That way, we end up with a bunch of cubes arranged in a
284    // spherical shape.
285
286    // Create the sphere mesh, and extract the positions of its vertices.
287    let sphere = Sphere::new(OUTER_RADIUS)
288        .mesh()
289        .ico(OUTER_SUBDIVISION_COUNT)
290        .unwrap();
291    let sphere_positions = sphere.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
292
293    // At each vertex, create a small cube.
294    for sphere_position in sphere_positions.as_float3().unwrap() {
295        let sphere_position = Vec3::from_slice(sphere_position);
296        let small_cube = commands
297            .spawn(Mesh3d(small_cube.clone()))
298            .insert(MeshMaterial3d(small_cube_material.clone()))
299            .insert(Transform::from_translation(sphere_position))
300            .id();
301        commands.entity(sphere_parent).add_child(small_cube);
302    }
303}
```

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#459)

#### pub fn [get\_bytes](#method.get_bytes)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Flattens the [`VertexAttributeValues`](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues") into a sequence of bytes. This is useful for serialization and sending to the GPU.

## Trait Implementations

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#237)

### impl<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#237)

#### fn [deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)<\_\_D>( \_\_deserializer: \_\_D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), <\_\_D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where \_\_D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html#tymethod.deserialize)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#841)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [VertexFormat](enum.VertexFormat.html "enum bevy::mesh::VertexFormat")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#842)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(values: &[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")) -> [VertexFormat](enum.VertexFormat.html "enum bevy::mesh::VertexFormat")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#81)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#81)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#83)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#83)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#85)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#85)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#89)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#89)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#91)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#91)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#93)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#93)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#72)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#72)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#74)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#74)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#75)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#75)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#77)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#77)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#71)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#71)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#73)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#73)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#76)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#76)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#80)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#80)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#82)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#82)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#84)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#84)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#88)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#88)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#90)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#90)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#92)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#92)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#70)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#70)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#79)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#79)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#87)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>> for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#87)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(vec: [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>) -> [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

### impl [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

#### fn [eq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq)(&self, other: &[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/143800 "Tracking issue for const_cmp")) · [Source](https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#263)

#### fn [ne](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne)(&self, other: [&Rhs](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#237)

### impl [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#237)

#### fn [serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)<\_\_S>( &self, \_\_serializer: \_\_S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <\_\_S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where \_\_S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html#tymethod.serialize)

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#236)

### impl [StructuralPartialEq](https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#127)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#127)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#127)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#128)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#128)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#128)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#129)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#129)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#129)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#130)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#130)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#130)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#131)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#131)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#131)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#132)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#132)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#132)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#133)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#133)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#133)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#134)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#134)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#134)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#136)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#136)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#136)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#137)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#137)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#137)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#138)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#138)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#138)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#139)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#139)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#139)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#140)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#140)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#140)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#141)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#141)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#141)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#142)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#142)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#142)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#144)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#144)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#144)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#145)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#145)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#145)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#146)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#146)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#146)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#147)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#147)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#147)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [3](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#148)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#148)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#148)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#149)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#149)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#149)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#150)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#150)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#150)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")\> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#152)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#152)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#152)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#153)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#153)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#153)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#155)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#155)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#155)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#156)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#156)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#156)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#158)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#158)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#158)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#159)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#159)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#159)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#161)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#161)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#161)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [2](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#162)

### impl [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\> for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#162)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = FromVertexAttributeError

The type returned in the event of a conversion error.

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/conversions.rs.html#162)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)( value: [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>, <[Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<\[[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html); [4](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")\>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

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

[Source](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/src/serde_core/de/mod.rs.html#633)

### impl<T> [DeserializeOwned](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.DeserializeOwned.html "trait serde_core::de::DeserializeOwned") for T

where T: for<'de> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'de>,

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#233-235)

### impl<T> [Serialize](../reflect/erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize") for T

where T: [Serialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#237)

#### fn [erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.erased_serialize)(&self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](../reflect/erased_serde/struct.Error.html "struct bevy::reflect::erased_serde::Error")\>

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#245)

#### fn [do\_erased\_serialize](../reflect/erased_serde/trait.Serialize.html#tymethod.do_erased_serialize)( &self, serializer: &mut dyn [Serializer](../reflect/erased_serde/trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), ErrorImpl>

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

{"&\[u8\]":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}