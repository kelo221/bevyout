[bevy](../../../index.html)::[math](../../index.html)::[sampling](../index.html)::[shape\_sampling](index.html)

# Trait ShapeSample 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#54)

```rust
pub trait ShapeSample {
    type Output;

    // Required methods
    fn sample_interior<R>(&self, rng: &mut R) -> Self::Output
       where R: RngExt + ?Sized;
    fn sample_boundary<R>(&self, rng: &mut R) -> Self::Output
       where R: RngExt + ?Sized;

    // Provided methods
    fn interior_dist(self) -> impl Distribution<Self::Output>
       where Self: Sized { ... }
    fn boundary_dist(self) -> impl Distribution<Self::Output>
       where Self: Sized { ... }
}
```

Available on **crate feature `rand`** only.

Exposes methods to uniformly sample a variety of primitive shapes.

## Required Associated Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#56)

#### type [Output](#associatedtype.Output)

The type of vector returned by the sample methods, [`Vec2`](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2") for 2D shapes and [`Vec3`](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") for 3D shapes.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#70)

#### fn [sample\_interior](#tymethod.sample_interior)<R>(&self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> Self::[Output](../../../prelude/trait.ShapeSample.html#associatedtype.Output "type bevy::prelude::ShapeSample::Output")

where R: [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample a point from inside the area/volume of this shape, centered on 0.

Shapes like [`Cylinder`](../../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder"), [`Capsule2d`](../../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d") and [`Capsule3d`](../../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d") are oriented along the y-axis.

##### Example

```rust
let square = Rectangle::new(2.0, 2.0);

// Returns a Vec2 with both x and y between -1 and 1.
println!("{}", square.sample_interior(&mut rand::rng()));
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#85)

#### fn [sample\_boundary](#tymethod.sample_boundary)<R>(&self, rng: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> Self::[Output](../../../prelude/trait.ShapeSample.html#associatedtype.Output "type bevy::prelude::ShapeSample::Output")

where R: [RngExt](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Uniformly sample a point from the surface of this shape, centered on 0.

Shapes like [`Cylinder`](../../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder"), [`Capsule2d`](../../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d") and [`Capsule3d`](../../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d") are oriented along the y-axis.

##### Example

```rust
let square = Rectangle::new(2.0, 2.0);

// Returns a Vec2 where one of the coordinates is at ±1,
//  and the other is somewhere between -1 and 1.
println!("{}", square.sample_boundary(&mut rand::rng()));
```

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#102-104)

#### fn [interior\_dist](#method.interior_dist)(self) -> impl [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<Self::[Output](../../../prelude/trait.ShapeSample.html#associatedtype.Output "type bevy::prelude::ShapeSample::Output")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Extract a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution") whose samples are points of this shape’s interior, taken uniformly.

##### Example

```rust
let square = Rectangle::new(2.0, 2.0);
let rng = rand::rng();

// Iterate over points randomly drawn from `square`'s interior:
for random_val in square.interior_dist().sample_iter(rng).take(5) {
    println!("{}", random_val);
}
```

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/math/random\_sampling.rs ([line 188](../../../../src/random_sampling/random_sampling.rs.html#188))

```rust
137fn handle_keypress(
138    mut commands: Commands,
139    keyboard: Res<ButtonInput<KeyCode>>,
140    mut mode: ResMut<Mode>,
141    shape: Res<SampledShape>,
142    mut random_source: ResMut<RandomSource>,
143    sample_mesh: Res<PointMesh>,
144    sample_material: Res<PointMaterial>,
145    samples: Query<Entity, With<SamplePoint>>,
146) {
147    // R => restart, deleting all samples
148    if keyboard.just_pressed(KeyCode::KeyR) {
149        for entity in &samples {
150            commands.entity(entity).despawn();
151        }
152    }
153
154    // S => sample once
155    if keyboard.just_pressed(KeyCode::KeyS) {
156        let rng = &mut random_source.0;
157
158        // Get a single random Vec3:
159        let sample: Vec3 = match *mode {
160            Mode::Interior => shape.0.sample_interior(rng),
161            Mode::Boundary => shape.0.sample_boundary(rng),
162        };
163
164        // Spawn a sphere at the random location:
165        commands.spawn((
166            Mesh3d(sample_mesh.0.clone()),
167            MeshMaterial3d(sample_material.0.clone()),
168            Transform::from_translation(sample),
169            SamplePoint,
170        ));
171
172        // NOTE: The point is inside the cube created at setup just because of how the
173        // scene is constructed; in general, you would want to use something like
174        // `cube_transform.transform_point(sample)` to get the position of where the sample
175        // would be after adjusting for the position and orientation of the cube.
176        //
177        // If the spawned point also needed to follow the position of the cube as it moved,
178        // then making it a child entity of the cube would be a good approach.
179    }
180
181    // D => generate many samples
182    if keyboard.just_pressed(KeyCode::KeyD) {
183        let mut rng = &mut random_source.0;
184
185        // Get 100 random Vec3s:
186        let samples: Vec<Vec3> = match *mode {
187            Mode::Interior => {
188                let dist = shape.0.interior_dist();
189                dist.sample_iter(&mut rng).take(100).collect()
190            }
191            Mode::Boundary => {
192                let dist = shape.0.boundary_dist();
193                dist.sample_iter(&mut rng).take(100).collect()
194            }
195        };
196
197        // For each sample point, spawn a sphere:
198        for sample in samples {
199            commands.spawn((
200                Mesh3d(sample_mesh.0.clone()),
201                MeshMaterial3d(sample_material.0.clone()),
202                Transform::from_translation(sample),
203                SamplePoint,
204            ));
205        }
206
207        // NOTE: See the previous note above regarding the positioning of these samples
208        // relative to the transform of the cube containing them.
209    }
210
211    // M => toggle mode between interior and boundary.
212    if keyboard.just_pressed(KeyCode::KeyM) {
213        match *mode {
214            Mode::Interior => *mode = Mode::Boundary,
215            Mode::Boundary => *mode = Mode::Interior,
216        }
217    }
218}
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#124-126)

#### fn [boundary\_dist](#method.boundary_dist)(self) -> impl [Distribution](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution")<Self::[Output](../../../prelude/trait.ShapeSample.html#associatedtype.Output "type bevy::prelude::ShapeSample::Output")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Extract a [`Distribution`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/distribution/trait.Distribution.html "trait rand::distr::distribution::Distribution") whose samples are points of this shape’s boundary, taken uniformly.

##### Example

```rust
let square = Rectangle::new(2.0, 2.0);
let rng = rand::rng();

// Iterate over points randomly drawn from `square`'s boundary:
for random_val in square.boundary_dist().sample_iter(rng).take(5) {
    println!("{}", random_val);
}
```

##### [Examples found in repository](#scraped-examples-1)[?](../../../../scrape-examples-help.html)

examples/math/random\_sampling.rs ([line 192](../../../../src/random_sampling/random_sampling.rs.html#192))

```rust
137fn handle_keypress(
138    mut commands: Commands,
139    keyboard: Res<ButtonInput<KeyCode>>,
140    mut mode: ResMut<Mode>,
141    shape: Res<SampledShape>,
142    mut random_source: ResMut<RandomSource>,
143    sample_mesh: Res<PointMesh>,
144    sample_material: Res<PointMaterial>,
145    samples: Query<Entity, With<SamplePoint>>,
146) {
147    // R => restart, deleting all samples
148    if keyboard.just_pressed(KeyCode::KeyR) {
149        for entity in &samples {
150            commands.entity(entity).despawn();
151        }
152    }
153
154    // S => sample once
155    if keyboard.just_pressed(KeyCode::KeyS) {
156        let rng = &mut random_source.0;
157
158        // Get a single random Vec3:
159        let sample: Vec3 = match *mode {
160            Mode::Interior => shape.0.sample_interior(rng),
161            Mode::Boundary => shape.0.sample_boundary(rng),
162        };
163
164        // Spawn a sphere at the random location:
165        commands.spawn((
166            Mesh3d(sample_mesh.0.clone()),
167            MeshMaterial3d(sample_material.0.clone()),
168            Transform::from_translation(sample),
169            SamplePoint,
170        ));
171
172        // NOTE: The point is inside the cube created at setup just because of how the
173        // scene is constructed; in general, you would want to use something like
174        // `cube_transform.transform_point(sample)` to get the position of where the sample
175        // would be after adjusting for the position and orientation of the cube.
176        //
177        // If the spawned point also needed to follow the position of the cube as it moved,
178        // then making it a child entity of the cube would be a good approach.
179    }
180
181    // D => generate many samples
182    if keyboard.just_pressed(KeyCode::KeyD) {
183        let mut rng = &mut random_source.0;
184
185        // Get 100 random Vec3s:
186        let samples: Vec<Vec3> = match *mode {
187            Mode::Interior => {
188                let dist = shape.0.interior_dist();
189                dist.sample_iter(&mut rng).take(100).collect()
190            }
191            Mode::Boundary => {
192                let dist = shape.0.boundary_dist();
193                dist.sample_iter(&mut rng).take(100).collect()
194            }
195        };
196
197        // For each sample point, spawn a sphere:
198        for sample in samples {
199            commands.spawn((
200                Mesh3d(sample_mesh.0.clone()),
201                MeshMaterial3d(sample_material.0.clone()),
202                Transform::from_translation(sample),
203                SamplePoint,
204            ));
205        }
206
207        // NOTE: See the previous note above regarding the positioning of these samples
208        // relative to the transform of the cube containing them.
209    }
210
211    // M => toggle mode between interior and boundary.
212    if keyboard.just_pressed(KeyCode::KeyM) {
213        match *mode {
214            Mode::Interior => *mode = Mode::Boundary,
215            Mode::Boundary => *mode = Mode::Interior,
216        }
217    }
218}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#225)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Annulus](../../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#226)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#498)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Capsule2d](../../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#499)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#551)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Capsule3d](../../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#552)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#154)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Circle](../../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#155)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#173)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [CircularSector](../../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#174)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#301)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Cuboid](../../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#302)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#467)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Cylinder](../../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#468)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#276)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Rectangle](../../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#277)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#256)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Rhombus](../../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#257)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#210)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Sphere](../../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#211)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#410)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Tetrahedron](../../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#411)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#386)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Triangle2d](../../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#387)

#### type [Output](#associatedtype.Output) = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#398)

### impl [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Triangle3d](../../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#399)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#601)

### impl<P> [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample") for [Extrusion](../../../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion")<P>

where P: [Primitive2d](../../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Measured2d](../../../prelude/trait.Measured2d.html "trait bevy::prelude::Measured2d") + [ShapeSample](../../../prelude/trait.ShapeSample.html "trait bevy::prelude::ShapeSample")<Output = [Vec2](../../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/sampling/shape_sampling.rs.html#602)

#### type [Output](#associatedtype.Output) = [Vec3](../../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")