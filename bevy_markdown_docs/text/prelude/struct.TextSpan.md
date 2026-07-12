[bevy](../../index.html)::[text](../index.html)::[prelude](index.html)

# Struct TextSpan 

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#193)

```rust
pub struct TextSpan(pub String);
```

A span of text in a tree of spans.

A `TextSpan` is only valid when it exists as a child of a parent that has either `Text` or `Text2d`. The parent’s `Text` / `Text2d` component contains the base text content. Any children with `TextSpan` extend this text by appending their content to the parent’s text in sequence to form a [`ComputedTextBlock`](../struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"). The parent’s [`TextLayout`](../../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout") determines the layout of the block but each node has its own [`TextFont`](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont") and [`TextColor`](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor").

## Tuple Fields

`0: [String](../../prelude/struct.String.html "struct bevy::prelude::String")`

## Implementations

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#195)

### impl [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#197)

#### pub fn [new](#method.new)(text: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[String](../../prelude/struct.String.html "struct bevy::prelude::String")\>) -> [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

Makes a new text span component.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/motion\_blur.rs ([line 249](../../../src/motion_blur/motion_blur.rs.html#249))

```rust
237fn setup_ui(mut commands: Commands) {
238    commands.spawn((
239        Text::default(),
240        Node {
241            position_type: PositionType::Absolute,
242            top: px(12),
243            left: px(12),
244            ..default()
245        },
246        children![
247            TextSpan::default(),
248            TextSpan::default(),
249            TextSpan::new("1/2: -/+ shutter angle (blur amount)\n"),
250            TextSpan::new("3/4: -/+ sample count (blur quality)\n"),
251            TextSpan::new("Spacebar: cycle camera\n"),
252        ],
253    ));
254}
```

Hide additional examples

examples/ecs/one\_shot\_systems.rs ([line 103](../../../src/one_shot_systems/one_shot_systems.rs.html#103))

```rust
92fn setup_ui(mut commands: Commands) {
93    commands.spawn(Camera2d);
94    commands.spawn((
95        Text::default(),
96        TextLayout::justify(Justify::Center),
97        Node {
98            align_self: AlignSelf::Center,
99            justify_self: JustifySelf::Center,
100            ..default()
101        },
102        children![
103            (TextSpan::new("Press A or B to trigger a one-shot system\n")),
104            (TextSpan::new("Last Triggered: ")),
105            (
106                TextSpan::new("-"),
107                TextColor(bevy::color::palettes::css::ORANGE.into()),
108            )
109        ],
110    ));
111}
```

examples/app/log\_layers\_ecs.rs ([line 157](../../../src/log_layers_ecs/log_layers_ecs.rs.html#157))

```rust
144fn print_logs(
145    mut log_message_reader: MessageReader<LogMessage>,
146    mut commands: Commands,
147    log_viewer_root: Single<Entity, With<LogViewerRoot>>,
148) {
149    let root_entity = *log_viewer_root;
150
151    commands.entity(root_entity).with_children(|child| {
152        for log_message in log_message_reader.read() {
153            child.spawn((
154                Text::default(),
155                children![
156                    (
157                        TextSpan::new(format!("{:5} ", log_message.level)),
158                        TextColor(level_color(&log_message.level)),
159                    ),
160                    TextSpan::new(&log_message.message),
161                ],
162            ));
163        }
164    });
165}
```

examples/window/window\_drag\_move.rs ([lines 74-76](../../../src/window_drag_move/window_drag_move.rs.html#74-76))

```rust
58fn setup(mut commands: Commands) {
59    // Camera
60    commands.spawn(Camera3d::default());
61
62    // UI
63    commands.spawn((
64        Node {
65            position_type: PositionType::Absolute,
66            padding: UiRect::all(px(5)),
67            ..default()
68        },
69        BackgroundColor(Color::BLACK.with_alpha(0.75)),
70        GlobalZIndex(i32::MAX),
71        children![(
72            Text::default(),
73            children![
74                TextSpan::new(
75                    "Demonstrate drag move and drag resize without window decorations.\n\n",
76                ),
77                TextSpan::new("Controls:\n"),
78                TextSpan::new("A - change left click action ["),
79                TextSpan::new("Move"),
80                TextSpan::new("]\n"),
81                TextSpan::new("S / D - change resize direction ["),
82                TextSpan::new("NorthWest"),
83                TextSpan::new("]\n"),
84            ]
85        )],
86    ));
87}
```

examples/math/render\_primitives.rs ([line 383](../../../src/render_primitives/render_primitives.rs.html#383))

```rust
365fn setup_text(mut commands: Commands, cameras: Query<(Entity, &Camera)>) {
366    let active_camera = cameras
367        .iter()
368        .find_map(|(entity, camera)| camera.is_active.then_some(entity))
369        .expect("run condition ensures existence");
370    commands.spawn((
371        HeaderNode,
372        Node {
373            justify_self: JustifySelf::Center,
374            top: px(5),
375            ..Default::default()
376        },
377        UiTargetCamera(active_camera),
378        children![(
379            Text::default(),
380            HeaderText,
381            TextLayout::justify(Justify::Center),
382            children![
383                TextSpan::new("Primitive: "),
384                TextSpan(format!("{text}", text = PrimitiveSelected::default())),
385                TextSpan::new("\n\n"),
386                TextSpan::new(
387                    "Press 'C' to switch between 2D and 3D mode\n\
388                    Press 'Up' or 'Down' to switch to the next/previous primitive",
389                ),
390                TextSpan::new("\n\n"),
391                TextSpan::new("(If nothing is displayed, there's no rendering support yet)",),
392            ]
393        )],
394    ));
395}
```

examples/window/low\_power.rs ([line 199](../../../src/low_power/low_power.rs.html#199))

```rust
167    pub fn setup(
168        mut commands: Commands,
169        mut meshes: ResMut<Assets<Mesh>>,
170        mut materials: ResMut<Assets<StandardMaterial>>,
171        mut request_redraw_writer: MessageWriter<RequestRedraw>,
172    ) {
173        commands.spawn((
174            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
175            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
176            Rotator,
177        ));
178
179        commands.spawn((
180            DirectionalLight::default(),
181            Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
182        ));
183        commands.spawn((
184            Camera3d::default(),
185            Transform::from_xyz(-2.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
186        ));
187        request_redraw_writer.write(RequestRedraw);
188        commands.spawn((
189            Text::default(),
190            Node {
191                align_self: AlignSelf::FlexStart,
192                position_type: PositionType::Absolute,
193                top: px(12),
194                left: px(12),
195                ..default()
196            },
197            ModeText,
198            children![
199                TextSpan::new("Press space bar to cycle modes\n"),
200                (TextSpan::default(), TextColor(LIME.into())),
201                (TextSpan::new("\nFrame: "), TextColor(YELLOW.into())),
202                (TextSpan::new(""), TextColor(YELLOW.into())),
203            ],
204        ));
205    }
```

Additional examples can be found in:  

*   [examples/3d/order\_independent\_transparency.rs](../../../src/order_independent_transparency/order_independent_transparency.rs.html#61)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#61)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../../src/overflow_debug/overflow_debug.rs.html#100)
*   [examples/testbed/2d.rs](../../../src/testbed_2d/2d.rs.html#272)
*   [examples/ui/text/text\_background\_colors.rs](../../../src/text_background_colors/text_background_colors.rs.html#60)
*   [examples/showcase/stepping.rs](../../../src/breakout/stepping.rs.html#152)
*   [examples/showcase/game\_menu.rs](../../../src/game_menu/game_menu.rs.html#191)
*   [examples/stress\_tests/bevymark\_3d.rs](../../../src/bevymark_3d/bevymark_3d.rs.html#246)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#132)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#272)
*   [examples/3d/shadow\_biases.rs](../../../src/shadow_biases/shadow_biases.rs.html#109)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#59)
*   [examples/3d/parallax\_mapping.rs](../../../src/parallax_mapping/parallax_mapping.rs.html#312)
*   [examples/ui/text/text.rs](../../../src/text/text.rs.html#128)
*   [examples/3d/lighting.rs](../../../src/lighting/lighting.rs.html#223)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#170)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#262)

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [String](../../prelude/struct.String.html "struct bevy::prelude::String")\>

1.7.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1057)

#### pub fn [as\_str](#method.as_str)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Extracts a string slice containing the entire `String`.

##### Examples

```rust
let s = String::from("foo");

assert_eq!("foo", s.as_str());
```

1.7.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1080)

#### pub fn [as\_mut\_str](#method.as_mut_str)(&mut self) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Converts a `String` into a mutable string slice.

##### Examples

```rust
let mut s = String::from("foobar");
let s_mut_str = s.as_mut_str();

s_mut_str.make_ascii_uppercase();

assert_eq!("FOOBAR", s_mut_str);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1106)

#### pub fn [push\_str](#method.push_str)(&mut self, string: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Available on **non-`no_global_oom_handling`** only.

Appends a given string slice onto the end of this `String`.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

```rust
let mut s = String::from("foo");

s.push_str("bar");

assert_eq!("foobar", s);
```

1.87.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1152-1154)

#### pub fn [extend\_from\_within](#method.extend_from_within)<R>(&mut self, src: R)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Available on **non-`no_global_oom_handling`** only.

Copies elements from `src` range to the end of the string.

##### Panics

Panics if the range has `start_bound > end_bound`, if the range is bounded on either end and does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary, or if the new capacity exceeds `isize::MAX` bytes.

##### Examples

```rust
let mut string = String::from("abcde");

string.extend_from_within(2..);
assert_eq!(string, "abcdecde");

string.extend_from_within(..2);
assert_eq!(string, "abcdecdeab");

string.extend_from_within(4..8);
assert_eq!(string, "abcdecdeabecde");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1177)

#### pub fn [capacity](#method.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns this `String`’s capacity, in bytes.

##### Examples

```rust
let s = String::with_capacity(10);

assert!(s.capacity() >= 10);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1224)

#### pub fn [reserve](#method.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Available on **non-`no_global_oom_handling`** only.

Reserves capacity for at least `additional` bytes more than the current length. The allocator may reserve more space to speculatively avoid frequent allocations. After calling `reserve`, capacity will be greater than or equal to `self.len() + additional`. Does nothing if capacity is already sufficient.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

Basic usage:

```rust
let mut s = String::new();

s.reserve(10);

assert!(s.capacity() >= 10);
```

This might not actually increase the capacity:

```rust
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

// s now has a length of 2 and a capacity of at least 10
let capacity = s.capacity();
assert_eq!(2, s.len());
assert!(capacity >= 10);

// Since we already have at least an extra 8 capacity, calling this...
s.reserve(8);

// ... doesn't actually increase.
assert_eq!(capacity, s.capacity());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1274)

#### pub fn [reserve\_exact](#method.reserve_exact)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Available on **non-`no_global_oom_handling`** only.

Reserves the minimum capacity for at least `additional` bytes more than the current length. Unlike [`reserve`](../../prelude/struct.String.html#method.reserve "method bevy::prelude::String::reserve"), this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling `reserve_exact`, capacity will be greater than or equal to `self.len() + additional`. Does nothing if the capacity is already sufficient.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

Basic usage:

```rust
let mut s = String::new();

s.reserve_exact(10);

assert!(s.capacity() >= 10);
```

This might not actually increase the capacity:

```rust
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

// s now has a length of 2 and a capacity of at least 10
let capacity = s.capacity();
assert_eq!(2, s.len());
assert!(capacity >= 10);

// Since we already have at least an extra 8 capacity, calling this...
s.reserve_exact(8);

// ... doesn't actually increase.
assert_eq!(capacity, s.capacity());
```

1.57.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1309)

#### pub fn [try\_reserve](#method.try_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

Tries to reserve capacity for at least `additional` bytes more than the current length. The allocator may reserve more space to speculatively avoid frequent allocations. After calling `try_reserve`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if capacity is already sufficient. This method preserves the contents even if an error occurs.

##### Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use std::collections::TryReserveError;

fn process_data(data: &str) -> Result<String, TryReserveError> {
    let mut output = String::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve(data.len())?;

    // Now we know this can't OOM in the middle of our complex work
    output.push_str(data);

    Ok(output)
}
```

1.57.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1350)

#### pub fn [try\_reserve\_exact](#method.try_reserve_exact)( &mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://doc.rust-lang.org/nightly/alloc/collections/struct.TryReserveError.html "struct alloc::collections::TryReserveError")\>

Tries to reserve the minimum capacity for at least `additional` bytes more than the current length. Unlike [`try_reserve`](../../prelude/struct.String.html#method.try_reserve "method bevy::prelude::String::try_reserve"), this will not deliberately over-allocate to speculatively avoid frequent allocations. After calling `try_reserve_exact`, capacity will be greater than or equal to `self.len() + additional` if it returns `Ok(())`. Does nothing if the capacity is already sufficient.

Note that the allocator may give the collection more space than it requests. Therefore, capacity can not be relied upon to be precisely minimal. Prefer [`try_reserve`](../../prelude/struct.String.html#method.try_reserve "method bevy::prelude::String::try_reserve") if future insertions are expected.

##### Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use std::collections::TryReserveError;

fn process_data(data: &str) -> Result<String, TryReserveError> {
    let mut output = String::new();

    // Pre-reserve the memory, exiting if we can't
    output.try_reserve_exact(data.len())?;

    // Now we know this can't OOM in the middle of our complex work
    output.push_str(data);

    Ok(output)
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1370)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit)(&mut self)

Available on **non-`no_global_oom_handling`** only.

Shrinks the capacity of this `String` to match its length.

##### Examples

```rust
let mut s = String::from("foo");

s.reserve(100);
assert!(s.capacity() >= 100);

s.shrink_to_fit();
assert_eq!(3, s.capacity());
```

1.56.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1397)

#### pub fn [shrink\_to](#method.shrink_to)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Available on **non-`no_global_oom_handling`** only.

Shrinks the capacity of this `String` with a lower bound.

The capacity will remain at least as large as both the length and the supplied value.

If the current capacity is less than the lower limit, this is a no-op.

##### Examples

```rust
let mut s = String::from("foo");

s.reserve(100);
assert!(s.capacity() >= 100);

s.shrink_to(10);
assert!(s.capacity() >= 10);
s.shrink_to(0);
assert!(s.capacity() >= 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1421)

#### pub fn [push](#method.push)(&mut self, ch: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

Available on **non-`no_global_oom_handling`** only.

Appends the given [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") to the end of this `String`.

##### Panics

Panics if the new capacity exceeds `isize::MAX` _bytes_.

##### Examples

```rust
let mut s = String::from("abc");

s.push('1');
s.push('2');
s.push('3');

assert_eq!("abc123", s);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1450)

#### pub fn [as\_bytes](#method.as_bytes)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Returns a byte slice of this `String`’s contents.

The inverse of this method is [`from_utf8`](../../prelude/struct.String.html#method.from_utf8 "associated function bevy::prelude::String::from_utf8").

##### Examples

```rust
let s = String::from("hello");

assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1478)

#### pub fn [truncate](#method.truncate)(&mut self, new\_len: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shortens this `String` to the specified length.

If `new_len` is greater than or equal to the string’s current length, this has no effect.

Note that this method has no effect on the allocated capacity of the string

##### Panics

Panics if `new_len` does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("hello");

s.truncate(2);

assert_eq!("he", s);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1502)

#### pub fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>

Removes the last character from the string buffer and returns it.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if this `String` is empty.

##### Examples

```rust
let mut s = String::from("abč");

assert_eq!(s.pop(), Some('č'));
assert_eq!(s.pop(), Some('b'));
assert_eq!(s.pop(), Some('a'));

assert_eq!(s.pop(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1535)

#### pub fn [remove](#method.remove)(&mut self, idx: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)

Removes a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") from this `String` at byte position `idx` and returns it.

Copies all bytes after the removed char to new positions.

Note that calling this in a loop can result in quadratic behavior.

##### Panics

Panics if `idx` is larger than or equal to the `String`’s length, or if it does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("abç");

assert_eq!(s.remove(0), 'a');
assert_eq!(s.remove(1), 'ç');
assert_eq!(s.remove(0), 'b');
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1572)

#### pub fn [remove\_matches](#method.remove_matches)<P>(&mut self, pat: P)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`string_remove_matches`)

Available on **non-`no_global_oom_handling`** only.

Remove all matches of pattern `pat` in the `String`.

##### Examples

```rust
#![feature(string_remove_matches)]
let mut s = String::from("Trees are not green, the sky is not blue.");
s.remove_matches("not ");
assert_eq!("Trees are green, the sky is blue.", s);
```

Matches will be detected and removed iteratively, so in cases where patterns overlap, only the first pattern will be removed:

```rust
#![feature(string_remove_matches)]
let mut s = String::from("banana");
s.remove_matches("ana");
assert_eq!("bna", s);
```

1.26.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1649-1651)

#### pub fn [retain](#method.retain)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([char](https://doc.rust-lang.org/nightly/std/primitive.char.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the characters specified by the predicate.

In other words, remove all characters `c` such that `f(c)` returns `false`. This method operates in place, visiting each character exactly once in the original order, and preserves the order of the retained characters.

##### Examples

```rust
let mut s = String::from("f_o_ob_ar");

s.retain(|c| c != '_');

assert_eq!(s, "foobar");
```

Because the elements are visited exactly once in the original order, external state may be used to decide which elements to keep.

```rust
let mut s = String::from("abcde");
let keep = [false, true, true, false, true];
let mut iter = keep.iter();
s.retain(|_| *iter.next().unwrap());
assert_eq!(s, "bce");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1731)

#### pub fn [insert](#method.insert)(&mut self, idx: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ch: [char](https://doc.rust-lang.org/nightly/std/primitive.char.html))

Available on **non-`no_global_oom_handling`** only.

Inserts a character into this `String` at byte position `idx`.

Reallocates if `self.capacity()` is insufficient, which may involve copying all `self.capacity()` bytes. Makes space for the insertion by copying all bytes of `&self[idx..]` to new positions.

Note that calling this in a loop can result in quadratic behavior.

##### Panics

Panics if `idx` is larger than the `String`’s length, or if it does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::with_capacity(3);

s.insert(0, 'f');
s.insert(1, 'o');
s.insert(2, 'o');

assert_eq!("foo", s);
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1788)

#### pub fn [insert\_str](#method.insert_str)(&mut self, idx: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), string: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Available on **non-`no_global_oom_handling`** only.

Inserts a string slice into this `String` at byte position `idx`.

Reallocates if `self.capacity()` is insufficient, which may involve copying all `self.capacity()` bytes. Makes space for the insertion by copying all bytes of `&self[idx..]` to new positions.

Note that calling this in a loop can result in quadratic behavior.

##### Panics

Panics if `idx` is larger than the `String`’s length, or if it does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("bar");

s.insert_str(0, "foo");

assert_eq!("foobar", s);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1841)

#### pub unsafe fn [as\_mut\_vec](#method.as_mut_vec)(&mut self) -> &mut [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\> [ⓘ](#)

Returns a mutable reference to the contents of this `String`.

##### Safety

This function is unsafe because the returned `&mut Vec` allows writing bytes which are not valid UTF-8. If this constraint is violated, using the original `String` after dropping the `&mut Vec` may violate memory safety, as the rest of the standard library assumes that `String`s are valid UTF-8.

##### Examples

```rust
let mut s = String::from("hello");

unsafe {
    let vec = s.as_mut_vec();
    assert_eq!(&[104, 101, 108, 108, 111][..], &vec[..]);

    vec.reverse();
}
assert_eq!(s, "olleh");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1865)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the length of this `String`, in bytes, not [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s or graphemes. In other words, it might not be what a human considers the length of the string.

##### Examples

```rust
let a = String::from("foo");
assert_eq!(a.len(), 3);

let fancy_f = String::from("ƒoo");
assert_eq!(fancy_f.len(), 4);
assert_eq!(fancy_f.chars().count(), 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1885)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this `String` has a length of zero, and `false` otherwise.

##### Examples

```rust
let mut v = String::new();
assert!(v.is_empty());

v.push('a');
assert!(!v.is_empty());
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1917)

#### pub fn [split\_off](#method.split_off)(&mut self, at: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Splits the string into two at the given byte index.

Returns a newly allocated `String`. `self` contains bytes `[0, at)`, and the returned `String` contains bytes `[at, len)`. `at` must be on the boundary of a UTF-8 code point.

Note that the capacity of `self` does not change.

##### Panics

Panics if `at` is not on a `UTF-8` code point boundary, or if it is beyond the last code point of the string.

##### Examples

```rust
let mut hello = String::from("Hello, World!");
let world = hello.split_off(7);
assert_eq!(hello, "Hello, ");
assert_eq!(world, "World!");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1941)

#### pub fn [clear](#method.clear)(&mut self)

Truncates this `String`, removing all contents.

While this means the `String` will have a length of zero, it does not touch its capacity.

##### Examples

```rust
let mut s = String::from("foo");

s.clear();

assert!(s.is_empty());
assert_eq!(0, s.len());
assert_eq!(3, s.capacity());
```

1.6.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#1980-1982)

#### pub fn [drain](#method.drain)<R>(&mut self, range: R) -> [Drain](https://doc.rust-lang.org/nightly/alloc/string/struct.Drain.html "struct alloc::string::Drain")<'\_> [ⓘ](#)

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Removes the specified range from the string in bulk, returning all removed characters as an iterator.

The returned iterator keeps a mutable borrow on the string to optimize its implementation.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Leaking

If the returned iterator goes out of scope without being dropped (due to [`core::mem::forget`](https://doc.rust-lang.org/nightly/core/mem/fn.forget.html "fn core::mem::forget"), for example), the string may still contain a copy of any drained characters, or may have lost characters arbitrarily, including characters outside the range.

##### Examples

```rust
let mut s = String::from("α is alpha, β is beta");
let beta_offset = s.find('β').unwrap_or(s.len());

// Remove the range up until the β from the string
let t: String = s.drain(..beta_offset).collect();
assert_eq!(t, "α is alpha, ");
assert_eq!(s, "β is beta");

// A full range clears the string, like `clear()` does
s.drain(..);
assert_eq!(s, "");
```

1.27.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2080-2082)

#### pub fn [replace\_range](#method.replace_range)<R>(&mut self, range: R, replace\_with: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

where R: [RangeBounds](https://doc.rust-lang.org/nightly/core/ops/range/trait.RangeBounds.html "trait core::ops::range::RangeBounds")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>,

Available on **non-`no_global_oom_handling`** only.

Removes the specified range in the string, and replaces it with the given string. The given string doesn’t need to be the same length as the range.

##### Panics

Panics if the range has `start_bound > end_bound`, or, if the range is bounded on either end and does not lie on a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") boundary.

##### Examples

```rust
let mut s = String::from("α is alpha, β is beta");
let beta_offset = s.find('β').unwrap_or(s.len());

// Replace the range up until the β from the string
s.replace_range(..beta_offset, "Α is capital alpha; ");
assert_eq!(s, "Α is capital alpha; β is beta");
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2122)

#### pub fn [replace\_first](#method.replace_first)<P>(&mut self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`string_replace_in_place`)

Available on **non-`no_global_oom_handling`** only.

Replaces the leftmost occurrence of a pattern with another string, in-place.

This method can be preferred over [`string = string.replacen(..., 1);`](../../std/primitive.str.html#method.replacen), as it can use the `String`’s existing capacity to prevent a reallocation if sufficient space is available.

##### Examples

Basic usage:

```rust
#![feature(string_replace_in_place)]

let mut s = String::from("Test Results: ❌❌❌");

// Replace the leftmost ❌ with a ✅
s.replace_first('❌', "✅");
assert_eq!(s, "Test Results: ✅❌❌");
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2148-2150)

#### pub fn [replace\_last](#method.replace_last)<P>(&mut self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`string_replace_in_place`)

Available on **non-`no_global_oom_handling`** only.

Replaces the rightmost occurrence of a pattern with another string, in-place.

##### Examples

Basic usage:

```rust
#![feature(string_replace_in_place)]

let mut s = String::from("Test Results: ❌❌❌");

// Replace the rightmost ❌ with a ✅
s.replace_last('❌', "✅");
assert_eq!(s, "Test Results: ❌❌✅");
```

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#154)

#### pub fn [len](#method.len-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the length of `self`.

This length is in bytes, not [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s or graphemes. In other words, it might not be what a human considers the length of the string.

##### Examples

```rust
let len = "foo".len();
assert_eq!(3, len);

assert_eq!("ƒoo".len(), 4); // fancy f!
assert_eq!("ƒoo".chars().count(), 3);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#174)

#### pub fn [is\_empty](#method.is_empty-1)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if `self` has a length of zero bytes.

##### Examples

```rust
let s = "";
assert!(s.is_empty());

let s = "not empty";
assert!(!s.is_empty());
```

1.9.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#374)

#### pub fn [is\_char\_boundary](#method.is_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks that `index`\-th byte is the first byte in a UTF-8 code point sequence or the end of the string.

The start and end of the string (when `index == self.len()`) are considered to be boundaries.

Returns `false` if `index` is greater than `self.len()`.

##### Examples

```rust
let s = "Löwe 老虎 Léopard";
assert!(s.is_char_boundary(0));
// start of `老`
assert!(s.is_char_boundary(6));
assert!(s.is_char_boundary(s.len()));

// second byte of `ö`
assert!(!s.is_char_boundary(2));

// third byte of `老`
assert!(!s.is_char_boundary(8));
```

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#423)

#### pub fn [floor\_char\_boundary](#method.floor_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Finds the closest `x` not exceeding `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

This method can help you truncate a string so that it’s still valid UTF-8, but doesn’t exceed a given number of bytes. Note that this is done purely at the character level and can still visually split graphemes, even though the underlying characters aren’t split. For example, the emoji 🧑‍🔬 (scientist) could be split so that the string only includes 🧑 (person) instead.

##### Examples

```rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.floor_char_boundary(13);
assert_eq!(closest, 10);
assert_eq!(&s[..closest], "❤️🧡");
```

1.91.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#480)

#### pub fn [ceil\_char\_boundary](#method.ceil_char_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Finds the closest `x` not below `index` where [`is_char_boundary(x)`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_char_boundary "method str::is_char_boundary") is `true`.

If `index` is greater than the length of the string, this returns the length of the string.

This method is the natural complement to [`floor_char_boundary`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.floor_char_boundary "method str::floor_char_boundary"). See that method for more details.

##### Examples

```rust
let s = "❤️🧡💛💚💙💜";
assert_eq!(s.len(), 26);
assert!(!s.is_char_boundary(13));

let closest = s.ceil_char_boundary(13);
assert_eq!(closest, 14);
assert_eq!(&s[..closest], "❤️🧡💛");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#513)

#### pub fn [as\_bytes](#method.as_bytes-1)(&self) -> &\[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Converts a string slice to a byte slice. To convert the byte slice back into a string slice, use the [`from_utf8`](https://doc.rust-lang.org/nightly/core/str/converts/fn.from_utf8.html "fn core::str::converts::from_utf8") function.

##### Examples

```rust
let bytes = "bors".as_bytes();
assert_eq!(b"bors", bytes);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#558)

#### pub unsafe fn [as\_bytes\_mut](#method.as_bytes_mut)(&mut self) -> &mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\] [ⓘ](#)

Converts a mutable string slice to a mutable byte slice.

##### Safety

The caller must ensure that the content of the slice is valid UTF-8 before the borrow ends and the underlying `str` is used.

Use of a `str` whose contents are not valid UTF-8 is undefined behavior.

##### Examples

Basic usage:

```rust
let mut s = String::from("Hello");
let bytes = unsafe { s.as_bytes_mut() };

assert_eq!(b"Hello", bytes);
```

Mutability:

```rust
let mut s = String::from("🗻∈🌏");

unsafe {
    let bytes = s.as_bytes_mut();

    bytes[0] = 0xF0;
    bytes[1] = 0x9F;
    bytes[2] = 0x8D;
    bytes[3] = 0x94;
}

assert_eq!("🍔∈🌏", s);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#589)

#### pub fn [as\_ptr](#method.as_ptr)(&self) -> [\*const](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

Converts a string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

The caller must ensure that the returned pointer is never written to. If you need to mutate the contents of the string slice, use [`as_mut_ptr`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.as_mut_ptr "method str::as_mut_ptr").

##### Examples

```rust
let s = "Hello";
let ptr = s.as_ptr();
```

1.36.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#608)

#### pub fn [as\_mut\_ptr](#method.as_mut_ptr)(&mut self) -> [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

Converts a mutable string slice to a raw pointer.

As string slices are a slice of bytes, the raw pointer points to a [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8"). This pointer will be pointing to the first byte of the string slice.

It is your responsibility to make sure that the string slice only gets modified in a way that it remains valid UTF-8.

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#634)

#### pub fn [get](#method.get)<I>(&self, i: I) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### Examples

```rust
let v = String::from("🗻∈🌏");

assert_eq!(Some("🗻"), v.get(0..4));

// indices not on UTF-8 sequence boundaries
assert!(v.get(1..).is_none());
assert!(v.get(..8).is_none());

// out of bounds
assert!(v.get(..42).is_none());
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#667)

#### pub fn [get\_mut](#method.get_mut)<I>( &mut self, i: I, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")\>

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a mutable subslice of `str`.

This is the non-panicking alternative to indexing the `str`. Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") whenever equivalent indexing operation would panic.

##### Examples

```rust
let mut v = String::from("hello");
// correct length
assert!(v.get_mut(0..5).is_some());
// out of bounds
assert!(v.get_mut(..42).is_none());
assert_eq!(Some("he"), v.get_mut(0..2).map(|v| &*v));

assert_eq!("hello", v);
{
    let s = v.get_mut(0..2);
    let s = s.map(|s| {
        s.make_ascii_uppercase();
        &*s
    });
    assert_eq!(Some("HE"), s);
}
assert_eq!("HEllo", v);
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#699)

#### pub unsafe fn [get\_unchecked](#method.get_unchecked)<I>(&self, i: I) -> &<I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns an unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### Safety

Callers of this function are responsible that these preconditions are satisfied:

*   The starting index must not exceed the ending index;
*   Indexes must be within bounds of the original slice;
*   Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### Examples

```rust
let v = "🗻∈🌏";
unsafe {
    assert_eq!("🗻", v.get_unchecked(0..4));
    assert_eq!("∈", v.get_unchecked(4..7));
    assert_eq!("🌏", v.get_unchecked(7..11));
}
```

1.20.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#734)

#### pub unsafe fn [get\_unchecked\_mut](#method.get_unchecked_mut)<I>( &mut self, i: I, ) -> &mut <I as [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>::[Output](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html#associatedtype.Output "type core::slice::index::SliceIndex::Output")

where I: [SliceIndex](https://doc.rust-lang.org/nightly/core/slice/index/trait.SliceIndex.html "trait core::slice::index::SliceIndex")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>,

Returns a mutable, unchecked subslice of `str`.

This is the unchecked alternative to indexing the `str`.

##### Safety

Callers of this function are responsible that these preconditions are satisfied:

*   The starting index must not exceed the ending index;
*   Indexes must be within bounds of the original slice;
*   Indexes must lie on UTF-8 sequence boundaries.

Failing that, the returned string slice may reference invalid memory or violate the invariants communicated by the `str` type.

##### Examples

```rust
let mut v = String::from("🗻∈🌏");
unsafe {
    assert_eq!("🗻", v.get_unchecked_mut(0..4));
    assert_eq!("∈", v.get_unchecked_mut(4..7));
    assert_eq!("🌏", v.get_unchecked_mut(7..11));
}
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#785)

#### pub unsafe fn [slice\_unchecked](#method.slice_unchecked)(&self, begin: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), end: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.29.0:

use `get_unchecked(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`Index`](https://doc.rust-lang.org/nightly/core/ops/index/trait.Index.html "trait core::ops::index::Index").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get a mutable string slice instead, see the [`slice_mut_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_mut_unchecked "method str::slice_mut_unchecked") method.

##### Safety

Callers of this function are responsible that three preconditions are satisfied:

*   `begin` must not exceed `end`.
*   `begin` and `end` must be byte positions within the string slice.
*   `begin` and `end` must lie on UTF-8 sequence boundaries.

##### Examples

```rust
let s = "Löwe 老虎 Léopard";

unsafe {
    assert_eq!("Löwe 老虎 Léopard", s.slice_unchecked(0, 21));
}

let s = "Hello, world!";

unsafe {
    assert_eq!("world", s.slice_unchecked(7, 12));
}
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#819)

#### pub unsafe fn [slice\_mut\_unchecked](#method.slice_mut_unchecked)( &mut self, begin: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), end: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.29.0:

use `get_unchecked_mut(begin..end)` instead

Creates a string slice from another string slice, bypassing safety checks.

This is generally not recommended, use with caution! For a safe alternative see [`str`](https://doc.rust-lang.org/nightly/std/primitive.str.html "primitive str") and [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/index/trait.IndexMut.html "trait core::ops::index::IndexMut").

This new slice goes from `begin` to `end`, including `begin` but excluding `end`.

To get an immutable string slice instead, see the [`slice_unchecked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.slice_unchecked "method str::slice_unchecked") method.

##### Safety

Callers of this function are responsible that three preconditions are satisfied:

*   `begin` must not exceed `end`.
*   `begin` and `end` must be byte positions within the string slice.
*   `begin` and `end` must lie on UTF-8 sequence boundaries.

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#859)

#### pub fn [split\_at](#method.split_at)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Divides one string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut "method str::split_at_mut") method.

##### Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked").

##### Examples

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at(3);

assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);
```

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#900)

#### pub fn [split\_at\_mut](#method.split_at_mut)(&mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> (&mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))

Divides one mutable string slice into two at an index.

The argument, `mid`, should be a byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get immutable string slices instead, see the [`split_at`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at "method str::split_at") method.

##### Panics

Panics if `mid` is not on a UTF-8 code point boundary, or if it is past the end of the last code point of the string slice. For a non-panicking alternative see [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked").

##### Examples

```rust
let mut s = "Per Martin-Löf".to_string();
{
    let (first, last) = s.split_at_mut(3);
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#940)

#### pub fn [split\_at\_checked](#method.split_at_checked)(&self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

Divides one string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get mutable string slices instead, see the [`split_at_mut_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_mut_checked "method str::split_at_mut_checked") method.

##### Examples

```rust
let s = "Per Martin-Löf";

let (first, last) = s.split_at_checked(3).unwrap();
assert_eq!("Per", first);
assert_eq!(" Martin-Löf", last);

assert_eq!(None, s.split_at_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_checked(16));  // Beyond the string length
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#981)

#### pub fn [split\_at\_mut\_checked](#method.split_at_mut_checked)( &mut self, mid: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &mut [str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

Divides one mutable string slice into two at an index.

The argument, `mid`, should be a valid byte offset from the start of the string. It must also be on the boundary of a UTF-8 code point. The method returns `None` if that’s not the case.

The two slices returned go from the start of the string slice to `mid`, and from `mid` to the end of the string slice.

To get immutable string slices instead, see the [`split_at_checked`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_at_checked "method str::split_at_checked") method.

##### Examples

```rust
let mut s = "Per Martin-Löf".to_string();
if let Some((first, last)) = s.split_at_mut_checked(3) {
    first.make_ascii_uppercase();
    assert_eq!("PER", first);
    assert_eq!(" Martin-Löf", last);
}
assert_eq!("PER Martin-Löf", s);

assert_eq!(None, s.split_at_mut_checked(13));  // Inside “ö”
assert_eq!(None, s.split_at_mut_checked(16));  // Beyond the string length
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1078)

#### pub fn [chars](#method.chars)(&self) -> [Chars](https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html "struct core::str::iter::Chars")<'\_> [ⓘ](#)

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns such an iterator.

It’s important to remember that [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") represents a Unicode Scalar Value, and might not match your idea of what a ‘character’ is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust’s standard library, check crates.io instead.

##### Examples

Basic usage:

```rust
let word = "goodbye";

let count = word.chars().count();
assert_eq!(7, count);

let mut chars = word.chars();

assert_eq!(Some('g'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('o'), chars.next());
assert_eq!(Some('d'), chars.next());
assert_eq!(Some('b'), chars.next());
assert_eq!(Some('y'), chars.next());
assert_eq!(Some('e'), chars.next());

assert_eq!(None, chars.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
let y = "y̆";

let mut chars = y.chars();

assert_eq!(Some('y'), chars.next()); // not 'y̆'
assert_eq!(Some('\u{0306}'), chars.next());

assert_eq!(None, chars.next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1135)

#### pub fn [char\_indices](#method.char_indices)(&self) -> [CharIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html "struct core::str::iter::CharIndices")<'\_> [ⓘ](#)

Returns an iterator over the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s of a string slice, and their positions.

As a string slice consists of valid UTF-8, we can iterate through a string slice by [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"). This method returns an iterator of both these [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, as well as their byte positions.

The iterator yields tuples. The position is first, the [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char") is second.

##### Examples

Basic usage:

```rust
let word = "goodbye";

let count = word.char_indices().count();
assert_eq!(7, count);

let mut char_indices = word.char_indices();

assert_eq!(Some((0, 'g')), char_indices.next());
assert_eq!(Some((1, 'o')), char_indices.next());
assert_eq!(Some((2, 'o')), char_indices.next());
assert_eq!(Some((3, 'd')), char_indices.next());
assert_eq!(Some((4, 'b')), char_indices.next());
assert_eq!(Some((5, 'y')), char_indices.next());
assert_eq!(Some((6, 'e')), char_indices.next());

assert_eq!(None, char_indices.next());
```

Remember, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s might not match your intuition about characters:

```rust
let yes = "y̆es";

let mut char_indices = yes.char_indices();

assert_eq!(Some((0, 'y')), char_indices.next()); // not (0, 'y̆')
assert_eq!(Some((1, '\u{0306}')), char_indices.next());

// note the 3 here - the previous character took up two bytes
assert_eq!(Some((3, 'e')), char_indices.next());
assert_eq!(Some((4, 's')), char_indices.next());

assert_eq!(None, char_indices.next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1158)

#### pub fn [bytes](#method.bytes)(&self) -> [Bytes](https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html "struct core::str::iter::Bytes")<'\_> [ⓘ](#)

Returns an iterator over the bytes of a string slice.

As a string slice consists of a sequence of bytes, we can iterate through a string slice by byte. This method returns such an iterator.

##### Examples

```rust
let mut bytes = "bors".bytes();

assert_eq!(Some(b'b'), bytes.next());
assert_eq!(Some(b'o'), bytes.next());
assert_eq!(Some(b'r'), bytes.next());
assert_eq!(Some(b's'), bytes.next());

assert_eq!(None, bytes.next());
```

1.1.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1210)

#### pub fn [split\_whitespace](#method.split_whitespace)(&self) -> [SplitWhitespace](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html "struct core::str::iter::SplitWhitespace")<'\_> [ⓘ](#)

Splits a string slice by whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of whitespace.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`. If you only want to split on ASCII whitespace instead, use [`split_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_ascii_whitespace "method str::split_ascii_whitespace").

##### Examples

Basic usage:

```rust
let mut iter = "A few words".split_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

All kinds of whitespace are considered:

```rust
let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all whitespace, the iterator yields no string slices:

```rust
assert_eq!("".split_whitespace().next(), None);
assert_eq!("   ".split_whitespace().next(), None);
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1264)

#### pub fn [split\_ascii\_whitespace](#method.split_ascii_whitespace)(&self) -> [SplitAsciiWhitespace](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html "struct core::str::iter::SplitAsciiWhitespace")<'\_> [ⓘ](#)

Splits a string slice by ASCII whitespace.

The iterator returned will return string slices that are sub-slices of the original string slice, separated by any amount of ASCII whitespace.

This uses the same definition as [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace"). To split by Unicode `Whitespace` instead, use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace"). Note that because of this difference in definition, even if `s.is_ascii()` is `true`, `s.split_ascii_whitespace()` behavior will differ from `s.split_whitespace()` if `s` contains U+000B VERTICAL TAB.

##### Examples

Basic usage:

```rust
let mut iter = "A few words".split_ascii_whitespace();

assert_eq!(Some("A"), iter.next());
assert_eq!(Some("few"), iter.next());
assert_eq!(Some("words"), iter.next());

assert_eq!(None, iter.next());
```

Various kinds of ASCII whitespace are considered (see [`char::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.is_ascii_whitespace "method char::is_ascii_whitespace")):

```rust
let mut iter = " Mary   had\ta little  \n\t lamb".split_ascii_whitespace();
assert_eq!(Some("Mary"), iter.next());
assert_eq!(Some("had"), iter.next());
assert_eq!(Some("a"), iter.next());
assert_eq!(Some("little"), iter.next());
assert_eq!(Some("lamb"), iter.next());

assert_eq!(None, iter.next());
```

If the string is empty or all ASCII whitespace, the iterator yields no string slices:

```rust
assert_eq!("".split_ascii_whitespace().next(), None);
assert_eq!("   ".split_ascii_whitespace().next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1328)

#### pub fn [lines](#method.lines)(&self) -> [Lines](https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html "struct core::str::iter::Lines")<'\_> [ⓘ](#)

Returns an iterator over the lines of a string, as string slices.

Lines are split at line endings that are either newlines (`\n`) or sequences of a carriage return followed by a line feed (`\r\n`).

Line terminators are not included in the lines returned by the iterator.

Note that any carriage return (`\r`) not immediately followed by a line feed (`\n`) does not split a line. These carriage returns are thereby included in the produced lines.

The final line ending is optional. A string that ends with a final line ending will return the same lines as an otherwise identical string without a final line ending.

An empty string returns an empty iterator.

##### Examples

Basic usage:

```rust
let text = "foo\r\nbar\n\nbaz\r";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
// Trailing carriage return is included in the last line
assert_eq!(Some("baz\r"), lines.next());

assert_eq!(None, lines.next());
```

The final line does not require any ending:

```rust
let text = "foo\nbar\n\r\nbaz";
let mut lines = text.lines();

assert_eq!(Some("foo"), lines.next());
assert_eq!(Some("bar"), lines.next());
assert_eq!(Some(""), lines.next());
assert_eq!(Some("baz"), lines.next());

assert_eq!(None, lines.next());
```

An empty string returns an empty iterator:

```rust
let text = "";
let mut lines = text.lines();

assert_eq!(lines.next(), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1337)

#### pub fn [lines\_any](#method.lines_any)(&self) -> [LinesAny](https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html "struct core::str::iter::LinesAny")<'\_> [ⓘ](#)

👎Deprecated since 1.4.0:

use lines() instead now

Returns an iterator over the lines of a string.

1.8.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1357)

#### pub fn [encode\_utf16](#method.encode_utf16)(&self) -> [EncodeUtf16](https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html "struct core::str::iter::EncodeUtf16")<'\_> [ⓘ](#)

Returns an iterator of `u16` over the string encoded as native endian UTF-16 (without byte-order mark).

##### Examples

```rust
let text = "Zażółć gęślą jaźń";

let utf8_len = text.len();
let utf16_len = text.encode_utf16().count();

assert!(utf16_len <= utf8_len);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1382)

#### pub fn [contains](#method.contains)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns `true` if the given pattern matches a sub-slice of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.contains("nana"));
assert!(!bananas.contains("apples"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1420)

#### pub fn [starts\_with](#method.starts_with)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns `true` if the given pattern matches a prefix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, in which case this function will return true if the `&str` is a prefix of this string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can also be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches. These will only be checked against the first character of this string slice. Look at the second example below regarding behavior for slices of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.starts_with("bana"));
assert!(!bananas.starts_with("nana"));
```

```rust
let bananas = "bananas";

// Note that both of these assert successfully.
assert!(bananas.starts_with(&['b', 'a', 'n', 'a']));
assert!(bananas.starts_with(&['a', 'b', 'c', 'd']));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1445-1447)

#### pub fn [ends\_with](#method.ends_with)<P>(&self, pat: P) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns `true` if the given pattern matches a suffix of this string slice.

Returns `false` if it does not.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let bananas = "bananas";

assert!(bananas.ends_with("anas"));
assert!(!bananas.ends_with("nana"));
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1496)

#### pub fn [find](#method.find)<P>(&self, pat: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns the byte index of the first character of this string slice that matches the pattern.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.find('L'), Some(0));
assert_eq!(s.find('é'), Some(14));
assert_eq!(s.find("pard"), Some(17));
```

More complex patterns using point-free style and closures:

```rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.find(char::is_whitespace), Some(5));
assert_eq!(s.find(char::is_lowercase), Some(1));
assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

Not finding the pattern:

```rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.find(x), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1542-1544)

#### pub fn [rfind](#method.rfind)<P>(&self, pat: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns the byte index for the first character of the last match of the pattern in this string slice.

Returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the pattern doesn’t match.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
let s = "Löwe 老虎 Léopard Gepardi";

assert_eq!(s.rfind('L'), Some(13));
assert_eq!(s.rfind('é'), Some(14));
assert_eq!(s.rfind("pard"), Some(24));
```

More complex patterns with closures:

```rust
let s = "Löwe 老虎 Léopard";

assert_eq!(s.rfind(char::is_whitespace), Some(12));
assert_eq!(s.rfind(char::is_lowercase), Some(20));
```

Not finding the pattern:

```rust
let s = "Löwe 老虎 Léopard";
let x: &[_] = &['1', '2'];

assert_eq!(s.rfind(x), None);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1670)

#### pub fn [split](#method.split)<P>(&self, pat: P) -> [Split](https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html "struct core::str::iter::Split")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

If there are no matches the full string slice is returned as the only item in the iterator.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit "method str::rsplit") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

let v: Vec<&str> = "".split('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
assert_eq!(v, ["lion", "", "tiger", "leopard"]);

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

let v: Vec<&str> = "AABBCC".split("DD").collect();
assert_eq!(v, ["AABBCC"]);

let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
assert_eq!(v, ["abc", "def", "ghi"]);

let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);
```

If the pattern is a slice of chars, split on each occurrence of any of the characters:

```rust
let v: Vec<&str> = "2020-11-03 23:59".split(&['-', ' ', ':', '@'][..]).collect();
assert_eq!(v, ["2020", "11", "03", "23", "59"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "def", "ghi"]);
```

If a string contains multiple contiguous separators, you will end up with empty strings in the output:

```rust
let x = "||||a||b|c".to_string();
let d: Vec<_> = x.split('|').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

Contiguous separators are separated by the empty string.

```rust
let x = "(///)".to_string();
let d: Vec<_> = x.split('/').collect();

assert_eq!(d, &["(", "", "", ")"]);
```

Separators at the start or end of a string are neighbored by empty strings.

```rust
let d: Vec<_> = "010".split("0").collect();
assert_eq!(d, &["", "1", ""]);
```

When the empty string is used as a separator, it separates every character in the string, along with the beginning and end of the string.

```rust
let f: Vec<_> = "rust".split("").collect();
assert_eq!(f, &["", "r", "u", "s", "t", ""]);
```

Contiguous separators can lead to possibly surprising behavior when whitespace is used as the separator. This code is correct:

```rust
let x = "    a  b c".to_string();
let d: Vec<_> = x.split(' ').collect();

assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
```

It does _not_ give you:

[ⓘ](# "This example is not tested")

```rust
assert_eq!(d, &["a", "b", "c"]);
```

Use [`split_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_whitespace "method str::split_whitespace") for this behavior.

1.51.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1711)

#### pub fn [split\_inclusive](#method.split_inclusive)<P>(&self, pat: P) -> [SplitInclusive](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html "struct core::str::iter::SplitInclusive")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

Differs from the iterator produced by `split` in that `split_inclusive` leaves the matched part as the terminator of the substring.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
```

If the last element of the string is matched, that element will be considered the terminator of the preceding substring. That substring will be the last item returned by the iterator.

```rust
let v: Vec<&str> = "Mary had a little lamb\nlittle lamb\nlittle lamb.\n"
    .split_inclusive('\n').collect();
assert_eq!(v, ["Mary had a little lamb\n", "little lamb\n", "little lamb.\n"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1766-1768)

#### pub fn [rsplit](#method.rsplit)<P>(&self, pat: P) -> [RSplit](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html "struct core::str::iter::RSplit")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
assert_eq!(v, ["lamb", "little", "a", "had", "Mary"]);

let v: Vec<&str> = "".rsplit('X').collect();
assert_eq!(v, [""]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplit('X').collect();
assert_eq!(v, ["leopard", "tiger", "", "lion"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplit("::").collect();
assert_eq!(v, ["leopard", "tiger", "lion"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".rsplit(|c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "def", "abc"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1815)

#### pub fn [split\_terminator](#method.split_terminator)<P>(&self, pat: P) -> [SplitTerminator](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html "struct core::str::iter::SplitTerminator")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of the given string slice, separated by characters matched by a pattern.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_, rather than _separated_ by a pattern.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rsplit_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplit_terminator "method str::rsplit_terminator") method can be used.

##### Examples

```rust
let v: Vec<&str> = "A.B.".split_terminator('.').collect();
assert_eq!(v, ["A", "B"]);

let v: Vec<&str> = "A..B..".split_terminator(".").collect();
assert_eq!(v, ["A", "", "B", ""]);

let v: Vec<&str> = "A.B:C.D".split_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["A", "B", "C", "D"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1861-1863)

#### pub fn [rsplit\_terminator](#method.rsplit_terminator)<P>(&self, pat: P) -> [RSplitTerminator](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html "struct core::str::iter::RSplitTerminator")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of `self`, separated by characters matched by a pattern and yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

Equivalent to [`split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split"), except that the trailing substring is skipped if empty.

This method can be used for string data that is _terminated_, rather than _separated_ by a pattern.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be double ended if a forward/reverse search yields the same elements.

For iterating from the front, the [`split_terminator`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split_terminator "method str::split_terminator") method can be used.

##### Examples

```rust
let v: Vec<&str> = "A.B.".rsplit_terminator('.').collect();
assert_eq!(v, ["B", "A"]);

let v: Vec<&str> = "A..B..".rsplit_terminator(".").collect();
assert_eq!(v, ["", "B", "", "A"]);

let v: Vec<&str> = "A.B:C.D".rsplit_terminator(&['.', ':'][..]).collect();
assert_eq!(v, ["D", "C", "B", "A"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1916)

#### pub fn [splitn](#method.splitn)<P>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pat: P) -> [SplitN](https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html "struct core::str::iter::SplitN")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over substrings of the given string slice, separated by a pattern, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

If the pattern allows a reverse search, the [`rsplitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rsplitn "method str::rsplitn") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
assert_eq!(v, ["Mary", "had", "a little lambda"]);

let v: Vec<&str> = "lionXXtigerXleopard".splitn(3, "X").collect();
assert_eq!(v, ["lion", "", "tigerXleopard"]);

let v: Vec<&str> = "abcXdef".splitn(1, 'X').collect();
assert_eq!(v, ["abcXdef"]);

let v: Vec<&str> = "".splitn(1, 'X').collect();
assert_eq!(v, [""]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".splitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["abc", "defXghi"]);
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1965-1967)

#### pub fn [rsplitn](#method.rsplitn)<P>(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), pat: P) -> [RSplitN](https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html "struct core::str::iter::RSplitN")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over substrings of this string slice, separated by a pattern, starting from the end of the string, restricted to returning at most `n` items.

If `n` substrings are returned, the last substring (the `n`th substring) will contain the remainder of the string.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will not be double ended, because it is not efficient to support.

For splitting from the front, the [`splitn`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.splitn "method str::splitn") method can be used.

##### Examples

Simple patterns:

```rust
let v: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
assert_eq!(v, ["lamb", "little", "Mary had a"]);

let v: Vec<&str> = "lionXXtigerXleopard".rsplitn(3, 'X').collect();
assert_eq!(v, ["leopard", "tiger", "lionX"]);

let v: Vec<&str> = "lion::tiger::leopard".rsplitn(2, "::").collect();
assert_eq!(v, ["leopard", "lion::tiger"]);
```

A more complex pattern, using a closure:

```rust
let v: Vec<&str> = "abc1defXghi".rsplitn(2, |c| c == '1' || c == 'X').collect();
assert_eq!(v, ["ghi", "abc1def"]);
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#1985)

#### pub fn [split\_once](#method.split_once)<P>(&self, delimiter: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Splits the string on the first occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### Examples

```rust
assert_eq!("cfg".split_once('='), None);
assert_eq!("cfg=".split_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".split_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".split_once('='), Some(("cfg", "foo=bar")));
```

1.52.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2004-2006)

#### pub fn [rsplit\_once](#method.rsplit_once)<P>(&self, delimiter: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html))>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Splits the string on the last occurrence of the specified delimiter and returns prefix before delimiter and suffix after delimiter.

##### Examples

```rust
assert_eq!("cfg".rsplit_once('='), None);
assert_eq!("cfg=".rsplit_once('='), Some(("cfg", "")));
assert_eq!("cfg=foo".rsplit_once('='), Some(("cfg", "foo")));
assert_eq!("cfg=foo=bar".rsplit_once('='), Some(("cfg=foo", "bar")));
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2044)

#### pub fn [matches](#method.matches)<P>(&self, pat: P) -> [Matches](https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html "struct core::str::iter::Matches")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over the disjoint matches of a pattern within the given string slice.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatches "method str::rmatches") method can be used.

##### Examples

```rust
let v: Vec<&str> = "abcXXXabcYYYabc".matches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
assert_eq!(v, ["1", "2", "3"]);
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2078-2080)

#### pub fn [rmatches](#method.rmatches)<P>(&self, pat: P) -> [RMatches](https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html "struct core::str::iter::RMatches")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over the disjoint matches of a pattern within this string slice, yielded in reverse order.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.matches "method str::matches") method can be used.

##### Examples

```rust
let v: Vec<&str> = "abcXXXabcYYYabc".rmatches("abc").collect();
assert_eq!(v, ["abc", "abc", "abc"]);

let v: Vec<&str> = "1abc2abc3".rmatches(char::is_numeric).collect();
assert_eq!(v, ["3", "2", "1"]);
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2122)

#### pub fn [match\_indices](#method.match_indices)<P>(&self, pat: P) -> [MatchIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html "struct core::str::iter::MatchIndices")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns an iterator over the disjoint matches of a pattern within this string slice as well as the index that the match starts at.

For matches of `pat` within `self` that overlap, only the indices corresponding to the first match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if the pattern allows a reverse search and forward/reverse search yields the same elements. This is true for, e.g., [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), but not for `&str`.

If the pattern allows a reverse search but its results might differ from a forward search, the [`rmatch_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.rmatch_indices "method str::rmatch_indices") method can be used.

##### Examples

```rust
let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
```

1.5.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2162-2164)

#### pub fn [rmatch\_indices](#method.rmatch_indices)<P>(&self, pat: P) -> [RMatchIndices](https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html "struct core::str::iter::RMatchIndices")<'\_, P> [ⓘ](#)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns an iterator over the disjoint matches of a pattern within `self`, yielded in reverse order along with the index of the match.

For matches of `pat` within `self` that overlap, only the indices corresponding to the last match are returned.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Iterator behavior

The returned iterator requires that the pattern supports a reverse search, and it will be a [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") if a forward/reverse search yields the same elements.

For iterating from the front, the [`match_indices`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.match_indices "method str::match_indices") method can be used.

##### Examples

```rust
let v: Vec<_> = "abcXXXabcYYYabc".rmatch_indices("abc").collect();
assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);

let v: Vec<_> = "1abcabc2".rmatch_indices("abc").collect();
assert_eq!(v, [(4, "abc"), (1, "abc")]);

let v: Vec<_> = "ababa".rmatch_indices("aba").collect();
assert_eq!(v, [(2, "aba")]); // only the last `aba`
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2186)

#### pub fn [trim](#method.trim)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading and trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Examples

```rust
let s = "\n Hello\tworld\t\n";

assert_eq!("Hello\tworld", s.trim());
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2225)

#### pub fn [trim\_start](#method.trim_start)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### Examples

Basic usage:

```rust
let s = "\n Hello\tworld\t\n";
assert_eq!("Hello\tworld\t\n", s.trim_start());
```

Directionality:

```rust
let s = "  English  ";
assert!(Some('E') == s.trim_start().chars().next());

let s = "  עברית  ";
assert!(Some('ע') == s.trim_start().chars().next());
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2264)

#### pub fn [trim\_end](#method.trim_end)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`, which includes newlines.

##### Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### Examples

Basic usage:

```rust
let s = "\n Hello\tworld\t\n";
assert_eq!("\n Hello\tworld", s.trim_end());
```

Directionality:

```rust
let s = "  English  ";
assert!(Some('h') == s.trim_end().chars().rev().next());

let s = "  עברית  ";
assert!(Some('ת') == s.trim_end().chars().rev().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2304)

#### pub fn [trim\_left](#method.trim_left)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.33.0:

superseded by `trim_start`

Returns a string slice with leading whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### Examples

Basic usage:

```rust
let s = " Hello\tworld\t";

assert_eq!("Hello\tworld\t", s.trim_left());
```

Directionality:

```rust
let s = "  English";
assert!(Some('E') == s.trim_left().chars().next());

let s = "  עברית";
assert!(Some('ע') == s.trim_left().chars().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2344)

#### pub fn [trim\_right](#method.trim_right)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

👎Deprecated since 1.33.0:

superseded by `trim_end`

Returns a string slice with trailing whitespace removed.

‘Whitespace’ is defined according to the terms of the Unicode Derived Core Property `White_Space`.

##### Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### Examples

Basic usage:

```rust
let s = " Hello\tworld\t";

assert_eq!(" Hello\tworld", s.trim_right());
```

Directionality:

```rust
let s = "English  ";
assert!(Some('h') == s.trim_right().chars().rev().next());

let s = "עברית  ";
assert!(Some('ת') == s.trim_right().chars().rev().next());
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2377-2379)

#### pub fn [trim\_matches](#method.trim_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [DoubleEndedSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.DoubleEndedSearcher.html "trait core::str::pattern::DoubleEndedSearcher")<'a>,

Returns a string slice with all prefixes and suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
assert_eq!("123foo1bar123".trim_matches(char::is_numeric), "foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1foo1barXX".trim_matches(|c| c == '1' || c == 'X'), "foo1bar");
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2424)

#### pub fn [trim\_start\_matches](#method.trim_start_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. `start` in this context means the first position of that byte string; for a left-to-right language like English or Russian, this will be left side, and for right-to-left languages like Arabic or Hebrew, this will be the right side.

##### Examples

```rust
assert_eq!("11foo1bar11".trim_start_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_start_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_start_matches(x), "foo1bar12");
```

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2458)

#### pub fn [strip\_prefix](#method.strip_prefix)<P>(&self, prefix: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Returns a string slice with the prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches"), this method removes the prefix exactly once.

If the string does not start with `prefix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("foo:bar".strip_prefix("foo:"), Some("bar"));
assert_eq!("foo:bar".strip_prefix("bar"), None);
assert_eq!("foofoo".strip_prefix("foo"), Some("foo"));
```

1.45.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2486-2488)

#### pub fn [strip\_suffix](#method.strip_suffix)<P>(&self, suffix: P) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with the suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix, wrapped in `Some`. Unlike [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes the suffix exactly once.

If the string does not end with `suffix`, returns `None`.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("bar:foo".strip_suffix(":foo"), Some("bar"));
assert_eq!("bar:foo".strip_suffix("bar"), None);
assert_eq!("foofoo".strip_suffix("foo"), Some("foo"));
```

1.98.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2520-2522)

#### pub fn [strip\_circumfix](#method.strip_circumfix)<P, S>(&self, prefix: P, suffix: S) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), S: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <S as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with the prefix and suffix removed.

If the string starts with the pattern `prefix` and ends with the pattern `suffix`, returns the substring after the prefix and before the suffix, wrapped in `Some`. Unlike [`trim_start_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start_matches "method str::trim_start_matches") and [`trim_end_matches`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end_matches "method str::trim_end_matches"), this method removes both the prefix and suffix exactly once.

If the string does not start with `prefix` or does not end with `suffix`, returns `None`.

Each [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
assert_eq!("bar:hello:foo".strip_circumfix("bar:", ":foo"), Some("hello"));
assert_eq!("bar:foo".strip_circumfix("foo", "foo"), None);
assert_eq!("foo:bar;".strip_circumfix("foo:", ';'), Some("bar"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2560)

#### pub fn [trim\_prefix](#method.trim_prefix)<P>(&self, prefix: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional prefix removed.

If the string starts with the pattern `prefix`, returns the substring after the prefix. Unlike [`strip_prefix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_prefix "method str::strip_prefix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not start with `prefix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

// Prefix present - removes it
assert_eq!("foo:bar".trim_prefix("foo:"), "bar");
assert_eq!("foofoo".trim_prefix("foo"), "foo");

// Prefix absent - returns original string
assert_eq!("foo:bar".trim_prefix("bar"), "foo:bar");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2597-2599)

#### pub fn [trim\_suffix](#method.trim_suffix)<P>(&self, suffix: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

🔬This is a nightly-only experimental API. (`trim_prefix_suffix`)

Returns a string slice with the optional suffix removed.

If the string ends with the pattern `suffix`, returns the substring before the suffix. Unlike [`strip_suffix`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.strip_suffix "method str::strip_suffix"), this method always returns `&str` for easy method chaining, instead of returning [`Option<&str>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option").

If the string does not end with `suffix`, returns the original string unchanged.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Examples

```rust
#![feature(trim_prefix_suffix)]

// Suffix present - removes it
assert_eq!("bar:foo".trim_suffix(":foo"), "bar");
assert_eq!("foofoo".trim_suffix("foo"), "foo");

// Suffix absent - returns original string
assert_eq!("bar:foo".trim_suffix("bar"), "bar:foo");

// Method chaining example
assert_eq!("<https://example.com/>".trim_prefix('<').trim_suffix('>'), "https://example.com/");
```

1.30.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2640-2642)

#### pub fn [trim\_end\_matches](#method.trim_end_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. `end` in this context means the last position of that byte string; for a left-to-right language like English or Russian, this will be right side, and for right-to-left languages like Arabic or Hebrew, this will be the left side.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_end_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_end_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_end_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1fooX".trim_end_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2684)

#### pub fn [trim\_left\_matches](#method.trim_left_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

👎Deprecated since 1.33.0:

superseded by `trim_start_matches`

Returns a string slice with all prefixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. ‘Left’ in this context means the first position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _right_ side, not the left.

##### Examples

```rust
assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
assert_eq!("123foo1bar123".trim_left_matches(char::is_numeric), "foo1bar123");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2727-2729)

#### pub fn [trim\_right\_matches](#method.trim_right_matches)<P>(&self, pat: P) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"), <P as [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern")\>::[Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher "type core::str::pattern::Pattern::Searcher")<'a>: for<'a> [ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html "trait core::str::pattern::ReverseSearcher")<'a>,

👎Deprecated since 1.33.0:

superseded by `trim_end_matches`

Returns a string slice with all suffixes that match a pattern repeatedly removed.

The [pattern](https://doc.rust-lang.org/nightly/core/str/pattern/index.html "mod core::str::pattern") can be a `&str`, [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char"), a slice of [`char`](https://doc.rust-lang.org/nightly/std/primitive.char.html "primitive char")s, or a function or closure that determines if a character matches.

##### Text directionality

A string is a sequence of bytes. ‘Right’ in this context means the last position of that byte string; for a language like Arabic or Hebrew which are ‘right to left’ rather than ‘left to right’, this will be the _left_ side, not the right.

##### Examples

Simple patterns:

```rust
assert_eq!("11foo1bar11".trim_right_matches('1'), "11foo1bar");
assert_eq!("123foo1bar123".trim_right_matches(char::is_numeric), "123foo1bar");

let x: &[_] = &['1', '2'];
assert_eq!("12foo1bar12".trim_right_matches(x), "12foo1bar");
```

A more complex pattern, using a closure:

```rust
assert_eq!("1fooX".trim_right_matches(|c| c == '1' || c == 'X'), "1foo");
```

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2778)

#### pub fn [parse](#method.parse)<F>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<F, <F as [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr")\>::[Err](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "type core::str::traits::FromStr::Err")\>

where F: [FromStr](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr"),

Parses this string slice into another type.

Because `parse` is so general, it can cause problems with type inference. As such, `parse` is one of the few times you’ll see the syntax affectionately known as the ‘turbofish’: `::<>`. This helps the inference algorithm understand specifically which type you’re trying to parse into.

`parse` can parse into any type that implements the [`FromStr`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html "trait core::str::traits::FromStr") trait.

##### Errors

Will return [`Err`](https://doc.rust-lang.org/nightly/core/str/traits/trait.FromStr.html#associatedtype.Err "associated type core::str::traits::FromStr::Err") if it’s not possible to parse this string slice into the desired type.

##### Examples

Basic usage:

```rust
let four: u32 = "4".parse().unwrap();

assert_eq!(4, four);
```

Using the ‘turbofish’ instead of annotating `four`:

```rust
let four = "4".parse::<u32>();

assert_eq!(Ok(4), four);
```

Failing to parse:

```rust
let nope = "j".parse::<u32>();

assert!(nope.is_err());
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2799)

#### pub fn [is\_ascii](#method.is_ascii)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if all characters in this string are within the ASCII range.

An empty string returns `true`.

##### Examples

```rust
let ascii = "hello!\n";
let non_ascii = "Grüße, Jürgen ❤";

assert!(ascii.is_ascii());
assert!(!non_ascii.is_ascii());
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2811)

#### pub fn [as\_ascii](#method.as_ascii)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&\[[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\]>

🔬This is a nightly-only experimental API. (`ascii_char`)

If this string slice [`is_ascii`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.is_ascii "method str::is_ascii"), returns it as a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), otherwise returns `None`.

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2825)

#### pub unsafe fn [as\_ascii\_unchecked](#method.as_ascii_unchecked)(&self) -> &\[[AsciiChar](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar")\]

🔬This is a nightly-only experimental API. (`ascii_char`)

Converts this string slice into a slice of [ASCII characters](https://doc.rust-lang.org/nightly/core/ascii/ascii_char/enum.AsciiChar.html "enum core::ascii::ascii_char::AsciiChar"), without checking whether they are valid.

##### Safety

Every character in this string must be ASCII, or else this is UB.

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2856)

#### pub fn [eq\_ignore\_ascii\_case](#method.eq_ignore_ascii_case)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks that two strings are an ASCII case-insensitive match.

Same as `to_ascii_lowercase(a) == to_ascii_lowercase(b)`, but without allocating and copying temporaries.

For Unicode-aware case-insensitive matching, consider [`str::eq_ignore_case_unnormalized`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.eq_ignore_case_unnormalized "method str::eq_ignore_case_unnormalized").

##### Examples

```rust
assert!("Ferris".eq_ignore_ascii_case("FERRIS"));
assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2907)

#### pub fn [eq\_ignore\_case\_unnormalized](#method.eq_ignore_case_unnormalized)(&self, other: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

🔬This is a nightly-only experimental API. (`casefold`)

Checks that two strings are a caseless match, according to [Definition 144](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G53513) in Chapter 3 of the Unicode Standard.

Same as `a.to_casefold_unnormalized() == b.to_casefold_unnormalized()`, but without allocating. See that method’s documentation, as well as [`char::to_casefold_unnormalized()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_casefold_unnormalized "method char::to_casefold_unnormalized"), for more information about case folding.

No [normalization](https://www.unicode.org/faq/normalization.html) (e.g. NFC) is performed, so visually and semantically identical strings might still compare unequal. For example, `"Å"` (U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE) is considered distinct from `"Å"` (A followed by U+030A COMBINING RING ABOVE), even though Unicode considers them canonically equivalent.

In addition, this method is independent of language/locale, so the special behavior of I/ı/İ/i in Turkish and Azeri is not handled.

##### Examples

```rust
#![feature(casefold)]
assert!("Ferris".eq_ignore_case_unnormalized("FERRIS"));
assert!("Ferrös".eq_ignore_case_unnormalized("FERRÖS"));
assert!("ẞ".eq_ignore_case_unnormalized("ss"));
```

No NFC [normalization](https://www.unicode.org/faq/normalization.html) is performed:

```rust
#![feature(casefold)]
// These two strings are visually and semantically identical...
let comp = "Å";
let decomp = "Å";

// ... but not codepoint-for-codepoint equal.
assert_eq!(comp, "\u{C5}");
assert_eq!(decomp, "A\u{030A}");

// Their case-foldings are likewise unequal:
assert!(!comp.eq_ignore_case_unnormalized(decomp));
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2935)

#### pub fn [make\_ascii\_uppercase](#method.make_ascii_uppercase)(&mut self)

Converts this string to its ASCII upper case equivalent in-place.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To return a new uppercased value without modifying the existing one, use [`to_ascii_uppercase()`](#method.to_ascii_uppercase).

##### Examples

```rust
let mut s = String::from("Grüße, Jürgen ❤");

s.make_ascii_uppercase();

assert_eq!("GRüßE, JüRGEN ❤", s);
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2963)

#### pub fn [make\_ascii\_lowercase](#method.make_ascii_lowercase)(&mut self)

Converts this string to its ASCII lower case equivalent in-place.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To return a new lowercased value without modifying the existing one, use [`to_ascii_lowercase()`](#method.to_ascii_lowercase).

##### Examples

```rust
let mut s = String::from("GRÜßE, JÜRGEN ❤");

s.make_ascii_lowercase();

assert_eq!("grÜße, jÜrgen ❤", s);
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#2991)

#### pub fn [trim\_ascii\_start](#method.trim_ascii_start)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim_start`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_start "method str::trim_start").

##### Examples

```rust
assert_eq!(" \t \u{3000}hello world\n".trim_ascii_start(), "\u{3000}hello world\n");
assert_eq!("  ".trim_ascii_start(), "");
assert_eq!("".trim_ascii_start(), "");
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3019)

#### pub fn [trim\_ascii\_end](#method.trim_ascii_end)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim_end`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim_end "method str::trim_end").

##### Examples

```rust
assert_eq!("\r hello world\u{3000}\n ".trim_ascii_end(), "\r hello world\u{3000}");
assert_eq!("  ".trim_ascii_end(), "");
assert_eq!("".trim_ascii_end(), "");
```

1.80.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3048)

#### pub fn [trim\_ascii](#method.trim_ascii)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a string slice with leading and trailing ASCII whitespace removed.

‘Whitespace’ refers to the definition used by [`u8::is_ascii_whitespace`](https://doc.rust-lang.org/nightly/std/primitive.u8.html#method.is_ascii_whitespace "method u8::is_ascii_whitespace"). Importantly, this definition excludes the U+000B code point even though it has the Unicode [`White_Space`](https://www.unicode.org/reports/tr44/#White_Space) property and is removed by [`str::trim`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.trim "method str::trim").

##### Examples

```rust
assert_eq!("\r hello world\n ".trim_ascii(), "hello world");
assert_eq!("  ".trim_ascii(), "");
assert_eq!("".trim_ascii(), "");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3091)

#### pub fn [escape\_debug](#method.escape_debug)(&self) -> [EscapeDebug](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html "struct core::str::iter::EscapeDebug")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_debug`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_debug "method char::escape_debug").

Note: only extended grapheme codepoints that begin the string will be escaped.

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_debug() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_debug());
```

Both are equivalent to:

```rust
println!("❤\\n!");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_debug().to_string(), "❤\\n!");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3137)

#### pub fn [escape\_default](#method.escape_default)(&self) -> [EscapeDefault](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html "struct core::str::iter::EscapeDefault")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_default`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_default "method char::escape_default").

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_default() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_default());
```

Both are equivalent to:

```rust
println!("\\u{{2764}}\\n!");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_default().to_string(), "\\u{2764}\\n!");
```

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3175)

#### pub fn [escape\_unicode](#method.escape_unicode)(&self) -> [EscapeUnicode](https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html "struct core::str::iter::EscapeUnicode")<'\_> [ⓘ](#)

Returns an iterator that escapes each char in `self` with [`char::escape_unicode`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.escape_unicode "method char::escape_unicode").

##### Examples

As an iterator:

```rust
for c in "❤\n!".escape_unicode() {
    print!("{c}");
}
println!();
```

Using `println!` directly:

```rust
println!("{}", "❤\n!".escape_unicode());
```

Both are equivalent to:

```rust
println!("\\u{{2764}}\\u{{a}}\\u{{21}}");
```

Using `to_string`:

```rust
assert_eq!("❤\n!".escape_unicode().to_string(), "\\u{2764}\\u{a}\\u{21}");
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3209)

#### pub fn [substr\_range](#method.substr_range)(&self, substr: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Range](https://doc.rust-lang.org/nightly/core/range/struct.Range.html "struct core::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>

🔬This is a nightly-only experimental API. (`substr_range`)

Returns the range that a substring points to.

Returns `None` if `substr` does not point within `self`.

Unlike [`str::find`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.find "method str::find"), **this does not search through the string**. Instead, it uses pointer arithmetic to find where in the string `substr` is derived from.

This is useful for extending [`str::split`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.split "method str::split") and similar methods.

Note that this method may return false positives (typically either `Some(0..0)` or `Some(self.len()..self.len())`) if `substr` is a zero-length `str` that points at the beginning or end of another, independent, `str`.

##### Examples

```rust
#![feature(substr_range)]
use core::range::Range;

let data = "a, b, b, a";
let mut iter = data.split(", ").map(|s| data.substr_range(s).unwrap());

assert_eq!(iter.next(), Some(Range { start: 0, end: 1 }));
assert_eq!(iter.next(), Some(Range { start: 3, end: 4 }));
assert_eq!(iter.next(), Some(Range { start: 6, end: 7 }));
assert_eq!(iter.next(), Some(Range { start: 9, end: 10 }));
```

[Source](https://doc.rust-lang.org/nightly/src/core/str/mod.rs.html#3220)

#### pub fn [as\_str](#method.as_str-1)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

🔬This is a nightly-only experimental API. (`str_as_str`)

Returns the same string as a string slice `&str`.

This method is redundant when used directly on `&str`, but it helps dereferencing other string-like types to string slices, for example references to `Box<str>` or `Arc<str>`.

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#308)

#### pub fn [replace](#method.replace)<P>(&self, from: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Available on **non-`no_global_oom_handling`** only.

Replaces all matches of a pattern with another string.

`replace` creates a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice.

##### Examples

```rust
let s = "this is old";

assert_eq!("this is new", s.replace("old", "new"));
assert_eq!("than an old", s.replace("is", "an"));
```

When the pattern doesn’t match, it returns this string slice as [`String`](../../prelude/struct.String.html "struct bevy::prelude::String"):

```rust
let s = "this is old";
assert_eq!(s, s.replace("cookie monster", "little lamb"));
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#366)

#### pub fn [replacen](#method.replacen)<P>(&self, pat: P, to: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), count: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

where P: [Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html "trait core::str::pattern::Pattern"),

Available on **non-`no_global_oom_handling`** only.

Replaces first N matches of a pattern with another string.

`replacen` creates a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String"), and copies the data from this string slice into it. While doing so, it attempts to find matches of a pattern. If it finds any, it replaces them with the replacement string slice at most `count` times.

##### Examples

```rust
let s = "foo foo 123 foo";
assert_eq!("new new 123 foo", s.replacen("foo", "new", 2));
assert_eq!("faa fao 123 foo", s.replacen('o', "a", 3));
assert_eq!("foo foo new23 foo", s.replacen(char::is_numeric, "new", 1));
```

When the pattern doesn’t match, it returns this string slice as [`String`](../../prelude/struct.String.html "struct bevy::prelude::String"):

```rust
let s = "this is old";
assert_eq!(s, s.replacen("cookie monster", "little lamb", 10));
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#433)

#### pub fn [to\_lowercase](#method.to_lowercase)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns the lowercase equivalent of this string slice, as a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String").

‘Lowercase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34432) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](../../prelude/struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Unlike [`char::to_lowercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_lowercase "method char::to_lowercase"), this method fully handles the context-dependent casing of Greek sigma. However, like that method, it does not handle locale-specific casing, like Turkish and Azeri I/ı/İ/i. See its documentation for more information.

##### Examples

Basic usage:

```rust
let s = "HELLO WORLD";

assert_eq!("hello world", s.to_lowercase());
```

Tricky examples, with sigma:

```rust
let sigma = "Σ";

assert_eq!("σ", sigma.to_lowercase());

// but at the end of a word, it's ς, not σ:
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());

let odysseus_king_of_ithaca = "Ο ΟΔΥΣΣΈΑΣ ΒΑΣΙΛΙΆΣ ΤΗΣ ΙΘΆΚΗΣ";

assert_eq!("ο οδυσσέας βασιλιάς της ιθάκης", odysseus_king_of_ithaca.to_lowercase());
```

Languages without case are not changed:

```rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_lowercase());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#554)

#### pub fn [word\_to\_titlecase](#method.word_to_titlecase)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`titlecase`)

Available on **non-`no_global_oom_handling`** only.

Returns the titlecase equivalent of this string slice, which is assumed to represent a single word, as a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String").

Essentially, this consists of uppercasing the first cased letter (with [`char::to_titlecase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_titlecase "method char::to_titlecase")), and lowercasing everything that follows.

‘Titlecase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34082) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](../../prelude/struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Unlike [`char::to_lowercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_lowercase "method char::to_lowercase"), this method fully handles the context-dependent casing of Greek sigma. However, like that method, it does not handle locale-specific casing, like Turkish and Azeri I/ı/İ/i. See its documentation for more information.

This method does not perform any kind of word segmentation.

##### Examples

Basic usage:

```rust
#![feature(titlecase)]
let s = "HELLO WORLD";

assert_eq!("Hello world", s.word_to_titlecase());
```

The first _cased_ letter is uppercased:

```rust
#![feature(titlecase)]
let the_night_before_christmas = "'twas";

assert_eq!("'Twas", the_night_before_christmas.word_to_titlecase());
```

Languages without case are not changed:

```rust
#![feature(titlecase)]
let new_year = "农历新年";

assert_eq!(new_year, new_year.word_to_titlecase());
```

Georgian uppercase (“Mtavruli”) letters are not used in titlecase:

```rust
#![feature(titlecase)]
let georgian = "ერთობაშია";

assert_eq!(georgian, georgian.word_to_titlecase());
```

No word segmentation is performed, so only the first cased letter in the whole string gets uppercased:

```rust
#![feature(titlecase)]
let blazingly_fast = "ferris and I";

assert_eq!("Ferris and i", blazingly_fast.word_to_titlecase());
```

Tricky examples, with sigma:

```rust
#![feature(titlecase)]
let odysseus = "ὈΔΥΣΣΕΎΣ";

assert_eq!("Ὀδυσσεύς", odysseus.word_to_titlecase());

let odysseus_king_of_ithaca = "Ο ΟΔΥΣΣΈΑΣ ΒΑΣΙΛΙΆΣ ΤΗΣ ΙΘΆΚΗΣ";

assert_eq!("Ο οδυσσέας βασιλιάς της ιθάκης", odysseus_king_of_ithaca.word_to_titlecase());
```

1.2.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#640)

#### pub fn [to\_uppercase](#method.to_uppercase)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns the uppercase equivalent of this string slice, as a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String").

‘Uppercase’ is defined according to the terms of [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G34431) of the Unicode standard.

Since some characters can expand into multiple characters when changing the case, this function returns a [`String`](../../prelude/struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

Like [`char::to_uppercase()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_uppercase "method char::to_uppercase") this method does not handle language-specific casing, like Turkish and Azeri I/ı/İ/i. See that method’s documentation for more information.

##### Examples

Basic usage:

```rust
let s = "hello world";

assert_eq!("HELLO WORLD", s.to_uppercase());
```

Scripts without case are not changed:

```rust
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_uppercase());
```

One character can become multiple:

```rust
let s = "tschüß";

assert_eq!("TSCHÜSS", s.to_uppercase());
```

[Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#742)

#### pub fn [to\_casefold\_unnormalized](#method.to_casefold_unnormalized)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

🔬This is a nightly-only experimental API. (`casefold`)

Available on **non-`no_global_oom_handling`** only.

Returns the case-folded equivalent of this string slice, as a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String").

Case folding is a transformation, mostly matching lowercase, that is meant to be used for case-insensitive string comparisons. Case-folded strings should not usually be exposed directly to users.

For the precise specification of case folding, see [Chapter 3 (Conformance)](https://www.unicode.org/versions/latest/core-spec/chapter-3/#G63737) of the Unicode standard.

Since some characters can expand into multiple characters when case folding, this function returns a [`String`](../../prelude/struct.String.html "struct bevy::prelude::String") instead of modifying the parameter in-place.

No [normalization](https://www.unicode.org/faq/normalization.html) (e.g. NFC) is performed, so visually and semantically identical strings might still casefold differently. For example, `"Å"` (U+00C5 LATIN CAPITAL LETTER A WITH RING ABOVE) is considered distinct from `"Å"` (A followed by U+030A COMBINING RING ABOVE), even though Unicode considers them canonically equivalent.

Like [`char::to_casefold_unnormalized()`](https://doc.rust-lang.org/nightly/std/primitive.char.html#method.to_casefold_unnormalized "method char::to_casefold_unnormalized") this method does not handle language-specific casing, like Turkish and Azeri I/ı/İ/i. See that method’s documentation for more information.

##### Examples

Basic usage:

```rust
#![feature(casefold)]
let s0 = "HELLO";
let s1 = "Hello";

assert_eq!(s0.to_casefold_unnormalized(), s1.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), "hello")
```

Scripts without case are not changed:

```rust
#![feature(casefold)]
let new_year = "农历新年";

assert_eq!(new_year, new_year.to_casefold_unnormalized());
```

One character can become multiple:

```rust
#![feature(casefold)]
let s0 = "TSCHÜẞ";
let s1 = "TSCHÜSS";
let s2 = "tschüß";

assert_eq!(s0.to_casefold_unnormalized(), s1.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), s2.to_casefold_unnormalized());
assert_eq!(s0.to_casefold_unnormalized(), "tschüss");
```

No NFC [normalization](https://www.unicode.org/faq/normalization.html) is performed:

```rust
#![feature(casefold)]
// These two strings are visually and semantically identical...
let comp = "Å";
let decomp = "Å";

// ... but not codepoint-for-codepoint equal.
assert_eq!(comp, "\u{C5}");
assert_eq!(decomp, "A\u{030A}");

// Their case-foldings are likewise unequal:
assert_eq!(comp.to_casefold_unnormalized(), "\u{E5}");
assert_eq!(decomp.to_casefold_unnormalized(), "a\u{030A}");
```

1.16.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#808)

#### pub fn [repeat](#method.repeat)(&self, n: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Creates a new [`String`](../../prelude/struct.String.html "struct bevy::prelude::String") by repeating a string `n` times.

##### Panics

This function will panic if the capacity would overflow.

##### Examples

Basic usage:

```rust
assert_eq!("abc".repeat(4), String::from("abcabcabcabc"));
```

A panic upon overflow:

[ⓘ](# "This example panics")

```rust
// this will panic at runtime
let huge = "0123456789abcdef".repeat(usize::MAX);
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#838)

#### pub fn [to\_ascii\_uppercase](#method.to_ascii_uppercase)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII upper case equivalent.

ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters are unchanged.

To uppercase the value in-place, use [`make_ascii_uppercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_uppercase "method str::make_ascii_uppercase").

To uppercase ASCII characters in addition to non-ASCII characters, use [`to_uppercase`](#method.to_uppercase).

##### Examples

```rust
let s = "Grüße, Jürgen ❤";

assert_eq!("GRüßE, JüRGEN ❤", s.to_ascii_uppercase());
```

1.23.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#870)

#### pub fn [to\_ascii\_lowercase](#method.to_ascii_lowercase)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Available on **non-`no_global_oom_handling`** only.

Returns a copy of this string where each character is mapped to its ASCII lower case equivalent.

ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters are unchanged.

To lowercase the value in-place, use [`make_ascii_lowercase`](https://doc.rust-lang.org/nightly/std/primitive.str.html#method.make_ascii_lowercase "method str::make_ascii_lowercase").

To lowercase ASCII characters in addition to non-ASCII characters, use [`to_lowercase`](#method.to_lowercase).

##### Examples

```rust
let s = "Grüße, Jürgen ❤";

assert_eq!("grüße, jürgen ❤", s.to_ascii_lowercase());
```

## Trait Implementations

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

where [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

**Required Components**: [`TextFont`](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont"), [`TextColor`](../../prelude/struct.TextColor.html "struct bevy::prelude::TextColor"), [`LineHeight`](../enum.LineHeight.html "enum bevy::text::LineHeight"), [`LetterSpacing`](../enum.LetterSpacing.html "enum bevy::text::LetterSpacing").

A component’s Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### const [STORAGE\_TYPE](../../prelude/trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::Table

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### type [Mutability](../../prelude/trait.Component.html#associatedtype.Mutability) = [Mutable](../../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](../../prelude/trait.Component.html "trait bevy::prelude::Component"). [Read more](../../prelude/trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [register\_required\_components](../../prelude/trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](../../prelude/trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [clone\_behavior](../../prelude/trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](../../prelude/trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [relationship\_accessor](../../prelude/trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>>

Returns [`ComponentRelationshipAccessor`](../../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](../../prelude/trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](../../prelude/trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](../../prelude/trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](../../prelude/trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](../../prelude/trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](../../prelude/trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](../../prelude/trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](../../prelude/trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [String](../../prelude/struct.String.html "struct bevy::prelude::String")

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#211)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#212)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#217)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[String](../../prelude/struct.String.html "struct bevy::prelude::String")\> for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#218)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(value: [String](../../prelude/struct.String.html "struct bevy::prelude::String")) -> [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

Converts to this type from the input type.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### type [This](../../reflect/func/args/trait.FromArg.html#associatedtype.This)<'from\_arg> = [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

The type to convert into. [Read more](../../reflect/func/args/trait.FromArg.html#associatedtype.This)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [from\_arg](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)(arg: [Arg](../../reflect/func/args/struct.Arg.html "struct bevy::reflect::func::args::Arg")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan") as [FromArg](../../reflect/func/args/trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](../../reflect/func/args/trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'\_>, [ArgError](../../reflect/func/enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Creates an item from an argument. [Read more](../../reflect/func/args/trait.FromArg.html#tymethod.from_arg)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [from\_reflect](../../prelude/trait.FromReflect.html#tymethod.from_reflect)(reflect: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>

Constructs a concrete instance of `Self` from a reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#43-45)

#### fn [take\_from\_reflect](../../prelude/trait.FromReflect.html#method.take_from_reflect)( reflect: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to downcast the given value to `Self` using, constructing the value using [`from_reflect`](../../prelude/trait.FromReflect.html#tymethod.from_reflect "associated function bevy::prelude::FromReflect::from_reflect") if that fails. [Read more](../../prelude/trait.FromReflect.html#method.take_from_reflect)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [GetOwnership](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [ownership](../../reflect/func/args/trait.GetOwnership.html#method.ownership)() -> [Ownership](../../reflect/func/args/enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

Returns the ownership of [`Self`](../../reflect/func/args/trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership").

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [get\_type\_registration](../../reflect/trait.GetTypeRegistration.html#tymethod.get_type_registration)() -> [TypeRegistration](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")

Returns the default [`TypeRegistration`](../../reflect/struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") for this type.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [register\_type\_dependencies](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)(registry: &mut [TypeRegistry](../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"))

Registers other types needed by this type. [Read more](../../reflect/trait.GetTypeRegistration.html#method.register_type_dependencies)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [IntoReturn](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [into\_return](../../reflect/func/trait.IntoReturn.html#tymethod.into_return)<'into\_return>(self) -> [Return](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return")<'into\_return>

where [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan"): 'into\_return,

Converts [`Self`](../../reflect/func/trait.IntoReturn.html "trait bevy::reflect::func::IntoReturn") into a [`Return`](../../reflect/func/enum.Return.html "enum bevy::reflect::func::Return") value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [get\_represented\_type\_info](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")\>

Returns the [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") of the type _represented_ by this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.get_represented_type_info)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [try\_apply](../../prelude/trait.PartialReflect.html#tymethod.try_apply)( &mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ApplyError](../../reflect/enum.ApplyError.html "enum bevy::reflect::ApplyError")\>

Tries to [`apply`](../../prelude/trait.PartialReflect.html#method.apply "method bevy::prelude::PartialReflect::apply") a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.try_apply)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [reflect\_kind](../../prelude/trait.PartialReflect.html#method.reflect_kind)(&self) -> [ReflectKind](../../reflect/enum.ReflectKind.html "enum bevy::reflect::ReflectKind")

Returns a zero-sized enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_kind)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [reflect\_ref](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)(&self) -> [ReflectRef](../../reflect/enum.ReflectRef.html "enum bevy::reflect::ReflectRef")<'\_>

Returns an immutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_ref)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)(&mut self) -> [ReflectMut](../../reflect/enum.ReflectMut.html "enum bevy::reflect::ReflectMut")<'\_>

Returns a mutable enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_mut)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [reflect\_owned](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>) -> [ReflectOwned](../../reflect/enum.ReflectOwned.html "enum bevy::reflect::ReflectOwned")

Returns an owned enumeration of “kinds” of type. [Read more](../../prelude/trait.PartialReflect.html#tymethod.reflect_owned)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [try\_into\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_into_reflect)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Attempts to cast this type to a boxed, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [try\_as\_reflect](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [try\_as\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.try_as_reflect_mut)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)>

Attempts to cast this type to a mutable, [fully-reflected](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [into\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Casts this type to a boxed, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.into_partial_reflect)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [as\_partial\_reflect](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)(&self) -> &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [as\_partial\_reflect\_mut](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)(&mut self) -> &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)

Casts this type to a mutable, reflected value. [Read more](../../prelude/trait.PartialReflect.html#tymethod.as_partial_reflect_mut)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [reflect\_partial\_eq](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

Returns a “partial equality” comparison result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [reflect\_partial\_cmp](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering")\>

Returns a “partial comparison” result. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_partial_cmp)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#191)

#### fn [debug](../../prelude/trait.PartialReflect.html#method.debug)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Debug formatter for the value. [Read more](../../prelude/trait.PartialReflect.html#method.debug)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#191)

#### fn [reflect\_clone](../../prelude/trait.PartialReflect.html#method.reflect_clone)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

Attempts to clone `Self` using reflection. [Read more](../../prelude/trait.PartialReflect.html#method.reflect_clone)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#206)

#### fn [apply](../../prelude/trait.PartialReflect.html#method.apply)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Applies a reflected value to this value. [Read more](../../prelude/trait.PartialReflect.html#method.apply)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#277)

#### fn [to\_dynamic](../../prelude/trait.PartialReflect.html#method.to_dynamic)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Converts this reflected value into its dynamic representation based on its [kind](../../prelude/trait.PartialReflect.html#method.reflect_kind "method bevy::prelude::PartialReflect::reflect_kind"). [Read more](../../prelude/trait.PartialReflect.html#method.to_dynamic)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#321-323)

#### fn [reflect\_clone\_and\_take](../../prelude/trait.PartialReflect.html#method.reflect_clone_and_take)<T>(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ReflectCloneError](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError")\>

where T: 'static, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

For a type implementing [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), combines `reflect_clone` and `take` in a useful fashion, automatically constructing an appropriate [`ReflectCloneError`](../../reflect/enum.ReflectCloneError.html "enum bevy::reflect::ReflectCloneError") if the downcast fails.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#336)

#### fn [reflect\_hash](../../prelude/trait.PartialReflect.html#method.reflect_hash)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

Returns a hash of the value (which includes the type). [Read more](../../prelude/trait.PartialReflect.html#method.reflect_hash)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#391)

#### fn [is\_dynamic](../../prelude/trait.PartialReflect.html#method.is_dynamic)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Indicates whether or not this type is a _dynamic_ type. [Read more](../../prelude/trait.PartialReflect.html#method.is_dynamic)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [into\_any](../../prelude/trait.Reflect.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.into_any)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [as\_any](../../prelude/trait.Reflect.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [as\_any\_mut](../../prelude/trait.Reflect.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"). [Read more](../../prelude/trait.Reflect.html#tymethod.as_any_mut)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [into\_reflect](../../prelude/trait.Reflect.html#tymethod.into_reflect)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")\>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [as\_reflect](../../prelude/trait.Reflect.html#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [as\_reflect\_mut](../../prelude/trait.Reflect.html#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [set](../../prelude/trait.Reflect.html#tymethod.set)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value. [Read more](../../prelude/trait.Reflect.html#tymethod.set)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#202)

### impl [TextSection](../trait.TextSection.html "trait bevy::text::TextSection") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#203)

#### fn [get\_text](../trait.TextSection.html#tymethod.get_text)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the text for this section.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#206)

#### fn [get\_text\_mut](../trait.TextSection.html#tymethod.get_text_mut)(&mut self) -> &mut [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Returns a mutable reference to the text for this section.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [TupleStruct](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [field](../../prelude/trait.TupleStruct.html#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field with index `index` as a `&dyn Reflect`.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [field\_mut](../../prelude/trait.TupleStruct.html#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field with index `index` as a `&mut dyn Reflect`.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [field\_len](../../prelude/trait.TupleStruct.html#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the tuple struct.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [iter\_fields](../../prelude/trait.TupleStruct.html#tymethod.iter_fields)(&self) -> [TupleStructFieldIter](../../reflect/tuple_struct/struct.TupleStructFieldIter.html "struct bevy::reflect::tuple_struct::TupleStructFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the tuple struct’s fields.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [to\_dynamic\_tuple\_struct](../../prelude/trait.TupleStruct.html#method.to_dynamic_tuple_struct)(&self) -> [DynamicTupleStruct](../../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

Creates a new [`DynamicTupleStruct`](../../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct") from this tuple struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#71)

#### fn [get\_represented\_tuple\_struct\_info](../../prelude/trait.TupleStruct.html#method.get_represented_tuple_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TupleStructInfo](../../reflect/tuple_struct/struct.TupleStructInfo.html "struct bevy::reflect::tuple_struct::TupleStructInfo")\>

Will return `None` if [`TypeInfo`](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [type\_path](../../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [short\_type\_path](../../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [type\_ident](../../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [crate\_name](../../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [module\_path](../../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

#### fn [type\_info](../../reflect/trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Auto Trait Implementations

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

### impl [RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

### impl [UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [TextSpan](../../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](../../prelude/trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](../../prelude/trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

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

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#722)

### impl<T> [ErasedBundleTemplate](../../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#723)

#### unsafe fn [apply](../../scene/trait.ErasedBundleTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../../scene/trait.ErasedBundleTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#729)

#### fn [clone\_template](../../scene/trait.ErasedBundleTemplate.html#tymethod.clone_template)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedBundleTemplate](../../scene/trait.ErasedBundleTemplate.html "trait bevy::scene::ErasedBundleTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#686)

### impl<T> [ErasedComponentTemplate](../../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate") for T

where T: [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <T as [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")\>::[Output](../../prelude/trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"): [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#687-691)

#### unsafe fn [apply](../../scene/trait.ErasedComponentTemplate.html#tymethod.apply)( &self, context: &mut [TemplateContext](../../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, bundle\_writer: &mut [BundleWriter](../../ecs/bundle/struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Applies this template to the given `entity`. [Read more](../../scene/trait.ErasedComponentTemplate.html#tymethod.apply)

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/resolved_scene.rs.html#701)

#### fn [clone\_template](../../scene/trait.ErasedComponentTemplate.html#tymethod.clone_template)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [ErasedComponentTemplate](../../scene/trait.ErasedComponentTemplate.html "trait bevy::scene::ErasedComponentTemplate")\>

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#207)

### impl<S> [GetTupleStructField](../../prelude/trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField") for S

where S: [TupleStruct](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#208)

#### fn [get\_field](../../prelude/trait.GetTupleStructField.html#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#213)

#### fn [get\_field\_mut](../../prelude/trait.GetTupleStructField.html#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](../../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../../reflect/trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#203-206)

### impl<T> [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source") for T

where T: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), <T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target"): [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source"),

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#208)

#### type [Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice)<'a> = <<T as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target") as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'a> where T: 'a

A type this `Source` can be sliced into.

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#213)

#### fn [len](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Length of the source

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#217-219)

#### fn [read](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)<'a, Chunk>(&'a self, offset: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Chunk>

where Chunk: [Chunk](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Chunk.html "trait logos::source::Chunk")<'a>,

Read a chunk of bytes into an array. Returns `None` when reading out of bounds would occur. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.read)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#224)

#### fn [slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)(&self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>>

Get a slice of the source at given range. This is analogous to `slice::get(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#229)

#### unsafe fn [slice\_unchecked](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)( &self, range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>, ) -> <T as [Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html "trait logos::source::Source")\>::[Slice](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#associatedtype.Slice "type logos::source::Source::Slice")<'\_>

Available on **non-crate feature `forbid_unsafe`** only.

Get a slice of the source at given range. This is analogous to `slice::get_unchecked(range)`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.slice_unchecked)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#233)

#### fn [is\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Check if `index` is valid for this `Source`, that is: [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#tymethod.is_boundary)

[Source](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/src/logos/source.rs.html#237)

#### fn [find\_boundary](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

For `&str` sources attempts to find the closest `char` boundary at which source can be sliced, starting from `index`. [Read more](https://docs.rs/logos/0.16.1/x86_64-unknown-linux-gnu/logos/source/trait.Source.html#method.find_boundary)

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

{"&\[u8\]":"<h3>Notable traits for <code>&amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for &amp;\[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","&mut Vec<u8>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../prelude/struct.Vec.html\\" title=\\"struct bevy::prelude::Vec\\">Vec</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/alloc/trait.Allocator.html\\" title=\\"trait core::alloc::Allocator\\">Allocator</a>,</div></div>","&mut \[u8\]":"<h3>Notable traits for <code>&amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for &amp;mut \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\]</div>","Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Bytes<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html\\" title=\\"struct core::str::iter::Bytes\\">Bytes</a>&lt;'\_&gt;</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Bytes.html\\" title=\\"struct core::str::iter::Bytes\\">Bytes</a>&lt;'\_&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>;</div>","CharIndices<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html\\" title=\\"struct core::str::iter::CharIndices\\">CharIndices</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.CharIndices.html\\" title=\\"struct core::str::iter::CharIndices\\">CharIndices</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>);</div>","Chars<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html\\" title=\\"struct core::str::iter::Chars\\">Chars</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Chars.html\\" title=\\"struct core::str::iter::Chars\\">Chars</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Drain<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/string/struct.Drain.html\\" title=\\"struct alloc::string::Drain\\">Drain</a>&lt;'\_&gt;</code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/string/struct.Drain.html\\" title=\\"struct alloc::string::Drain\\">Drain</a>&lt;'\_&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","EncodeUtf16<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html\\" title=\\"struct core::str::iter::EncodeUtf16\\">EncodeUtf16</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EncodeUtf16.html\\" title=\\"struct core::str::iter::EncodeUtf16\\">EncodeUtf16</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\\">u16</a>;</div>","EscapeDebug<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html\\" title=\\"struct core::str::iter::EscapeDebug\\">EscapeDebug</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDebug.html\\" title=\\"struct core::str::iter::EscapeDebug\\">EscapeDebug</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","EscapeDefault<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html\\" title=\\"struct core::str::iter::EscapeDefault\\">EscapeDefault</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeDefault.html\\" title=\\"struct core::str::iter::EscapeDefault\\">EscapeDefault</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","EscapeUnicode<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html\\" title=\\"struct core::str::iter::EscapeUnicode\\">EscapeUnicode</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.EscapeUnicode.html\\" title=\\"struct core::str::iter::EscapeUnicode\\">EscapeUnicode</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.char.html\\">char</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Lines<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html\\" title=\\"struct core::str::iter::Lines\\">Lines</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Lines.html\\" title=\\"struct core::str::iter::Lines\\">Lines</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","LinesAny<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html\\" title=\\"struct core::str::iter::LinesAny\\">LinesAny</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.LinesAny.html\\" title=\\"struct core::str::iter::LinesAny\\">LinesAny</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","MatchIndices<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html\\" title=\\"struct core::str::iter::MatchIndices\\">MatchIndices</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.MatchIndices.html\\" title=\\"struct core::str::iter::MatchIndices\\">MatchIndices</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>);</div>","Matches<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html\\" title=\\"struct core::str::iter::Matches\\">Matches</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Matches.html\\" title=\\"struct core::str::iter::Matches\\">Matches</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RMatchIndices<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html\\" title=\\"struct core::str::iter::RMatchIndices\\">RMatchIndices</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatchIndices.html\\" title=\\"struct core::str::iter::RMatchIndices\\">RMatchIndices</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>);</div>","RMatches<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html\\" title=\\"struct core::str::iter::RMatches\\">RMatches</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RMatches.html\\" title=\\"struct core::str::iter::RMatches\\">RMatches</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplit<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html\\" title=\\"struct core::str::iter::RSplit\\">RSplit</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplit.html\\" title=\\"struct core::str::iter::RSplit\\">RSplit</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplitN<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html\\" title=\\"struct core::str::iter::RSplitN\\">RSplitN</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitN.html\\" title=\\"struct core::str::iter::RSplitN\\">RSplitN</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","RSplitTerminator<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html\\" title=\\"struct core::str::iter::RSplitTerminator\\">RSplitTerminator</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.RSplitTerminator.html\\" title=\\"struct core::str::iter::RSplitTerminator\\">RSplitTerminator</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,\\n &lt;P as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html#associatedtype.Searcher\\" title=\\"type core::str::pattern::Pattern::Searcher\\">Searcher</a>&lt;'a&gt;: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html\\" title=\\"trait core::str::pattern::ReverseSearcher\\">ReverseSearcher</a>&lt;'a&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","Split<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html\\" title=\\"struct core::str::iter::Split\\">Split</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.Split.html\\" title=\\"struct core::str::iter::Split\\">Split</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitAsciiWhitespace<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html\\" title=\\"struct core::str::iter::SplitAsciiWhitespace\\">SplitAsciiWhitespace</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitAsciiWhitespace.html\\" title=\\"struct core::str::iter::SplitAsciiWhitespace\\">SplitAsciiWhitespace</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitInclusive<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html\\" title=\\"struct core::str::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitInclusive.html\\" title=\\"struct core::str::iter::SplitInclusive\\">SplitInclusive</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitN<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html\\" title=\\"struct core::str::iter::SplitN\\">SplitN</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitN.html\\" title=\\"struct core::str::iter::SplitN\\">SplitN</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitTerminator<'\_, P>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html\\" title=\\"struct core::str::iter::SplitTerminator\\">SplitTerminator</a>&lt;'a, P&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, P&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitTerminator.html\\" title=\\"struct core::str::iter::SplitTerminator\\">SplitTerminator</a>&lt;'a, P&gt;<div class=\\"where\\">where\\n P: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html\\" title=\\"trait core::str::pattern::Pattern\\">Pattern</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","SplitWhitespace<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html\\" title=\\"struct core::str::iter::SplitWhitespace\\">SplitWhitespace</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/str/iter/struct.SplitWhitespace.html\\" title=\\"struct core::str::iter::SplitWhitespace\\">SplitWhitespace</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>;</div>","TupleStructFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}