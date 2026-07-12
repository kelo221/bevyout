[bevy](../index.html)::[prelude](index.html)

# Trait Curve 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#323)

```rust
pub trait Curve<T> {
    // Required methods
    fn domain(&self) -> Interval;
    fn sample_unchecked(&self, t: f32) -> T;

    // Provided methods
    fn sample(&self, t: f32) -> Option<T> { ... }
    fn sample_clamped(&self, t: f32) -> T { ... }
}
```

A trait for a type that can represent values of type `T` parametrized over a fixed interval.

Typical examples of this are actual geometric curves where `T: VectorSpace`, but other kinds of output data can be represented as well. See the [module-level documentation](../math/curve/index.html "mod bevy::math::curve") for details.

## Required Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#327)

#### fn [domain](#tymethod.domain)(&self) -> [Interval](struct.Interval.html "struct bevy::prelude::Interval")

The interval over which this curve is parametrized.

This is the range of values of `t` where we can sample the curve and receive valid output.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#336)

#### fn [sample\_unchecked](#tymethod.sample_unchecked)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Sample a point on this curve at the parameter value `t`, extracting the associated value. This is the unchecked version of sampling, which should only be used if the sample time `t` is already known to lie within the curve’s domain.

Values sampled from outside of a curve’s domain are generally considered invalid; data which is nonsensical or otherwise useless may be returned in such a circumstance, and extrapolation beyond a curve’s domain should not be relied upon.

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#340)

#### fn [sample](#method.sample)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

