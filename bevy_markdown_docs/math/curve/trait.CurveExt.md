[bevy](../../index.html)::[math](../index.html)::[curve](index.html)

# Trait CurveExt 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#380)

```rust
pub trait CurveExt<T>: Sized + Curve<T> {
    // Provided methods
    fn sample_iter(
        &self,
        iter: impl IntoIterator<Item = f32>,
    ) -> impl Iterator<Item = Option<T>> { ... }
    fn sample_iter_unchecked(
        &self,
        iter: impl IntoIterator<Item = f32>,
    ) -> impl Iterator<Item = T> { ... }
    fn sample_iter_clamped(
        &self,
        iter: impl IntoIterator<Item = f32>,
    ) -> impl Iterator<Item = T> { ... }
    fn map<S, F>(self, f: F) -> MapCurve<T, S, Self, F>
       where F: Fn(T) -> S { ... }
    fn reparametrize<F>(
        self,
        domain: Interval,
        f: F,
    ) -> ReparamCurve<T, Self, F>
       where F: Fn(f32) -> f32 { ... }
    fn reparametrize_linear(
        self,
        domain: Interval,
    ) -> Result<LinearReparamCurve<T, Self>, LinearReparamError> { ... }
    fn reparametrize_by_curve<C>(
        self,
        other: C,
    ) -> CurveReparamCurve<T, Self, C>
       where C: Curve<f32> { ... }
    fn graph(self) -> GraphCurve<T, Self> { ... }
    fn zip<S, C>(
        self,
        other: C,
    ) -> Result<ZipCurve<T, S, Self, C>, InvalidIntervalError>
       where C: Curve<S> { ... }
    fn chain<C>(self, other: C) -> Result<ChainCurve<T, Self, C>, ChainError>
       where C: Curve<T> { ... }
    fn reverse(self) -> Result<ReverseCurve<T, Self>, ReverseError> { ... }
    fn repeat(self, count: usize) -> Result<RepeatCurve<T, Self>, RepeatError> { ... }
    fn forever(self) -> Result<ForeverCurve<T, Self>, RepeatError> { ... }
    fn ping_pong(self) -> Result<PingPongCurve<T, Self>, PingPongError> { ... }
    fn chain_continue<C>(
        self,
        other: C,
    ) -> Result<ContinuationCurve<T, Self, C>, ChainError>
       where T: VectorSpace,
             C: Curve<T> { ... }
    fn samples(
        &self,
        samples: usize,
    ) -> Result<impl Iterator<Item = T>, ResamplingError> { ... }
    fn by_ref(&self) -> &Self { ... }
    fn flip<U, V>(self) -> impl Curve<(V, U)>
       where Self: CurveExt<(U, V)> { ... }
}
```

Available on **crate feature `curve`** only.

Extension trait implemented by [curves](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), allowing access to a number of adaptors and convenience methods.

This trait is automatically implemented for all curves that are `Sized`. In particular, it is implemented for types like `Box<dyn Curve<T>>`. `CurveExt` is not dyn-compatible itself.

For more information, see the [module-level documentation](index.html "mod bevy::math::curve").

## Provided Methods

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#387)

#### fn [sample\_iter](#method.sample_iter)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, returning `None` if the point is outside of the curve’s domain.

The samples are returned in the same order as the parameter values `t_n` were provided and will include all results. This leaves the responsibility for things like filtering and sorting to the user for maximum flexibility.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#402-405)

#### fn [sample\_iter\_unchecked](#method.sample_iter_unchecked)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, extracting the associated values. This is the unchecked version of sampling, which should only be used if the sample times `t_n` are already known to lie within the curve’s domain.

Values sampled from outside of a curve’s domain are generally considered invalid; data which is nonsensical or otherwise useless may be returned in such a circumstance, and extrapolation beyond a curve’s domain should not be relied upon.

The samples are returned in the same order as the parameter values `t_n` were provided and will include all results. This leaves the responsibility for things like filtering and sorting to the user for maximum flexibility.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#415)

