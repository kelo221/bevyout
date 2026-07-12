[bevy](../index.html)::[prelude](index.html)

# Trait StableInterpolate 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#426)

```rust
pub trait StableInterpolate: Clone {
    // Required method
    fn interpolate_stable(&self, other: &Self, t: f32) -> Self;

    // Provided methods
    fn interpolate_stable_assign(&mut self, other: &Self, t: f32) { ... }
    fn smooth_nudge(&mut self, target: &Self, decay_rate: f32, delta: f32) { ... }
}
```

A type with a natural interpolation that provides strong subdivision guarantees.

Although the only required method is `interpolate_stable`, many things are expected of it:

1.  The notion of interpolation should follow naturally from the semantics of the type, so that inferring the interpolation mode from the type alone is sensible.
    
2.  The interpolation recovers something equivalent to the starting value at `t = 0.0` and likewise with the ending value at `t = 1.0`. They do not have to be data-identical, but they should be semantically identical. For example, [`Quat::slerp`](struct.Quat.html#method.slerp "method bevy::prelude::Quat::slerp") doesn’t always yield its second rotation input exactly at `t = 1.0`, but it always returns an equivalent rotation.
    
3.  Importantly, the interpolation must be _subdivision-stable_: for any interpolation curve between two (unnamed) values and any parameter-value pairs `(t0, p)` and `(t1, q)`, the interpolation curve between `p` and `q` must be the _linear_ reparameterization of the original interpolation curve restricted to the interval `[t0, t1]`.
    

The last of these conditions is very strong and indicates something like constant speed. It is called “subdivision stability” because it guarantees that breaking up the interpolation into segments and joining them back together has no effect.

Here is a diagram depicting it:

```
top curve = u.interpolate_stable(v, t)

             t0 => p   t1 => q
  |-------------|---------|-------------|
0 => u         /           \          1 => v
             /               \
           /                   \
         /        linear         \
       /     reparameterization    \
     /   t = t0 * (1 - s) + t1 * s   \
   /                                   \
  |-------------------------------------|
0 => p                                1 => q

bottom curve = p.interpolate_stable(q, s)
```

Note that some common forms of interpolation do not satisfy this criterion. For example, [`Quat::lerp`](struct.Quat.html#method.lerp "method bevy::prelude::Quat::lerp") and [`Rot2::nlerp`](struct.Rot2.html#method.nlerp "method bevy::prelude::Rot2::nlerp") are not subdivision-stable.

Furthermore, this is not to be used as a general trait for abstract interpolation. Consumers rely on the strong guarantees in order for behavior based on this trait to be well-behaved.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#433)

#### fn [interpolate\_stable](#tymethod.interpolate_stable)(&self, other: &Self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> Self

Interpolate between this value and the `other` given value using the parameter `t`. At `t = 0.0`, a value equivalent to `self` is recovered, while `t = 1.0` recovers a value equivalent to `other`, with intermediate values interpolating between the two. See the [trait-level documentation](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for details.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#438)

#### fn [interpolate\_stable\_assign](#method.interpolate_stable_assign)(&mut self, other: &Self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

A version of [`interpolate_stable`](trait.StableInterpolate.html#tymethod.interpolate_stable "method bevy::prelude::StableInterpolate::interpolate_stable") that assigns the result to `self` for convenience.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#467)

#### fn [smooth\_nudge](#method.smooth_nudge)(&mut self, target: &Self, decay\_rate: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), delta: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html))

Smoothly nudge this value towards the `target` at a given decay rate. The `decay_rate` parameter controls how fast the distance between `self` and `target` decays relative to the units of `delta`; the intended usage is for `decay_rate` to generally remain fixed, while `delta` is something like `delta_time` from an updating system. This produces a smooth following of the target that is independent of framerate.

More specifically, when this is called repeatedly, the result is that the distance between `self` and a fixed `target` attenuates exponentially, with the rate of this exponential decay given by `decay_rate`.

For example, at `decay_rate = 0.0`, this has no effect. At `decay_rate = f32::INFINITY`, `self` immediately snaps to `target`. In general, higher rates mean that `self` moves more quickly towards `target`.

##### Example

```rust
let mut object_position: Vec3 = Vec3::ZERO;
let target_position: Vec3 = Vec3::new(2.0, 3.0, 5.0);
// Decay rate of ln(10) => after 1 second, remaining distance is 1/10th
let decay_rate = f32::ln(10.0);
// Calling this repeatedly will move `object_position` towards `target_position`:
object_position.smooth_nudge(&target_position, decay_rate, delta_time);
```

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/transforms/align.rs ([line 151](../../src/align/align.rs.html#151))

```rust
140fn rotate_ship(ship: Single<(&mut Ship, &mut Transform)>, time: Res<Time>) {
141    let (mut ship, mut ship_transform) = ship.into_inner();
142
143    if !ship.in_motion {
144        return;
145    }
146
147    let target_rotation = ship.target_transform.rotation;
148
149    ship_transform
150        .rotation
151        .smooth_nudge(&target_rotation, 3.0, time.delta_secs());
152
153    if ship_transform.rotation.angle_between(target_rotation) <= f32::EPSILON {
154        ship.in_motion = false;
155    }
156}
```

Hide additional examples

examples/movement/smooth\_follow.rs ([line 131](../../src/smooth_follow/smooth_follow.rs.html#131))

```rust
119fn move_follower(
120    mut following: Single<&mut Transform, With<FollowingSphere>>,
121    target: Single<&Transform, (With<TargetSphere>, Without<FollowingSphere>)>,
122    decay_rate: Res<DecayRate>,
123    time: Res<Time>,
124) {
125    let decay_rate = decay_rate.0;
126    let delta_time = time.delta_secs();
127
128    // Calling `smooth_nudge` is what moves the following sphere smoothly toward the target.
129    following
130        .translation
131        .smooth_nudge(&target.translation, decay_rate, delta_time);
132}
```

examples/camera/2d\_top\_down\_camera.rs ([line 80](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#80))

```rust
68fn update_camera(
69    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
70    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
71    time: Res<Time>,
72) {
73    let Vec3 { x, y, .. } = player.translation;
74    let direction = Vec3::new(x, y, camera.translation.z);
75
76    // Applies a smooth effect to camera movement using stable interpolation
77    // between the camera position and the player position on the x and y axes.
78    camera
79        .translation
80        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
81}
```

examples/animation/animated\_mesh\_events.rs ([line 209](../../src/animated_mesh_events/animated_mesh_events.rs.html#209))

```rust
194fn simulate_particles(
195    mut commands: Commands,
196    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
197    time: Res<Time>,
198) {
199    for (entity, mut transform, mut particle) in &mut query {
200        if particle.lifetime_timer.tick(time.delta()).just_finished() {
201            commands.entity(entity).despawn();
202            return;
203        }
204
205        transform.translation += particle.velocity * time.delta_secs();
206        transform.scale = Vec3::splat(particle.size.lerp(0.0, particle.lifetime_timer.fraction()));
207        particle
208            .velocity
209            .smooth_nudge(&Vec3::ZERO, 4.0, time.delta_secs());
210    }
211}
```

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 298](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#298))

```rust
283fn simulate_particles(
284    mut commands: Commands,
285    mut query: Query<(Entity, &mut Transform, &mut Particle)>,
286    time: Res<Time>,
287) {
288    for (entity, mut transform, mut particle) in &mut query {
289        if particle.lifetime_timer.tick(time.delta()).just_finished() {
290            commands.entity(entity).despawn();
291            return;
292        }
293
294        transform.translation += particle.velocity * time.delta_secs();
295        transform.scale = Vec3::splat(particle.size.lerp(0.0, particle.lifetime_timer.fraction()));
296        particle
297            .velocity
298            .smooth_nudge(&Vec3::ZERO, 4.0, time.delta_secs());
299    }
300}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#535-541)

### impl<T> [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [(T₁, T₂, …, Tₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where T: [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

This trait is implemented for tuples up to 11 items long.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#535-541)

#### fn [interpolate\_stable](#tymethod.interpolate_stable)(&self, other: &[(T,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [(T,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#499)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Dir2](struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#506)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#513)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Dir3A](struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#38)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#40)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#38)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#492)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Quat](struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#485)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Rot2](struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#41)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#38)

### impl [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#475-477)

### impl<V> [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate") for V

where V: [NormedVectorSpace](../math/trait.NormedVectorSpace.html "trait bevy::math::NormedVectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,