Sample a point on this curve at the parameter value `t`, returning `None` if the point is outside of the curve’s domain.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/3d/light\_probe\_blending.rs ([line 397](../../src/light_probe_blending/light_probe_blending.rs.html#397))

```rust
391fn move_sphere(mut spheres: Query<&mut Transform, With<ReflectiveSphere>>, time: Res<Time>) {
392    let Some(t) = SmoothStepCurve
393        .ping_pong()
394        .unwrap()
395        .forever()
396        .unwrap()
397        .sample(time.elapsed_secs() * SPHERE_MOVEMENT_SPEED)
398    else {
399        return;
400    };
401    for mut sphere_transform in &mut spheres {
402        sphere_transform.translation.z = -ROOM_SEPARATION * t;
403    }
404}
```

Hide additional examples

examples/animation/easing\_functions.rs ([line 169](../../src/easing_functions/easing_functions.rs.html#169))

```rust
124fn display_curves(
125    mut gizmos: Gizmos,
126    ease_functions: Query<(&EaseFunctionPlot, &Transform, &Children)>,
127    mut transforms: Query<&mut Transform, Without<EaseFunctionPlot>>,
128    mut ui_text: Single<&mut Text>,
129    time: Res<Time>,
130) {
131    let samples = 100;
132    let duration = 2.5;
133    let time_margin = 0.5;
134
135    let now = ((time.elapsed_secs() % (duration + time_margin * 2.0) - time_margin) / duration)
136        .clamp(0.0, 1.0);
137
138    ui_text.0 = format!("Progress: {now:.2}");
139
140    for (EaseFunctionPlot(function, color), transform, children) in &ease_functions {
141        let center = transform.translation.xy();
142        let half_size = PLOT_SIZE / 2.0;
143
144        // Draw a box around the curve
145        gizmos.linestrip_2d(
146            [
147                center + half_size,
148                center + half_size * Vec2::new(-1., 1.),
149                center + half_size * Vec2::new(-1., -1.),
150                center + half_size * Vec2::new(1., -1.),
151                center + half_size,
152            ],
153            color.darker(0.4),
154        );
155
156        // Draw the curve
157        let f = EasingCurve::new(0.0, 1.0, *function);
158        let drawn_curve = f
159            .by_ref()
160            .graph()
161            .map(|(x, y)| center - half_size + Vec2::new(x, y) * PLOT_SIZE);
162        gizmos.curve_2d(
163            &drawn_curve,
164            drawn_curve.domain().spaced_points(samples).unwrap(),
165            *color,
166        );
167
168        // Show progress along the curve for the current time
169        let y = f.sample(now).unwrap() * PLOT_SIZE.y;
170        transforms.get_mut(children[0]).unwrap().translation.y = -half_size.y + y;
171        transforms.get_mut(children[1]).unwrap().translation =
172            -half_size.extend(0.0) + Vec3::new(now * PLOT_SIZE.x, y, 0.0);
173
174        // Show horizontal bar at y value
175        gizmos.linestrip_2d(
176            [
177                center - half_size + Vec2::Y * y,
178                center - half_size + Vec2::new(PLOT_SIZE.x, y),
179            ],
180            color.darker(0.2),
181        );
182    }
183}
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#349)

#### fn [sample\_clamped](#method.sample_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T

Sample a point on this curve at the parameter value `t`, clamping `t` to lie inside the domain of the curve.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#120)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[Quat](struct.Quat.html "struct bevy::prelude::Quat")\> for [CubicRotationCurve](../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1016)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [BackInCurve](struct.BackInCurve.html "struct bevy::prelude::BackInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1018)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [BackInOutCurve](struct.BackInOutCurve.html "struct bevy::prelude::BackInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1017)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [BackOutCurve](struct.BackOutCurve.html "struct bevy::prelude::BackOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1019)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [BounceInCurve](struct.BounceInCurve.html "struct bevy::prelude::BounceInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1021)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [BounceInOutCurve](struct.BounceInOutCurve.html "struct bevy::prelude::BounceInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1020)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [BounceOutCurve](struct.BounceOutCurve.html "struct bevy::prelude::BounceOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1007)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [CircularInCurve](struct.CircularInCurve.html "struct bevy::prelude::CircularInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1009)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [CircularInOutCurve](struct.CircularInOutCurve.html "struct bevy::prelude::CircularInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1008)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [CircularOutCurve](struct.CircularOutCurve.html "struct bevy::prelude::CircularOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#989)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [CubicInCurve](struct.CubicInCurve.html "struct bevy::prelude::CubicInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#991)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [CubicInOutCurve](struct.CubicInOutCurve.html "struct bevy::prelude::CubicInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#990)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [CubicOutCurve](struct.CubicOutCurve.html "struct bevy::prelude::CubicOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1350)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [EaseFunction](enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1035)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ElasticCurve](struct.ElasticCurve.html "struct bevy::prelude::ElasticCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1013)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ElasticInCurve](struct.ElasticInCurve.html "struct bevy::prelude::ElasticInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1015)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ElasticInOutCurve](struct.ElasticInOutCurve.html "struct bevy::prelude::ElasticInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1014)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ElasticOutCurve](struct.ElasticOutCurve.html "struct bevy::prelude::ElasticOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1010)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ExponentialInCurve](struct.ExponentialInCurve.html "struct bevy::prelude::ExponentialInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1012)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ExponentialInOutCurve](struct.ExponentialInOutCurve.html "struct bevy::prelude::ExponentialInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1011)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [ExponentialOutCurve](struct.ExponentialOutCurve.html "struct bevy::prelude::ExponentialOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#985)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [LinearCurve](struct.LinearCurve.html "struct bevy::prelude::LinearCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#986)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuadraticInCurve](struct.QuadraticInCurve.html "struct bevy::prelude::QuadraticInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#988)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuadraticInOutCurve](struct.QuadraticInOutCurve.html "struct bevy::prelude::QuadraticInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#987)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuadraticOutCurve](struct.QuadraticOutCurve.html "struct bevy::prelude::QuadraticOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#992)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuarticInCurve](struct.QuarticInCurve.html "struct bevy::prelude::QuarticInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#994)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuarticInOutCurve](struct.QuarticInOutCurve.html "struct bevy::prelude::QuarticInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#993)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuarticOutCurve](struct.QuarticOutCurve.html "struct bevy::prelude::QuarticOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#995)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuinticInCurve](struct.QuinticInCurve.html "struct bevy::prelude::QuinticInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#997)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuinticInOutCurve](struct.QuinticInOutCurve.html "struct bevy::prelude::QuinticInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#996)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [QuinticOutCurve](struct.QuinticOutCurve.html "struct bevy::prelude::QuinticOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1004)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SineInCurve](struct.SineInCurve.html "struct bevy::prelude::SineInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1006)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SineInOutCurve](struct.SineInOutCurve.html "struct bevy::prelude::SineInOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1005)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SineOutCurve](struct.SineOutCurve.html "struct bevy::prelude::SineOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1000)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SmoothStepCurve](struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#998)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SmoothStepInCurve](struct.SmoothStepInCurve.html "struct bevy::prelude::SmoothStepInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#999)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SmoothStepOutCurve](struct.SmoothStepOutCurve.html "struct bevy::prelude::SmoothStepOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1003)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SmootherStepCurve](struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1001)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SmootherStepInCurve](struct.SmootherStepInCurve.html "struct bevy::prelude::SmootherStepInCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1002)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [SmootherStepOutCurve](struct.SmootherStepOutCurve.html "struct bevy::prelude::SmootherStepOutCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#1023)

### impl [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> for [StepsCurve](struct.StepsCurve.html "struct bevy::prelude::StepsCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#49)

### impl<P> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<P> for [CubicCurve](struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#13)

### impl<P> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<P> for [CubicSegment](struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#124)

### impl<P> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<P> for [RationalCurve](struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Available on **crate feature `alloc`** only.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/curve_impls.rs.html#88)

### impl<P> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<P> for [RationalSegment](struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#494-497)

### impl<S, T, C, D> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[(S, T)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\> for [ZipCurve](struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<S>, D: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#263-266)

### impl<S, T, C, F> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [MapCurve](struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<S>, F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(S) -> T,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#535-538)

### impl<T, C, D> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ChainCurve](struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>, D: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#800-804)

### impl<T, C, D> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ContinuationCurve](struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where T: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace"), C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>, D: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#428-431)

### impl<T, C, D> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>, D: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#355-358)

### impl<T, C, D> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for D

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), D: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = C>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#359-362)

### impl<T, C, F> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ReparamCurve](struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>, F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#461-463)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), T)> for [GraphCurve](struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#687-689)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ForeverCurve](struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#394-396)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#742-744)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [PingPongCurve](struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#628-630)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [RepeatCurve](struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#585-587)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ReverseCurve](struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where C: [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#157-160)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[WithDerivative](../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>> for [SampleDerivativeWrapper](derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

where T: [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleDerivative](derivatives/trait.SampleDerivative.html "trait bevy::prelude::derivatives::SampleDerivative")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#190-193)

### impl<T, C> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<[WithTwoDerivatives](../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>> for [SampleTwoDerivativesWrapper](derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>

where T: [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent"), C: [SampleTwoDerivatives](derivatives/trait.SampleTwoDerivatives.html "trait bevy::prelude::derivatives::SampleTwoDerivatives")<T>,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#167-169)

### impl<T, F> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [FunctionCurve](struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#88-91)

### impl<T, I> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [SampleCurve](struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#248-251)

### impl<T, I> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), I: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> T,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#727-729)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [AnimatableKeyframeCurve](struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>

where T: [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_gradient.rs.html#54-56)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ColorCurve](../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>

where T: [Mix](trait.Mix.html "trait bevy::prelude::Mix") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#63-65)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [ConstantCurve](struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#321-323)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [EasingCurve](struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>

where T: [Ease](trait.Ease.html "trait bevy::prelude::Ease") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#144-146)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>

where T: [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#17-19)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [SteppedKeyframeCurve](../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#319-321)

### impl<T> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<T> for [UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>

where T: [StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#56-58)

### impl<V> [Curve](trait.Curve.html "trait bevy::prelude::Curve")<V> for [CubicKeyframeCurve](../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<V>

where V: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace")<Scalar = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,