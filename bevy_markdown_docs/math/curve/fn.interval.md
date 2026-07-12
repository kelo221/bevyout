[bevy](../../index.html)::[math](../index.html)::[curve](index.html)

# Function interval 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/interval.rs.html#199)

```rust
pub const fn interval(
    start: f32,
    end: f32,
) -> Result<Interval, InvalidIntervalError>
```

Available on **crate feature `curve`** only.

Create an [`Interval`](../../prelude/struct.Interval.html "struct bevy::prelude::Interval") with a given `start` and `end`. Alias of [`Interval::new`](../../prelude/struct.Interval.html#method.new "associated function bevy::prelude::Interval::new").

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/animation/eased\_motion.rs ([line 104](../../../src/eased_motion/eased_motion.rs.html#104))

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