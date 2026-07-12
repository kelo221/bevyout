[bevy](../../index.html)::[color](../index.html)::[color\_difference](index.html)

# Trait EuclideanDistance 

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_difference.rs.html#7)

```rust
pub trait EuclideanDistance: Sized {
    // Required method
    fn distance_squared(&self, other: &Self) -> f32;

    // Provided method
    fn distance(&self, other: &Self) -> f32 { ... }
}
```

Calculate the distance between this and another color as if they were coordinates in a Euclidean space. Alpha is not considered in the distance calculation.

## Required Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_difference.rs.html#14)

#### fn [distance\_squared](#tymethod.distance_squared)(&self, other: &Self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Distance squared from `self` to `other`.

## Provided Methods

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_difference.rs.html#9)

#### fn [distance](#method.distance)(&self, other: &Self) -> [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

Distance from `self` to `other`.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/2d/cpu\_draw.rs ([line 128](../../../src/cpu_draw/cpu_draw.rs.html#128))

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

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#916)

### impl [EuclideanDistance](trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#242)

### impl [EuclideanDistance](trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance") for [LinearRgba](../../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#160)

### impl [EuclideanDistance](trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance") for [Oklaba](../../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#194)

### impl [EuclideanDistance](trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance") for [Oklcha](../../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#292)

### impl [EuclideanDistance](trait.EuclideanDistance.html "trait bevy::color::color_difference::EuclideanDistance") for [Srgba](../../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")