#### fn [sample\_iter\_clamped](#method.sample_iter_clamped)( &self, iter: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>

Sample a collection of `n >= 0` points on this curve at the parameter values `t_n`, clamping `t_n` to lie inside the domain of the curve.

The samples are returned in the same order as the parameter values `t_n` were provided and will include all results. This leaves the responsibility for things like filtering and sorting to the user for maximum flexibility.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#423-425)

#### fn [map](#method.map)<S, F>(self, f: F) -> [MapCurve](../../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<T, S, Self, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> S,

Create a new curve by mapping the values of this curve via a function `f`; i.e., if the sample at time `t` for this curve is `x`, the value at time `t` on the new curve will be `f(x)`.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/animation/easing\_functions.rs ([line 161](../../../src/easing_functions/easing_functions.rs.html#161))

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#465-467)

#### fn [reparametrize](#method.reparametrize)<F>(self, domain: [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval"), f: F) -> [ReparamCurve](../../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, Self, F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html),

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") whose parameter space is related to the parameter space of this curve by `f`. For each time `t`, the sample from the new curve at time `t` is the sample from this curve at time `f(t)`. The given `domain` will be the domain of the new curve. The function `f` is expected to take `domain` into `self.domain()`.

Note that this is the opposite of what one might expect intuitively; for example, if this curve has a parameter domain of `[0, 1]`, then stretching the parameter domain to `[0, 2]` would be performed as follows, dividing by what might be perceived as the scaling factor rather than multiplying:

```rust
let my_curve = ConstantCurve::new(Interval::UNIT, 1.0);
let scaled_curve = my_curve.reparametrize(interval(0.0, 2.0).unwrap(), |t| t / 2.0);
```

This kind of linear remapping is provided by the convenience method [`CurveExt::reparametrize_linear`](../../prelude/trait.CurveExt.html#method.reparametrize_linear "method bevy::prelude::CurveExt::reparametrize_linear"), which requires only the desired domain for the new curve.

##### Examples

```rust
// Reverse a curve:
let my_curve = ConstantCurve::new(Interval::UNIT, 1.0);
let domain = my_curve.domain();
let reversed_curve = my_curve.reparametrize(domain, |t| domain.end() - (t - domain.start()));

// Take a segment of a curve:
let curve_segment = my_curve.reparametrize(interval(0.0, 0.5).unwrap(), |t| 0.5 + t);
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#484-487)

#### fn [reparametrize\_linear](#method.reparametrize_linear)( self, domain: [Interval](../../prelude/struct.Interval.html "struct bevy::prelude::Interval"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[LinearReparamCurve](../../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, Self>, [LinearReparamError](../../prelude/enum.LinearReparamError.html "enum bevy::prelude::LinearReparamError")\>

Linearly reparametrize this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), producing a new curve whose domain is the given `domain` instead of the current one. This operation is only valid for curves with bounded domains.

##### Errors

If either this curve’s domain or the given `domain` is unbounded, an error is returned.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/animation/eased\_motion.rs ([line 113](../../../src/eased_motion/eased_motion.rs.html#113))

```rust
92    fn create(
93        animation_graphs: &mut Assets<AnimationGraph>,
94        animation_clips: &mut Assets<AnimationClip>,
95    ) -> AnimationInfo {
96        // Create an ID that identifies the text node we're going to animate.
97        let animation_target_name = Name::new("Cube");
98        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
99
100        // Allocate an animation clip.
101        let mut animation_clip = AnimationClip::default();
102
103        // Each leg of the translation motion should take 3 seconds.
104        let animation_domain = interval(0.0, 3.0).unwrap();
105
106        // The easing curve is parametrized over [0, 1], so we reparametrize it and
107        // then ping-pong, which makes it spend another 3 seconds on the return journey.
108        let translation_curve = EasingCurve::new(
109            vec3(-6., 2., 0.),
110            vec3(6., 2., 0.),
111            EaseFunction::CubicInOut,
112        )
113        .reparametrize_linear(animation_domain)
114        .expect("this curve has bounded domain, so this should never fail")
115        .ping_pong()
116        .expect("this curve has bounded domain, so this should never fail");
117
118        // Something similar for rotation. The repetition here is an illusion caused
119        // by the symmetry of the cube; it rotates on the forward journey and never
120        // rotates back.
121        let rotation_curve = EasingCurve::new(
122            Quat::IDENTITY,
123            Quat::from_rotation_y(FRAC_PI_2),
124            EaseFunction::ElasticInOut,
125        )
126        .reparametrize_linear(interval(0.0, 4.0).unwrap())
127        .expect("this curve has bounded domain, so this should never fail");
128
129        animation_clip.add_curve_to_target(
130            animation_target_id,
131            AnimatableCurve::new(animated_field!(Transform::translation), translation_curve),
132        );
133        animation_clip.add_curve_to_target(
134            animation_target_id,
135            AnimatableCurve::new(animated_field!(Transform::rotation), rotation_curve),
136        );
137
138        // Save our animation clip as an asset.
139        let animation_clip_handle = animation_clips.add(animation_clip);
140
141        // Create an animation graph with that clip.
142        let (animation_graph, animation_node_index) =
143            AnimationGraph::from_clip(animation_clip_handle);
144        let animation_graph_handle = animation_graphs.add(animation_graph);
145
146        AnimationInfo {
147            target_name: animation_target_name,
148            target_id: animation_target_id,
149            graph: animation_graph_handle,
150            node_index: animation_node_index,
151        }
152    }
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#509-511)

#### fn [reparametrize\_by\_curve](#method.reparametrize_by_curve)<C>(self, other: C) -> [CurveReparamCurve](../../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, Self, C>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Reparametrize this [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by sampling from another curve.

The resulting curve samples at time `t` by first sampling `other` at time `t`, which produces another sample time `s` which is then used to sample this curve. The domain of the resulting curve is the domain of `other`.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#527)

#### fn [graph](#method.graph)(self) -> [GraphCurve](../../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, Self>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") which is the graph of this one; that is, its output echoes the sample time as part of a tuple.

For example, if this curve outputs `x` at time `t`, then the produced curve will produce `(t, x)` at time `t`. In particular, if this curve is a `Curve<T>`, the output of this method is a `Curve<(f32, T)>`.

##### [Examples found in repository](#scraped-examples-2)[?](../../../scrape-examples-help.html)

examples/animation/easing\_functions.rs ([line 160](../../../src/easing_functions/easing_functions.rs.html#160))

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#543-545)

#### fn [zip](#method.zip)<S, C>( self, other: C, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ZipCurve](../../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<T, S, Self, C>, [InvalidIntervalError](../../prelude/interval/struct.InvalidIntervalError.html "struct bevy::prelude::interval::InvalidIntervalError")\>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<S>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by zipping this curve together with another.

The sample at time `t` in the new curve is `(x, y)`, where `x` is the sample of `self` at time `t` and `y` is the sample of `other` at time `t`. The domain of the new curve is the intersection of the domains of its constituents.

##### Errors

If the domain intersection would be empty, an error is returned instead.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#564-566)

#### fn [chain](#method.chain)<C>(self, other: C) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ChainCurve](../../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, Self, C>, [ChainError](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError")\>

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by composing this curve end-to-start with another, producing another curve with outputs of the same type. The domain of the other curve is translated so that its start coincides with where this curve ends.

##### Errors

A [`ChainError`](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError") is returned if this curve’s domain doesn’t have a finite end or if `other`’s domain doesn’t have a finite start.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#589)

#### fn [reverse](#method.reverse)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ReverseCurve](../../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, Self>, [ReverseError](../../prelude/enum.ReverseError.html "enum bevy::prelude::ReverseError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") inverting this curve on the x-axis, producing another curve with outputs of the same type, effectively playing backwards starting at `self.domain().end()` and transitioning over to `self.domain().start()`. The domain of the new curve is still the same.

##### Errors

A [`ReverseError`](../../prelude/enum.ReverseError.html "enum bevy::prelude::ReverseError") is returned if this curve’s domain isn’t bounded.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#613)

#### fn [repeat](#method.repeat)(self, count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RepeatCurve](../../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, Self>, [RepeatError](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") repeating this curve `N` times, producing another curve with outputs of the same type. The domain of the new curve will be bigger by a factor of `n + 1`.

##### Notes

*   this doesn’t guarantee a smooth transition from one occurrence of the curve to its next iteration. The curve will make a jump if `self.domain().start() != self.domain().end()`!
*   for `count == 0` the output of this adaptor is basically identical to the previous curve
*   the value at the transitioning points (`domain.end() * n` for `n >= 1`) in the results is the value at `domain.end()` in the original curve

##### Errors

A [`RepeatError`](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError") is returned if this curve’s domain isn’t bounded.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#646)

#### fn [forever](#method.forever)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ForeverCurve](../../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, Self>, [RepeatError](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") repeating this curve forever, producing another curve with outputs of the same type. The domain of the new curve will be unbounded.

##### Notes

*   this doesn’t guarantee a smooth transition from one occurrence of the curve to its next iteration. The curve will make a jump if `self.domain().start() != self.domain().end()`!
*   the value at the transitioning points (`domain.end() * n` for `n >= 1`) in the results is the value at `domain.end()` in the original curve

##### Errors

A [`RepeatError`](../../prelude/enum.RepeatError.html "enum bevy::prelude::RepeatError") is returned if this curve’s domain isn’t bounded.

##### [Examples found in repository](#scraped-examples-3)[?](../../../scrape-examples-help.html)

examples/3d/light\_probe\_blending.rs ([line 395](../../../src/light_probe_blending/light_probe_blending.rs.html#395))

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#663)

#### fn [ping\_pong](#method.ping_pong)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[PingPongCurve](../../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, Self>, [PingPongError](../../prelude/enum.PingPongError.html "enum bevy::prelude::PingPongError")\>

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") chaining the original curve with its inverse, producing another curve with outputs of the same type. The domain of the new curve will be twice as long. The transition point is guaranteed to not make any jumps.

##### Errors

A [`PingPongError`](../../prelude/enum.PingPongError.html "enum bevy::prelude::PingPongError") is returned if this curve’s domain isn’t right-finite.

##### [Examples found in repository](#scraped-examples-4)[?](../../../scrape-examples-help.html)

examples/3d/light\_probe\_blending.rs ([line 393](../../../src/light_probe_blending/light_probe_blending.rs.html#393))

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

examples/animation/eased\_motion.rs ([line 115](../../../src/eased_motion/eased_motion.rs.html#115))

```rust
92    fn create(
93        animation_graphs: &mut Assets<AnimationGraph>,
94        animation_clips: &mut Assets<AnimationClip>,
95    ) -> AnimationInfo {
96        // Create an ID that identifies the text node we're going to animate.
97        let animation_target_name = Name::new("Cube");
98        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);
99
100        // Allocate an animation clip.
101        let mut animation_clip = AnimationClip::default();
102
103        // Each leg of the translation motion should take 3 seconds.
104        let animation_domain = interval(0.0, 3.0).unwrap();
105
106        // The easing curve is parametrized over [0, 1], so we reparametrize it and
107        // then ping-pong, which makes it spend another 3 seconds on the return journey.
108        let translation_curve = EasingCurve::new(
109            vec3(-6., 2., 0.),
110            vec3(6., 2., 0.),
111            EaseFunction::CubicInOut,
112        )
113        .reparametrize_linear(animation_domain)
114        .expect("this curve has bounded domain, so this should never fail")
115        .ping_pong()
116        .expect("this curve has bounded domain, so this should never fail");
117
118        // Something similar for rotation. The repetition here is an illusion caused
119        // by the symmetry of the cube; it rotates on the forward journey and never
120        // rotates back.
121        let rotation_curve = EasingCurve::new(
122            Quat::IDENTITY,
123            Quat::from_rotation_y(FRAC_PI_2),
124            EaseFunction::ElasticInOut,
125        )
126        .reparametrize_linear(interval(0.0, 4.0).unwrap())
127        .expect("this curve has bounded domain, so this should never fail");
128
129        animation_clip.add_curve_to_target(
130            animation_target_id,
131            AnimatableCurve::new(animated_field!(Transform::translation), translation_curve),
132        );
133        animation_clip.add_curve_to_target(
134            animation_target_id,
135            AnimatableCurve::new(animated_field!(Transform::rotation), rotation_curve),
136        );
137
138        // Save our animation clip as an asset.
139        let animation_clip_handle = animation_clips.add(animation_clip);
140
141        // Create an animation graph with that clip.
142        let (animation_graph, animation_node_index) =
143            AnimationGraph::from_clip(animation_clip_handle);
144        let animation_graph_handle = animation_graphs.add(animation_graph);
145
146        AnimationInfo {
147            target_name: animation_target_name,
148            target_id: animation_target_id,
149            graph: animation_graph_handle,
150            node_index: animation_node_index,
151        }
152    }
```

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#688-691)

#### fn [chain\_continue](#method.chain_continue)<C>( self, other: C, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[ContinuationCurve](../../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, Self, C>, [ChainError](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError")\>

where T: [VectorSpace](../trait.VectorSpace.html "trait bevy::math::VectorSpace"), C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,

Create a new [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") by composing this curve end-to-start with another, producing another curve with outputs of the same type. The domain of the other curve is translated so that its start coincides with where this curve ends.

Additionally the transition of the samples is guaranteed to make no sudden jumps. This is useful if you really just know about the shapes of your curves and don’t want to deal with stitching them together properly when it would just introduce useless complexity. It is realized by translating the other curve so that its start sample point coincides with the current curves’ end sample point.

##### Errors

A [`ChainError`](../../prelude/enum.ChainError.html "enum bevy::prelude::ChainError") is returned if this curve’s domain doesn’t have a finite end or if `other`’s domain doesn’t have a finite start.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#717)

#### fn [samples](#method.samples)( &self, samples: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T>, [ResamplingError](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError")\>

Extract an iterator over evenly-spaced samples from this curve.

##### Errors

If `samples` is less than 2 or if this curve has unbounded domain, a [`ResamplingError`](../../prelude/enum.ResamplingError.html "enum bevy::prelude::ResamplingError") is returned.

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#750)

#### fn [by\_ref](#method.by_ref)(&self) -> &Self

Borrow this curve rather than taking ownership of it. This is essentially an alias for a prefix `&`; the point is that intermediate operations can be performed while retaining access to the original curve.

##### Example

```rust
let my_curve = FunctionCurve::new(Interval::UNIT, |t| t * t + 1.0);

// Borrow `my_curve` long enough to resample a mapped version. Note that `map` takes
// ownership of its input.
let samples = my_curve.by_ref().map(|x| x * 2.0).resample_auto(100).unwrap();

// Do something else with `my_curve` since we retained ownership:
let new_curve = my_curve.reparametrize_linear(interval(-1.0, 1.0).unwrap()).unwrap();
```

##### [Examples found in repository](#scraped-examples-5)[?](../../../scrape-examples-help.html)

examples/animation/easing\_functions.rs ([line 159](../../../src/easing_functions/easing_functions.rs.html#159))

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

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#756-758)

#### fn [flip](#method.flip)<U, V>(self) -> impl [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<[(V, U)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Self: [CurveExt](../../prelude/trait.CurveExt.html "trait bevy::prelude::CurveExt")<[(U, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

Flip this curve so that its tuple output is arranged the other way.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/mod.rs.html#764)

### impl<C, T> [CurveExt](../../prelude/trait.CurveExt.html "trait bevy::prelude::CurveExt")<T> for C

where C: [Curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve")<T>,